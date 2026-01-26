use anyhow::{Result, anyhow};
use chrono::{DateTime, TimeDelta, Utc};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::{Dictionary, format::Pixel};

use opencv::{
    core::{self, CV_8UC3},
    dnn,
    prelude::*,
};
use serde::Serialize;
use similari::prelude::{SortTrack, Universal2DBox};
use similari::trackers::sort::PositionalMetricType::IoU;
use similari::trackers::sort::simple_api::Sort;
use std::borrow::BorrowMut;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use std::{ops::AddAssign, thread::sleep};

mod api_client;
mod api_types;

use crate::api_types::*;

const STRIDE: i32 = 2;
const CONF_THRESHOLD: f32 = 0.2;
const NMS_THRESHOLD: f32 = 0.5;
const BBOX_HISTORY: usize = 100;
const MAX_IDLE_EPOCHS: usize = 100;
const IOU_THRESHOLD: f32 = 0.1;
const SORT_CONF_THRESHOLD: f32 = 0.2;
// const MODEL_DIMENSION: i32 = 416;
const MODEL_DIMENSION: i32 = 640;

#[derive(Default, Debug)]
struct FrameTask {
    frame_index: i32,
    timestamp: DateTime<Utc>,
    class_ids: core::Vector<i32>,
    confidences: core::Vector<f32>,
    raw_inferencing_bboxes: core::Vector<core::Rect_<i32>>,
    inferencing_bboxes: Vec<core::Rect_<i32>>,
    tracking_bboxes: Vec<(Universal2DBox, Option<i64>)>,
    tracks: Vec<SortTrack>,
    tracks_by_id: Arc<Mutex<HashMap<u64, Vec<SortTrack>>>>,
}

impl FrameTask {
    pub fn new() -> FrameTask {
        FrameTask {
            ..Default::default()
        }
    }
}

#[derive(Default, Serialize, Debug)]
struct DetectionSummary {
    class_name: String,
    weighted_score: f64,
    average_score: f64,
    detected_frame_count: i64,
    handled_frame_count: i64,
}

fn main() -> Result<()> {
    let api_url = std::env::var("API_URL").unwrap_or("".to_string());
    if api_url.is_empty() {
        return Err(anyhow!("API_URL env var empty or unset"));
    }

    let raw_source_path = std::env::var("SOURCE_PATH").unwrap_or("".to_string());
    if raw_source_path.is_empty() {
        return Err(anyhow!("SOURCE_PATH env var empty or unset"));
    }
    let source_path = Path::new(&raw_source_path);
    if !source_path.exists() {
        return Err(anyhow!(format!(
            "SOURCE_PATH={raw_source_path} does not exist"
        )));
    }

    let one_shot_file_name = std::env::var("ONE_SHOT_FILE_NAME").unwrap_or("".to_string());

    println!("main() entered");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let client = api_client::DjangolangClient {
        client: reqwest::Client::new(),
        base_url: reqwest::Url::parse(&api_url)?,
    };

    let mut vid_startup_duration = Duration::default();
    let mut decoding_duration = Duration::default();
    let mut processing_duration = Duration::default();
    let load_image_duration = Duration::default();
    let mut scale_frame_duration = Duration::default();
    let mut load_scaled_image_duration = Duration::default();
    let mut inferencing_duration = Duration::default();
    let mut tracking_duration = Duration::default();
    let mut total_frame_duration = Duration::default();
    let mut total_file_duration = Duration::default();
    let mut api_duration = Duration::default();

    // let config = "people-r-people.cfg";
    // let weights = "people-r-people.weights";
    // let names = "people-r-people.names";

    // let config = "yolov4-tiny.cfg";
    // let weights = "yolov4-tiny.weights";
    // let names = "coco.names";

    // let config = "yolov7-tiny.cfg";
    // let weights = "yolov7-tiny.weights";
    // let names = "coco.names";

    let config = "yolov7.cfg";
    let weights = "yolov7.weights";
    let names = "coco.names";

    println!("config: {config}");
    println!("weights: {weights}");
    println!("names: {names}");

    let raw_names = fs::read_to_string(names)?;

    let mut name_by_class_id = HashMap::<i64, String>::new();
    for (i, line) in raw_names.trim().split("\n").enumerate() {
        name_by_class_id.insert(i as i64, line.trim().into());
    }

    let net = dnn::read_net(weights, config, "Darknet")?;
    let mut model = dnn::DetectionModel::new_1(&net)?;

    model.set_preferable_backend(dnn::Backend::DNN_BACKEND_CUDA)?;
    model.set_preferable_target(dnn::Target::DNN_TARGET_CUDA)?;

    let scale: f64 = 1.0 / 255.0;

    let size = core::Size {
        width: MODEL_DIMENSION,
        height: MODEL_DIMENSION,
    };

    let mean = core::Scalar {
        0: [0.0, 0.0, 0.0, 0.0],
    };

    let swap_rb: bool = false;
    let crop: bool = false;

    model.set_input_params(scale, size, mean, swap_rb, crop)?;

    let mut tracker = Sort::new(
        1,
        BBOX_HISTORY,
        MAX_IDLE_EPOCHS,
        IoU(IOU_THRESHOLD),
        SORT_CONF_THRESHOLD,
        None,
    );

    let mut total_boxes = 0;

    loop {
        let file_before_all = Instant::now();

        let mut frame_tasks: Vec<FrameTask> = vec![];

        //
        // figure out the video to handle
        //

        let mut file_name: String = String::new();
        let mut possible_video: Option<Video> = None;

        let res: GetVideoResponse = if !one_shot_file_name.is_empty() {
            println!(
                "\ntrying to find video with file_name of ONE_SHOT_FILE_NAME={:?}...",
                one_shot_file_name
            );

            let req = GetVideosRequest {
                file_name_eq: Some(one_shot_file_name.clone()),
                ..Default::default()
            };

            rt.block_on(client.get_videos(req))?
        } else {
            println!("\ntrying to claim video...");

            let req = PostObjectDetectorClaimVideosRequest {
                body: PostObjectDetectorClaimVideosRequestBody {
                    timeout_seconds: Some(10.0),
                    until: Some(Utc::now() + Duration::from_secs(70)),
                },
                status_eq: Some("needs detection".to_string()),
                ..Default::default()
            };

            rt.block_on(client.post_object_detector_claim_videos(req))?
        };

        match res {
            GetVideoResponse::Ok(res) => {
                if let Some(objects) = res.objects
                    && !objects.is_empty()
                    && let Some(object_file_name) = &objects[0].file_name
                {
                    file_name = object_file_name.clone();
                    possible_video = Some(objects[0].clone());
                }
            }
            GetVideoResponse::Unknown(_res) => {
                sleep(Duration::from_secs(1));
                continue;
            }
        }

        if file_name.is_empty() || possible_video.is_none() {
            println!(
                "warning: file_name unexpectedly empty / video unexpectedly none; probably nothing to claim"
            );

            sleep(Duration::from_secs(1));

            continue;
        }

        let video = possible_video.unwrap();

        if video.started_at.is_none() {
            return Err(anyhow!("video.started_at unexpectedly none"));
        }

        let video_started_at = video.started_at.unwrap();

        let file_path = source_path.join(Path::new(&file_name));

        if !file_path.exists() {
            return Err(anyhow!(format!(
                "file_path={:?} does not exist",
                file_path.to_str().unwrap_or(""),
            )));
        }

        println!("\nvideo: {:?}", video);

        {
            let req = PatchVideoRequest {
                primary_key: video.id.unwrap(),
                body: PatchVideoRequestBody {
                    status: Some("detecting".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            };

            let _res = rt.block_on(client.patch_video(req))?;
        }

        //
        // handle the video
        //

        let mut options = Dictionary::new();

        options.set("hwaccel", "cuda");
        options.set("hwaccel_output_format", "cuda");

        let mut ictx = ffmpeg::format::input_with_dictionary(&file_path, options)?;

        println!("\nopened {:?}", file_path);

        let vid_startup_before = Instant::now();

        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;

        let video_stream_index = input.index();

        let mut decoder = match ffmpeg::codec::decoder::find_by_name("h264_cuvid") {
            Some(codec) => {
                println!("\nUsing CUDA hardware decoder (h264_cuvid)");
                let mut ctx = ffmpeg::codec::context::Context::new_with_codec(codec);
                ctx.set_parameters(input.parameters())?;
                ctx.decoder().video()?
            }
            None => {
                return Err(anyhow!(
                    "failed to get CUDA hardware decoder; cannot continue!"
                ));
            }
        };

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::BGR24,
            MODEL_DIMENSION as u32,
            MODEL_DIMENSION as u32,
            Flags::FAST_BILINEAR,
        )?;

        let frame_rate = match decoder.frame_rate() {
            Some(frame_rate) => (frame_rate.numerator() / frame_rate.denominator()) as i64,
            None => 20,
        };

        println!("frame_rate: {frame_rate}");

        println!("decoder.width(): {:?}", decoder.width());
        println!("decoder.height(): {:?}", decoder.height());

        println!("MODEL_DIMENSION: {:?}", MODEL_DIMENSION);

        let scale_x = decoder.width() as f32 / MODEL_DIMENSION as f32;
        let scale_y = decoder.height() as f32 / MODEL_DIMENSION as f32;

        println!("scale_x: {scale_x}");
        println!("scale_y: {scale_y}");

        let mut frame_index = 0;
        let mut handled_frame_count = 0;
        let _rgb_frame = ffmpeg::frame::Video::empty();
        let mut rgb_frame_scaled = ffmpeg::frame::Video::empty();

        let vid_startup_after = Instant::now();
        vid_startup_duration.add_assign(vid_startup_after - vid_startup_before);

        let mut decoded: ffmpeg::frame::Video = ffmpeg::frame::Video::empty();
        let _img: Mat = Mat::default();
        let mut img_scaled = Mat::default();

        let tracks_by_id: Arc<Mutex<HashMap<u64, Vec<SortTrack>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<()> {
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let decoding_before_all = Instant::now();

                    #[allow(clippy::modulo_one)]
                    if frame_index % STRIDE == 0 {
                        let mut frame_task = FrameTask::new();

                        frame_task.frame_index = frame_index;

                        frame_task.tracks_by_id = tracks_by_id.clone();

                        let frame_timestamp_millis =
                            (frame_index as f64 * 1000.0 / frame_rate as f64) as i64;

                        frame_task.timestamp =
                            video_started_at + TimeDelta::milliseconds(frame_timestamp_millis);

                        let before = Instant::now();

                        scaler.run(&decoded, &mut rgb_frame_scaled)?;

                        let after = Instant::now();
                        scale_frame_duration.add_assign(after - before);

                        // more performant than the safe variant (which implies a copy)
                        let before = Instant::now();
                        img_scaled = unsafe {
                            Mat::new_rows_cols_with_data_unsafe(
                                MODEL_DIMENSION,
                                MODEL_DIMENSION,
                                CV_8UC3,
                                rgb_frame_scaled.data(0).as_ptr() as *mut _,
                                rgb_frame_scaled.stride(0),
                            )?
                        };
                        let after = Instant::now();
                        load_scaled_image_duration.add_assign(after - before);

                        //
                        // handle inferencing
                        //

                        let before = Instant::now();
                        model.detect(
                            &img_scaled,
                            &mut frame_task.class_ids,
                            &mut frame_task.confidences,
                            &mut frame_task.raw_inferencing_bboxes,
                            CONF_THRESHOLD,
                            NMS_THRESHOLD,
                        )?;
                        total_boxes += frame_task.raw_inferencing_bboxes.len();
                        let after = Instant::now();
                        inferencing_duration.add_assign(after - before);

                        let before = Instant::now();
                        for (i, scaled_b) in frame_task.raw_inferencing_bboxes.iter().enumerate() {
                            let cid = frame_task.class_ids.get(i)?;

                            let mut b = scaled_b;

                            b.x = (b.x as f32 * scale_x) as i32;
                            b.y = (b.y as f32 * scale_y) as i32;
                            b.width = (b.width as f32 * scale_x) as i32;
                            b.height = (b.height as f32 * scale_y) as i32;

                            let cf = frame_task.confidences.get(i)?;

                            frame_task.inferencing_bboxes.push(b);

                            let xc = b.x as f32 + (b.width as f32 / 2.0);
                            let yc = b.y as f32 + (b.height as f32 / 2.0);

                            frame_task.tracking_bboxes.push((
                                Universal2DBox::new_with_confidence(
                                    xc,
                                    yc,
                                    None,
                                    b.width as f32 / b.height as f32,
                                    b.height as f32,
                                    cf,
                                ),
                                Some(cid.into()),
                            ));
                        }

                        //
                        // handle tracking
                        //

                        frame_task.tracks = tracker.predict(frame_task.tracking_bboxes.as_slice());

                        for t in frame_task.tracks.iter() {
                            if let Ok(mut tracks_by_id) =
                                frame_task.tracks_by_id.borrow_mut().lock()
                            {
                                let tracks_for_id = &mut match tracks_by_id.get_mut(&t.id) {
                                    Some(tracks_for_id) => tracks_for_id,
                                    None => {
                                        tracks_by_id.insert(t.id, vec![]);

                                        tracks_by_id.get_mut(&t.id).unwrap()
                                    }
                                };

                                tracks_for_id.push(t.clone());
                            }
                        }
                        let after = Instant::now();
                        tracking_duration.add_assign(after - before);

                        handled_frame_count += 1;

                        frame_tasks.push(frame_task);
                    }

                    frame_index += 1;

                    let decoding_after_all = Instant::now();
                    total_frame_duration.add_assign(decoding_after_all - decoding_before_all);
                }

                Ok(())
            };

        println!("\nstreaming packets into decoder...");
        for (stream, packet) in ictx.packets() {
            if stream.index() != video_stream_index {
                continue;
            }

            let before = Instant::now();
            decoder.send_packet(&packet)?;
            let after = Instant::now();
            decoding_duration.add_assign(after - before);

            let before = Instant::now();
            receive_and_process_decoded_frames(&mut decoder)?;
            let after = Instant::now();
            processing_duration.add_assign(after - before);
        }

        let before = Instant::now();
        println!("\nwriting an eof into the decoder...");
        decoder.send_eof()?;

        println!("\nreceiving and processing the eof...");
        receive_and_process_decoded_frames(&mut decoder)?;
        let after = Instant::now();
        processing_duration.add_assign(after - before);

        let before = Instant::now();

        let mut detections: Vec<Detection> = vec![];

        for frame_task in frame_tasks.iter_mut() {
            for track in frame_task.tracks.iter_mut() {
                if let Ok(this_tracks_by_id) = frame_task.tracks_by_id.borrow_mut().lock() {
                    let last_track = this_tracks_by_id
                        .get(&track.id)
                        .ok_or(anyhow!(format!(
                            "failed to get last track for track {:?}",
                            track.id
                        )))?
                        .last()
                        .ok_or(anyhow!(format!(
                            "failed to get last track for track {:?}",
                            track.id
                        )))?;

                    // println!(
                    //     "epoch: {:?}, id: {:?} {:?}, length: {:?}, bbox: {:?}; last_track.length: {:?}",
                    //     track.epoch,
                    //     track.id,
                    //     track.custom_object_id,
                    //     track.length,
                    //     track.observed_bbox,
                    //     last_track.length,
                    // );

                    let center_x = track.observed_bbox.xc;
                    let center_y = track.observed_bbox.yc;

                    let height = track.observed_bbox.height;

                    let width = track.observed_bbox.aspect * height;

                    let tl_x = center_x - (width / 2.0);
                    let tl_y = center_y - (height / 2.0);

                    let br_x = center_x + (width / 2.0);
                    let br_y = center_y + (height / 2.0);

                    let centroid = DetectionCentroid {
                        x: Some(center_x.into()),
                        y: Some(center_y.into()),
                    };

                    // closed polygon for postgres
                    let bounding_box: Vec<DetectionCentroid> = vec![
                        // top left
                        DetectionCentroid {
                            x: Some(tl_x.into()),
                            y: Some(tl_y.into()),
                        },
                        // top right
                        DetectionCentroid {
                            x: Some(br_x.into()),
                            y: Some(tl_y.into()),
                        },
                        // bottom right
                        DetectionCentroid {
                            x: Some(br_x.into()),
                            y: Some(br_y.into()),
                        },
                        // bottom left
                        DetectionCentroid {
                            x: Some(tl_x.into()),
                            y: Some(br_y.into()),
                        },
                        // top left again
                        DetectionCentroid {
                            x: Some(tl_x.into()),
                            y: Some(tl_y.into()),
                        },
                    ];

                    let class_id = last_track.custom_object_id.ok_or(anyhow!(format!(
                        "failed to custom object id for last track {:?}",
                        track.id
                    )))?;

                    let class_name = name_by_class_id.get(&class_id).ok_or(anyhow!(format!(
                        "failed to class name for class id {:?}",
                        class_id
                    )))?;

                    let confidence = track.predicted_bbox.confidence;

                    let frame_index = frame_task.frame_index;
                    let timestamp = frame_task.timestamp;

                    let last_track_id = last_track.id;
                    let last_track_length = last_track.length;

                    if last_track_length < 2 {
                        println!(
                            "skip: {frame_index} {timestamp:?} | {class_name} ({class_id}) @ {confidence} from {last_track_id} for {last_track_length} [({tl_x}, {tl_y}), ({br_x}, {br_y})] ({center_x}, {center_y})"
                        );
                        continue;
                    }

                    let detection = Detection {
                        bounding_box: Some(bounding_box),
                        camera_id: video.camera_id,
                        centroid: Some(centroid),
                        class_id: Some(class_id),
                        class_name: Some(class_name.clone()),
                        score: Some(confidence.into()),
                        seen_at: Some(frame_task.timestamp),
                        video_id: video.id,
                        ..Default::default()
                    };

                    println!(
                        "keep: {frame_index} {timestamp:?} | {class_name} ({class_id}) @ {confidence} from {last_track_id} for {last_track_length} [({tl_x}, {tl_y}), ({br_x}, {br_y})] ({center_x}, {center_y})"
                    );

                    detections.push(detection);
                }
            }

            // for (i, bbox) in frame_task.inferencing_bboxes.iter().enumerate() {
            //     let tl = bbox.tl();
            //     let br = bbox.br();

            //     let width = bbox.width;
            //     let height = bbox.height;

            //     let center_x = tl.x + (width / 2);
            //     let center_y = tl.y + (height / 2);

            //     let centroid = DetectionCentroid {
            //         x: Some(center_x.into()),
            //         y: Some(center_y.into()),
            //     };

            //     // closed polygon for postgres
            //     let bounding_box: Vec<DetectionCentroid> = vec![
            //         // top left
            //         DetectionCentroid {
            //             x: Some(tl.x.into()),
            //             y: Some(tl.y.into()),
            //         },
            //         // top right
            //         DetectionCentroid {
            //             x: Some(br.x.into()),
            //             y: Some(tl.y.into()),
            //         },
            //         // bottom right
            //         DetectionCentroid {
            //             x: Some(br.x.into()),
            //             y: Some(br.y.into()),
            //         },
            //         // bottom left
            //         DetectionCentroid {
            //             x: Some(tl.x.into()),
            //             y: Some(br.y.into()),
            //         },
            //         // top left again
            //         DetectionCentroid {
            //             x: Some(tl.x.into()),
            //             y: Some(tl.y.into()),
            //         },
            //     ];

            //     let class_id = match frame_task.class_ids.get(i) {
            //         Ok(class_id) => Some(class_id as i64),
            //         Err(_) => None,
            //     };

            //     let confidence = match frame_task.confidences.get(i) {
            //         Ok(confidence) => Some(confidence as f64),
            //         Err(_) => None,
            //     };

            //     let mut class_name = None;

            //     if let Some(class_id) = class_id
            //         && let Some(possible_class_name) = name_by_class_id.get(&class_id)
            //     {
            //         class_name = Some(possible_class_name.clone());
            //     }

            //     {
            //         let class_name = class_name.clone().unwrap();
            //         let class_id = class_id.unwrap();
            //         let confidence = confidence.unwrap();

            //         let tlx = tl.x;
            //         let tly = tl.y;

            //         let brx = br.x;
            //         let bry = br.y;

            //         println!(
            //             "{class_name} ({class_id}) @ {confidence} [({tlx}, {tly}), ({brx}, {bry})] ({center_x}, {center_y})"
            //         );
            //     }

            //     let detection = Detection {
            //         bounding_box: Some(bounding_box),
            //         camera_id: video.camera_id,
            //         centroid: Some(centroid),
            //         class_id,
            //         class_name,
            //         score: confidence,
            //         seen_at: Some(frame_task.timestamp),
            //         video_id: video.id,
            //         ..Default::default()
            //     };

            //     detections.push(detection);
            // }
        }

        let mut detections_by_class_name = HashMap::<String, Vec<Detection>>::new();

        for detection in detections.iter() {
            if let Some(class_name) = &detection.class_name {
                if !detections_by_class_name.contains_key(class_name) {
                    detections_by_class_name.insert(class_name.to_string(), vec![]);
                }

                if let Some(detections_for_class_name) =
                    detections_by_class_name.get_mut(class_name)
                {
                    detections_for_class_name.push(detection.clone());
                };
            };
        }

        let mut detection_summary_by_class_name = HashMap::<String, DetectionSummary>::new();

        for (class_name, detections) in detections_by_class_name.iter() {
            let detected_frame_count = detections.len();

            let mut scores: Vec<f64> = vec![];

            for detection in detections.iter() {
                if let Some(score) = detection.score {
                    scores.push(score);
                }
            }

            let average_score: f64 = scores.iter().sum::<f64>() / scores.len() as f64;

            let weighted_score: f64 = match handled_frame_count > 0 {
                true => (average_score * detected_frame_count as f64) / handled_frame_count as f64,
                false => 0.0,
            };

            let detection_summary = DetectionSummary {
                class_name: class_name.to_string(),
                weighted_score,
                average_score,
                detected_frame_count: detected_frame_count as i64,
                handled_frame_count,
            };

            // 5 frames at 20 fps with a stride of 4 = 1s?
            if detected_frame_count < 5 {
                println!("low frames: {:?}", detection_summary);
                continue;
            }

            if weighted_score < 0.33 {
                println!("low score : {:?}", detection_summary);
                continue;
            }

            println!("keeping   : {:?}", detection_summary);
            detection_summary_by_class_name.insert(class_name.to_string(), detection_summary);
        }

        let mut detection_summaries: Vec<&DetectionSummary> =
            detection_summary_by_class_name.values().collect();

        detection_summaries.sort_by(|a, b| a.weighted_score.total_cmp(&b.weighted_score));

        detection_summaries.reverse();

        println!("\nposting {:?} detections", detections.len());

        if !detections.is_empty() {
            let req = PostDetectionsRequest {
                body: detections,
                ..Default::default()
            };

            let _res = rt.block_on(client.post_detections(req))?;
        }

        println!("\npatching video...");

        {
            let req = PatchVideoRequest {
                primary_key: video.id.unwrap(),
                body: PatchVideoRequestBody {
                    object_detector_claimed_until: Some(Utc::now()),
                    status: Some("needs tracking".to_string()),
                    detection_summary: Some(serde_json::to_value(detection_summaries)?),
                    ..Default::default()
                },
                ..Default::default()
            };

            let _res = rt.block_on(client.patch_video(req))?;
        }

        let after = Instant::now();
        api_duration.add_assign(after - before);

        let file_after_all = Instant::now();
        total_file_duration.add_assign(file_after_all - file_before_all);

        println!("vid_startup_duration: {:?}", vid_startup_duration);
        println!("decoding_duration: {:?}", decoding_duration);
        println!("processing_duration: {:?}", processing_duration);
        println!("load_image_duration: {:?}", load_image_duration);
        println!("scale_frame_duration: {:?}", scale_frame_duration);
        println!(
            "load_scaled_image_duration: {:?}",
            load_scaled_image_duration
        );
        println!("inferencing_duration: {:?}", inferencing_duration);
        println!("tracking_duration: {:?}", tracking_duration);
        println!("api_duration: {:?}", api_duration);
        println!("total_frame_duration: {:?}", total_frame_duration);
        println!("total_file_duration: {:?}", total_file_duration);

        if !one_shot_file_name.is_empty() {
            println!("exiting because this was a one-shot run");
            exit(0);
        }
    }
}

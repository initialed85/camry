use anyhow::{Result, anyhow};
use chrono::Utc;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::{Dictionary, format::Pixel};

use opencv::{
    core::{self, CV_8UC3},
    dnn,
    prelude::*,
};
use similari::prelude::{SortTrack, Universal2DBox};
use similari::trackers::sort::PositionalMetricType::IoU;
use similari::trackers::sort::simple_api::Sort;
use std::{
    collections::HashMap,
    path::Path,
    process::exit,
    time::{Duration, Instant},
};
use std::{ops::AddAssign, thread::sleep};

mod api_client;
mod api_types;

use crate::api_types::*;

const STRIDE: i32 = 4;
const CONF_THRESHOLD: f32 = 0.1;
const NMS_THRESHOLD: f32 = 0.1;
const BBOX_HISTORY: usize = 65536;
const MAX_IDLE_EPOCHS: usize = 65536 * 256;
const IOU_THRESHOLD: f32 = 0.1;
const SORT_CONF_THRESHOLD: f32 = 0.1;
const MIN_CLASS_ID: i32 = -128;
const MAX_CLASS_ID: i32 = 128;
const PADDING_I: i32 = 20;
const PADDING_F: f32 = PADDING_I as f32;
const LINE_THICKNESS: i32 = 2;
const TEXT_SCALE_SMALL: f64 = 0.5;
const TEXT_SCALE_LARGE: f64 = 2.0;
const MODEL_DIMENSION: i32 = 416;

const BLUE: core::Scalar = core::Scalar {
    0: [255.0, 0.0, 0.0, 0.0],
};

const GREEN: core::Scalar = core::Scalar {
    0: [0.0, 255.0, 0.0, 0.0],
};

const RED: core::Scalar = core::Scalar {
    0: [0.0, 0.0, 255.0, 0.0],
};

const WHITE: core::Scalar = core::Scalar {
    0: [255.0, 255.0, 255.0, 0.0],
};

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
    let drawing_duration = Duration::default();
    let mut total_frame_duration = Duration::default();

    // let config = "people-r-people.cfg";
    // let weights = "people-r-people.weights";

    let config = "yolov4-tiny.cfg";
    let weights = "yolov4-tiny.weights";

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

    let mut class_ids = core::Vector::<i32>::new();
    let mut confidences = core::Vector::<f32>::new();
    let mut scaled_boxes = core::Vector::<core::Rect>::new();
    let mut boxes = vec![];
    let mut bboxes = vec![];
    let mut tracks = vec![];
    let mut all_boxes = vec![];
    let mut all_tracks_by_id: HashMap<u64, Vec<SortTrack>> = HashMap::new();

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
        //
        // figure out the video to handle
        //

        let mut file_name: String = String::new();

        let res: GetVideoResponse = if !one_shot_file_name.is_empty() {
            println!(
                "trying to find video with file_name of ONE_SHOT_FILE_NAME={:?}...",
                one_shot_file_name
            );

            let req = GetVideosRequest {
                file_name_eq: Some(one_shot_file_name.clone()),
                ..Default::default()
            };

            rt.block_on(client.get_videos(req))?
        } else {
            println!("trying to claim video...");

            let req = PostObjectDetectorClaimVideosRequest {
                body: PostObjectDetectorClaimVideosRequestBody {
                    timeout_seconds: Some(10.0),
                    until: Some(Utc::now() + Duration::from_secs(70)),
                },
            };

            rt.block_on(client.post_object_detector_claim_videos(req))?
        };

        match res {
            GetVideoResponse::Ok(res) => {
                println!("res: {:?}", res);

                if let Some(objects) = res.objects
                    && !objects.is_empty()
                    && let Some(object_file_name) = &objects[0].file_name
                {
                    file_name = object_file_name.clone()
                }
            }
            GetVideoResponse::Unknown(_res) => {
                sleep(Duration::from_secs(1));
                continue;
            }
        }

        if file_name.is_empty() {
            return Err(anyhow!("file_name unexpectedly empty"));
        }

        let file_path = source_path.join(Path::new(&file_name));

        if !file_path.exists() {
            return Err(anyhow!(format!(
                "file_path={:?} does not exist",
                file_path.to_str().unwrap_or(""),
            )));
        }

        //
        // handle the video
        //

        let mut options = Dictionary::new();

        options.set("hwaccel", "cuda");
        options.set("hwaccel_output_format", "cuda");

        let mut ictx = ffmpeg::format::input_with_dictionary(&file_path, options)?;

        println!("opened {:?}", file_path);

        let vid_startup_before = Instant::now();

        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;

        let video_stream_index = input.index();

        let mut decoder = match ffmpeg::codec::decoder::find_by_name("h264_cuvid") {
            Some(codec) => {
                println!("Using CUDA hardware decoder (h264_cuvid)");
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

        // ffmpeg::log::set_level(ffmpeg::log::Level::Trace);

        // let mut filter_graph = ffmpeg::filter::Graph::new();

        // filter_graph.add(
        //     &ffmpeg::filter::find("buffer").unwrap(),
        //     "in",
        //     "video_size=uhd2160:pix_fmt=yuv420p:time_base=1/1000:frame_rate=20",
        // )?;

        // filter_graph.add(&ffmpeg::filter::find("buffersink").unwrap(), "out", "")?;

        // filter_graph
        //     .output("in", 0)?
        //     .input("out", 0)?
        //     .parse("scale_cuda=416:416:format=yuv420p:interp_algo=bilinear")?;

        // println!("before");
        // filter_graph.validate()?;
        // println!("after");

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::BGR24,
            MODEL_DIMENSION as u32,
            MODEL_DIMENSION as u32,
            Flags::FAST_BILINEAR,
        )?;

        let scale_x = decoder.width() as f32 / MODEL_DIMENSION as f32;
        let scale_y = decoder.height() as f32 / MODEL_DIMENSION as f32;

        let mut frame_index = 0;
        let _rgb_frame = ffmpeg::frame::Video::empty();
        let mut rgb_frame_scaled = ffmpeg::frame::Video::empty();

        let vid_startup_after = Instant::now();
        vid_startup_duration.add_assign(vid_startup_after - vid_startup_before);

        let mut decoded: ffmpeg::frame::Video = ffmpeg::frame::Video::empty();
        let _img: Mat = Mat::default();
        let mut img_scaled = Mat::default();

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<()> {
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let before_all = Instant::now();

                    let timestamp = decoded.timestamp();

                    let before = Instant::now();

                    scaler.run(&decoded, &mut rgb_frame_scaled)?;

                    // match filter_graph.get("in") {
                    //     Some(mut filter_graph) => {
                    //         filter_graph.source().add(&decoded)?;
                    //     }
                    //     None => continue,
                    // }

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
                    // handle the model and the tracker
                    //

                    #[allow(clippy::modulo_one)]
                    if frame_index % STRIDE == 0 {
                        class_ids.clear();
                        confidences.clear();
                        scaled_boxes.clear();
                        boxes.clear();
                        bboxes.clear();

                        let before = Instant::now();
                        model.detect(
                            &img_scaled,
                            &mut class_ids,
                            &mut confidences,
                            &mut scaled_boxes,
                            CONF_THRESHOLD,
                            NMS_THRESHOLD,
                        )?;
                        let after = Instant::now();
                        inferencing_duration.add_assign(after - before);

                        total_boxes += scaled_boxes.len();

                        let before = Instant::now();
                        for (i, scaled_b) in scaled_boxes.iter().enumerate() {
                            let mut b = scaled_b;

                            b.x = (b.x as f32 * scale_x) as i32;
                            b.y = (b.y as f32 * scale_y) as i32;
                            b.width = (b.width as f32 * scale_x) as i32;
                            b.height = (b.height as f32 * scale_y) as i32;

                            boxes.push(b);
                            all_boxes.push((b, timestamp));

                            let cid = class_ids.get(i)?;
                            if !(MIN_CLASS_ID..=MAX_CLASS_ID).contains(&cid) {
                                continue;
                            }

                            let cf = confidences.get(i)?;

                            bboxes.push((
                                Universal2DBox::new_with_confidence(
                                    b.x as f32,
                                    b.y as f32,
                                    None,
                                    b.width as f32 / b.height as f32,
                                    b.height as f32,
                                    cf,
                                ),
                                None::<i64>,
                            ));
                        }

                        tracks = tracker.predict(bboxes.as_slice());

                        for t in tracks.iter() {
                            let tracks_for_id = &mut match all_tracks_by_id.get_mut(&t.id) {
                                Some(tracks_for_id) => tracks_for_id,
                                None => {
                                    all_tracks_by_id.insert(t.id, vec![]);

                                    all_tracks_by_id.get_mut(&t.id).unwrap()
                                }
                            };

                            tracks_for_id.push(t.clone());
                        }
                        let after = Instant::now();
                        tracking_duration.add_assign(after - before);
                    }

                    frame_index += 1;

                    let after_all = Instant::now();
                    total_frame_duration.add_assign(after_all - before_all);
                }

                Ok(())
            };

        println!("streaming packets into decoder...");
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
        println!("writing an eof into the decoder...");
        decoder.send_eof()?;

        println!("receiving and processing the eof...");
        receive_and_process_decoded_frames(&mut decoder)?;
        let after = Instant::now();
        processing_duration.add_assign(after - before);

        println!("all_boxes: {:?}", all_boxes);
        println!("all_tracks_by_id: {:?}", all_tracks_by_id);
        println!("total_boxes: {total_boxes}");
        println!("all_tracks_by_id_len: {:}", all_tracks_by_id.len());
        println!();

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
        println!("drawing_duration: {:?}", drawing_duration);
        println!("total_frame_duration: {:?}", total_frame_duration);
        println!();

        if !one_shot_file_name.is_empty() {
            println!("exiting because this was a one-shot run");
            exit(0);
        }
    }
}

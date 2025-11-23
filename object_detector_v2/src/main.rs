use anyhow::Result;
use ffmpeg::format::{Pixel, input};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use itertools::izip;
use opencv::{
    core::{self, CV_8UC3},
    dnn, highgui, imgproc,
    prelude::*,
};
use similari::prelude::Universal2DBox;
use similari::trackers::sort::PositionalMetricType::IoU;
use similari::trackers::sort::simple_api::Sort;
use std::{env, time::Instant};

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
    let config = "yolov4-tiny.cfg";
    // let config = "people-r-people.cfg";

    let weights = "yolov4-tiny.weights";
    // let weights = "people-r-people.weights";

    let net = dnn::read_net(weights, config, "Darknet")?;

    let mut model = dnn::DetectionModel::new_1(&net)?;

    #[cfg(target_os = "linux")]
    {
        model.set_preferable_backend(dnn::Backend::DNN_BACKEND_CUDA)?;
        model.set_preferable_target(dnn::Target::DNN_TARGET_CUDA)?;
    }

    #[cfg(target_os = "macos")]
    {
        highgui::named_window("object-detector", highgui::WINDOW_FULLSCREEN)?;

        model.set_preferable_backend(dnn::Backend::DNN_BACKEND_OPENCV)?;
        model.set_preferable_target(dnn::Target::DNN_TARGET_OPENCL)?;
    }

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
    let mut boxes = core::Vector::<core::Rect>::new();
    let mut bboxes = vec![];
    let mut tracks = vec![];

    let mut tracker = Sort::new(
        1,
        BBOX_HISTORY,
        MAX_IDLE_EPOCHS,
        IoU(IOU_THRESHOLD),
        SORT_CONF_THRESHOLD,
        None,
    );

    let mut total_boxes = 0;
    let mut total_tracks = 0;

    ffmpeg::init().unwrap();

    if let Ok(mut ictx) = input(&env::args().nth(1).expect("Cannot open file.")) {
        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;

        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        // TODO: multi-threaded decoding; doesn't seem to move the needle
        // decoder.set_threading(ffmpeg::threading::Config {
        //     kind: ffmpeg::threading::Type::Frame,
        //     count: 0,
        // });

        let mut passthrough = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::BGR24,
            decoder.width(),
            decoder.height(),
            Flags::FAST_BILINEAR,
        )?;

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
        let mut rgb_frame = Video::empty();
        let mut rgb_frame_scaled = Video::empty();

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<()> {
                let mut decoded = Video::empty();
                let mut img: Mat = Default::default();
                let mut img_scaled: Mat = Default::default();

                while decoder.receive_frame(&mut decoded).is_ok() {
                    let before_all = Instant::now();

                    #[cfg(target_os = "macos")]
                    {
                        let before = Instant::now();
                        passthrough.run(&decoded, &mut rgb_frame)?;
                        let after = Instant::now();
                        println!("passthrough frame took {:?}", after - before);

                        // more performant than the safe variant (which implies a copy)
                        let before = Instant::now();
                        img = unsafe {
                            Mat::new_rows_cols_with_data_unsafe(
                                decoder.height() as i32,
                                decoder.width() as i32,
                                CV_8UC3,
                                rgb_frame.data(0).as_ptr() as *mut _,
                                rgb_frame.stride(0),
                            )?
                        };
                        let after = Instant::now();
                        println!("loading passthrough image took {:?}", after - before);
                    }

                    let before = Instant::now();
                    scaler.run(&decoded, &mut rgb_frame_scaled)?;
                    let after = Instant::now();
                    println!("scaling frame took {:?}", after - before);

                    // more performant than the safe variant (which implies a copy)
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
                    println!("loading scaled image took {:?}", after - before);

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
                        println!(
                            "inferencing took {:?} for {:?} boxes",
                            after - before,
                            scaled_boxes.len()
                        );

                        total_boxes += scaled_boxes.len();

                        let before = Instant::now();
                        for (i, scaled_b) in scaled_boxes.iter().enumerate() {
                            let mut b = scaled_b;

                            b.x = (b.x as f32 * scale_x) as i32;
                            b.y = (b.y as f32 * scale_y) as i32;
                            b.width = (b.width as f32 * scale_x) as i32;
                            b.height = (b.height as f32 * scale_y) as i32;

                            boxes.push(b);

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
                        let after = Instant::now();
                        println!(
                            "tracking took {:?} for {:?} tracks",
                            after - before,
                            tracks.len()
                        );

                        total_tracks += tracks.len();
                    }

                    #[cfg(target_os = "macos")]
                    {
                        let before = Instant::now();

                        //
                        // draw the detected objects
                        //

                        for (cid, _cf, b) in izip!(&class_ids, &confidences, &boxes) {
                            if !(MIN_CLASS_ID..=MAX_CLASS_ID).contains(&cid) {
                                continue;
                            }

                            imgproc::rectangle(
                                &mut img,
                                core::Rect::new(
                                    b.x + PADDING_I,
                                    b.y + PADDING_I,
                                    b.width - PADDING_I * 2,
                                    b.height - PADDING_I * 2,
                                ),
                                RED,
                                LINE_THICKNESS,
                                imgproc::LineTypes::LINE_8.into(),
                                0,
                            )?;
                        }

                        //
                        // draw the observed tracked objects
                        //

                        for t in &tracks {
                            let b = &t.observed_bbox;

                            let width = b.height * b.aspect;

                            imgproc::rectangle(
                                &mut img,
                                core::Rect::new(
                                    b.xc as i32,
                                    b.yc as i32,
                                    width as i32,
                                    b.height as i32,
                                ),
                                BLUE,
                                LINE_THICKNESS,
                                imgproc::LineTypes::LINE_8.into(),
                                0,
                            )?;
                        }

                        //
                        // draw the predicted tracked objects
                        //

                        for t in &tracks {
                            let b = &t.predicted_bbox;

                            let width = b.height * b.aspect;

                            imgproc::rectangle(
                                &mut img,
                                core::Rect::new(
                                    b.xc as i32,
                                    b.yc as i32,
                                    width as i32,
                                    b.height as i32,
                                ),
                                GREEN,
                                LINE_THICKNESS,
                                imgproc::LineTypes::LINE_8.into(),
                                0,
                            )?;

                            imgproc::put_text_def(
                                &mut img,
                                &format!("{:}", t.id),
                                core::Point::new(
                                    (b.xc + ((b.aspect * b.height) / 3.0)) as i32,
                                    (b.yc + b.height / 2.0) as i32,
                                ),
                                imgproc::FONT_HERSHEY_TRIPLEX,
                                TEXT_SCALE_LARGE,
                                WHITE,
                            )?;
                        }

                        highgui::imshow("object-detector", &img)?;

                        match highgui::wait_key(1)? {
                            32 => _ = highgui::wait_key(0)?,
                            27 => {
                                break;
                            }
                            _ => {}
                        }

                        let after = Instant::now();
                        println!("drawing {:?}", after - before);
                    }

                    frame_index += 1;

                    let after_all = Instant::now();
                    println!("frame took {:?}", after_all - before_all);
                    println!();
                }

                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            // note: ignoring the audio streams
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }

    println!("total_boxes: {total_boxes}");
    println!("total_tracks: {total_tracks}");

    Ok(())
}

use image::ImageReader;

pub mod args;

// pub mod terminal;
// use crate::terminal::{get_scale, get_terminal_size};

pub mod video;

pub mod ffutils;

pub mod textifier;
use textifier::Textifier;

use crate::{
    args::{MediaModes, Options},
    ffutils::Meta,
};

// do cli parsing pseudo code
//
// inputscale, duration, frames = getMeta(path)
// internalScale, outputscale = getScales(characterSizes, specifiedOutputScale, inputScale, outputScaleLimit)
//
// textifier = init(internalScale, outputScale, renderOptions)
//
// image {
//     image = getImage(path, internalScale)
//     text = textifier(image)
//     print(text)
// }
//
// frames {
//     frames = getFrames(path, internalScale, duration, frames, videoOptions)
//
//     image = frames.next() {
//         text = textifier(image)
//         print(text)
//     }
// }
//

// pseudo try two
//
// renderer = renderBuilder();
//
// if let arg = cli arg {
//    renderer.withArg(arg);
// }
//
// renderer.withResolutions(scales);
// renderer = renderer.build();
//
//
// if let img = cli img {
//    renderer.render(img)
// }
//
//

/* TODO create some more general helpers here, instead of the magical run() fn */

// pub fn run(args: &Options) -> Result<(), String> {
//     let metadata = ffutils::get_meta(&args.general.path)?;

//     let (internal_scale, output_scale) = get_scale(
//         (args.render.char_width, args.render.char_height),
//         (args.render.width, args.render.height),
//         metadata.scale,
//         get_terminal_size(),
//     );

//     let mut textifier = Textifier::new(
//         &args.render,
//         args.general.processing_mode,
//         internal_scale,
//         output_scale,
//     );

//     let is_video: bool = match args.general.media_mode {
//         Some(MediaModes::Video) => true,
//         Some(MediaModes::Image) => false,
//         None => {
//             match metadata.frames {
//                 // More then 1 frame means video
//                 Some(x) if x > 1 => true,
//                 // Else image
//                 _ => false,
//             }
//         }
//     };

//     if is_video {
//         return do_video_stuff(args, &mut textifier, &internal_scale, metadata);
//     } else {
//         return do_image_stuff(args, &mut textifier, &internal_scale);
//     }
// }

// fn do_image_stuff(
//     args: &Options,
//     textifier: &mut Textifier,
//     internal_scale: &(u32, u32),
// ) -> Result<(), String> {
//     let reader_result = ImageReader::open(&args.general.path);
//     if reader_result.is_err() {
//         return Err("Cannot find file".to_string());
//     }
//     let img_result = reader_result.unwrap().decode();

//     if let Ok(image) = img_result {
//         let image = image.resize_exact(
//             internal_scale.0,
//             internal_scale.1,
//             image::imageops::FilterType::Nearest,
//         );
//         print!("{}", textifier.to_text(image)?);
//         return Ok(());
//     } else {
//         return Err("Cannot open as an image".to_string());
//     }
// }

// fn do_video_stuff(
//     args: &Options,
//     textifier: &mut Textifier,
//     internal_scale: &(u32, u32),
//     meta: Meta,
// ) -> Result<(), String> {
//     let video_frame_grabber =
//         video::FrameGrabber::new(&args.general.path, &args.video, &internal_scale, meta).unwrap();

//     let mut rows = 0;

//     for image in video_frame_grabber {
//         // while let Some(image) = video_frame_grabber.next() {
//         // ESCAPE 1 G = move cursor all the way to the left
//         // ESCAPE n A = move cursor n up
//         let ansi: String;
//         if rows != 0 {
//             ansi = format!("\x1b[1G\x1b[{}A", rows);
//         } else {
//             ansi = "".to_owned();
//         }

//         let text = textifier.to_text(image)?;
//         rows = text.lines().count() - 1;
//         print!("{}{}", ansi, text);
//     }

//     return Ok(());
// }

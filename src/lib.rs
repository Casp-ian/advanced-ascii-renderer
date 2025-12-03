use image::ImageReader;

pub mod cli;
use cli::*;

pub mod video;

pub mod ffutils;

pub mod terminal;

pub mod textifier;
use textifier::Textifier;

use crate::terminal::{get_scale, get_terminal_size};

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
//
//

pub fn run(args: &Args) -> Result<(), String> {
    let (input_scale, duration, frames) = ffutils::get_meta(&args.path)?;

    let (internal_scale, output_scale) = get_scale(
        (args.char_width, args.char_height),
        (args.width, args.height),
        input_scale,
        get_terminal_size(),
    );

    let mut textifier = Textifier::new(&args, internal_scale, output_scale);

    let is_video: bool = match args.media_mode {
        Some(MediaModes::Video) | Some(MediaModes::Stream) => true,
        Some(MediaModes::Image) => false,
        None => {
            match frames {
                // More then 1 frame means video
                Some(x) if x > 1 => true,
                // Else image
                _ => false,
            }
        }
    };

    if is_video {
        return do_video_stuff(args, &mut textifier, &internal_scale, &duration);
    } else {
        return do_image_stuff(args, &mut textifier, &internal_scale);
    }
}

fn do_image_stuff(
    args: &Args,
    textifier: &mut Textifier,
    internal_scale: &(u32, u32),
) -> Result<(), String> {
    let reader_result = ImageReader::open(&args.path);
    if reader_result.is_err() {
        return Err("Cannot find file".to_string());
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        let image = image.resize_exact(
            internal_scale.0,
            internal_scale.1,
            image::imageops::FilterType::Nearest,
        );
        print!("{}", textifier.to_text(image)?);
        return Ok(());
    } else {
        return Err("Cannot open as an image".to_string());
    }
}

fn do_video_stuff(
    args: &Args,
    textifier: &mut Textifier,
    internal_scale: &(u32, u32),
    duration: &Option<f32>,
) -> Result<(), String> {
    let mut video_frame_grabber =
        video::FrameGrabber::new(args, &internal_scale, duration).unwrap();

    let mut rows = 0;

    // let mut first_loop = true;
    while let Some(image) = video_frame_grabber.next() {
        // skip on first run
        let ansi: String;
        // ESCAPE 1 G = move cursor all the way to the left
        // ESCAPE n A = move cursor n up
        if rows != 0 {
            ansi = format!("\x1b[1G\x1b[{}A", rows);
        } else {
            ansi = "".to_owned();
        }

        let text = textifier.to_text(image)?;
        rows = text.lines().count() - 1;
        print!("{}{}", ansi, text);
    }

    return Ok(());
}

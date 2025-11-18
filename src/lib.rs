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
// image {
//     textifier = init(internalScale, outputScale, renderOptions)
//     image = getImage(path, internalScale)
//     text = textifier(image)
//     print(text)
// }
//
// frames {
//     textifier = init(internalScale, outputScale, renderOptions)
//     frames = getFrames(path, internalScale, duration, frames, videoOptions)
//
//     frame = frames.next() {
//         text = textifier(frame)
//         print(text)
//     }
// }
//

pub fn run(args: &Args) -> Result<(), String> {
    let (input_scale, _, frames) = match ffutils::get_meta(&args.path) {
        Err(e) => return Err(format!("{} for {:?}", e, args.path).to_string()),
        Ok(x) => x,
    };

    let (internal_scale, output_scale) = get_scale(
        (args.char_width, args.char_height),
        (args.width, args.height),
        input_scale,
        get_terminal_size(),
    );

    let mut textifier = Textifier::new(&args, internal_scale, output_scale);

    let result: Result<(), String> = match args.media_mode {
        Some(MediaModes::Image) => {
            do_image_stuff(args, &mut textifier, &internal_scale, &output_scale)
        }
        Some(MediaModes::Video) | Some(MediaModes::Stream) => {
            do_video_stuff(args, &mut textifier, &internal_scale, &output_scale)
        }
        None => {
            match frames {
                // More then 1 frame means video
                Some(x) if x > 1 => {
                    do_video_stuff(args, &mut textifier, &internal_scale, &output_scale)
                }
                // Else image
                _ => do_image_stuff(args, &mut textifier, &internal_scale, &output_scale),
            }
        }
    };
    return result;
}

fn do_image_stuff(
    args: &Args,
    textifier: &mut Textifier,
    internal_scale: &(u32, u32),
    output_scale: &(u32, u32),
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
    output_scale: &(u32, u32),
) -> Result<(), String> {
    let mut video_frame_grabber = video::FrameGrabber::new(args, &internal_scale).unwrap();

    let rows = output_scale.1;

    let mut first_loop = true;
    while let Some(image) = video_frame_grabber.next() {
        // skip on first run
        let ansi: String;
        if first_loop {
            first_loop = false;
            ansi = "".to_owned();
        } else {
            // ESCAPE 1 G = move cursor all the way to the left
            // ESCAPE n A = move cursor n up
            ansi = format!("\x1b[1G\x1b[{}A", rows - 1);
        }

        let text = textifier.to_text(image)?;
        print!("{}{}", ansi, text);
    }

    return Ok(());
}

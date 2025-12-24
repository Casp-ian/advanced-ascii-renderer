use std::process::ExitCode;

mod cli;
use cli::get_cli_args;

mod terminal;
use image::ImageReader;
use terminal::{get_scale, get_terminal_size};

use aar::{
    args::{MediaModes, Options},
    ffutils::{self, Meta},
    textifier::Textifier,
    video,
};

fn do_before_exit() {
    // new line because we might still be on another line
    // also clear ansi color code
    println!("\x1b[0m");
}

fn main() -> ExitCode {
    let _ = ctrlc::set_handler(|| {
        do_before_exit();

        // TODO, this doesnt stop any of the other processes gracefully, so sometimes a error message gets shown at exit
        std::process::exit(0)
    });

    let options = get_cli_args();

    let result: Result<(), String> = run(&options);

    do_before_exit();

    match result {
        Ok(_) => {
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    }
}

fn run(options: &Options) -> Result<(), String> {
    let metadata = ffutils::get_meta(&options.general.path)?;

    let (internal_scale, output_scale) = get_scale(
        (options.render.char_width, options.render.char_height),
        (options.render.width, options.render.height),
        metadata.scale,
        get_terminal_size(),
    );

    let mut textifier = Textifier::new(
        &options.render,
        options.general.processing_mode,
        internal_scale,
        output_scale,
    );

    let is_video: bool = match options.general.media_mode {
        Some(MediaModes::Video) => true,
        Some(MediaModes::Image) => false,
        None => {
            match metadata.frames {
                // More then 1 frame means video
                Some(x) if x > 1 => true,
                // Else image
                _ => false,
            }
        }
    };

    if is_video {
        return do_video_stuff(options, &mut textifier, &internal_scale, metadata);
    } else {
        return do_image_stuff(options, &mut textifier, &internal_scale);
    }
}

fn do_image_stuff(
    args: &Options,
    textifier: &mut Textifier,
    internal_scale: &(u32, u32),
) -> Result<(), String> {
    let reader_result = ImageReader::open(&args.general.path);
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
    args: &Options,
    textifier: &mut Textifier,
    internal_scale: &(u32, u32),
    meta: Meta,
) -> Result<(), String> {
    let video_frame_grabber =
        video::FrameGrabber::new(&args.general.path, &args.video, &internal_scale, meta).unwrap();

    let mut rows = 0;

    for image in video_frame_grabber {
        // while let Some(image) = video_frame_grabber.next() {
        // ESCAPE 1 G = move cursor all the way to the left
        // ESCAPE n A = move cursor n up
        let ansi: String;
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

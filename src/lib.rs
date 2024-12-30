use image::io::Reader;

pub mod cli;
use cli::*;

pub mod video;

pub mod terminal;

pub mod textifier;
use textifier::Textifier;

pub fn run_with_args(args: &Args) -> Result<(), String> {
    let result: Result<(), String> = match args.media_mode {
        MediaModes::Try => try_them_all(args),
        MediaModes::Image => do_image_stuff(args),
        MediaModes::Video | MediaModes::Stream => do_video_stuff(args),
    };
    return result;
}

fn try_them_all(args: &Args) -> Result<(), String> {
    let image_result = do_image_stuff(args);
    // TODO make sure .gif and other multiuse formats falls into video first then image

    // list of non working formats
    // webp (video) - ffmpeg doesnt support (image works fine)
    // avif - image crate doesnt support
    // gif - should prioritise video not image

    // TODO only do this if error is "cannot open as image", I should do errors as enums
    if image_result.is_err() {
        eprintln!("Can not open as image, now trying to open as video");
        return do_video_stuff(args);
    } else {
        return image_result;
    }
}

fn do_image_stuff(args: &Args) -> Result<(), String> {
    let reader_result = Reader::open(&args.path);
    if reader_result.is_err() {
        return Err("Cannot find file".to_string());
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        let mut thing = Textifier::new(&args);
        print!("{}", thing.to_text(image)?);

        // clear ansi color code
        println!("\x1b[0m");

        return Ok(());
    } else {
        return Err("Cannot open as an image".to_string());
    }
}

fn do_video_stuff(args: &Args) -> Result<(), String> {
    let mut textifier = Textifier::new(&args);

    let video_frame_grabber = video::FrameGrabber::new(args).unwrap();

    let mut first_loop = true;
    while let Some(image_result) = video_frame_grabber.grab() {
        match image_result {
            Ok(image) => {
                // these are ansi codes for 'clear current screen (dont clear scrollback)', and 'move cursor to top left'

                // TODO this still eats part of the scrollback if the height is not the entire terminal
                // skip on first run
                let ansi: &str;
                if first_loop {
                    first_loop = false;
                    ansi = "";
                } else {
                    ansi = "\x1b[2J\x1b[0;0H";
                }

                let text = textifier.to_text(image)?;
                print!("{}{}", ansi, text);
            }
            Err(e) => {
                // TODO if error of 'out of frames' then return Ok()

                // new line because we might still be on another line
                // also clear ansi color code
                println!("\x1b[0m");
                return Err(e);
            }
        }
    }
    return Ok(());
}

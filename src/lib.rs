use image::io::Reader;

pub mod cli;
use cli::*;

pub mod video;

pub mod ffutils;

pub mod terminal;

pub mod textifier;
use textifier::Textifier;

pub fn run(args: &Args) -> Result<(), String> {
    println!("{:?}", ffutils::get_meta(&args.path));

    let result: Result<(), String> = match args.media_mode {
        None => try_them_all(args),
        Some(MediaModes::Image) => do_image_stuff(args),
        Some(MediaModes::Video) | Some(MediaModes::Stream) => do_video_stuff(args),
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

fn calculate_dimensions(args: &Args) -> (u32, u32, u32, u32) {
    // TODO improve error handling
    let meta = ffutils::get_meta(&args.path).expect("Image not found");

    let image_width = meta.0;
    let image_height = meta.1;
    let (columns, rows) = terminal::get_cols_and_rows(
        args.char_width,
        args.char_height,
        args.width,
        args.height,
        image_width,
        image_height,
    );

    return (image_width, image_height, columns, rows);
}

fn do_image_stuff(args: &Args) -> Result<(), String> {
    let reader_result = Reader::open(&args.path);
    if reader_result.is_err() {
        return Err("Cannot find file".to_string());
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        let (in_width, in_height, out_width, out_height) = calculate_dimensions(&args);
        let mut textifier = Textifier::new(&args, in_width, in_height, out_width, out_height);
        print!("{}", textifier.to_text(image)?);

        // clear ansi color code
        println!("\x1b[0m");

        return Ok(());
    } else {
        return Err("Cannot open as an image".to_string());
    }
}

fn do_video_stuff(args: &Args) -> Result<(), String> {
    let (in_width, in_height, out_width, out_height) = calculate_dimensions(&args);
    let mut textifier = Textifier::new(&args, in_width, in_height, out_width, out_height);

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

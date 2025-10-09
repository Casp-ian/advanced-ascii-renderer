use image::io::Reader;

pub mod cli;
use cli::*;

pub mod video;

pub mod ffutils;

pub mod terminal;

pub mod textifier;
use textifier::Textifier;

pub fn run(args: &Args) -> Result<(), String> {
    let (img_width, img_height, duration, frames) = match ffutils::get_meta(&args.path) {
        None => return Err(format!("no valid image or video found at {:?}", args.path).to_string()),
        Some(x) => x,
    };

    let (cols, rows) = calculate_dimensions(args, img_width, img_height);

    let mut textifier = Textifier::new(&args, img_width, img_height, cols, rows);

    let result: Result<(), String> = match args.media_mode {
        Some(MediaModes::Image) => do_image_stuff(args, &mut textifier, &rows),
        Some(MediaModes::Video) | Some(MediaModes::Stream) => {
            do_video_stuff(args, &mut textifier, &rows)
        }
        None => {
            match frames {
                // More then 1 frame means video
                Some(x) if x > 1 => do_video_stuff(args, &mut textifier, &rows),
                // Else image
                _ => do_image_stuff(args, &mut textifier, &rows),
            }
        }
    };
    return result;
}

fn calculate_dimensions(args: &Args, image_width: u32, image_height: u32) -> (u32, u32) {
    let (columns, rows) = terminal::get_cols_and_rows(
        args.char_width,
        args.char_height,
        args.width,
        args.height,
        image_width,
        image_height,
    );

    return (columns, rows);
}

fn do_image_stuff(args: &Args, textifier: &mut Textifier, rows: &u32) -> Result<(), String> {
    let reader_result = Reader::open(&args.path);
    if reader_result.is_err() {
        return Err("Cannot find file".to_string());
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        print!("{}", textifier.to_text(image)?);

        // clear ansi color code
        println!("\x1b[0m");

        return Ok(());
    } else {
        return Err("Cannot open as an image".to_string());
    }
}

fn do_video_stuff(args: &Args, textifier: &mut Textifier, rows: &u32) -> Result<(), String> {
    let mut video_frame_grabber = video::FrameGrabber::new(args).unwrap();

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

    // new line because we might still be on another line
    // also clear ansi color code
    println!("\x1b[0m");
    return Ok(());
}

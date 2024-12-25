use clap::{Parser, ValueEnum};
use image::io::Reader;
use std::fmt::Debug;
use std::fs::remove_file;
use std::process::ExitCode;

mod video;

mod terminal;

mod textifier;
use textifier::Textifier;

/// Take an image and turn it into text
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path of image
    path: std::path::PathBuf,

    /// Width in characters
    #[arg(long)]
    width: Option<u32>,

    /// Height in characters
    #[arg(long)]
    height: Option<u32>,

    /// Color of text
    #[arg(long, default_value_t, value_enum)]
    color: ColorSet,

    /// Characters used for result
    #[arg(long, default_value_t, value_enum)]
    set: CharSet,

    /// Only affects videos, lower value is high quality, higher value is
    #[arg(short, long, default_value_t = 5)]
    quality: u8,

    /// Only affects videos, sets audio volume, clamps to 100
    #[arg(short, long, default_value_t = 0)]
    volume: u8,

    /// Only affects videos, sets ffmpeg format if ffmpeg cant auto detect
    #[arg(short, long)]
    format: Option<String>,

    /// make dark areas light, and light areas dark
    #[arg(long)]
    inverted: bool,

    /// remove the lines characters like /-\|
    #[arg(long)]
    no_lines: bool,

    // this can only be checked by getting the space taken per character, and the spacing between characters from the terminal,
    // i do not know how to get these, so for now we have hardcoded defaults
    /// the width of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 10)]
    char_width: u32,

    /// the height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 18)]
    char_height: u32,

    /// Characters used for result
    #[arg(long, default_value_t, value_enum)]
    mode: Modes,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
enum Modes {
    #[default]
    /// try image and then video if image fails
    Try,
    Image,
    /// textify frames as fast as it can, requires ffmpeg
    Video,
    /// just like video but for things like your webcam
    Stream,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
enum ColorSet {
    #[default]
    None,
    All,
}

// The actual arrays of characters used for the character sets could be stored inside this enum, but i dont think it really matters
// and if it does its an easy refactor for later, ill just keep it like this so its similar to the color set
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
enum CharSet {
    #[default]
    Ascii,
    Braile,
    Numbers,
    Discord,
}

fn do_before_exit() {
    let _ = remove_file(video::TEMPORARY_IMAGE_FILE_NAME);

    // TODO, this doesnt stop any of the other processes in a neat way, so sometimes a error message gets shown at exit
    std::process::exit(0);
}

fn main() -> ExitCode {
    let args = Args::parse();

    let _ = ctrlc::set_handler(do_before_exit);

    let result: Result<(), String> = match &args.mode {
        Modes::Try => try_them_all(&args),
        Modes::Image => do_image_stuff(&args),
        Modes::Video | Modes::Stream => do_video_stuff(&args),
    };

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

fn try_them_all(args: &Args) -> Result<(), String> {
    let image_result = do_image_stuff(args);
    // TODO make sure .gif and other multiuse formats falls into video first then image

    // list of non working formats
    // webp (video) - ffmpeg doesnt support (image works fine)
    // avif - rust image doesnt support
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
        print!("{}", thing.to_text(image));

        // clear ansi color code
        println!("\x1b[0m");

        return Ok(());
    } else {
        return Err("Cannot open as an image".to_string());
    }
}

fn do_video_stuff(args: &Args) -> Result<(), String> {
    let mut textifier = Textifier::new(&args);

    let video_frame_grabber = video::FrameGrabber::new(&args.path, args).unwrap();

    while let Some(image_result) = video_frame_grabber.grab() {
        match image_result {
            Ok(image) => {
                // these are ansi codes for 'clear current screen (dont clear scrollback)', and 'move cursor to top left'
                let ansi = "\x1b[2J\x1b[0;0H";
                let result = textifier.to_text(image);
                print!("{}{}", ansi, result);
            }
            Err(e) => {
                // TODO if error of 'out of frames' then return Ok()

                // new line because we might still be on another line
                // also clear ansi color code
                eprintln!("\x1b[0m");
                return Err(e);
            }
        }
    }
    return Ok(());
}

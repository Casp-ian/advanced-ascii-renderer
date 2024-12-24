use std::io;
use std::path::PathBuf;
use std::{fmt::Debug, time::Instant};

use clap::{Parser, ValueEnum};
use image::{io::Reader, DynamicImage};

mod processing;
use processing::image::*;

use std::process::{Command, ExitCode, Output};

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
    /// the width and height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 10)]
    char_width: u32,

    /// the width and height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 18)]
    char_height: u32,

    /// Characters used for result
    #[arg(long, default_value_t, value_enum)]
    mode: Modes,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
enum Modes {
    #[default]
    /// try image and video if image fails
    Try,
    Image,
    /// requires ffmpeg
    Video,
    /// just like video but for thing like your webcam
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

const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.png";

fn do_before_exit() {
    let _ = std::fs::remove_file(TEMPORARY_IMAGE_FILE_NAME);

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
            return ExitCode::SUCCESS;
        }
    }
}

fn try_them_all(args: &Args) -> Result<(), String> {
    let image_result = do_image_stuff(args);
    // TODO make sure .gif and other multiuse formats falls into video first then image
    // webm is weird too?

    // TODO only do this if error is "cannot open as image", I should do errors as enums
    if image_result.is_err() {
        // we could do a message here that we are trying video instead, but you wont have time to read it anyways
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

        // clear ansii color code
        println!("\x1b[0m");

        return Ok(());
    } else {
        return Err("Cannot open as an image".to_string());
        // eprintln!();
    }
}

fn do_video_stuff(args: &Args) -> Result<(), String> {
    let mut textifier = Textifier::new(&args);

    let video_textifier = VideoFrameGrabber::new(&args.path, args).unwrap();
    loop {
        match video_textifier.get_frame_as_image(&args.path) {
            Ok(image) => {
                // these are ansii codes for 'clear current screen (dont clear scrollback)', and 'move cursor to top left'
                let ansii = "\x1b[2J\x1b[0;0H";
                let result = textifier.to_text(image);
                print!("{}{}", ansii, result);
            }
            Err(e) => {
                // TODO if error of 'out of frames' then return Ok()

                // new line because we might still be on another line
                // also clear ansii color code
                eprintln!("\x1b[0m");
                return Err(e);
            }
        }
    }
}

// NOTE, ffmpeg-next or the other ffmpeg/video packages seemed quite large, and not have this specific usecase in mind
// so we just run the ffmpeg command of the system, it might be terrible, but it does work nice for now, and can be changed later
struct VideoFrameGrabber<'a> {
    args: &'a Args,
    start_time: Instant,
    length: f32,
}
impl<'b> VideoFrameGrabber<'b> {
    fn new<'a>(path: &PathBuf, args: &'a Args) -> Result<VideoFrameGrabber<'a>, String> {
        let length: f32;
        if args.mode != Modes::Stream {
            let command_result = Command::new("ffprobe")
                .args(["-i", path.to_str().unwrap()])
                .args(["-show_entries", "format=duration"])
                .args(["-v", "quiet"])
                .args(["-of", "default=noprint_wrappers=1:nokey=1"])
                .output();

            if let Err(error) = command_result {
                eprintln!(
                    "probably couldnt find ffmprobe (often installed with ffmpeg) on your system"
                );
                // eprintln!("{}", error);
                return Err(error.to_string());
            }

            let output = String::from_utf8(command_result.unwrap().stdout).unwrap();
            if let Ok(number) = output.replace("\n", "").replace("\r", "").parse::<f32>() {
                length = number;
            } else {
                return Err(format!(
                    "Could not get video length instead got: {}",
                    output
                ));
            }
        } else {
            length = f32::MAX; // still need to set length, but this wont be checked
        }

        if args.volume > 0 {
            Command::new("ffplay")
                .args([path.to_str().unwrap()])
                .args(["-nodisp"])
                .args(["-autoexit"])
                .args(["-v", "quiet"])
                .args(["-volume", &args.volume.to_string()])
                .spawn()
                .expect("audio broke");
        }

        let start_time = Instant::now();

        return Ok(VideoFrameGrabber {
            args,
            start_time,
            length,
        });
    }

    fn get_frame_as_image(&self, path: &PathBuf) -> Result<DynamicImage, String> {
        if self.length < self.start_time.elapsed().as_secs_f32() {
            return Err("out of video".to_string());
        }
        let command_result: io::Result<Output>;

        let mut command = &mut Command::new("ffmpeg");
        command = command.arg("-y");

        if let Some(format) = &self.args.format {
            command = command.args(["-f", format.as_str()]);
        }

        if self.args.mode != Modes::Stream {
            command = command.args([
                "-ss",
                self.start_time.elapsed().as_secs_f32().to_string().as_str(),
            ]);
        }

        command = command
            .args(["-i", path.to_str().unwrap()])
            .args(["-q:v", &self.args.quality.to_string()])
            .args(["-frames:v", "1"])
            .arg(TEMPORARY_IMAGE_FILE_NAME);

        command_result = command.output();

        match command_result {
            Err(e) => return Err(e.to_string()),
            Ok(s) => {
                // if s.status.code().unwrap_or(1) == 0 {
                //     return Err("ffmpeg status code not 0".to_string());
                // }
                let reader_result = Reader::open(TEMPORARY_IMAGE_FILE_NAME);
                if let Err(e) = reader_result {
                    return Err(e.to_string());
                }
                let image = reader_result.unwrap().decode().unwrap();
                let _ = std::fs::remove_file(TEMPORARY_IMAGE_FILE_NAME);
                return Ok(image);
            }
        }
    }
}

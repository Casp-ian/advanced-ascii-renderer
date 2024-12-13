use std::io;
use std::path::PathBuf;
use std::{fmt::Debug, time::Instant};

use clap::{Parser, ValueEnum};
use image::{io::Reader, DynamicImage};

use crossterm::terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen};

mod processing;
use processing::image::*;
use processing::text::*;

use std::process::Command;

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

    /// make dark areas light, and light areas dark
    #[arg(long)]
    inverted: bool,

    /// remove the lines with /-\|
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
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
enum ColorSet {
    #[default]
    None,
    All,
    ColorFull,
    FewColors,
    Real,
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

fn get_cols_and_rows(
    char_width: u32,
    char_height: u32,
    columns: Option<u32>,
    rows: Option<u32>,
    image_width: u32,
    image_height: u32,
) -> (u32, u32) {
    let (columns, rows) = match (columns, rows) {
        (Some(x), Some(y)) => {
            eprintln!(
                "you specified both image collumns and rows, image aspect ratio might be messed up"
            );
            return (x, y);
        }
        (Some(x), None) => (
            x,
            calculate_other_side_by_aspect(x, char_height, char_width, image_height, image_width),
        ),
        (None, Some(y)) => (
            calculate_other_side_by_aspect(y, char_width, char_height, image_width, image_height),
            y,
        ),
        (None, None) => get_fitting_terminal(char_width, char_height, image_width, image_height),
    };

    (columns, rows)
}

fn calculate_other_side_by_aspect(
    x: u32,
    source_aspect_x: u32,
    source_aspect_y: u32,
    target_aspect_x: u32,
    target_aspect_y: u32,
) -> u32 {
    (x as f32 * (target_aspect_y as f32 / source_aspect_y as f32)
        / (target_aspect_x as f32 / source_aspect_x as f32))
        .floor() as u32 //floor or round?
}

fn get_fitting_terminal(
    char_width: u32,
    char_height: u32,
    image_width: u32,
    image_height: u32,
) -> (u32, u32) {
    let max_terminal_chars_x: u32;
    let max_terminal_chars_y: u32;

    if let Ok(size) = terminal::size() {
        max_terminal_chars_x = size.0 as u32;
        max_terminal_chars_y = size.1 as u32 - 3; // minus 3 to adjust for prompt size, maybe we can actually get that prompt height somehow, but well try later
    } else {
        max_terminal_chars_x = 200;
        max_terminal_chars_y = 50;
        eprintln!(
            "Could not get width and height from terminal, resorting to hardcoded {} by {}",
            max_terminal_chars_x, max_terminal_chars_y
        );
    }

    let y_chars = calculate_other_side_by_aspect(
        max_terminal_chars_x,
        char_width,
        char_height,
        image_width,
        image_height,
    );

    if y_chars <= max_terminal_chars_y {
        return (y_chars, max_terminal_chars_x);
    }

    let x_chars = calculate_other_side_by_aspect(
        max_terminal_chars_y,
        char_height,
        char_width,
        image_height,
        image_width,
    );
    return (max_terminal_chars_y, x_chars);
}

fn main() {
    let args = Args::parse();

    let path = args.path.clone(); // PARTIAL CLONE>???????!?!?!?!??!>!

    let mut thing = Magic::new(args);

    // TRY IMAGE =====
    let reader_result = Reader::open(path.clone());
    if reader_result.is_err() {
        eprintln!("Cannot find file");
        return;
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        println!("{}", thing.do_magic(image));
        return;
    } else {
        eprintln!("Cannot open as an image");
        // TODO does the reader_result memmory get cleared???
    }

    // TRY VIDEO =====

    // maybe create an option to disable trying as video, but it doesnt really matter
    eprintln!("Trying to open as a video");

    crossterm::execute!(io::stdout(), EnterAlternateScreen);
    do_video_stuff(thing, path);
    crossterm::execute!(io::stdout(), LeaveAlternateScreen);
}

fn do_video_stuff(mut args: Magic, path: PathBuf) {
    let videoMagic = VideoThing::new(&path).unwrap();
    // print actual image
    loop {
        if let Ok(image) = videoMagic.getFrameAsImage(&path) {
            let result = args.do_magic(image);
            crossterm::execute!(io::stdout(), Clear(terminal::ClearType::All));
            println!("{}", result);
        } else {
            break;
        }

        // print actual image

        // TODO move up the amount of rows calculated, this still does not work, i think because you cant move up more than the terminal height
    }

    // TODO if the command gets terminated, the intermediate output does not get cleaned up
}

struct VideoThing {
    quality: String,
    start_time: Instant,
    length: f32,
    intermediate_output: String,
}
impl VideoThing {
    fn new(path: &PathBuf) -> Result<VideoThing, String> {
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

        // TODO allow user to decide this
        let quality = "5".to_string(); //nothig wrong with this being a string, as this will come from the user input later anyways
        let start_time = Instant::now();
        let length = String::from_utf8(command_result.unwrap().stdout)
            .unwrap()
            .replace("\n", "")
            .parse::<f32>()
            .unwrap();
        return Ok(VideoThing {
            quality,
            start_time,
            length,
            intermediate_output: "shit.png".to_string(),
        });
    }
    fn getFrameAsImage(&self, path: &PathBuf) -> Result<DynamicImage, String> {
        if self.length < self.start_time.elapsed().as_secs_f32() {
            return Err("out of video".to_string());
        }
        let command_result = Command::new("ffmpeg")
            .arg("-y")
            .args([
                "-ss",
                self.start_time.elapsed().as_secs_f32().to_string().as_str(),
            ])
            .args(["-i", path.to_str().unwrap()])
            .args(["-q:v", &self.quality])
            .args(["-frames:v", "1"])
            .arg(&self.intermediate_output)
            .output();

        if command_result.is_ok() {
            let reader_result = Reader::open(&self.intermediate_output);
            if reader_result.is_err() {
                eprintln!("Cannot find file");
                return Err("Cannot find file".to_string());
            }
            let image = reader_result.unwrap().decode().unwrap();
            let _ = Command::new("rm").arg(&self.intermediate_output).output();
            return Ok(image);
        }
        return Err("fuck you".to_string());
    }
}

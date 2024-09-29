use std::fmt::Debug;

use clap::{Parser, ValueEnum};
use image::{io::Reader, DynamicImage};

use crossterm::terminal;

mod processing;
use processing::image::*;

use std::process::Command;
// use processing::video::*;

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
        max_terminal_chars_y = size.1 as u32;
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

    // TRY IMAGE =====
    let reader_result = Reader::open(&args.path);
    if reader_result.is_err() {
        eprintln!("Cannot find file");
        return;
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        println!("{}", do_image_stuff(image, &args));
        return;
    } else {
        eprintln!("Cannot open as an image");
        // TODO does the reader_result memmory get cleared???
    }

    // TRY VIDEO =====

    // maybe create an option to disable trying as video, but it doesnt really matter
    eprintln!("Trying to open as a video");

    let intermediate_output = "output.jpg";
    for i in 0..5 {
        let quality = "5"; //nothig wrong with this being a string, as this will come from the user input later anyways
        let command_result = Command::new("ffmpeg")
            .arg("-y")
            .args(["-ss", &i.to_string()])
            .args(["-i", &args.path.to_str().unwrap()])
            .args(["-q:v", quality])
            .args(["-frames:v", "1"])
            .arg(intermediate_output)
            .output();

        if let Ok(good) = command_result {
            let reader_result = Reader::open(intermediate_output);
            if reader_result.is_err() {
                eprintln!("Cannot find file");
                return;
            }
            let img_result = reader_result.unwrap().decode();
            println!("{}", do_image_stuff(img_result.unwrap(), &args));
        }
    }

    let _ = Command::new("rm").arg(intermediate_output).output();
    // TODO delete intermedia output
}

fn do_image_stuff(image: DynamicImage, args: &Args) -> String {
    let (columns, rows) = get_cols_and_rows(
        args.char_width,
        args.char_height,
        args.height,
        args.width,
        image.width(),
        image.height(),
    );
    eprintln!("columns: {}, rows: {}", columns, rows);

    let pixel_info = process_image(image);

    let result = translate_to_text(
        pixel_info,
        columns,
        rows,
        args.set,
        args.color,
        args.inverted,
        args.no_lines,
    );

    // print actual image
    return result;
}

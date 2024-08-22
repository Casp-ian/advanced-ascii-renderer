use clap::{Parser, ValueEnum};
use image::io::Reader;

mod processing;
use processing::image::*;

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

    /// the width and height of a character in pixels, only use if the defaults dont suit your needs
    #[arg(long, num_args(2))]
    character_size: Option<Vec<u32>>,
}

#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
enum ColorSet {
    #[default]
    None,
    Simple,
}

#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
enum CharSet {
    #[default]
    Braile,
    Ascii,
    Numbers,
    Discord,
}

fn get_cols_and_rows(
    character_dimensions: Option<Vec<u32>>,
    columns: Option<u32>,
    rows: Option<u32>,
    image_width: u32,
    image_height: u32,
) -> (u32, u32) {
    let char_width: u32;
    let char_height: u32;
    if let Some(vec) = character_dimensions {
        char_width = vec.get(0).unwrap().clone();
        char_height = vec.get(1).unwrap().clone();
    } else {
        // TODO default to measured from terminal, if unavailable default to sensible numbers
        char_width = 10;
        char_height = 20;
    }

    let (columns, rows) = match (columns, rows) {
        (Some(x), Some(y)) => (x, y), // take user inputted cols and rows, resolution might be distorted tho
        (Some(x), None) => (
            x,
            calculate_other_dimension(x, char_height, char_width, image_height, image_width),
        ),
        (None, Some(y)) => (
            calculate_other_dimension(y, char_width, char_height, image_width, image_height),
            y,
        ),
        (None, None) => get_fitting_terminal(char_width, char_height, image_width, image_height),
    };

    println!("columns: {}, rows: {}", columns, rows);

    (columns, rows)
}

// TODO stupid func name, stupid var names
fn calculate_other_dimension(
    char1: u32,
    thicness1: u32,
    thicness2: u32,
    image1: u32,
    image2: u32,
) -> u32 {
    (char1 * (image2 / thicness2)) / (image1 / thicness1)
    // TODO decide how i want this rounded or not
}

fn get_fitting_terminal(
    char_width: u32,
    char_height: u32,
    image_width: u32,
    image_height: u32,
) -> (u32, u32) {
    // TODO get terminal max x and y
    // TODO this also might not be the best place to get those, maybe in the calling method
    let max_terminal_chars_x = 213;
    let max_terminal_chars_y = 55;

    let y_chars = calculate_other_dimension(
        max_terminal_chars_x,
        char_width,
        char_height,
        image_width,
        image_height,
    );

    if y_chars <= max_terminal_chars_y {
        return (y_chars, max_terminal_chars_x);
    }

    let x_chars = calculate_other_dimension(
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

    let reader_result = Reader::open(args.path);
    if reader_result.is_err() {
        println!("\x1b[33mCannot find image");
        return;
    }
    let img_result = reader_result.unwrap().decode();
    if img_result.is_err() {
        println!("\x1b[33mCannot open image");
        return;
    }
    let image = img_result.unwrap();

    let (columns, rows) = get_cols_and_rows(
        args.character_size,
        args.height,
        args.width,
        image.width(),
        image.height(),
    );

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

    println!("{}", result);
}

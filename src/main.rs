use std::{f32::consts::PI, u32};

use clap::{Parser, ValueEnum};
use image::{io::Reader, DynamicImage, GenericImageView, Rgba};

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

const BLACK: &str = "\x1b[30m";
const DARK_RED: &str = "\x1b[31m";
const DARK_GREEN: &str = "\x1b[32m";
const DARK_YELLOW: &str = "\x1b[33m";
const DARK_BLUE: &str = "\x1b[34m";
const DARK_MAGENTA: &str = "\x1b[35m";
const DARK_CYAN: &str = "\x1b[36m";
const LIGHT_GRAY: &str = "\x1b[37m";
const DARK_GRAY: &str = "\x1b[90m";
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const ORANGE: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MANGENTA: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";

#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
enum ColorSet {
    #[default]
    None,
    GreenBlue,
    BluePurple,
}

impl ColorSet {
    fn get_color_prefix(&self, pixel: Rgba<u8>) -> &str {
        if self == &ColorSet::None {
            return "";
        }

        let red = pixel.0[0];
        let green = pixel.0[1];
        let blue = pixel.0[2];

        // TODO do these color codes only work on linux terminal?
        if self == &ColorSet::GreenBlue {
            if green > blue {
                return GREEN;
            } else {
                return BLUE;
            }
        }

        if self == &ColorSet::BluePurple {
            if green > blue {
                return BLUE;
            } else {
                return DARK_MAGENTA;
            }
        }

        return "";
    }
}

#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
enum CharSet {
    #[default]
    Braile,
    Ascii,
    Numbers,
    Discord,
}

// honestly i dont think i had a good reason to put this method inside of the enum, TODO move everything except the char vec
impl CharSet {
    fn get_char(&self, pixel: &PixelData, no_lines: bool) -> &str {
        if !no_lines && pixel.edgeness > 0.75 {
            let dir = pixel.direction;
            if (dir < (2.0 * PI / 3.0) && dir > (PI / 3.0))
                || (dir < (-2.0 * PI / 3.0) && dir > (-1.0 * PI / 3.0))
            {
                return "-";
            }
            if ((dir < PI / 6.0) && (dir > -1.0 * PI / 6.0))
                || ((dir > 5.0 * PI / 6.0) || (dir < -5.0 * PI / 6.0))
            {
                return "|";
            }
            if ((dir > PI / 6.0) && (dir < PI / 3.0))
                || ((dir > -5.0 * PI / 6.0) && (dir < -2.0 * PI / 3.0))
            {
                return "/";
            }
            if ((dir < -1.0 * PI / 6.0) && (dir > -1.0 * PI / 3.0))
                || ((dir < 5.0 * PI / 6.0) && (dir > 2.0 * PI / 3.0))
            {
                return "\\";
            }
        }

        let set = match self {
            &CharSet::Ascii => vec![" ", ".", "\"", "+", "o", "?", "#"],
            &CharSet::Numbers => vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            &CharSet::Discord => vec![
                ":black_large_square:",
                ":new_moon:",
                ":elephant:",
                ":fog:",
                ":white_large_square:",
            ],
            &CharSet::Braile => vec!["⠀", "⢀", "⡈", "⡊", "⢕", "⢝", "⣫", "⣟", "⣿"],
        };
        let id: f32 = pixel.brightness * (set.len() - 1) as f32;

        return set[id.round() as usize];
    }
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

// returns brightness between 0 and 1
fn get_brightness(pixel: Rgba<u8>) -> f32 {
    let red = pixel.0[0] as f32;
    let green = pixel.0[1] as f32;
    let blue = pixel.0[2] as f32;
    let alpha = pixel.0[3] as f32;

    // source https://en.wikipedia.org/wiki/Relative_luminance
    return ((red * 0.2126) + (green * 0.7152) + (blue * 0.0722) * (alpha / 255.0)) / 255.0;
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

    println!("width: {}, height: {}", image.width(), image.height());

    let (columns, rows) = get_cols_and_rows(
        args.character_size,
        args.width,
        args.height,
        image.width(),
        image.height(),
    );

    let pixel_info = preprocess_image(image, args.no_lines);

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

struct PixelData {
    edgeness: f32,
    direction: f32,
    brightness: f32,
    // TODO color???
}

// this will no doubt use a lot of memmory, no clue what to do about it tho :p
fn preprocess_image(image: DynamicImage, lines: bool) -> Vec<Vec<PixelData>> {
    let mut result: Vec<Vec<PixelData>> = vec![];
    for y in 0..image.height() {
        result.insert(y as usize, vec![]);
        for x in 0..image.width() {
            // TODO this calls .get_pixel() 9 times as much as it actually needs to be called
            // 1, 2, 3
            // 4, 5, 6
            // 7, 8, 9

            let pix_1 = if !(x == 0 || y == 0) {
                get_brightness(image.get_pixel(x - 1, y - 1))
            } else {
                0.0
            };
            let pix_2 = if !(y == 0) {
                get_brightness(image.get_pixel(x, y - 1))
            } else {
                0.0
            };
            let pix_3 = if !(x >= image.width() - 1 || y == 0) {
                get_brightness(image.get_pixel(x + 1, y - 1))
            } else {
                0.0
            };
            let pix_4 = if !(x == 0) {
                get_brightness(image.get_pixel(x - 1, y))
            } else {
                0.0
            };
            let pix_5 = get_brightness(image.get_pixel(x, y));
            let pix_6 = if !(x >= image.width() - 1) {
                get_brightness(image.get_pixel(x + 1, y))
            } else {
                0.0
            };
            let pix_7 = if !(x == 0 || y >= image.height() - 1) {
                get_brightness(image.get_pixel(x - 1, y + 1))
            } else {
                0.0
            };
            let pix_8 = if !(y >= image.height() - 1) {
                get_brightness(image.get_pixel(x, y + 1))
            } else {
                0.0
            };
            let pix_9 = if !(x >= image.width() - 1 || y >= image.height() - 1) {
                get_brightness(image.get_pixel(x + 1, y + 1))
            } else {
                0.0
            };

            // convolve
            let gx = (pix_1)
                + (pix_3 * -1.0)
                + (pix_4 * 2.0)
                + (pix_6 * -2.0)
                + (pix_7)
                + (pix_9 * -1.0);
            let gy = (pix_1)
                + (pix_7 * -1.0)
                + (pix_2 * 2.0)
                + (pix_8 * -2.0)
                + (pix_3)
                + (pix_9 * -1.0);

            // process

            let edgeness = (gx.powi(2) + gy.powi(2)).sqrt();
            let direction = gy.atan2(gx);
            // let edgeness = 0.0;
            // let direction = 0.0;
            let brightness = pix_5;

            // store
            result.get_mut(y as usize).unwrap().insert(
                x as usize,
                PixelData {
                    edgeness,
                    direction,
                    brightness,
                },
            );
        }
    }
    return result;
}

fn translate_to_text(
    image: Vec<Vec<PixelData>>,
    columns: u32,
    rows: u32,
    set: CharSet,
    color: ColorSet,
    inverted: bool,
    no_lines: bool,
) -> String {
    let mut result = "".to_string();

    // iterate over parts of image
    for y in 0..columns {
        let pixel_y_min = (image.len() as f32 * y as f32 / columns as f32) as u32;
        let pixel_y_max = (image.len() as f32 * (y + 1) as f32 / columns as f32) as u32;

        for x in 0..rows {
            let pixel_x_min =
                (image.get(0).unwrap().len() as f32 * (x as f32 / rows as f32)) as u32;
            let pixel_x_max =
                (image.get(0).unwrap().len() as f32 * ((x + 1) as f32 / rows as f32)) as u32;

            // get average value of part
            let pixel =
                get_center_pixel(&image, pixel_x_min, pixel_x_max, pixel_y_min, pixel_y_max);

            // println!("dir: {}, mag: {}", pixel.direction, pixel.edgeness);

            if inverted {
                // TODO invert brightness value
            }

            // place char in result string

            // TODO color here
            // result += color.get_color_prefix(pixel);

            result += set.get_char(pixel, no_lines);
        }
        result += "\n";
    }

    return result;
}

fn get_center_pixel(
    image: &Vec<Vec<PixelData>>,
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
) -> &PixelData {
    // println!("min x: {} y: {}", x_min, y_min);
    // println!("max x: {} y: {}", x_max, y_max);
    image
        .get(((y_min + y_max) / 2) as usize)
        .unwrap()
        .get(((x_min + x_max) / 2) as usize)
        .unwrap()
}

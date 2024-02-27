use std::u32;

use clap::{Parser, ValueEnum};
use image::{io::Reader, DynamicImage, GenericImageView, Pixel, Rgba};


// TODO add aspect ratio
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path of image
    path: std::path::PathBuf,

    /// Width in characters
    #[arg(long, default_value_t = 64)]
    width: u32,

    /// Color of text
    #[arg(long, default_value_t, value_enum)]
    color: ColorSet,

    /// Characters used for result
    #[arg(long, default_value_t, value_enum)]
    set: CharSet,

    /// Characters used for result
    #[arg(long)]
    inverted: bool,
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
    Filled,
    Braile,
    Discord,
}

impl CharSet {
    fn get_char(&self, pixel: Rgba<u8>) -> &str {
        let brightness = get_brightness(pixel);

        if self == &CharSet::Filled {

            // █#eo+,.     <- dont forget that one space is also a part

            if brightness > 215 {
                return "█";
            }
            if brightness > 180 {
                return "#";
            }
            if brightness > 145 {
                return "e";
            }
            if brightness > 110 {
                return "o";
            }
            if brightness > 75 {
                return ",";
            }
            if brightness > 40 {
                return ".";
            }
            return " ";
        }

        if self == &CharSet::Discord {

            // :white_large_square: :fog: :elephant: :new_moon:  :black_large_square: 

            if brightness > 200 {
                return ":white_large_square:";
            }
            if brightness > 150 {
                return ":fog:";
            }
            if brightness > 100 {
                return ":elephant:";
            }
            if brightness > 50 {
                return ":new_moon:";
            }
            return ":black_large_square:";
        }

        if self == &CharSet::Braile {

            // ⣿⣫⢕⡈⠀   <- dont forget the invisible character at the end

            if brightness > 200 {
                return "⣿";
            }
            if brightness > 150 {
                return "⣫";
            }
            if brightness > 100 {
                return "⢕";
            }
            if brightness > 50 {
                return "⡈";
            }
            // this is not a space, this is an empty braile character
            return "⠀";
        }

        return " ";
    }
}

fn get_brightness(pixel: Rgba<u8>) -> u8 {
    let red = pixel.0[0] as f64;
    let green = pixel.0[1] as f64;
    let blue = pixel.0[2] as f64;
    let alpha = pixel.0[3] as f64;

    // source https://en.wikipedia.org/wiki/Relative_luminance
    return ((red * 0.2126) + (green * 0.7152) + (blue * 0.0722) * (alpha / 255.0)) as u8;
}

fn main() {
    let args = Args::parse();

    // TODO is there a neater way to do this?
    let reader_result = Reader::open(args.path);
    if reader_result.is_err() {
        println!("\x1b[33mCant find image");
        return;
    }
    let img_result = reader_result.unwrap().decode();
    if img_result.is_err() {
        println!("\x1b[33mCant open image");
        return;
    }
    let image = img_result.unwrap();


    // match aspect ratio
    let img_aspect_ratio = image.width() as f32 / image.height() as f32;
    // todo this will differ between terminal and charset, might need to fix this but difference might be ignorable
    let char_aspect_ratio = 1 as f32; 

    let matched_height = (args.width as f32 / char_aspect_ratio / img_aspect_ratio).round() as u32;

    println!("{}", process_image(image, args.width, matched_height, args.set, args.color, args.inverted));
}

fn process_image(image: DynamicImage, width: u32, height: u32, set: CharSet, color: ColorSet, inverted: bool) -> String {

    let mut result = "".to_string();

    // iterate over parts of image
    for y in 0..height {
        let pixel_y_min = (image.height() as f32 * y as f32 / height as f32) as u32;
        let pixel_y_max = (image.height() as f32 * (y + 1) as f32 / height as f32) as u32 - 1;

        for x in 0..width {
            let pixel_x_min = (image.width() as f32 * x as f32 / width as f32) as u32;
            let pixel_x_max = (image.width() as f32 * (x + 1) as f32 / width as f32) as u32 - 1;
            
            // get average value of part
            let mut pixel = get_center_pixel(&image, pixel_x_min, pixel_x_max, pixel_y_min, pixel_y_max);

            if inverted {
                pixel.invert();
            }

            // place char in result string
            result += color.get_color_prefix(pixel);
            result += set.get_char(pixel);
        }
        result += "\n";
    }

    return result;
}

fn get_center_pixel(image: &DynamicImage, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> Rgba<u8> {
    image.get_pixel((x_min + x_max) / 2, (y_min + y_max) / 2)
}

// TODO, does this actually provide something?
fn get_blended_pixel(image: &DynamicImage, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> Rgba<u8> {
    let width = x_max - x_min;
    let height = y_max - y_min;

    let x = x_min;
    let y = y_min;

    // TODO clean up, for now it takes the average of 4 pixels spread around the given area
    let mut pixel1 = image.get_pixel(x + (    width / 4),  y + (    height / 4));
    let mut pixel2 = image.get_pixel(x + (    width / 4),  y + (3 * height / 4));
    let mut pixel3 = image.get_pixel(x + (3 * width / 4),  y + (    height / 4));
    let mut pixel4 = image.get_pixel(x + (3 * width / 4),  y + (3 * height / 4));

    pixel1.blend(&pixel2);
    pixel3.blend(&pixel4);

    pixel1.blend(&pixel3);

    return pixel1;
}

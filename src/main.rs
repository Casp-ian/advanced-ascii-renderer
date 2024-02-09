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

    /// Height in characters
    #[arg(long, default_value_t = 32)]
    height: u32,

    /// Color of text
    #[arg(long, default_value_t, value_enum)]
    color: ColorSet,

    /// Characters used for result
    #[arg(long, default_value_t, value_enum)]
    set: CharSet,
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
}

impl CharSet {
    fn get_char(&self, pixel: Rgba<u8>) -> &str {
        let brightness = get_brightness(pixel);

        if self == &CharSet::Filled {
            if brightness > 200 {
                return "#";
            }
            if brightness > 100 {
                return "/";
            }
            return " ";
        }

        if self == &CharSet::Braile {
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
            return "⠀";
        }

        return " ";
    }
}

fn get_brightness(pixel: Rgba<u8>) -> u8 {
    // TODO to luma? does it matter?
    let red = pixel.0[0];
    let green = pixel.0[1];
    let blue = pixel.0[2];
    let alpha = pixel.0[3];
    return (red / 3) + (green / 3) + (blue / 3) - (255 - alpha);
}

fn main() {
    let args = Args::parse();

    // TODO is there a neater way to do this?
    let reader_result = Reader::open(args.path);
    if reader_result.is_err() {
        return;
    }
    let img_result = reader_result.unwrap().decode();
    if img_result.is_err() {
        return;
    }
    let img = img_result.unwrap();

    // TODO change image to text
    println!("{}", process_image(img, args.width, args.height, args.set, args.color));
}


fn process_image(image: DynamicImage, width: u32, height: u32, set: CharSet, color: ColorSet) -> String {

    let part_height = image.height() / height;
    let part_width = image.width() / width;

    let mut result = "".to_string();

    // iterate over parts of image
    for y in 0..height {
        for x in 0..width {

            // get average value of part
            let pixel = get_blended_pixel(&image, part_width * x, part_height * y, part_width, part_height);

            // place char in result string
            result += color.get_color_prefix(pixel);
            result += set.get_char(pixel);
        }
        result += "\n";
    }

    return result;
}

fn get_blended_pixel(image: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> Rgba<u8> {
    let _ = (1..6).map(f64::from);

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
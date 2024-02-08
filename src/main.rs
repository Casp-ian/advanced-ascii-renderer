use clap::{Parser, ValueEnum};
use image::{io::Reader, DynamicImage, GenericImageView, Rgba};


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
    color: Color,

    /// Characters used for result
    #[arg(long, default_value_t, value_enum)]
    set: CharSet,
}


#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
enum Color {
    #[default]
    None,
    GreenBlue,
    BluePurple,
}

impl Color {
    fn get_color_prefix(&self, pixel: Rgba<u8>) -> &str {
        if self == &Color::None {
            return "";
        }

        // TODO redo colors
        let red = pixel.0[0];
        let green = pixel.0[1];
        let blue = pixel.0[2];

        // TODO do these color codes only work on linux terminal?
        if self == &Color::GreenBlue {
            if green > blue {
                return "\x1b[32m";
            } else {
                return "\x1b[36m";
            }
        }

        if self == &Color::BluePurple {
            if green > blue {
                return "\x1b[92m";
            } else {
                return "\x1b[94m";
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
            return " ";
        }

        return " ";
    }
}

fn get_brightness(pixel: Rgba<u8>) -> u8 {
    // TODO to luma?
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


fn process_image(image: DynamicImage, width: u32, height: u32, set: CharSet, color: Color) -> String {

    let part_height = image.height() / height;
    let part_width = image.width() / width;

    // TODO can optimize, most of the blurred pixels are never used
    // let blurred = image.blur(2.);
    let blurred = image.clone();

    let mut result = "".to_string();

    // iterate over parts of image
    for y in 0..height {
        for x in 0..width {
            // get average value of part
            let pixel = blurred.get_pixel((part_width * x) + (part_width / 2), (part_height * y) + (part_height / 2));
            // place char in result string
            result += color.get_color_prefix(pixel);
            result += set.get_char(pixel);
        }
        result += "\n";
    }

    return result;
}

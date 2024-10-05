use crate::processing::cpu::get_pixel_data;
use crate::processing::image::PixelData;

use crate::CharSet;
use crate::ColorSet;
use crate::Direction;

const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";
const GRAY: &str = "\x1b[90m";
const BRIGHT_RED: &str = "\x1b[91m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const BRIGHT_YELLOW: &str = "\x1b[93m";
const BRIGHT_BLUE: &str = "\x1b[94m";
const BRIGHT_MANGENTA: &str = "\x1b[95m";
const BRIGHT_CYAN: &str = "\x1b[96m";
const BRIGHT_WHITE: &str = "\x1b[97m";

pub fn translate_to_text(
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

            // TODO after we fix the whole resolution thing all data will already be inside the value, so this wont be needed anymore
            // TODO dont only get the center pixel, look at all pixels to decide the character, like in acerola's video
            let pixel = get_pixel_data(&image, pixel_x_min, pixel_x_max, pixel_y_min, pixel_y_max);

            result += get_ansi_color_code(&color, pixel.color.0).as_str();

            // place char in result string
            result += get_char(&set, pixel, inverted, no_lines).as_str();
        }
        result += "\n";
    }

    return result;
}

pub fn get_ansi_color_code(color_set: &ColorSet, color: [u8; 4]) -> String {
    let set: Vec<(&str, [u8; 4])>;

    if color_set == &ColorSet::ColorFull {
        set = vec![
            // (BLACK, [0, 0, 0, 0]),
            (RED, [170, 0, 0, 0]),
            (GREEN, [0, 170, 0, 0]),
            (YELLOW, [170, 170, 0, 0]),
            (BLUE, [0, 0, 170, 0]),
            (MAGENTA, [170, 0, 170, 0]),
            (CYAN, [0, 170, 170, 0]),
            // (WHITE, [170, 170, 170, 0]),
            // (GRAY, [85, 85, 85, 0]),
            (BRIGHT_RED, [255, 85, 85, 0]),
            (BRIGHT_GREEN, [85, 255, 85, 0]),
            (BRIGHT_YELLOW, [255, 255, 85, 0]),
            (BRIGHT_BLUE, [85, 85, 255, 0]),
            (BRIGHT_MANGENTA, [255, 85, 255, 0]),
            (BRIGHT_CYAN, [85, 255, 255, 0]),
            // (BRIGHT_WHITE, [255, 255, 255, 0]),
        ];
    } else if color_set == &ColorSet::All {
        set = vec![
            (BLACK, [0, 0, 0, 0]),
            (RED, [170, 0, 0, 0]),
            (GREEN, [0, 170, 0, 0]),
            (YELLOW, [170, 170, 0, 0]),
            (BLUE, [0, 0, 170, 0]),
            (MAGENTA, [170, 0, 170, 0]),
            (CYAN, [0, 170, 170, 0]),
            (WHITE, [170, 170, 170, 0]),
            (GRAY, [85, 85, 85, 0]),
            (BRIGHT_RED, [255, 85, 85, 0]),
            (BRIGHT_GREEN, [85, 255, 85, 0]),
            (BRIGHT_YELLOW, [255, 255, 85, 0]),
            (BRIGHT_BLUE, [85, 85, 255, 0]),
            (BRIGHT_MANGENTA, [255, 85, 255, 0]),
            (BRIGHT_CYAN, [85, 255, 255, 0]),
            (BRIGHT_WHITE, [255, 255, 255, 0]),
        ];
    } else if color_set == &ColorSet::FewColors {
        set = vec![
            // (BLACK, [0, 0, 0, 0]),
            // (RED, [170, 0, 0, 0]),
            // (GREEN, [0, 170, 0, 0]),
            // (YELLOW, [170, 170, 0, 0]),
            // (BLUE, [0, 0, 170, 0]),
            // (MAGENTA, [170, 0, 170, 0]),
            // (CYAN, [0, 170, 170, 0]),
            (WHITE, [170, 170, 170, 0]),
            // (GRAY, [85, 85, 85, 0]),
            (BRIGHT_RED, [255, 85, 85, 0]),
            (BRIGHT_GREEN, [85, 255, 85, 0]),
            (BRIGHT_YELLOW, [255, 255, 85, 0]),
            (BRIGHT_BLUE, [85, 85, 255, 0]),
            (BRIGHT_MANGENTA, [255, 85, 255, 0]),
            (BRIGHT_CYAN, [85, 255, 255, 0]),
            // (BRIGHT_WHITE, [255, 255, 255, 0]),
        ];
    } else if color_set == &ColorSet::Real {
        return format!("\x1b[38;2;{};{};{}m", color[0], color[1], color[2]).to_string();
    } else {
        return "".to_string();
    }

    // TODO this color quantization method sucks balls, maybe acerola can save us here as well https://www.youtube.com/watch?v=fv-wlo8yVhk
    let mut lowest_distance: usize = 0;
    let mut chosen_text: &str = "";

    for (text, text_color) in set {
        let distance = get_distance(color, text_color);
        if distance > lowest_distance {
            lowest_distance = distance;
            chosen_text = text;
        }
    }

    return chosen_text.to_string();
}

fn get_distance(one: [u8; 4], two: [u8; 4]) -> usize {
    let distance = ((one[0] + two[0]).pow(2)
        + (one[1] + two[1]).pow(2)
        + (one[2] + two[2]).pow(2)
        + (one[3] + two[3]).pow(2)) as f32;
    return distance.sqrt() as usize;
}

pub fn get_char(char_set: &CharSet, pixel: PixelData, inverted: bool, no_lines: bool) -> String {
    if !no_lines {
        match pixel.direction {
            Direction::TopToBottom => return "|".to_string(),
            Direction::LeftToRight => return "-".to_string(),
            Direction::TopleftToBotright => return "\\".to_string(),
            Direction::ToprightToBotleft => return "/".to_string(),
            _ => (),
        }
    }

    let set = match char_set {
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

    let brightness = if inverted {
        1.0 - pixel.brightness
    } else {
        pixel.brightness
    };
    let id: f32 = brightness * (set.len() - 1) as f32;

    return set[id.round() as usize].to_string();
}

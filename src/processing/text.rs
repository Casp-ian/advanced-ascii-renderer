use std::collections::HashMap;
use std::f32::consts::PI;
use std::usize::MAX;

use crate::processing::image::PixelData;

use crate::CharSet;
use crate::ColorSet;

// TODO find out on what enviorments these ansi codes work
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

pub fn get_color_prefix(color_set: &ColorSet, color: [u8; 4]) -> String {
    let set: Vec<(&str, [u8; 4])>;

    if color_set == &ColorSet::Simple {
        set = vec![
            // (BLACK, [0, 0, 0, 0]),
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
    } else {
        return "".to_string();
    }

    // i think this will be inverted but the results say otherwise, TODO revisit and make sense of it
    // let mut lowest_distance: usize = MAX;
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

// honestly i dont think i had a good reason to put this method inside of the enum, TODO move everything except the char vec
pub fn get_char(char_set: &CharSet, pixel: &PixelData, inverted: bool, no_lines: bool) -> String {
    if !no_lines && pixel.edgeness > 0.75 {
        let dir = pixel.direction;
        if (dir < (2.0 * PI / 3.0) && dir > (PI / 3.0))
            || (dir < (-2.0 * PI / 3.0) && dir > (-1.0 * PI / 3.0))
        {
            return "-".to_string();
        }
        if ((dir < PI / 6.0) && (dir > -1.0 * PI / 6.0))
            || ((dir > 5.0 * PI / 6.0) || (dir < -5.0 * PI / 6.0))
        {
            return "|".to_string();
        }
        if ((dir > PI / 6.0) && (dir < PI / 3.0))
            || ((dir > -5.0 * PI / 6.0) && (dir < -2.0 * PI / 3.0))
        {
            return "/".to_string();
        }
        if ((dir < -1.0 * PI / 6.0) && (dir > -1.0 * PI / 3.0))
            || ((dir < 5.0 * PI / 6.0) && (dir > 2.0 * PI / 3.0))
        {
            return "\\".to_string();
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

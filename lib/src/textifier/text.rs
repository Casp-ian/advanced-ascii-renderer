use super::types::*;

use crate::config::CharSet;
use crate::config::ColorSet;
use crate::config::Config;

// const BLACK: &str = "\x1b[30m";
// const RED: &str = "\x1b[31m";
// const GREEN: &str = "\x1b[32m";
// const YELLOW: &str = "\x1b[33m";
// const BLUE: &str = "\x1b[34m";
// const MAGENTA: &str = "\x1b[35m";
// const CYAN: &str = "\x1b[36m";
// const WHITE: &str = "\x1b[37m";
// const GRAY: &str = "\x1b[90m";
// const BRIGHT_RED: &str = "\x1b[91m";
// const BRIGHT_GREEN: &str = "\x1b[92m";
// const BRIGHT_YELLOW: &str = "\x1b[93m";
// const BRIGHT_BLUE: &str = "\x1b[94m";
// const BRIGHT_MANGENTA: &str = "\x1b[95m";
// const BRIGHT_CYAN: &str = "\x1b[96m";
// const BRIGHT_WHITE: &str = "\x1b[97m";

pub fn translate_to_text(args: &Config, data: Vec<Vec<PixelData>>) -> String {
    let mut result = "".to_string();

    // TODO also not really pixels
    // iterate over pixel data
    for (i, row) in data.iter().enumerate() {
        if i != 0 {
            result += "\n";
        }
        for data in row {
            result += get_ansi_color_code(&args.color, data.color.0).as_str();

            result += get_char(
                &args.set,
                data,
                args.inverted,
                args.no_lines,
                args.only_lines,
            )
            .as_str();
        }
    }

    return result;
}

pub fn get_ansi_color_code(color_set: &ColorSet, color: [u8; 3]) -> String {
    // TODO get some info on where the ansi codes work and dont
    // TODO this also needs to change for other color modes like html
    // TODO also color quantization, but that should happen in shader.wgsl
    if color_set == &ColorSet::All {
        return format!("\x1b[38;2;{};{};{}m", color[0], color[1], color[2]).to_string();
    } else {
        return "".to_string();
    }
}

pub fn get_char(
    char_set: &CharSet,
    pixel: &PixelData,
    inverted: bool,
    no_lines: bool,
    only_lines: bool,
) -> String {
    if !no_lines {
        match pixel.direction {
            Direction::Slash => return "/".to_string(),
            Direction::Backslash => return "\\".to_string(),
            Direction::Dash => return "-".to_string(), // NOTE this could be replaced with a em dash, but i think it looks better with the normal dash
            Direction::Bar => return "|".to_string(),
            Direction::Underscore => return "_".to_string(),
            _ => (),
        }
    }

    if !only_lines {
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

    return " ".to_string();
}

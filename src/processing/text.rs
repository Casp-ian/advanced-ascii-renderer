use std::f32::consts::PI;

use image::{DynamicImage, GenericImageView, Rgba};

use crate::processing::image::PixelData;

use crate::CharSet;
use crate::ColorSet;

// TODO find out on what enviorments these work
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

pub fn get_color_prefix(color_set: &ColorSet, pixel: Rgba<u8>) -> String {
    if color_set == &ColorSet::None {
        return "".to_string();
    }

    // TODO MEDIAN CUT ALGORITHM PLEASEEEEE
    let sRED: [u8; 4] = [255, 0, 0, 0];
    let sGREEN: [u8; 4] = [0, 255, 0, 0];
    let sBLUE: [u8; 4] = [0, 0, 255, 0];
    let sMAGENTA: [u8; 4] = [255, 0, 255, 0];
    let sCYAN: [u8; 4] = [0, 255, 255, 0];
    let sYELLOW: [u8; 4] = [255, 255, 0, 0];
    let sWHITE: [u8; 4] = [255, 255, 255, 0];
    let sBLACK: [u8; 4] = [0, 0, 0, 0];

    let dRED = get_distance(sRED, pixel.0);
    let dGREEN = get_distance(sGREEN, pixel.0);
    let dBLUE = get_distance(sBLUE, pixel.0);
    let dMAGENTA = get_distance(sMAGENTA, pixel.0);
    let dCYAN = get_distance(sCYAN, pixel.0);
    let dYELLOW = get_distance(sYELLOW, pixel.0);
    let dWHITE = get_distance(sWHITE, pixel.0);
    let dBLACK = get_distance(sBLACK, pixel.0);

    if dRED >= dGREEN && dRED >= dBLUE && dRED >= dMAGENTA && dRED >= dCYAN && dRED >= dYELLOW
    // && dRED >= dWHITE
    // && dRED >= dBLACK
    {
        return DARK_RED.to_string();
    }

    if dGREEN >= dRED
        && dGREEN >= dBLUE
        && dGREEN >= dMAGENTA
        && dGREEN >= dCYAN
        && dGREEN >= dYELLOW
    // && dGREEN >= dWHITE
    // && dGREEN >= dBLACK
    {
        return DARK_GREEN.to_string();
    }
    if dBLUE >= dGREEN && dBLUE >= dRED && dBLUE >= dMAGENTA && dBLUE >= dCYAN && dBLUE >= dYELLOW
    // && dBLUE >= dWHITE
    // && dBLUE >= dBLACK
    {
        return DARK_BLUE.to_string();
    }
    if dMAGENTA >= dGREEN
        && dMAGENTA >= dBLUE
        && dMAGENTA >= dRED
        && dMAGENTA >= dCYAN
        && dMAGENTA >= dYELLOW
    // && dMAGENTA >= dWHITE
    // && dMAGENTA >= dBLACK
    {
        return DARK_MAGENTA.to_string();
    }
    if dCYAN >= dGREEN && dCYAN >= dBLUE && dCYAN >= dMAGENTA && dCYAN >= dRED && dCYAN >= dYELLOW
    // && dCYAN >= dWHITE
    // && dCYAN >= dBLACK
    {
        return DARK_CYAN.to_string();
    }
    if dYELLOW >= dGREEN
        && dYELLOW >= dBLUE
        && dYELLOW >= dMAGENTA
        && dYELLOW >= dCYAN
        && dYELLOW >= dRED
    // && dYELLOW >= dWHITE
    // && dYELLOW >= dBLACK
    {
        return DARK_YELLOW.to_string();
    }
    // if dWHITE >= dGREEN
    //     && dWHITE >= dBLUE
    //     && dWHITE >= dMAGENTA
    //     && dWHITE >= dCYAN
    //     && dWHITE >= dRED
    //     && dWHITE >= dYELLOW
    //     && dWHITE >= dBLACK
    // {
    //     return WHITE.to_string();
    // }
    // if dBLACK >= dGREEN
    //     && dBLACK >= dBLUE
    //     && dBLACK >= dMAGENTA
    //     && dBLACK >= dCYAN
    //     && dBLACK >= dRED
    //     && dBLACK >= dWHITE
    //     && dBLACK >= dYELLOW
    // {
    //     return BLACK.to_string();
    // }

    return "".to_string();
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

use std::f32::consts::PI;

use image::{DynamicImage, GenericImageView, Rgba};

use crate::CharSet;
use crate::ColorSet;

use crate::processing::text::get_char;
use crate::processing::text::get_color_prefix;

#[derive(Clone)]
pub struct PixelData {
    pub direction: Direction,
    pub brightness: f32,
    pub color: Rgba<u8>,
}

#[derive(Clone)]
pub enum Direction {
    None,
    TopToBottom,
    ToprightToBotleft,
    TopleftToBotright,
    LeftToRight,
}

// this will no doubt use a lot of memory, no clue what to do about it tho :p
pub fn process_image(image: DynamicImage) -> Vec<Vec<PixelData>> {
    let edge_magnitude_threshold = 0.75;
    let mut result: Vec<Vec<PixelData>> = vec![];

    for y in 0..image.height() {
        result.insert(y as usize, vec![]);
        for x in 0..image.width() {
            // this calls .get_pixel() 9 times as much as it actually needs to be called, dont think it is avoidable or really bad tho
            // 1, 2, 3
            // 4, 5, 6
            // 7, 8, 9

            let center = image.get_pixel(x, y);

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
            let pix_5 = get_brightness(center);
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
            let edge_magnitude = (gx.powi(2) + gy.powi(2)).sqrt();
            let edge_direction = gy.atan2(gx);
            let brightness = pix_5;

            let direction: Direction;
            if edge_magnitude > edge_magnitude_threshold {
                let dir = edge_direction;
                if (dir < (2.0 * PI / 3.0) && dir > (PI / 3.0))
                    || (dir < (-2.0 * PI / 3.0) && dir > (-1.0 * PI / 3.0))
                {
                    direction = Direction::LeftToRight;
                } else if ((dir < PI / 6.0) && (dir > -1.0 * PI / 6.0))
                    || ((dir > 5.0 * PI / 6.0) || (dir < -5.0 * PI / 6.0))
                {
                    direction = Direction::TopToBottom;
                } else if ((dir > PI / 6.0) && (dir < PI / 3.0))
                    || ((dir > -5.0 * PI / 6.0) && (dir < -2.0 * PI / 3.0))
                {
                    direction = Direction::ToprightToBotleft;
                } else if ((dir < -1.0 * PI / 6.0) && (dir > -1.0 * PI / 3.0))
                    || ((dir < 5.0 * PI / 6.0) && (dir > 2.0 * PI / 3.0))
                {
                    direction = Direction::TopleftToBotright;
                } else {
                    direction = Direction::None;
                }
            } else {
                direction = Direction::None;
            }

            // store
            result.get_mut(y as usize).unwrap().insert(
                x as usize,
                PixelData {
                    direction,
                    brightness,
                    color: center,
                },
            );
        }
    }
    return result;
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

            // TODO dont only get the center pixel, look at all pixels to decide the character, like in acerola's video
            let pixel = get_pixel_data(&image, pixel_x_min, pixel_x_max, pixel_y_min, pixel_y_max);

            // TODO color here
            result += get_color_prefix(&color, pixel.color.0).as_str();

            // place char in result string
            result += get_char(&set, pixel, inverted, no_lines).as_str();
        }
        result += "\n";
    }

    return result;
}

fn get_pixel_data(
    image: &Vec<Vec<PixelData>>,
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
) -> PixelData {
    let mut count_top_to_bottom = 0;
    let mut count_topright_to_botleft = 0;
    let mut count_topleft_to_botright = 0;
    let mut count_left_to_right = 0;
    let mut count_none = 0;

    for y in y_min..y_max {
        for x in x_min..x_max {
            match image
                .get(y as usize)
                .unwrap()
                .get(x as usize)
                .unwrap()
                .direction
            {
                Direction::TopToBottom => count_top_to_bottom = count_top_to_bottom + 1,
                Direction::LeftToRight => count_left_to_right = count_left_to_right + 1,
                Direction::TopleftToBotright => {
                    count_topleft_to_botright = count_topleft_to_botright + 1
                }
                Direction::ToprightToBotleft => {
                    count_topright_to_botleft = count_topright_to_botleft + 1
                }
                Direction::None => count_none = count_none + 1,
            }
        }
    }

    let average_direction;
    if count_none >= (y_max - y_min) * (x_max - x_min) {
        average_direction = Direction::None;
    } else if count_top_to_bottom > count_topright_to_botleft
        && count_top_to_bottom > count_topleft_to_botright
        && count_top_to_bottom > count_left_to_right
    {
        average_direction = Direction::TopToBottom;
    } else if count_topright_to_botleft > count_topleft_to_botright
        && count_topright_to_botleft > count_left_to_right
    {
        average_direction = Direction::ToprightToBotleft;
    } else if count_topleft_to_botright > count_left_to_right {
        average_direction = Direction::TopleftToBotright;
    } else {
        average_direction = Direction::LeftToRight;
    }

    let mut result = image
        .get(((y_min + y_max) / 2) as usize)
        .unwrap()
        .get(((x_min + x_max) / 2) as usize)
        .unwrap()
        .clone();

    result.direction = average_direction;

    result
}

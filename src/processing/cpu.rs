use crate::{processing::image::*, Args};
use image::{DynamicImage, GenericImageView, Pixel, Rgba};
use std::f32::consts::PI;

// this will no doubt use a lot of memory, no clue what to do about it tho :p
pub fn process_on_cpu(
    image: DynamicImage,
    width: u32,
    height: u32,
    args: &Args,
) -> Vec<Vec<PixelData>> {
    let pixels_per_char: u32 = 5; //TODO make this not ugly
    let resized_image = image.resize(
        width * pixels_per_char,
        height * pixels_per_char,
        image::imageops::FilterType::Triangle,
    ); // TODO

    let edge_magnitude_threshold = 1.0; // TODO make in args
    let mut result: Vec<Vec<PixelData>> = vec![]; // Change to slices because we have width and height of output now

    for y in 0..resized_image.height() {
        result.insert(y as usize, vec![]);
        for x in 0..resized_image.width() {
            convolute(&resized_image, x, y, edge_magnitude_threshold, &mut result);
        }
    }
    return result;
}

fn convolute(
    image: &DynamicImage,
    x: u32,
    y: u32,
    edge_magnitude_threshold: f32,
    result: &mut Vec<Vec<PixelData>>,
) {
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
    let gx = (pix_1) + (pix_3 * -1.0) + (pix_4 * 2.0) + (pix_6 * -2.0) + (pix_7) + (pix_9 * -1.0);
    let gy = (pix_1) + (pix_7 * -1.0) + (pix_2 * 2.0) + (pix_8 * -2.0) + (pix_3) + (pix_9 * -1.0);

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
            color: center.to_rgb(),
        },
    );
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

pub fn get_pixel_data(
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

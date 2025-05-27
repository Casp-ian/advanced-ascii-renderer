use image::{DynamicImage, GenericImageView, Rgb};

use super::PixelData;

pub fn simple(
    image: &DynamicImage,
    image_width: u32,
    image_height: u32,
    columns: u32,
    rows: u32,
) -> Result<Vec<Vec<PixelData>>, String> {
    let mut result: Vec<Vec<PixelData>> = vec![];
    for y in 0..rows {
        result.push(vec![]);
        for x in 0..columns {
            let actual_x = (image_width / (columns + 1)) * x;
            let actual_y = (image_height / (rows + 1)) * y;
            let pixel = image.get_pixel(actual_x, actual_y).0;
            let color: Rgb<u8> = Rgb([pixel[0], pixel[1], pixel[2]]);
            let brightness = (pixel[0] as f32 / 256.0 * 0.2126)
                + (pixel[1] as f32 / 256.0 * 0.7152)
                + (pixel[2] as f32 / 256.0 * 0.0722);
            result.get_mut(y as usize).unwrap().push(PixelData {
                direction: super::Direction::None,
                brightness,
                color,
            });
        }
    }
    return Ok(result);
}

// TODO
// should probably have a version with parity with the gpu version, and a performant one
// performant should be done first
// also its fine to disable things like line detection for now for the performant version

use std::f32::consts::PI;
use std::io::Cursor;

use image::{DynamicImage, GenericImageView, Rgb, Rgba};
use pollster::FutureExt;

use super::gpu::WgpuContext;
use crate::processing::gpu;
use crate::processing::terminal::get_cols_and_rows;
use crate::{translate_to_text, Args};

#[derive(Clone, Debug)]
pub struct PixelData {
    pub direction: Direction,
    pub brightness: f32,
    pub color: Rgb<u8>,
}

#[derive(Clone, Debug)]
pub enum Direction {
    None,
    TopToBottom,
    ToprightToBotleft,
    TopleftToBotright,
    LeftToRight,
}

pub struct Magic {
    args: Args,
    gpu: Option<WgpuContext>,
}
impl Magic {
    pub fn new(args: Args) -> Magic {
        // NOTE gpu gets setup on the first run, because we need image aspect ratio for it
        return Magic { args, gpu: None };
    }

    fn setup_gpu(&mut self, columns: u32, rows: u32) {
        self.gpu = Some(
            gpu::WgpuContext::setup(256, 160, columns, rows)
                .block_on()
                .unwrap(),
        );
    }

    pub fn do_magic(&mut self, image: DynamicImage) -> String {
        let (image_width, image_height) = image.dimensions();
        let (columns, rows) = get_cols_and_rows(
            self.args.char_width,
            self.args.char_height,
            self.args.width,
            self.args.height,
            image_width,
            image_height,
        );

        eprintln!("c{}, r{}", columns, rows);

        // TODO handle failure
        if self.gpu.is_none() {
            self.setup_gpu(columns, rows);
        }
        let gpu = self.gpu.as_ref().unwrap();

        let color_buffer = image
            .resize_to_fill(256, 160, image::imageops::FilterType::Nearest)
            .to_rgba8();
        let buffer = gpu.process(color_buffer).block_on().unwrap();

        let data: Vec<PixelData> = buffer
            .chunks_exact(6)
            .map(|x| PixelData {
                direction: get_direction(x[0], x[1]),
                color: Rgb([
                    (x[2] * 255.0) as u8,
                    (x[3] * 255.0) as u8,
                    (x[4] * 255.0) as u8,
                ]),
                brightness: x[5],
            })
            .collect();

        let data = data
            .chunks(columns as usize)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<PixelData>>>();

        let result_string = translate_to_text(
            data,
            self.args.set,
            self.args.color,
            self.args.inverted,
            self.args.no_lines,
        );

        return result_string;
    }
}

fn get_direction(gx: f32, gy: f32) -> Direction {
    let magnitude_threshold = 0.8;
    let magnitude = (gx.powi(2) + gy.powi(2)).sqrt();
    let dir = gy.atan2(gx);

    let direction: Direction;
    if magnitude > magnitude_threshold {
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

    return direction;
}

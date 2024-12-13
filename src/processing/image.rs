use std::f32::consts::PI;
use std::io::Cursor;

use image::{DynamicImage, Rgb, Rgba};
use pollster::FutureExt;

use crate::processing::cpu::process_on_cpu;
use crate::processing::gpu;
use crate::{get_cols_and_rows, translate_to_text, Args};

use super::gpu::WgpuContext;

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

    fn setup_gpu(&mut self) {
        self.gpu = Some(
            gpu::WgpuContext::setup(256, 160, 256, 160)
                .block_on()
                .unwrap(),
        );
    }

    pub fn do_magic(&mut self, image: DynamicImage) -> String {
        let (columns, rows) = get_cols_and_rows(
            self.args.char_width,
            self.args.char_height,
            self.args.height,
            self.args.width,
            256, // TODO configurable
            160,
        );

        // TODO handle failure
        if self.gpu.is_none() {
            self.setup_gpu();
        }
        let gpu = self.gpu.as_ref().unwrap();

        let color_buffer = image
            .resize_to_fill(256, 160, image::imageops::FilterType::Nearest)
            .to_rgba8();
        let buffer = gpu.process(color_buffer.clone()).block_on().unwrap();

        // println!("gpu returned {:?}", buffer);

        // change buffer of random u8s to something legible
        let data: Vec<PixelData> = buffer
            .chunks_exact(4)
            .zip(color_buffer.chunks_exact(4))
            .map(|x| PixelData {
                direction: get_direction(x.0[0], x.0[1]),
                // direction: Direction::None,
                brightness: get_brightness(x.1),
                color: Rgb(x.1[0..3].try_into().expect("color buffer is broken")),
            })
            .collect();

        let result_string = translate_to_text(
            data.chunks(256)
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<PixelData>>>(),
            columns,
            rows,
            self.args.set,
            self.args.color,
            self.args.inverted,
            self.args.no_lines,
        );

        return result_string;
    }
}

fn get_direction(gx: i8, gy: i8) -> Direction {
    let gx = gx as f32 / 128.0;
    let gy = gy as f32 / 128.0;

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
// returns brightness between 0 and 1
fn get_brightness(colors: &[u8]) -> f32 {
    let red = colors[0] as f32;
    let green = colors[1] as f32;
    let blue = colors[2] as f32;
    let alpha = colors[3] as f32;

    // source https://en.wikipedia.org/wiki/Relative_luminance
    return ((red * 0.2126) + (green * 0.7152) + (blue * 0.0722) * (alpha / 255.0)) / 255.0;
}

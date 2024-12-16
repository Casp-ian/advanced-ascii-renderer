use image::{DynamicImage, GenericImageView, Rgb};
use pollster::FutureExt;

use super::gpu::WgpuContext;
use crate::processing::gpu;
use crate::processing::terminal::get_cols_and_rows;
use crate::processing::text::translate_to_text;
use crate::Args;

// TODO this struct actually is used for characters, but also isnt really character data, i dont know what to name this
// but when you rename it, rename it in shader.wgsl too
#[derive(Clone, Debug)]
pub struct PixelData {
    pub direction: Direction,
    pub brightness: f32,
    pub color: Rgb<u8>,
}

#[derive(Clone, Debug, Copy)]
pub enum Direction {
    None,
    TopToBottom,
    ToprightToBotleft,
    LeftToRight,
    TopleftToBotright,
}
impl Direction {
    fn test(int: u32) -> Direction {
        match int {
            0 => Direction::None,
            1 => Direction::TopToBottom,
            2 => Direction::ToprightToBotleft,
            3 => Direction::LeftToRight,
            4 => Direction::TopleftToBotright,
            _ => panic!(),
        }
    }
}

pub struct Textifier<'a> {
    args: &'a Args,
    gpu: Option<WgpuContext>,
}
impl<'b> Textifier<'b> {
    pub fn new<'a>(args: &'a Args) -> Textifier<'a> {
        // gpu gets setup on the first run, because we need image aspect ratio for it
        return Textifier { args, gpu: None };
    }

    fn setup_gpu(&mut self, gpu_image_width: u32, gpu_image_height: u32, columns: u32, rows: u32) {
        self.gpu = Some(
            gpu::WgpuContext::setup(gpu_image_width, gpu_image_height, columns, rows)
                .block_on()
                .unwrap(),
        );
    }

    pub fn to_text(&mut self, image: DynamicImage) -> String {
        let (image_width, image_height) = image.dimensions();
        let (columns, rows) = get_cols_and_rows(
            self.args.char_width,
            self.args.char_height,
            self.args.width,
            self.args.height,
            image_width,
            image_height,
        );

        if self.gpu.is_none() {
            self.setup_gpu(image_width, image_height, columns, rows);
        }

        let gpu = self.gpu.as_ref().unwrap();
        let buffer = gpu.process(image.to_rgba8()).block_on().unwrap();

        let data: Vec<PixelData> = buffer
            .chunks_exact(3)
            .map(|x| PixelData {
                direction: Direction::test(bytemuck::cast(x[0])),
                color: Rgb([
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[0],
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[1],
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[2],
                ]),
                brightness: x[2],
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

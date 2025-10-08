use image::{DynamicImage, Luma, Rgb};
use lines::get_line_pieces;
use pollster::FutureExt;

mod cpu;
mod gpu;
mod lines;
mod text;
mod types;

use self::gpu::WgpuContext;
use self::text::translate_to_text;
use self::types::*;
use crate::Args;

pub struct Textifier<'a> {
    args: &'a Args,
    abandon_gpu: bool,
    input_width: u32,
    input_height: u32,
    output_width: u32,
    output_height: u32,
    gpu: Option<WgpuContext>,
}
impl<'b> Textifier<'b> {
    pub fn new<'a>(
        args: &'a Args,
        input_width: u32,
        input_height: u32,
        output_width: u32,
        output_height: u32,
    ) -> Textifier<'a> {
        // process character magic
        return Textifier {
            args,
            abandon_gpu: false,
            gpu: None,
            input_width,
            input_height,
            output_width,
            output_height,
        };
    }

    fn setup_gpu(
        &mut self,
        gpu_image_width: u32,
        gpu_image_height: u32,
        columns: u32,
        rows: u32,
        line_pieces: image::ImageBuffer<Luma<u8>, Vec<u8>>,
    ) -> Result<(), String> {
        let context = gpu::WgpuContext::setup(
            gpu_image_width,
            gpu_image_height,
            columns,
            rows,
            self.args.threshold,
            line_pieces,
        )
        .block_on();
        match context {
            Err(e) => {
                return Err(e.to_string());
            }
            Ok(context) => {
                self.gpu = Some(context);
                return Ok(());
            }
        }
    }

    fn run_cpu(&mut self, image: &DynamicImage) -> Result<Vec<Vec<PixelData>>, String> {
        return cpu::simple(
            &image,
            self.input_width,
            self.input_height,
            self.output_width,
            self.output_height,
        );
    }

    fn run_gpu(&mut self, image: &DynamicImage) -> Result<Vec<Vec<PixelData>>, String> {
        if self.gpu.is_none() {
            self.setup_gpu(
                self.input_width,
                self.input_height,
                self.output_width,
                self.output_height,
                get_line_pieces(),
            )?;
        }

        let gpu = self.gpu.as_ref().unwrap();

        // maybe this should be moved to gpu module?
        let raw_data = gpu.process(image.to_rgba8()).block_on()?;

        let single_vec_data: Vec<PixelData> = raw_data
            .chunks_exact(3)
            .map(|x| PixelData {
                direction: Direction::from_int(bytemuck::cast(x[0])),
                color: Rgb([
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[0],
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[1],
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[2],
                ]),
                brightness: x[2],
            })
            .collect();

        let data = single_vec_data
            .chunks(self.output_width as usize)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<PixelData>>>();

        return Ok(data);
    }

    fn run_try(&mut self, image: &DynamicImage) -> Result<Vec<Vec<PixelData>>, String> {
        if self.abandon_gpu {
            return self.run_cpu(image);
        }

        let data = self.run_gpu(image);
        if data.is_err() {
            eprintln!(
                // "gpu failed, running as cpu_simple, if you want full features on cpu run with cpu_full. keep in mind it can be very slow"
                "gpu failed, running as cpu_simple, cpu_full is not yet implemented"
            );
            self.abandon_gpu = true;
            return self.run_cpu(image);
        }
        return data;
    }

    pub fn to_text(&mut self, image: DynamicImage) -> Result<String, String> {
        let data: Result<Vec<Vec<PixelData>>, String>;
        data = match self.args.processing_mode {
            None => self.run_try(&image),
            Some(crate::ProcessingModes::Gpu) => self.run_gpu(&image),
            Some(crate::ProcessingModes::CpuSimple) => self.run_gpu(&image),
            Some(crate::ProcessingModes::CpuFull) => todo!(),
        };

        if let Err(e) = data {
            return Err(e.to_string());
        }

        let result_string = translate_to_text(self.args, data.unwrap());

        return Ok(result_string);
    }
}

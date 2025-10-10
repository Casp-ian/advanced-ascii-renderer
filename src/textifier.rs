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
    input_width: u32,
    input_height: u32,
    output_width: u32,
    output_height: u32,
    abandon_gpu: bool,
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

        return gpu.process(image.to_rgba8()).block_on();
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

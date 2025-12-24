use image::{DynamicImage, Luma};
use lines::get_line_pieces;
use pollster::FutureExt;

mod cpu;
mod gpu;
mod lines;
mod text;
mod types;

use crate::args::{ProcessingModes, RenderOptions};

use self::gpu::WgpuContext;
use self::text::translate_to_text;
use self::types::*;

pub struct Textifier<'a> {
    args: &'a RenderOptions,
    mode: Option<ProcessingModes>,
    internal_scale: (u32, u32),
    // input_width: u32,
    // input_height: u32,
    output_scale: (u32, u32),
    // output_width: u32,
    // output_height: u32,
    gpu: Option<WgpuContext>,
}
impl<'b> Textifier<'b> {
    pub fn new<'a>(
        args: &'a RenderOptions,
        mode: Option<ProcessingModes>,
        internal_scale: (u32, u32),
        // input_width: u32,
        // input_height: u32,
        output_scale: (u32, u32),
        // output_width: u32,
        // output_height: u32,
    ) -> Textifier<'a> {
        return Textifier {
            args,
            mode,
            gpu: None,
            internal_scale,
            // input_width,
            // input_height,
            output_scale,
            // output_width,
            // output_height,
        };
    }

    fn setup_gpu(
        &mut self,
        gpu_image_width: u32,
        gpu_image_height: u32,
        columns: u32,
        rows: u32,
        // line_pieces: image::ImageBuffer<Luma<u8>, Vec<u8>>,
    ) -> Result<(), String> {
        let context = gpu::WgpuContext::setup(
            gpu_image_width,
            gpu_image_height,
            columns,
            rows,
            self.args.threshold,
            // line_pieces,
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

    fn run_cpu(&mut self, image: &DynamicImage) -> Result<Vec<Vec<CharacterData>>, String> {
        return cpu::simple(
            &image,
            self.internal_scale.0,
            self.internal_scale.1,
            self.output_scale.0,
            self.output_scale.1,
        );
    }

    fn run_gpu(&mut self, image: &DynamicImage) -> Result<Vec<Vec<CharacterData>>, String> {
        if self.gpu.is_none() {
            self.setup_gpu(
                self.internal_scale.0,
                self.internal_scale.1,
                self.output_scale.0,
                self.output_scale.1,
                // get_line_pieces(),
            )?;
        }

        let gpu = match self.gpu.as_ref() {
            Some(x) => x,
            // NOTE this should be actually impossible
            None => return Err("no gpu, even after setting up".to_string()),
        };

        return gpu.process(image.to_rgba8()).block_on();
    }

    fn run_try(&mut self, image: &DynamicImage) -> Result<Vec<Vec<CharacterData>>, String> {
        let data = self.run_gpu(image);
        if data.is_err() {
            eprintln!("gpu failed, switching mode");
            // Gpu failed, we are switching to cpu mode
            self.mode = Some(ProcessingModes::CpuSimple);
            return self.run_cpu(image);
        }

        // Gpu ran succesful, we are officially in gpu processing mode
        self.mode = Some(ProcessingModes::Gpu);
        return data;
    }

    pub fn to_text(&mut self, image: DynamicImage) -> Result<String, String> {
        let data: Result<Vec<Vec<CharacterData>>, String>;
        data = match self.mode {
            None => self.run_try(&image),
            Some(ProcessingModes::Gpu) => self.run_gpu(&image),
            Some(ProcessingModes::CpuSimple) => self.run_cpu(&image),
        };

        let result_string = match data {
            Err(e) => return Err(e.to_string()),
            Ok(x) => translate_to_text(self.args, x),
        };

        return Ok(result_string);
    }
}

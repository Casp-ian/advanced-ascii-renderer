use image::{DynamicImage, GenericImageView, Luma, Rgb};
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
use crate::config::{Config, ProcessingModes};

pub struct Textifier<'a> {
    args: &'a Config,
    abandon_gpu: bool,
    input_width: u32,
    input_height: u32,
    output_width: u32,
    output_height: u32,
    gpu: Option<WgpuContext>,
}
impl<'b> Textifier<'b> {
    pub fn new<'a>(args: &'a Config) -> Textifier<'a> {
        // process character magic
        return Textifier {
            args,
            abandon_gpu: false,
            gpu: None,
            input_width: 0, // we are treating 0 as None in this case, so we dont have to call unwrap on everything
            input_height: 0,
            output_width: 0,
            output_height: 0,
        };
    }

    fn setup_dimensions(&mut self, image: &DynamicImage) {
        // return if they are already set
        // im using 0 as unset because i dont want to have to .unwrap() every time
        if self.input_width != 0 {
            return;
        }

        let (image_width, image_height) = image.dimensions();

        if image_width == 0 || image_height == 0 || self.args.width == 0 || self.args.height == 0 {
            panic!("calculating dimensions failed, none of the values should be zero");
        }

        self.input_width = image_width;
        self.input_height = image_height;
        self.output_width = self.args.width;
        self.output_height = self.args.height;
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
            return cpu::simple(
                image,
                self.input_width,
                self.input_height,
                self.output_width,
                self.output_height,
            );
        }
        let data = self.run_gpu(image);
        if data.is_err() {
            eprintln!(
                // "gpu failed, running as cpu_simple, if you want full features on cpu run with cpu_full. keep in mind it can be very slow"
                "gpu failed, running as cpu_simple, cpu_full is not yet implemented"
            );
            self.abandon_gpu = true;
            return cpu::simple(
                image,
                self.input_width,
                self.input_height,
                self.output_width,
                self.output_height,
            );
        }
        return data;
    }

    pub fn to_text(&mut self, image: DynamicImage) -> Result<String, String> {
        self.setup_dimensions(&image);

        let data: Result<Vec<Vec<PixelData>>, String>;
        data = match self.args.processing_mode {
            ProcessingModes::Try => self.run_try(&image),
            ProcessingModes::Gpu => self.run_gpu(&image),
            ProcessingModes::CpuSimple => cpu::simple(
                &image,
                self.input_width,
                self.input_height,
                self.output_width,
                self.output_height,
            ),
            ProcessingModes::CpuFull => todo!(),
        };

        if let Err(e) = data {
            return Err(e.to_string());
        }

        let result_string = translate_to_text(self.args, data.unwrap());

        return Ok(result_string);
    }
}

use image::{DynamicImage, Rgb};

use crate::processing::cpu::process_on_cpu;
use crate::processing::gpu::try_process_on_gpu;
use crate::Args;

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

pub fn process_image(
    image: DynamicImage,
    width: u32,
    height: u32,
    args: &Args,
) -> Vec<Vec<PixelData>> {
    match try_process_on_gpu(image.clone(), width, height, args) {
        Ok(data) => return data,
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("gpu failed, trying cpu");
        }
    }
    return vec![];

    // FIXME disable cpu for now, i dont want to keep parity
    // return process_on_cpu(image.clone(), width, height, args);
}

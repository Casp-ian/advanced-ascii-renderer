use image::{DynamicImage, GenericImageView, Rgba};

use crate::processing::cpu::process_on_cpu;
use crate::processing::gpu;

#[derive(Clone)]
pub struct PixelData {
    pub direction: Direction,
    pub brightness: f32,
    pub color: Rgba<u8>,
}

#[derive(Clone)]
pub enum Direction {
    None,
    TopToBottom,
    ToprightToBotleft,
    TopleftToBotright,
    LeftToRight,
}

pub fn process_image(image: DynamicImage) -> Vec<Vec<PixelData>> {
    return process_on_cpu(image);
}

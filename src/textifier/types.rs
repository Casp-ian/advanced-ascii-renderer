use image::Rgb;

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
    pub fn from_int(int: u32) -> Direction {
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

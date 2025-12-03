use image::Rgb;

// TODO this struct actually is used for characters, but also isnt really character data, i dont know what to name this
// but when you rename it, rename it in shader.wgsl too
#[derive(Clone, Debug)]
pub struct CharacterData {
    pub direction: Direction,
    pub brightness: f32,
    pub color: Rgb<u8>,
}

// struct test
// pub char: Char,
// pub color1: Option<Rgb<u8>>,
// pub color2: Option<Rgb<u8>>,
//

#[derive(Clone, Debug, Copy)]
pub enum Direction {
    None,
    Slash,
    Backslash,
    Dash,
    Bar,
    Underscore,
}
impl Direction {
    pub fn from_int(int: u32) -> Direction {
        match int {
            0 => Direction::None,
            1 => Direction::Slash,
            2 => Direction::Backslash,
            3 => Direction::Dash,
            4 => Direction::Bar,
            5 => Direction::Underscore,
            _ => panic!("Received invalid direction enum from shader, {}", int),
        }
    }
}

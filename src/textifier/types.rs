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
    Slash,
    Backslash,
    Caret,
    V,
    LessThan,
    GreaterThan,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningCurlyBrackets,
    ClosingCurlyBrackets,
    Star,
    Underscore,
    VerticalBar,
    Plus,
    Equals,
    Minus,
    Semicolon,
    Comma,
}
impl Direction {
    pub fn from_int(int: u32) -> Direction {
        // TODO fix
        match int {
            0 => Direction::None,
            1 => Direction::VerticalBar,
            2 => Direction::Slash,
            3 => Direction::Minus,
            4 => Direction::Backslash,
            _ => panic!(),
        }
    }
}

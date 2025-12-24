pub struct Options {
    pub general: GeneralOptions,
    pub render: RenderOptions,
    pub video: VideoOptions,
}

pub struct GeneralOptions {
    /// Path of image
    pub path: std::path::PathBuf,

    /// choose how to read the file
    pub media_mode: Option<MediaModes>,

    /// choose wether to use gpu or cpu
    pub processing_mode: Option<ProcessingModes>,
}

pub struct RenderOptions {
    /// Output width in characters
    pub width: Option<u32>,

    /// Output height in characters
    pub height: Option<u32>,

    /// Color of text
    pub color: ColorSet,

    /// Characters used for result
    pub set: CharSet,

    /// make dark areas light, and light areas dark
    pub inverted: bool,

    /// from 0 to 1, this is the threshold for how clear of an edge something has to be
    pub threshold: f32,

    /// remove the lines characters like /-\|
    pub no_lines: bool,

    /// remove everythin appart from the lines
    pub only_lines: bool,

    // this can only be checked by getting the space taken per character, and the spacing between characters from the terminal,
    // i do not know how to get these, so for now we have hardcoded defaults
    /// the width of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    pub char_width: u32,

    /// the height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    pub char_height: u32,
}

impl Default for RenderOptions {
    fn default() -> Self {
        return Self {
            width: None,
            height: None,
            color: ColorSet::None,
            set: CharSet::Ascii,
            inverted: false,
            threshold: 0.5,
            no_lines: false,
            only_lines: false,
            char_width: 9,
            char_height: 20,
        };
    }
}

pub struct VideoOptions {
    /// Only affects videos, lower value is high quality, higher value is
    pub quality: u8,

    /// Only affects videos, sets audio volume, clamps to 100
    pub volume: u8,

    /// Only affects videos, sets ffmpeg format if ffmpeg cant auto detect
    pub format: Option<String>,
}
impl Default for VideoOptions {
    fn default() -> VideoOptions {
        return Self {
            quality: 1,
            volume: 0,
            format: None,
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MediaModes {
    /// just a single frame
    Image,
    /// textify frames as fast as it can, requires ffmpeg
    Video,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessingModes {
    /// runs on gpu, this is the main mode
    Gpu,
    /// runs on cpu but with less features than gpu
    CpuSimple,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSet {
    None,
    RGB,
}

// The actual arrays of characters used for the character sets could be stored inside this enum, but i dont think it really matters
// and if it does its an easy refactor for later, ill just keep it like this so its similar to the color set
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharSet {
    Ascii,
    Braile,
    Numbers,
    Discord,
}

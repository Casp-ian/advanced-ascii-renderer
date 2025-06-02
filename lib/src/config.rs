pub struct Config {
    /// Path of image
    pub path: std::path::PathBuf,

    /// Width in characters
    pub width: u32,

    /// Height in characters
    pub height: u32,

    /// Color of text
    pub color: ColorSet,

    /// Characters used for result
    pub set: CharSet,

    /// Only affects videos, lower value is high quality, higher value is, default 5?
    pub quality: u8,

    /// Only affects videos, sets audio volume, clamps to 100, default 0?
    pub volume: u8,

    /// Only affects videos, sets ffmpeg format if ffmpeg cant auto detect
    pub format: Option<String>,

    /// make dark areas light, and light areas dark
    pub inverted: bool,

    /// from -64 to 64, this is the threshold for how close a line needs to be to count, default 8.0
    pub threshold: f32,

    /// remove the lines characters like /-\|, default false
    pub no_lines: bool,

    /// remove everythin appart from the lines, default false
    pub only_lines: bool,

    // this can only be checked by getting the space taken per character, and the spacing between characters from the terminal,
    // i do not know how to get these, so for now we have hardcoded defaults
    /// the width of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    // #[arg(long, default_value_t = 9)]
    pub char_width: u32,

    /// the height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    // #[arg(long, default_value_t = 20)]
    pub char_height: u32,

    /// choose how to read the file
    // #[arg(long, default_value_t, value_enum)]
    pub media_mode: MediaModes,

    /// choose wether to use gpu or cpu
    // #[arg(long, default_value_t, value_enum)]
    pub processing_mode: ProcessingModes,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MediaModes {
    #[default]
    /// try image and then video if image fails
    Try,
    Image,
    /// textify frames as fast as it can, requires ffmpeg
    Video,
    /// just like video but for things like your webcam that dont have a set duration
    Stream,
    // TODO
    // /// just like video but wait for the entire thing to be rendered at a flat framerate before displaying
    // VideoPrerender,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ProcessingModes {
    #[default]
    /// try gpu and then cpu simple if gpu fails
    Try,
    Gpu,
    /// runs on cpu but with less features than gpu
    CpuSimple,
    /// runs on cpu but tries to look similar to gpu, might take a while
    CpuFull,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColorSet {
    #[default]
    None,
    All,
}

// The actual arrays of characters used for the character sets could be stored inside this enum, but i dont think it really matters
// and if it does its an easy refactor for later, ill just keep it like this so its similar to the color set
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CharSet {
    #[default]
    Ascii,
    Braile,
    Numbers,
    Discord,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: "".into(),
            width: 10,
            height: 10,
            color: ColorSet::default(),
            set: CharSet::default(),
            quality: 5,
            volume: 0,
            format: None,
            inverted: false,
            threshold: 8.0,
            no_lines: false,
            only_lines: false,
            char_width: 9,
            char_height: 20,
            media_mode: MediaModes::default(),
            processing_mode: ProcessingModes::default(),
        }
    }
}

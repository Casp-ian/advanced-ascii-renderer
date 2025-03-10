use clap::{Parser, ValueEnum};
use std::fmt::Debug;

/// Take an image and turn it into text
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path of image
    pub path: std::path::PathBuf,

    /// Width in characters
    #[arg(long)]
    pub width: Option<u32>,

    /// Height in characters
    #[arg(long)]
    pub height: Option<u32>,

    /// Color of text
    #[arg(long, default_value_t, value_enum)]
    pub color: ColorSet,

    /// Characters used for result
    #[arg(long, default_value_t, value_enum)]
    pub set: CharSet,

    /// Only affects videos, lower value is high quality, higher value is
    #[arg(short, long, default_value_t = 5)]
    pub quality: u8,

    /// Only affects videos, sets audio volume, clamps to 100
    #[arg(short, long, default_value_t = 0)]
    pub volume: u8,

    /// Only affects videos, sets ffmpeg format if ffmpeg cant auto detect
    #[arg(short, long)]
    pub format: Option<String>,

    /// make dark areas light, and light areas dark
    #[arg(long)]
    pub inverted: bool,

    /// remove the lines characters like /-\|
    #[arg(long)]
    pub no_lines: bool,

    /// remove everythin appart from the lines
    #[arg(long)]
    pub only_lines: bool,

    // this can only be checked by getting the space taken per character, and the spacing between characters from the terminal,
    // i do not know how to get these, so for now we have hardcoded defaults
    /// the width of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 10)]
    pub char_width: u32,

    /// the height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 18)]
    pub char_height: u32,

    /// choose how to read the file
    #[arg(long, default_value_t, value_enum)]
    pub media_mode: MediaModes,

    /// choose wether to use gpu or cpu
    #[arg(long, default_value_t, value_enum)]
    pub processing_mode: ProcessingModes,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
pub enum MediaModes {
    #[default]
    /// try image and then video if image fails
    Try,
    Image,
    /// textify frames as fast as it can, requires ffmpeg
    Video,
    /// just like video but for things like your webcam
    Stream,
    // TODO
    // /// just like video but wait for the entire thing to be rendered at a flat framerate before displaying
    // VideoPrerender,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
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

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
pub enum ColorSet {
    #[default]
    None,
    All,
}

// The actual arrays of characters used for the character sets could be stored inside this enum, but i dont think it really matters
// and if it does its an easy refactor for later, ill just keep it like this so its similar to the color set
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq)]
pub enum CharSet {
    #[default]
    Ascii,
    Braile,
    Numbers,
    Discord,
}

pub fn get_cli_args() -> Args {
    // TODO maybe do some erro handling here
    return Args::parse();
}

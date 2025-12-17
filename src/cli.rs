use clap::{Parser, ValueEnum, arg, builder::PossibleValue, command};

use crate::args::{
    CharSet, ColorSet, GeneralOptions, MediaModes, Options, ProcessingModes, RenderOptions,
    VideoOptions,
};

// TODO all of the default options are duplicated between Options and Args

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
    #[arg(long, default_value_t = ColorSet::None, value_enum)]
    pub color: ColorSet,

    /// Characters used for result
    #[arg(long, default_value_t = CharSet::Ascii, value_enum)]
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

    /// from 0 to 1, this is the threshold for how clear of an edge something has to be
    #[arg(long, default_value_t = 0.5)]
    pub threshold: f32,

    /// remove the lines characters like /-\|
    #[arg(long)]
    pub no_lines: bool,

    /// remove everythin appart from the lines
    #[arg(long)]
    pub only_lines: bool,

    // this can only be checked by getting the space taken per character, and the spacing between characters from the terminal,
    // i do not know how to get these, so for now we have hardcoded defaults
    /// the width of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 9)]
    pub char_width: u32,

    /// the height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 20)]
    pub char_height: u32,

    /// choose how to read the file
    #[arg(long, value_enum)]
    pub media: Option<MediaModes>,

    /// choose wether to use gpu or cpu
    #[arg(long, value_enum)]
    pub processing: Option<ProcessingModes>,
}

pub fn get_cli_args() -> Options {
    // TODO maybe do some erro handling here
    let args = Args::parse();

    let general = GeneralOptions {
        path: args.path,
        media_mode: args.media,
        processing_mode: args.processing,
    };

    let render = RenderOptions {
        width: args.width,
        height: args.height,
        color: args.color,
        set: args.set,
        inverted: args.inverted,
        threshold: args.threshold,
        no_lines: args.no_lines,
        only_lines: args.only_lines,
        char_width: args.char_width,
        char_height: args.char_height,
    };

    // TODO now i cant easily check if any of these have been explicitely set by the user, so i can give a warning about useless video settings if image mode is set
    let video = VideoOptions {
        quality: args.quality,
        volume: args.volume,
        format: args.format,
    };

    return Options {
        general,
        render,
        video,
    };
}

impl ValueEnum for ColorSet {
    fn value_variants<'a>() -> &'a [Self] {
        &[ColorSet::None, ColorSet::RGB]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            ColorSet::None => Some(PossibleValue::new("none")),
            ColorSet::RGB => Some(PossibleValue::new("rgb")),
        }
    }
}
impl ValueEnum for CharSet {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CharSet::Ascii,
            CharSet::Braile,
            CharSet::Numbers,
            CharSet::Discord,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            CharSet::Ascii => Some(PossibleValue::new("ascii")),
            CharSet::Braile => Some(PossibleValue::new("braile")),
            CharSet::Numbers => Some(PossibleValue::new("numbers")),
            CharSet::Discord => Some(PossibleValue::new("discord")),
        }
    }
}
impl ValueEnum for MediaModes {
    fn value_variants<'a>() -> &'a [Self] {
        &[MediaModes::Image, MediaModes::Video]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            MediaModes::Image => Some(PossibleValue::new("image")),
            MediaModes::Video => Some(PossibleValue::new("video")),
        }
    }
}
impl ValueEnum for ProcessingModes {
    fn value_variants<'a>() -> &'a [Self] {
        &[ProcessingModes::Gpu, ProcessingModes::CpuSimple]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            ProcessingModes::Gpu => Some(PossibleValue::new("gpu")),
            ProcessingModes::CpuSimple => Some(PossibleValue::new("cpu")),
        }
    }
}

use clap::{Parser, ValueEnum, arg, command};

use aar::args::{GeneralOptions, Options, RenderOptions, VideoOptions};

// TODO all of the default options are duplicated between Options and Args

/// Take an image and turn it into text
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path of image
    path: std::path::PathBuf,

    /// Width in characters
    #[arg(long)]
    width: Option<u32>,

    /// Height in characters
    #[arg(long)]
    height: Option<u32>,

    /// Color of text
    #[arg(long, value_enum, default_value_t)]
    color: Color,

    /// Characters used for result
    #[arg(long, value_enum, default_value_t)]
    set: CharSet,

    /// Only affects videos, lower value is high quality, higher value is
    #[arg(short, long, default_value_t = 5)]
    quality: u8,

    /// Only affects videos, sets audio volume, clamps to 100
    #[arg(short, long, default_value_t = 0)]
    volume: u8,

    /// Only affects videos, sets ffmpeg format if ffmpeg cant auto detect
    #[arg(short, long)]
    format: Option<String>,

    /// make dark areas light, and light areas dark
    #[arg(long)]
    inverted: bool,

    /// from 0 to 1, this is the threshold for how clear of an edge something has to be
    #[arg(long, default_value_t = 0.5)]
    threshold: f32,

    /// remove the lines characters like /-\|
    #[arg(long)]
    no_lines: bool,

    /// remove everythin appart from the lines
    #[arg(long)]
    only_lines: bool,

    // this can only be checked by getting the space taken per character, and the spacing between characters from the terminal,
    // i do not know how to get these, so for now we have hardcoded defaults
    /// the width of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 9)]
    char_width: u32,

    /// the height of a character in pixels, only use if the defaults dont suit your needs or dont match your font
    #[arg(long, default_value_t = 20)]
    char_height: u32,

    /// choose how to read the file
    #[arg(long, value_enum)]
    media: Option<Media>,

    /// choose wether to use gpu or cpu
    #[arg(long, value_enum)]
    mode: Option<Mode>,
}

pub fn get_cli_args() -> Options {
    // TODO maybe do some erro handling here
    let args = Args::parse();

    let general = GeneralOptions {
        path: args.path,
        media_mode: match args.media {
            Some(x) => Some(x.into()),
            None => None,
        },
        processing_mode: match args.mode {
            Some(x) => Some(x.into()),
            None => None,
        },
    };

    let render = RenderOptions {
        width: args.width,
        height: args.height,
        color: args.color.into(),
        set: args.set.into(),
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

// NOTE all of the enums need either a newly defined trait or type, to satisfy rusts orphan rule, a little annoying

#[derive(ValueEnum, Clone, Debug, Default)]
enum CharSet {
    #[default]
    Ascii,
    Braile,
    Numbers,
    Discord,
}
impl From<CharSet> for aar::args::CharSet {
    fn from(value: CharSet) -> Self {
        match value {
            CharSet::Ascii => aar::args::CharSet::Ascii,
            CharSet::Braile => aar::args::CharSet::Braile,
            CharSet::Numbers => aar::args::CharSet::Numbers,
            CharSet::Discord => aar::args::CharSet::Discord,
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum Color {
    #[default]
    None,
    RGB,
}
impl From<Color> for aar::args::ColorSet {
    fn from(value: Color) -> Self {
        match value {
            Color::None => aar::args::ColorSet::None,
            Color::RGB => aar::args::ColorSet::RGB,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum Media {
    Image,
    Video,
}
impl From<Media> for aar::args::MediaModes {
    fn from(value: Media) -> Self {
        match value {
            Media::Image => aar::args::MediaModes::Image,
            Media::Video => aar::args::MediaModes::Video,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Cpu,
    Gpu,
}
impl From<Mode> for aar::args::ProcessingModes {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Cpu => aar::args::ProcessingModes::CpuSimple,
            Mode::Gpu => aar::args::ProcessingModes::Gpu,
        }
    }
}

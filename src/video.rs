use std::process;
use std::time::{Duration, Instant};
use std::{env, fs, io::ErrorKind};

use image::{DynamicImage, ImageReader};

use crate::Args;

use crate::ffutils::{self, Meta, Pegger};

pub const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.bmp";

pub struct FrameGrabber<'a> {
    args: &'a Args,
    start_time: Instant,
    meta: Meta,
    pegger: Pegger,
}
impl<'b> FrameGrabber<'b> {
    pub fn new<'a>(
        args: &'a Args,
        internal_scale: &(u32, u32),
        meta: Meta,
    ) -> Result<FrameGrabber<'a>, String> {
        let start_time = Instant::now();

        // We use the pid so we can have multiple of our program running at the same time without issues
        let output_dir = env::temp_dir().join(process::id().to_string());

        match fs::create_dir(&output_dir) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        // TODO theoretically these could get out of sync if the surrounding code is too slow
        let pegger = Pegger::new(&args.path, &meta.fps, &args.format, &internal_scale).unwrap();

        // ffutils::start_getting_frames(&args.path &output_dir, &fps, &args.format, &internal_scale)?;
        if args.volume > 0 {
            // start audio sub-process
            ffutils::play_audio(&args.path, args.volume);
        }

        return Ok(FrameGrabber {
            args,
            start_time,
            meta,
            pegger,
        });
    }
}

impl<'a> Iterator for FrameGrabber<'a> {
    type Item = DynamicImage;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.pegger.yoink() {
            Some(buffer) => Some(DynamicImage::from(buffer)),
            None => None,
        };
    }
}

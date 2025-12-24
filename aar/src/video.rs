use std::path::PathBuf;

use image::DynamicImage;

use crate::args::VideoOptions;

use crate::ffutils::{self, Meta, Pegger};

pub const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.bmp";

pub struct FrameGrabber {
    pegger: Pegger,
}
impl FrameGrabber {
    pub fn new(
        path: &PathBuf,
        args: &VideoOptions,
        internal_scale: &(u32, u32),
        meta: Meta,
    ) -> Result<FrameGrabber, String> {
        // TODO theoretically these could get out of sync if the surrounding code is too slow
        let pegger = Pegger::new(&path, &meta.fps, &args.format, &internal_scale).unwrap();

        if args.volume > 0 {
            // start audio sub-process
            ffutils::play_audio(&path, args.volume);
        }

        return Ok(FrameGrabber { pegger });
    }
}

impl Iterator for FrameGrabber {
    type Item = DynamicImage;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.pegger.yoink() {
            Some(buffer) => Some(DynamicImage::from(buffer)),
            None => None,
        };
    }
}

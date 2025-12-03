use std::process;
use std::time::{Duration, Instant};
use std::{env, fs, io::ErrorKind};

use image::{DynamicImage, ImageReader};

use crate::Args;

use crate::ffutils::{self, Pegger};

pub const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.bmp";

pub struct FrameGrabber<'a> {
    args: &'a Args,
    start_time: Instant,
    fps: u8,
    duration: Option<Duration>,
    pegger: Pegger,
}
impl<'b> FrameGrabber<'b> {
    pub fn new<'a>(
        args: &'a Args,
        internal_scale: &(u32, u32),
        duration: &Option<f32>,
    ) -> Result<FrameGrabber<'a>, String> {
        let start_time = Instant::now();
        let duration: Option<Duration> = if let Some(duration) = duration {
            Some(Duration::from_secs_f32(*duration))
        } else {
            None
        };
        let fps: u8 = 15;

        // We use the pid so we can have multiple of our program running at the same time without issues
        let output_dir = env::temp_dir().join(process::id().to_string());

        match fs::create_dir(&output_dir) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        // TODO theoretically these could get out of sync if the surrounding code is too slow
        let pegger = Pegger::new(&args.path, &fps, &args.format, &internal_scale).unwrap();

        // ffutils::start_getting_frames(&args.path &output_dir, &fps, &args.format, &internal_scale)?;
        if args.volume > 0 {
            // start audio sub-process
            ffutils::play_audio(&args.path, args.volume);
        }

        return Ok(FrameGrabber {
            args,
            start_time,
            fps,
            duration,
            pegger,
        });
    }
}

impl<'a> Iterator for FrameGrabber<'a> {
    type Item = DynamicImage;

    fn next(&mut self) -> Option<Self::Item> {
        // match self.args.media_mode {
        //     Some(MediaModes::Stream) => (),
        //     _ => todo!(),
        // };
        return Some(DynamicImage::from(self.pegger.yoink()));
    }
}

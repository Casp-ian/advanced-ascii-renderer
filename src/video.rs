use std::process;
use std::time::Instant;
use std::{env, fs, io::ErrorKind};

use image::{DynamicImage, ImageReader};

use crate::{Args, MediaModes};

use crate::ffutils;

pub const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.bmp";

pub struct FrameGrabber<'a> {
    args: &'a Args,
    start_time: Instant,
    counter: u16,
}
impl<'b> FrameGrabber<'b> {
    pub fn new<'a>(args: &'a Args) -> Result<FrameGrabber<'a>, String> {
        let start_time = Instant::now();

        // We use the pid so we can have multiple of our program running at the same time without issues
        let output_dir = env::temp_dir().join(process::id().to_string());

        // TODO clean up after ourselves
        // match fs::remove_dir_all(&output_dir) {
        //     Err(e) if e.kind() == ErrorKind::NotFound => (), // it is already removed, we are happy
        //     Ok(_) => (),
        //     Err(e) => return Err(e.to_string()),
        // }

        match fs::create_dir(&output_dir) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        // TODO theoretically these could get out of sync if the surrounding code is too slow
        ffutils::start_getting_frames(
            &args.path,
            &output_dir,
            &args.quality,
            &args.format, //
        )?;
        if args.volume > 0 {
            // start audio sub-process
            ffutils::play_audio(&args.path, args.volume);
        }

        return Ok(FrameGrabber {
            args,
            start_time,
            counter: 0,
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

        self.counter += 1;

        let output_dir = env::temp_dir().join(process::id().to_string());
        let output_location = output_dir.join(format!("{:0>5}.bmp", &self.counter));

        loop {
            match ImageReader::open(&output_location) {
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    // eprintln!("counter {:0>5}, not yet here", &self.counter)
                } // it is already removed, we are happy
                Ok(x) => {
                    match x.decode() {
                        Ok(image) => {
                            fs::remove_file(output_location);
                            return Some(image);
                        }
                        // Err(e) if e.kind() == ErrorKind::NotFound => (),
                        _ => {
                            // NOTE this should only pass if invalid EOF error
                            // panic!()
                        }
                    }
                }
                Err(e) => panic!("{}", e.to_string()),
            }
        }
    }
}

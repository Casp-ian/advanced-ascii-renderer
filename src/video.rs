use std::{fs::remove_file, time::Instant};

use image::{DynamicImage, ImageReader};

use crate::{Args, MediaModes};

use crate::ffutils;

pub const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.bmp";

pub struct FrameGrabber<'a> {
    args: &'a Args,
    start_time: Instant,
}
impl<'b> FrameGrabber<'b> {
    pub fn new<'a>(args: &'a Args) -> Result<FrameGrabber<'a>, String> {
        let start_time = Instant::now();

        if args.volume > 0 {
            // start audio sub-process
            ffutils::play_audio(&args.path, args.volume);
        }

        return Ok(FrameGrabber { args, start_time });
    }
}

impl<'a> Iterator for FrameGrabber<'a> {
    type Item = DynamicImage;

    fn next(&mut self) -> Option<Self::Item> {
        // Dont do time if stream mode
        let time: Option<f32> = match self.args.media_mode {
            Some(MediaModes::Stream) => None,
            _ => Some(self.start_time.elapsed().as_secs_f32()),
        };

        let command_result = ffutils::get_frame_at(
            time,
            &self.args.path,
            &self.args.quality,
            &self.args.format,
            TEMPORARY_IMAGE_FILE_NAME,
        );

        match command_result {
            Err(e) => {
                // NOTE this should never happen
                eprintln!("{:?}", e);
                return None;
            }
            Ok(_) => {
                let reader_result = ImageReader::open(TEMPORARY_IMAGE_FILE_NAME);
                if let Err(e) = reader_result {
                    // probably would happen if the ffmpeg function failed, but then we would have stopped this earlier, so should never happen, TODO
                    eprintln!("{}", e.to_string());
                    return None;
                }
                let image = reader_result.unwrap().decode().unwrap();
                let _ = remove_file(TEMPORARY_IMAGE_FILE_NAME);
                return Some(image);
            }
        }
    }
}

use std::{fs::remove_file, time::Instant};

use image::{DynamicImage, io::Reader};

use crate::{Args, MediaModes};

use crate::ffutils;

pub const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.png";

// NOTE, ffmpeg-next or the other ffmpeg/video packages seemed quite large, and not have this specific usecase in mind
// so we just run the ffmpeg command of the system, it might be terrible, but it does work nice for now, and can be changed later
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

    pub fn grab(&self) -> Option<Result<DynamicImage, String>> {
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
            Err(e) => return Some(Err(e.to_string())),
            Ok(_) => {
                let reader_result = Reader::open(TEMPORARY_IMAGE_FILE_NAME);
                if let Err(_) = reader_result {
                    // TODO make sure this is actually a not found error
                    // TODO on the first run if the user typed a wrong path, it will just take this path and not show any error message
                    return None;
                }
                let image = reader_result.unwrap().decode().unwrap();
                let _ = remove_file(TEMPORARY_IMAGE_FILE_NAME);
                return Some(Ok(image));
            }
        }
    }
}

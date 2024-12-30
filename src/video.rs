use std::{
    fs::remove_file,
    process::{Command, Output},
    time::Instant,
};

use image::{io::Reader, DynamicImage};

use crate::{Args, MediaModes};

pub const TEMPORARY_IMAGE_FILE_NAME: &str = "ImageToTextTemp.png";

// NOTE, ffmpeg-next or the other ffmpeg/video packages seemed quite large, and not have this specific usecase in mind
// so we just run the ffmpeg command of the system, it might be terrible, but it does work nice for now, and can be changed later
pub struct FrameGrabber<'a> {
    args: &'a Args,
    start_time: Instant,
}
impl<'b> FrameGrabber<'b> {
    pub fn new<'a>(args: &'a Args) -> Result<FrameGrabber<'a>, String> {
        if args.volume > 0 {
            Command::new("ffplay")
                .args([args.path.to_str().unwrap()])
                .args(["-nodisp"])
                .args(["-autoexit"])
                .args(["-v", "quiet"])
                .args(["-volume", &args.volume.to_string()])
                .spawn()
                .expect("audio broke");
        }

        let start_time = Instant::now();

        return Ok(FrameGrabber { args, start_time });
    }

    pub fn grab(&self) -> Option<Result<DynamicImage, String>> {
        let command_result: std::io::Result<Output>;

        let mut command = &mut Command::new("ffmpeg");
        command = command.arg("-y");

        if let Some(format) = &self.args.format {
            command = command.args(["-f", format.as_str()]);
        }

        if self.args.media_mode != MediaModes::Stream {
            command = command.args([
                "-ss",
                self.start_time.elapsed().as_secs_f32().to_string().as_str(),
            ]);
        }

        command = command
            .args(["-i", &self.args.path.to_str().unwrap()])
            .args(["-q:v", &self.args.quality.to_string()])
            .args(["-frames:v", "1"])
            .arg(TEMPORARY_IMAGE_FILE_NAME);

        command_result = command.output();

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

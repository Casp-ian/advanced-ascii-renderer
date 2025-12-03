use std::{
    io::{BufReader, Read},
    process::{ChildStdout, Command, Stdio},
};

use image::{ImageBuffer, Rgb};

pub struct Pegger {
    buf: Box<[u8]>,
    reader: BufReader<ChildStdout>,
    x: u32,
    y: u32,
}

impl Pegger {
    pub fn new(
        input_file: &std::path::PathBuf,
        fps: &u8,
        format: &Option<String>,
        internal_scale: &(u32, u32),
    ) -> Result<Self, String> {
        let mut command = &mut Command::new("ffmpeg");
        command = command.arg("-y");

        // input format
        if let Some(format) = format {
            command = command.args(["-f", format.as_str()]);
        }

        command = command
            .args(["-readrate", "1.0"])
            .args(["-i", input_file.to_str().unwrap()])
            .args([
                "-vf",
                &format!(
                    "scale={}:{},fps={}/1",
                    internal_scale.0, internal_scale.1, fps
                ),
            ])
            .args(["-pix_fmt", "rgb24"])
            .args(["-f", "rawvideo"])
            .arg("pipe:1");

        // make sure output doesnt interupt our stdout
        command = command.stdout(Stdio::piped());
        command = command.stderr(Stdio::null()).stdin(Stdio::null());

        let child = command.spawn().expect("couldnt spawn");
        let stdout = child.stdout.unwrap();

        let reader = BufReader::new(stdout);

        let size = (internal_scale.0 * internal_scale.1 * 3) as usize;
        let buf = vec![0; size].into_boxed_slice();

        return Ok(Self {
            buf,
            reader,
            x: internal_scale.0,
            y: internal_scale.1,
        });
    }

    pub fn yoink(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        self.reader.read_exact(&mut self.buf).unwrap();

        // NOTE this clones buf, but i think the cloned value is pretty much the actual image, so thats optimal
        let test = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(self.x, self.y, self.buf.to_vec());

        return test.unwrap();
    }
}

pub fn play_audio(file_name: &std::path::PathBuf, volume: u8) {
    Command::new("ffplay")
        .args([file_name.to_str().unwrap()])
        .args(["-nodisp"])
        .args(["-autoexit"])
        .args(["-v", "quiet"])
        .args(["-volume", &volume.to_string()])
        .spawn()
        .expect("audio broke");
}

// TODO should just create a struct or enum for this return
pub fn get_meta(
    file_name: &std::path::PathBuf,
) -> Result<((u32, u32), Option<f32>, Option<u32>), String> {
    let output: std::process::Output = Command::new("ffprobe")
        .args([file_name.to_str().unwrap()])
        .args(["-v", "quiet"])
        .args(["-select_streams", "v:0"])
        .args(["-show_entries", "stream=width,height,duration,nb_frames"])
        .args(["-of", "csv=p=0"])
        .output()
        .expect("cant probe");

    if !output.status.success() {
        return Err("ffprobe failed, might not be available".to_string());
    }

    if output.stdout.is_empty() {
        return Err("file is not an image or video (or not parsable by ffmpeg)".to_string());
    }

    let stdout = str::from_utf8(&output.stdout).unwrap();

    let meta_string: Vec<&str> = stdout
        .trim_end_matches(&['\r', '\n']) // trim newline for windows and linux
        .split(',')
        .collect();

    let width: u32 = meta_string[0]
        .parse()
        .expect("no values gotten from ffprobe");
    let height: u32 = meta_string[1]
        .parse()
        .expect("no values gotten from ffprobe");

    // could have cleaner way of accounting for n/a
    let duration: Option<f32> = meta_string[2].parse().ok();
    let frames: Option<u32> = meta_string[3].parse().ok();

    return Ok(((width, height), duration, frames));
}

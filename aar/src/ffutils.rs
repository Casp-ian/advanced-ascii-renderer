use std::{
    io::{BufReader, ErrorKind, Read},
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
        fps: &Option<String>,
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
                    "scale={}:{},fps={}",
                    internal_scale.0,
                    internal_scale.1,
                    fps.clone().unwrap()
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

    pub fn yoink(&mut self) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        match self.reader.read_exact(&mut self.buf) {
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return None,
            _ => (),
        }

        // NOTE this clones buf, but i think the cloned value is pretty much the actual image, so thats optimal
        let test = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(self.x, self.y, self.buf.to_vec());

        return test;
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

pub struct Meta {
    pub scale: (u32, u32),
    pub duration: Option<f32>,
    pub frames: Option<u32>,
    pub fps: Option<String>,
}

pub fn get_meta(file_name: &std::path::PathBuf) -> Result<Meta, String> {
    let output: std::process::Output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-show_streams",
            "-select_streams",
            "v:0",
            file_name.to_str().unwrap(),
        ])
        .output()
        .expect("starting ffprobe failed");

    if !output.status.success() {
        return Err("ffprobe failed, might not be available".to_string());
    }

    if output.stdout.is_empty() {
        return Err("file is not an image or video (or not parsable by ffmpeg)".to_string());
    }

    let stdout = str::from_utf8(&output.stdout).unwrap();

    let mut width: u32 = 0;
    let mut height: u32 = 0;

    let mut duration: Option<f32> = None;
    let mut frames: Option<u32> = None;
    let mut fps: Option<String> = None;

    for line in stdout.lines() {
        if let Some(x) = line.strip_prefix("r_frame_rate=") {
            if x != "N/A" {
                fps = Some(x.to_string());
            }
        } else if let Some(x) = line.strip_prefix("nb_frames=") {
            frames = x.parse().ok();
        } else if let Some(x) = line.strip_prefix("duration=") {
            duration = x.parse().ok();
        } else if let Some(x) = line.strip_prefix("width=") {
            if let Ok(x) = x.parse::<u32>() {
                width = x;
            }
        } else if let Some(x) = line.strip_prefix("height=") {
            if let Ok(x) = x.parse::<u32>() {
                height = x;
            }
        }
    }

    if width == 0 || height == 0 {
        return Err("No width or height found in ffprobe result".to_string());
    }

    return Ok(Meta {
        scale: (width, height),
        duration,
        frames,
        fps,
    });
}

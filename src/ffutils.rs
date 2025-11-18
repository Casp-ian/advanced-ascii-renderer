use std::process::{Command, Stdio};

// NOTE, ffmpeg-next or the other ffmpeg/video packages seemed quite large, and do not have this specific usecase in mind
// so we just run the ffmpeg command of the system, can be changed later

// note that right now there is technically some wasted work encoding and decoding the files (tho to .bmp, so should be very light) and writing to disk

pub fn start_getting_frames(
    input_file: &std::path::PathBuf,
    output_directory: &std::path::PathBuf,
    // quality: &u8,
    fps: &u8,
    format: &Option<String>,
    internal_scale: &(u32, u32),
) -> Result<(), String> {
    let mut command = &mut Command::new("ffmpeg");
    command = command.arg("-y");

    if let Some(format) = format {
        command = command.args(["-f", format.as_str()]);
    }

    command = command
        .args(["-readrate", "1.0"])
        .args(["-i", input_file.to_str().unwrap()])
        .args([
            "-vf",
            &format!("scale={}:{},fps=20/1", internal_scale.0, internal_scale.1),
        ])
        .args(["-sws_flags", "fast_bilinear"])
        // .args(["-q:v", &quality.to_string()]) // NOTE test if quality even does anything
        .arg(output_directory.join("%05d.bmp"));

    // make sure output doesnt interupt our stdout
    command = command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    match command.spawn() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e.to_string()),
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

    let width: u32 = meta_string[0].parse().unwrap();
    let height: u32 = meta_string[1].parse().unwrap();

    // could have cleaner way of accounting for n/a
    let duration: Option<f32> = meta_string[2].parse().ok();
    let frames: Option<u32> = meta_string[3].parse().ok();

    return Ok(((width, height), duration, frames));
}

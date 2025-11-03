use std::process::{Command, Stdio};

// NOTE, ffmpeg-next or the other ffmpeg/video packages seemed quite large, and do not have this specific usecase in mind
// so we just run the ffmpeg command of the system, it might be terrible, but it does work nice for now, and can be changed later

pub fn start_getting_frames(
    input_file: &std::path::PathBuf,
    output_directory: &std::path::PathBuf,
    quality: &u8,
    format: &Option<String>,
    do_fps: bool,
) -> Result<(), String> {
    let mut command = &mut Command::new("ffmpeg");
    command = command.arg("-y");

    if let Some(format) = format {
        command = command.args(["-f", format.as_str()]);
    }

    command = command
        .args(["-readrate", "1.0"])
        .args(["-i", input_file.to_str().unwrap()])
        // .args(["-vf", "scale=-1:320"]) // NOTE also test equal scale
        .args(["-q:v", &quality.to_string()]); // NOTE test if quality even does anything

    if do_fps {
        command = command.args(["-filter:v", "fps=60/1"]);
    }

    command = command.arg(output_directory.join("%05d.bmp"));

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

// TODO should just create a struct for this return
pub fn get_meta(file_name: &std::path::PathBuf) -> Option<(u32, u32, Option<f32>, Option<u32>)> {
    let output: std::process::Output = Command::new("ffprobe")
        .args([file_name.to_str().unwrap()])
        .args(["-v", "quiet"])
        .args(["-select_streams", "v:0"])
        .args(["-show_entries", "stream=width,height,duration,nb_frames"])
        .args(["-of", "csv=p=0"])
        .output()
        .expect("cant probe");

    if !output.status.success() {
        return None;
    }

    if output.stdout.is_empty() {
        return None;
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

    return Some((width, height, duration, frames));
}

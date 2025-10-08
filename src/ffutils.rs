use std::process::Command;

// TODO
// Problems with this:
// - its slow, restarting ffmpeg cli every frame
// - kinda weird time being an option
pub fn get_frame_at(
    time: Option<f32>,
    file_name: &std::path::PathBuf,
    quality: &u8,
    format: &Option<String>,
    output_name: &str,
) -> Result<(), String> {
    let mut command = &mut Command::new("ffmpeg");
    command = command.arg("-y");

    if let Some(format) = format {
        command = command.args(["-f", format.as_str()]);
    }

    if let Some(time) = time {
        command = command.args(["-ss", time.to_string().as_str()]);
    }

    command = command
        .args(["-i", file_name.to_str().unwrap()])
        .args(["-q:v", &quality.to_string()])
        .args(["-frames:v", "1"])
        .arg(output_name);

    match command.output() {
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

// TODO should just create a struct for this
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

    let stdout = str::from_utf8(&output.stdout).unwrap();

    let test: Vec<&str> = stdout
        .trim_end_matches(&['\r', '\n']) // trim newline for windows and linux
        .split(',')
        .collect();

    let width: u32 = test[0].parse().unwrap();
    let height: u32 = test[1].parse().unwrap();

    // could have cleaner way of accounting for n/a
    let duration: Option<f32> = test[2].parse().ok();
    let frames: Option<u32> = test[3].parse().ok();

    return Some((width, height, duration, frames));
}

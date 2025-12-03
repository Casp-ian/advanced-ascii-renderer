use aar::run;
use std::{
    env, fs,
    io::ErrorKind,
    process::{self, ExitCode},
};

use aar::cli::get_cli_args;

fn do_before_exit() {
    // new line because we might still be on another line
    // also clear ansi color code
    println!("\x1b[0m");

    // remove the temp directory we might have used
    let output_dir = env::temp_dir().join(process::id().to_string());
    match fs::remove_dir_all(&output_dir) {
        Err(e) if e.kind() == ErrorKind::NotFound => (), // it is already removed, we are happy
        Ok(_) => (),
        Err(e) => eprint!("somehow failed in deleting {:?}, {:?}", output_dir, e),
    }
}

fn main() -> ExitCode {
    let _ = ctrlc::set_handler(|| {
        do_before_exit();

        // TODO, this doesnt stop any of the other processes gracefully, so sometimes a error message gets shown at exit
        std::process::exit(0)
    });

    let args = get_cli_args();

    let result: Result<(), String> = run(&args);

    do_before_exit();

    match result {
        Ok(_) => {
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    }
}

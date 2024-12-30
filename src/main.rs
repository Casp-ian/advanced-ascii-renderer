use aar::run_with_args;
use std::{fs::remove_file, process::ExitCode};

use aar::cli::get_cli_args;

fn do_before_exit() {
    let _ = remove_file(aar::video::TEMPORARY_IMAGE_FILE_NAME);

    // TODO, this doesnt stop any of the other processes in a neat way, so sometimes a error message gets shown at exit
    std::process::exit(0);
}

fn main() -> ExitCode {
    let _ = ctrlc::set_handler(do_before_exit);

    let args = get_cli_args();

    let result: Result<(), String> = run_with_args(&args);

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

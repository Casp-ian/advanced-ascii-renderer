use aar::run;
use std::process::ExitCode;

use aar::cli::get_cli_args;

fn do_before_exit() {
    // new line because we might still be on another line
    // also clear ansi color code
    println!("\x1b[0m");

    // TODO, this doesnt stop any of the other processes gracefully, so sometimes a error message gets shown at exit
    std::process::exit(0);
}

fn main() -> ExitCode {
    let _ = ctrlc::set_handler(do_before_exit);

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

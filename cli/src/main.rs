use aar_lib::{config::Config, run};
use std::{default, fs::remove_file, process::ExitCode};

use crate::args::get_cli_args;

pub mod args;

fn do_before_exit() {
    // let _ = remove_file(aar::video::TEMPORARY_IMAGE_FILE_NAME);

    // TODO, this doesnt stop any of the other processes gracefully, so sometimes a error message gets shown at exit
    std::process::exit(0);
}

fn main() -> ExitCode {
    let _ = ctrlc::set_handler(do_before_exit);

    let args = get_cli_args();

    let config: Config = Config {
        path: args.path,
        ..Default::default() // a
                             // width: todo!(),
                             // height: todo!(),
                             // color: todo!(),
                             // set: todo!(),
                             // quality: todo!(),
                             // volume: todo!(),
                             // format: todo!(),
                             // inverted: todo!(),
                             // threshold: todo!(),
                             // no_lines: todo!(),
                             // only_lines: todo!(),
                             // char_width: todo!(),
                             // char_height: todo!(),
                             // media_mode: todo!(),
                             // processing_mode: todo!(),
    };

    let result: Result<(), String> = run(&config);

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

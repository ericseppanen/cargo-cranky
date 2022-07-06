use std::env;
use std::error::Error;
use std::process::{exit, Command};

use crate::config::CrankyConfig;

mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let mut left_args: Vec<String> = Vec::default();
    let mut right_args: Vec<String> = Vec::default();

    let mut found_double_dash = false;

    // Discard the first two arguments, which are (0) the bin name, and (1) the cargo subcommand name "cranky".
    for arg in env::args().skip(2) {
        match (found_double_dash, arg == "--") {
            (false, false) => left_args.push(arg),
            (false, true) => found_double_dash = true,
            (true, false) => right_args.push(arg),
            (true, true) => panic!("found >1 double-dash argument"),
        }
    }

    let config = CrankyConfig::get_config()?;

    right_args.append(&mut config.extra_right_args());

    let all_args = if right_args.is_empty() {
        left_args
    } else {
        left_args.push("--".to_string());
        left_args.append(&mut right_args);
        left_args
    };

    let cmd_result = Command::new("cargo").arg("clippy").args(all_args).status();
    let exit_code = match cmd_result {
        Ok(exit_status) => {
            // Subprocess may return no exit code if it was killed by a signal.
            exit_status.code().unwrap_or(1)
        }
        Err(_) => 1,
    };
    exit(exit_code);
}

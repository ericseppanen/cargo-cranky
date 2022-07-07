use std::env;
use std::error::Error;
use std::process::{exit, Command};

use crate::config::CrankyConfig;

mod config;

const USAGE: &str = "Usage:
    cargo cranky [options] [--] [<opts>...]

Options:
    -h, --help       Show this help text.
    -v, --verbose    Print the inner `cargo clippy` command (additional
                     invocations will be passed though).
    --dry-run        Don't run `cargo clippy`; just print what would be run.
";

#[derive(Debug, Default)]
struct Options {
    dry_run: bool,
    verbose: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut left_args: Vec<String> = Vec::default();
    let mut right_args: Vec<String> = Vec::default();

    let mut found_double_dash = false;

    let mut options = Options::default();

    // Discard the first two arguments, which are (0) the bin name, and (1) the cargo subcommand name "cranky".
    let mut arg_iter = env::args();
    // Ignore the first argument (the name of the binary)
    arg_iter.next();
    // The second argument is probably the subcommand ("cranky") if run as "cargo cranky".
    // But if it was run as cargo-cranky we may get confused. So enforce that it's present
    // and is the expected subcommand.
    let subcommand = arg_iter.next();
    if subcommand != Some("cranky".into()) {
        eprint!("{}", USAGE);
    }

    for arg in arg_iter {
        if arg == "-h" || arg == "--help" {
            print!("{}", USAGE);
            exit(0);
        }
        if arg == "--dry-run" {
            options.dry_run = true;
            continue;
        }
        if arg == "-v" || arg == "--verbose" {
            options.verbose += 1;
            if options.verbose == 1 {
                // Swallow the first call to --verbose. Subsequent calls will be passed through.
                continue;
            }
        }
        match (found_double_dash, arg == "--") {
            (false, false) => left_args.push(arg),
            (false, true) => found_double_dash = true,
            (true, false) => right_args.push(arg),
            (true, true) => panic!("found >1 double-dash argument"),
        }
    }

    let config = CrankyConfig::get_config(&options)?;

    right_args.append(&mut config.extra_right_args());

    let all_args = if right_args.is_empty() {
        left_args
    } else {
        left_args.push("--".to_string());
        left_args.append(&mut right_args);
        left_args
    };

    if options.dry_run || options.verbose > 0 {
        let print_args = all_args.join(" ");
        println!("> cargo clippy {}", print_args);
    }

    if !options.dry_run {
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
    Ok(())
}

use std::env::{self, current_dir};
use std::fs::File;
use std::io::Read;
use std::process::{exit, Command};

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct CrankyConfig {
    warn: Vec<String>,
}

impl CrankyConfig {
    fn get_config() -> CrankyConfig {
        // Search for Cranky.toml in all parent directories.
        let mut dir = current_dir()
            .expect("current dir")
            .canonicalize()
            .expect("canonicalize current dir");

        loop {
            let mut config_path = dir.clone();
            config_path.push("Cranky.toml");
            // We don't care if the file open fails; we'll just keep
            // searching the parent directory.
            // FIXME: this should explicitly check for "nonexistent file";
            // other errors like permissions should be a hard error.
            if let Ok(mut f) = File::open(config_path) {
                let mut toml_bytes = Vec::new();
                f.read_to_end(&mut toml_bytes).expect("toml file read");
                let config: CrankyConfig = toml::from_slice(&toml_bytes).expect("toml parse");
                return config;
            }

            // Go up one directory and try again.
            match dir.parent().to_owned() {
                None => break,
                Some(parent) => dir = parent.to_owned(),
            }
        }

        CrankyConfig {
            warn: vec!["empty_structs_with_brackets".into()],
        }
    }

    fn extra_right_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        for lint in &self.warn {
            args.push("--warn".to_string());
            args.push(format!("clippy::{}", lint));
        }
        args
    }
}

fn main() {
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

    let config = CrankyConfig::get_config();

    right_args.append(&mut config.extra_right_args());

    // ["/home/eric/.cargo/bin/cargo-cranky", "cranky",
    //
    // "--workspace", "--message-format=json",
    // "--manifest-path", "/home/eric/work/rust/cargo-cranky/Cargo.toml",
    // "--all-targets"]

    // 1. collect all the arguments.
    // 2. Split the arguments into "before the --" and "after the --"
    // 3. Read our config file(s)
    // 4. Decide what arguments to append
    // 5. Build the command line and spawn

    //eprintln!("{:?}", args);

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

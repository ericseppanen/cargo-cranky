use std::env::current_dir;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct CrankyConfig {
    #[serde(default)]
    allow: Vec<String>,
    #[serde(default)]
    warn: Vec<String>,
    #[serde(default)]
    deny: Vec<String>,
}

impl CrankyConfig {
    pub(crate) fn get_config() -> Result<CrankyConfig, Box<dyn Error>> {
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
                let config: CrankyConfig = toml::from_slice(&toml_bytes)?;
                return Ok(config);
            }

            // Go up one directory and try again.
            match dir.parent().to_owned() {
                None => break,
                Some(parent) => dir = parent.to_owned(),
            }
        }

        // We didn't find a config file. Just run clippy with no additional arguments.
        Ok(CrankyConfig::default())
    }

    pub(crate) fn extra_right_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        for lint in &self.allow {
            args.push("-A".to_string());
            args.push(format!("clippy::{}", lint));
        }
        for lint in &self.warn {
            args.push("--warn".to_string());
            args.push(format!("clippy::{}", lint));
        }
        for lint in &self.deny {
            args.push("-D".to_string());
            args.push(format!("clippy::{}", lint));
        }
        args
    }
}

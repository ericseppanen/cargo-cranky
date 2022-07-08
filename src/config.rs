use std::env::current_dir;
use std::fs;
use std::io;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::Options;

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
    pub(crate) fn get_config(options: &Options) -> Result<CrankyConfig> {
        // Search for Cranky.toml in all parent directories.
        let mut dir = current_dir()
            .expect("current dir")
            .canonicalize()
            .expect("canonicalize current dir");

        loop {
            let mut config_path = dir.clone();
            config_path.push("Cranky.toml");
            match fs::read(&config_path) {
                Ok(toml_bytes) => {
                    if options.verbose > 0 {
                        eprintln!("Read config file at {:?}", config_path);
                    }
                    let config: CrankyConfig = toml::from_slice(&toml_bytes)?;
                    return Ok(config);
                }
                Err(e) => {
                    match e.kind() {
                        // Not found? Go up one directory and try again.
                        io::ErrorKind::NotFound => match dir.parent() {
                            None => break,
                            Some(parent) => dir = parent.to_owned(),
                        },
                        // Any other error kind is fatal.
                        _ => {
                            Err(e).with_context(|| format!("Failed to read {:?}", config_path))?;
                        }
                    }
                }
            }
        }

        if options.verbose > 0 {
            eprintln!("No Cranky.toml file found.");
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

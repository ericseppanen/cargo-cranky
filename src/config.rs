use std::env::current_dir;
use std::io;

use anyhow::{bail, Result};
use cargo::util::command_prelude::ArgMatchesExt;
use serde::Deserialize;

use crate::Options;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub(crate) struct CrankyConfig {
    #[serde(default)]
    allow: Vec<String>,
    #[serde(default)]
    warn: Vec<String>,
    #[serde(default)]
    deny: Vec<String>,
}

impl CrankyConfig {
    pub(crate) fn get_config(_options: &Options) -> Result<CrankyConfig> {
        // Search for Cargo.toml in all parent directories.
        let dir = current_dir()
            .expect("current dir")
            .canonicalize()
            .expect("canonicalize current dir");


        // redirect cargo output into the void
        let cursor = io::Cursor::new(Vec::<u8>::new());
        let shell = cargo::core::Shell::from_write(Box::new(cursor));
        let Some(home_dir) = cargo::util::homedir(&dir) else {
            bail!("Could not find home directory");
        };
        let mut cargo_config = cargo::Config::new(shell, dir, home_dir);

        cargo_config.configure(
            0,
            false,
            None,
            false,
            false,
            false,
            &None,
            &[],
            &[],
        )?;


        let arg_matches = cargo::util::command_prelude::ArgMatches::default();
        let mut ws = arg_matches.workspace(&cargo_config)?;

        fn get_from_custom_metadata_lints_table(value: &toml_edit::easy::Value) -> Result<CrankyConfig> {
            Ok(value.clone().try_into()?)
        }

        const METADATA_KEY: &str = "lints";

        fn get_from_custom_metadata(value: Option<&toml_edit::easy::Value>) -> Result<CrankyConfig> {
            Ok(value.and_then(|metadata| metadata.get(METADATA_KEY))
                .map(get_from_custom_metadata_lints_table).transpose()?.unwrap_or_default())
        }

        let cfg = get_from_custom_metadata(ws.custom_metadata())?;

        {
            ws.load_workspace_config()?;

            for p in ws.members() {
                let _cfg_for_package = get_from_custom_metadata(p.manifest().custom_metadata())?;

                // TODO: maybe also get package-specific lints here
            }
        }

        Ok(cfg)
    }

    pub(crate) fn extra_right_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        for lint in &self.deny {
            args.push(format!("-D{}", lint));
        }
        for lint in &self.warn {
            args.push(format!("-W{}", lint));
        }
        for lint in &self.allow {
            args.push(format!("-A{}", lint));
        }
        args
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_toml_1() {
        let toml_bytes = br#"
            warn = [
                "aaa",
                "bbb",
            ]"#;
        let config: CrankyConfig = toml::from_slice(toml_bytes).unwrap();

        assert_eq!(
            config,
            CrankyConfig {
                allow: vec![],
                warn: vec!["aaa".into(), "bbb".into()],
                deny: vec![],
            }
        )
    }

    #[test]
    fn parse_toml_2() {
        let toml_bytes = br#"
            allow = [ "aaa" ]
            warn = [ "bbb" ]
            deny = [ "ccc" ]
        "#;
        let config: CrankyConfig = toml::from_slice(toml_bytes).unwrap();

        assert_eq!(
            config,
            CrankyConfig {
                allow: vec!["aaa".into()],
                warn: vec!["bbb".into()],
                deny: vec!["ccc".into()],
            }
        );

        let args = config.extra_right_args().join(" ");
        // Ordering matters! deny -> warn -> allow is the intended behavior.
        assert_eq!(args, "-Dccc -Wbbb -Aaaa");
    }
}

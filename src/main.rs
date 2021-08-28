mod auth;
mod commands;
mod graphql;

use anyhow::{anyhow, bail, Context, Result};
use clap::App;
use commands::dashboard::Dashboard;
use commands::{auth as authenticate, tldr};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    cid: String,
}

pub struct CuddleApp<'a> {
    pub get_config: &'a mut dyn FnMut() -> Result<Config>,
    pub set_config: &'a mut dyn FnMut(Config) -> Result<()>,
}

fn main() -> Result<()> {
    let matches = App::new("Cuddle")
        .version("1.0")
        .author("Henry Gressmann <mail@henrygressmann.de>")
        .about("Interactive Code University App Dashboard")
        .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .subcommand(App::new("tldr").about("Summary of important information"))
        .subcommand(App::new("auth").about("Authenticate this command"))
        .get_matches();

    let get_config_path = || -> Result<std::path::PathBuf> {
        let config = matches
            .value_of("config")
            .ok_or_else(|| anyhow!("config is not a valid string!"))?;
        let path = fs::canonicalize(config)?;
        Ok(path)
    };

    let mut get_config = || -> Result<Config> {
        if matches.is_present("config") {
            confy::load_path(get_config_path()?)
        } else {
            confy::load("cuddle")
        }
        .with_context(|| "Failed to load config")
    };

    let mut set_config = |cfg: Config| -> Result<()> {
        if matches.is_present("config") {
            confy::store_path(get_config_path()?, cfg)
        } else {
            confy::store("cuddle", cfg)
        }
        .with_context(|| "Failed to set config")
    };

    let cfg: Config = get_config().context("error while loading configuration")?;
    let app = CuddleApp {
        get_config: &mut get_config,
        set_config: &mut set_config,
    };

    if let Some(ref _matches) = matches.subcommand_matches("auth") {
        return authenticate::run(app);
    }

    if cfg.cid.is_empty() {
        bail!("invalid cid, set using `cuddle auth` or the config file");
    }

    if let Some(ref _matches) = matches.subcommand_matches("tldr") {
        return tldr::run(app);
    }

    let dashboard = Dashboard::new(app)?;
    dashboard.draw()?;
    Ok(())
}

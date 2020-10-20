#![deny(clippy::all)]

use anyhow::Result;
use log::debug;
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
struct Dungeon {
    name: String,
    tanks: u8,
    healers: u8,
    strikers: u8,
}

fn read_config() -> Result<Vec<Dungeon>> {
    debug!("Reading config");
    let from_config = fs::read_to_string("dungeons.yaml")?;
    let dungeons = serde_yaml::from_str(&from_config)?;
    debug!("{:#?}", dungeons);
    Ok(dungeons)
}

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "lfg=debug");
    }
    pretty_env_logger::init();
    let dungeons = read_config()?;

    Ok(())
}

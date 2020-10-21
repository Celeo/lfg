#![deny(clippy::all)]

use anyhow::Result;
use std::env;

mod models;
use models::read_config;

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "lfg=debug");
    }
    pretty_env_logger::init();
    let _dungeons = read_config()?;

    Ok(())
}

//! ``ReadStor`` is a simple CLI for exporting user-generated data from Apple
//! Books. The goal of this project is to facilitate data-migration from Apple
//! Books to any other platform. Currently Apple Books provides no simple way to
//! do this. Exporting is possible but not ideal and often times truncates long
//! annotations.

#![warn(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility
)]
#![allow(
    clippy::module_name_repetitions,
    // TODO: How is this fixed?
    clippy::multiple_crate_versions,
)]

mod cli;
pub mod lib;

use clap::Parser;

use crate::cli::app::{App, AppResult};
use crate::cli::args::Args;
use crate::cli::config::app::AppConfig;
use crate::cli::config::dev::{is_development_env, DevConfig};
use crate::cli::config::Config;
use crate::lib::applebooks::utils::applebooks_is_running;

fn main() -> AppResult<()> {
    cli::utils::init_logger();
    color_eyre::install()?;

    let args = Args::parse();

    log::debug!("{:#?}", &args);

    if !args.options.force && applebooks_is_running() {
        println!(
            "Apple Books is currently running. \
            To ignore this, use the `-f, --force` flag."
        );
        return Ok(());
    }

    // Selects the appropriate Config depending on the environment. In a
    // development environment this sets the `databases` to local mock databases
    // directory and the `output` to a temp directory on disk.
    //
    // Note that the appropriate environment variable to signal a development
    // env should be set in the `.cargo/config.toml` file.
    let config: Box<dyn Config> = if is_development_env() {
        Box::new(DevConfig::from(args.options))
    } else {
        Box::new(AppConfig::from(args.options))
    };

    log::debug!("{:#?}.", &config);

    App::new(config).run(args.command)
}

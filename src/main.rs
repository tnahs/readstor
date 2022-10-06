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
    // Produces some false positives in docs.
    clippy::doc_markdown,
    // TODO: How is this fixed?
    clippy::multiple_crate_versions,
)]

mod cli;
pub mod lib;

use clap::Parser;

use crate::cli::app::{App, AppResult};
use crate::cli::config::Config;
use crate::cli::Cli;
use crate::lib::applebooks::utils::applebooks_is_running;

fn main() -> AppResult<()> {
    cli::utils::init_logger();
    color_eyre::install()?;

    let args = Cli::parse();

    log::debug!("{:#?}", &args);

    if !args.options.force && applebooks_is_running() {
        println!(
            "Apple Books is currently running. \
            To ignore this, use the `-f, --force` flag."
        );
        return Ok(());
    }

    let config = Config::from(args.options);

    log::debug!("{:#?}.", &config);

    App::new(config).run(args.command)
}

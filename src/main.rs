//! ``ReadStor`` is a simple CLI for exporting user-generated data from Apple
//! Books. The goal of this project is to facilitate data-migration from Apple
//! Books to any other platform. Currently Apple Books provides no simple way to
//! do this. Exporting is possible but not ideal and often times truncates long
//! annotations.

mod cli;

use clap::Parser;

use crate::cli::app::{App, Result};
use crate::cli::config::Config;
use crate::cli::Cli;

use lib::applebooks::macos::utils::applebooks_is_running;

fn main() -> Result<()> {
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

    log::debug!("{:#?}", &config);

    App::new(config).run(args.command)
}

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, rustdoc::private_intra_doc_links)]

mod cli;
pub mod lib;

use clap::Parser;
use loggerv::Logger;

use crate::cli::app::AnyhowResult;
use crate::cli::app::App;
use crate::cli::args::Args;
use crate::cli::config::AppConfig;
use crate::lib::applebooks::utils::applebooks_is_running;

fn main() -> AnyhowResult<()> {
    let args = Args::parse();

    Logger::new()
        .verbosity(args.verbosity)
        .level(true)
        .init()
        .unwrap();

    log::debug!("Running with: {:#?}.", &args);

    if !args.force && applebooks_is_running() {
        println!(
            "Apple Books is currently running. \
            To ignore this, use the `-f, --force` flag."
        );
        return Ok(());
    }

    let config: AppConfig = args.into();

    log::debug!("Running with: {:#?}.", &config);

    App::new(config)?.run()?;

    Ok(())
}

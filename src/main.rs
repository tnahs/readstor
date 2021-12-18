#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, rustdoc::private_intra_doc_links)]

mod cli;
pub mod lib;

use loggerv::Logger;
use structopt::StructOpt;

use cli::app::AnyhowResult;
use cli::app::App;
use cli::config::AppConfig;
use cli::opt::Opt;
use lib::applebooks::utils::applebooks_is_running;

fn main() -> AnyhowResult<()> {
    let opt = Opt::from_args();

    Logger::new()
        .verbosity(opt.verbosity)
        .level(true)
        .init()
        .unwrap();

    log::debug!("Running with: {:#?}.", &opt);

    if !opt.force && applebooks_is_running() {
        println!(
            "Apple Books is currently running. \
            To ignore this, use the `-f, --force` flag."
        );
        return Ok(());
    }

    let config: AppConfig = opt.into();

    log::debug!("Running with: {:#?}.", &config);

    App::new(config)?.run()?;

    Ok(())
}

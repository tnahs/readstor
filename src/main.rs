//! ``ReadStor`` is a simple CLI for exporting user-generated data from Apple Books. The goal of this
//! project is to facilitate data-migration from Apple Books to any other platform. Currently Apple
//! Books provides no simple way to do this. Exporting is possible but not ideal and often times
//! truncates long annotations.

mod cli;

use clap::Parser;
use cli::args::Args;
use cli::CliResult;

fn main() -> CliResult<()> {
    cli::utils::init_logger();
    color_eyre::install()?;

    let args = Args::parse();

    cli::run(args.command)
}

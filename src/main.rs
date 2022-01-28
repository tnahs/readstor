#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, rustdoc::private_intra_doc_links)]

mod cli;
pub mod lib;

use anyhow::Context;
use clap::Parser;
use loggerv::Logger;

use crate::cli::app::AnyhowResult;
use crate::cli::app::App;
use crate::cli::args::Args;
use crate::cli::args::Command;
use crate::cli::config::Config;
use crate::lib::applebooks::utils::applebooks_is_running;

fn main() -> AnyhowResult<()> {
    let args = Args::parse();

    Logger::new()
        .verbosity(args.verbosity)
        .level(true)
        .init()
        .unwrap();

    log::debug!("Running with `args`: {:#?}.", &args);

    if !args.force && applebooks_is_running() {
        println!(
            "Apple Books is currently running. \
            To ignore this, use the `-f, --force` flag."
        );
        return Ok(());
    }

    let config = Config::new(&args);
    let mut app = App::new(config);

    println!("* Building stor...");

    app.init().context("ReadStor failed while building stor")?;

    match &args.command {
        Command::Export => {
            println!("* Exporting data...");
            app.export_data()
                .context("ReadStor failed while exporting data")?;
        }
        Command::Render { ref template } => {
            println!("* Rendering template...");
            app.render_templates(template.as_ref())
                .context("ReadStor failed while rendering template")?;
        }
        Command::Backup => {
            println!("* Backing up databases...");
            app.backup_databases()
                .context("ReadStor failed while backing up databases")?;
        }
    }

    println!(
        "* Saved {} annotations from {} books.",
        app.count_annotations(),
        app.count_books()
    );

    Ok(())
}

use std::path::PathBuf;
use std::result::Result;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, setting(AppSettings::DeriveDisplayOrder))]
pub struct Args {
    /// Sets the [output] path [default: ~/.readstor]
    #[clap(
        short,
        long,
        value_name = "PATH",
        global = true,
        parse(try_from_str = validate_path_exists),
    )]
    pub output: Option<PathBuf>,

    /// Runs even if Apple Books is open
    #[clap(short, long, global = true)]
    pub force: bool,

    /// Sets the logging verbosity
    #[clap(short, global = true, parse(from_occurrences))]
    pub verbosity: u64,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub enum Command {
    /// Exports Apple Books' data to [output]
    Export,

    /// Renders annotations via a template to [output]
    Render {
        /// Sets a custom template
        #[clap(
            short,
            long,
            value_name = "FILE",
            parse(try_from_str = validate_path_exists),
        )]
        template: Option<PathBuf>,
    },

    /// Backs-up Apple Books' databases to [output]
    Backup,
}

/// Validates that a path exists.
pub fn validate_path_exists(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if !path.exists() {
        return Err(format!("path does not exist: `{}`", value));
    }

    Ok(path)
}

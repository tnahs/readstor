use std::path::PathBuf;
use std::result::Result;

use clap::{AppSettings, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about, setting(AppSettings::DeriveDisplayOrder))]
pub struct Args {
    /// Sets the [output] path [default: ~/.readstor]
    #[clap(
        short,
        long,
        value_name = "PATH",
        parse(try_from_str = validate_path_exists),
    )]
    pub output: Option<PathBuf>,

    /// Sets a custom export template
    #[clap(
        short,
        long,
        value_name = "FILE",
        parse(try_from_str = validate_path_exists),
    )]
    pub template: Option<PathBuf>,

    /// Exports annotations via template to [output]
    #[clap(short, long)]
    pub export: bool,

    /// Backs-up Apple Books' databases to [output]
    #[clap(short, long)]
    pub backup: bool,

    /// Runs even if Apple Books is open
    #[clap(short, long, global = true)]
    pub force: bool,

    /// Sets the level of verbosity
    #[clap(short, long, parse(from_occurrences), global = true)]
    pub verbosity: u64,
}

/// Validates that a path exists.
pub fn validate_path_exists(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if !path.exists() {
        return Err(format!("path does not exist: `{}`", value));
    }

    Ok(path)
}

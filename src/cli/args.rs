use std::path::PathBuf;
use std::result::Result;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, setting(AppSettings::DeriveDisplayOrder))]
pub struct Args {
    /// Sets the OUTPUT path [default: ~/.readstor]
    #[clap(
        short,
        long,
        global = true,
        parse(try_from_str = validate_path_exists),
    )]
    pub output: Option<PathBuf>,

    /// Sets a custom templates directory
    #[clap(
        short,
        long,
        global = true,
        parse(try_from_str = validate_path_exists),
    )]
    pub templates: Option<PathBuf>,

    /// Runs even if Apple Books is open
    #[clap(short, long, global = true)]
    pub force: bool,

    /// Silences output messages
    #[clap(short, long = "quiet", global = true)]
    pub is_quiet: bool,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub enum Command {
    /// Exports Apple Books' data to OUTPUT
    Export,

    /// Renders annotations via templates to OUTPUT
    Render,

    /// Backs-up Apple Books' databases to OUTPUT
    Backup,
}

pub fn validate_path_exists(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if !path.exists() {
        return Err(format!("path does not exist: `{}`", value));
    }

    Ok(path)
}

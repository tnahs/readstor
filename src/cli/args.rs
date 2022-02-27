use std::path::PathBuf;
use std::result::Result;

use clap::{AppSettings, Parser, Subcommand};

use super::config::RenderMode;

// NOTE Global options will eventually be settable from a config file.
#[derive(Debug, Parser)]
#[clap(author, version, about, setting(AppSettings::DeriveDisplayOrder))]
pub struct Args {
    #[clap(flatten)]
    pub options: ArgOptions,

    #[clap(subcommand)]
    pub command: ArgCommand,
}

#[derive(Debug, Parser)]
pub struct ArgOptions {
    /// TODO Document
    #[clap(
            short,
            long,
            global = true,
            parse(try_from_str = validate_path_exists),
        )]
    pub databases: Option<PathBuf>,

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

    // TODO Can we add extra help text using `clap`?
    /// Sets the template mode
    #[clap(arg_enum, short = 'm', long, global = true)]
    pub render_mode: Option<RenderMode>,

    /// Runs even if Apple Books is open
    #[clap(short, long, global = true)]
    pub force: bool,

    /// Silences output messages
    #[clap(short, long = "quiet", global = true)]
    pub is_quiet: bool,
}

#[derive(Debug, Clone, Copy, Subcommand)]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub enum ArgCommand {
    /// Exports Apple Books' data to OUTPUT
    Export,

    /// Renders annotations via templates to OUTPUT
    Render,

    /// Backs-up Apple Books' databases to OUTPUT
    Backup,
}

/// TODO Document
pub fn validate_path_exists(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);

    if !path.exists() {
        return Err(format!("path does not exist: `{}`", value));
    }

    Ok(path)
}

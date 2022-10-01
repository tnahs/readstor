use std::path::PathBuf;
use std::result::Result;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(flatten)]
    pub options: ArgOptions,

    #[clap(subcommand)]
    pub command: ArgCommand,
}

#[derive(Debug, Parser)]
pub struct ArgOptions {
    /// Sets a custom databases directory
    #[arg(short, long, global = true, value_parser(validate_path_exists))]
    pub databases: Option<PathBuf>,

    /// Sets the OUTPUT directory [default: ~/.readstor]
    #[arg(short, long, global = true, value_parser(validate_path_exists))]
    pub output: Option<PathBuf>,

    /// Runs even if Apple Books is open
    #[arg(short, long, global = true)]
    pub force: bool,

    /// Silences output messages
    #[arg(short, long = "quiet", global = true)]
    pub is_quiet: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ArgCommand {
    /// Exports Apple Books' data to OUTPUT
    Export,

    /// Renders annotations via templates to OUTPUT
    Render {
        /// Sets a custom templates directory
        #[arg(short, long, value_parser(validate_path_exists))]
        templates: Option<PathBuf>,
    },

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

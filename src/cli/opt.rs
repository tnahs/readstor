use std::path::PathBuf;
use std::result::Result;

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ReadStor",
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    settings = &[AppSettings::DeriveDisplayOrder],
)]

pub struct Opt {
    #[structopt(
        short,
        long,
        help = "Sets the [output] path [default: ~/.readstor]",
        value_name = "PATH",
        parse(try_from_str = validate_path_exists),
    )]
    pub output: Option<PathBuf>,

    #[structopt(
        short,
        long,
        help = "Sets a custom export template",
        value_name = "FILE",
        parse(try_from_str = validate_path_exists),
    )]
    pub template: Option<PathBuf>,

    #[structopt(short, long, help = "Exports annotations via template to [output]")]
    pub export: bool,

    #[structopt(short, long, help = "Backs-up Apple Books' databases to [output]")]
    pub backup: bool,

    #[structopt(short, long, help = "Runs even if Apple Books is open")]
    pub force: bool,

    #[structopt(short, parse(from_occurrences), help = "Sets the level of verbosity")]
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

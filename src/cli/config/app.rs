use std::path::PathBuf;

use crate::cli::args::Args;
use crate::cli::defaults as cli_defaults;
use crate::lib::applebooks::defaults as applebooks_defaults;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

use super::Configuration;

#[derive(Debug)]
pub struct AppConfig {
    output: PathBuf,
}

impl Configuration for AppConfig {
    fn databases(&self) -> &PathBuf {
        &applebooks_defaults::DATABASES
    }

    fn output(&self) -> &PathBuf {
        &self.output
    }
}

impl AppConfig {
    pub fn new(args: &Args) -> Self {
        // If no output directory is supplied fall back to the default.
        let output = args
            .output
            .clone()
            .unwrap_or_else(|| cli_defaults::OUTPUT.to_owned());

        Self { output }
    }
}

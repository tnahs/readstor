use std::path::PathBuf;

use crate::cli;
use crate::cli::args::Args;
use crate::lib::applebooks;
use crate::lib::utils;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

use super::{Config, ConfigOptions};

#[derive(Debug)]
pub struct AppConfig {
    options: ConfigOptions,
}

impl Config for AppConfig {
    fn databases(&self) -> &PathBuf {
        &applebooks::defaults::DATABASES
    }

    fn options(&self) -> &ConfigOptions {
        &self.options
    }
}

impl AppConfig {
    pub fn new(args: &Args) -> Self {
        Self {
            options: args.into(),
        }
    }
}

impl From<&Args> for ConfigOptions {
    fn from(args: &Args) -> Self {
        let output = args
            .output
            .clone()
            .unwrap_or_else(|| cli::defaults::OUTPUT.to_owned());

        let templates = args
            .templates
            .clone()
            .map_or_else(Vec::new, |path| utils::iter_dir(&path).collect());

        Self {
            output,
            templates,
            is_quiet: args.is_quiet,
        }
    }
}

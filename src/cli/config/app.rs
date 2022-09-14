use crate::cli;
use crate::cli::args::ArgOptions;
use crate::lib::applebooks;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

use super::{Config, ConfigOptions};

#[derive(Debug)]
pub struct AppConfig {
    options: ConfigOptions,
}

impl Config for AppConfig {
    fn options(&self) -> &ConfigOptions {
        &self.options
    }
}

impl From<ArgOptions> for AppConfig {
    fn from(options: ArgOptions) -> Self {
        let databases = options
            .databases
            .unwrap_or_else(|| applebooks::defaults::DATABASES.to_owned());

        let output = options
            .output
            .unwrap_or_else(|| cli::defaults::OUTPUT.to_owned());

        Self {
            options: ConfigOptions {
                databases,
                output,
                templates: options.templates,
                is_quiet: options.is_quiet,
            },
        }
    }
}

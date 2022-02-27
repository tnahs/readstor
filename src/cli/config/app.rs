use crate::cli;
use crate::cli::args::ArgOptions;
use crate::lib::applebooks;
use crate::lib::utils;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

use super::RenderMode;
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

impl From<&ArgOptions> for AppConfig {
    fn from(options: &ArgOptions) -> Self {
        let databases = options
            .databases
            .clone()
            .unwrap_or_else(|| applebooks::defaults::DATABASES.to_owned());

        let output = options
            .output
            .clone()
            .unwrap_or_else(|| cli::defaults::OUTPUT.to_owned());

        let templates = options
            .templates
            .clone()
            .map_or_else(Vec::new, |path| utils::iter_dir(&path).collect());

        let render_mode = options.render_mode.unwrap_or(RenderMode::Single);

        Self {
            options: ConfigOptions {
                databases,
                output,
                templates,
                render_mode,
                is_quiet: options.is_quiet,
            },
        }
    }
}

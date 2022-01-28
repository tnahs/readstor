use std::env;
use std::path::PathBuf;

use super::args::Args;
use super::defaults::{DATABASES_DEV, DEV_READSTOR, OUTPUT};
use crate::lib::applebooks::defaults::DATABASES;

#[derive(Debug)]
pub struct AppConfig {
    pub output: PathBuf,
    pub databases: PathBuf,
    pub template: Option<PathBuf>,
    pub backup: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            output: OUTPUT.to_owned(),
            databases: DATABASES.to_owned(),
            template: None,
            backup: false,
        }
    }
}

impl From<Args> for AppConfig {
    fn from(args: Args) -> Self {
        // Bypass the official database if the application is being worked on.
        let databases = match env::var_os(DEV_READSTOR) {
            Some(_) => DATABASES_DEV.to_owned().join("books-annotated"),
            None => DATABASES.to_owned(),
        };

        AppConfig {
            output: args.output.unwrap_or_else(|| OUTPUT.to_owned()),
            databases,
            template: args.template,
            backup: args.backup,
        }
    }
}

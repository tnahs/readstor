use std::env;
use std::path::PathBuf;

use super::defaults::{DATABASES_DEV, DEV_READSTOR, OUTPUT, TEMPLATE};
use super::opt::Opt;
use crate::lib::applebooks::defaults::DATABASES;

#[derive(Debug)]

pub struct AppConfig {
    pub output: PathBuf,
    pub databases: PathBuf,
    pub template: PathBuf,
    pub backup: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            output: OUTPUT.to_owned(),
            databases: DATABASES.to_owned(),
            template: TEMPLATE.to_owned(),
            backup: false,
        }
    }
}

impl From<Opt> for AppConfig {
    fn from(opt: Opt) -> Self {
        // Bypass the official database if the application is being worked on.
        let databases = match env::var_os(DEV_READSTOR) {
            Some(_) => DATABASES_DEV.to_owned().join("books-annotated"),
            None => DATABASES.to_owned(),
        };

        AppConfig {
            output: opt.output.unwrap_or_else(|| OUTPUT.to_owned()),
            databases,
            template: opt.template.unwrap_or_else(|| TEMPLATE.to_owned()),
            backup: opt.backup,
        }
    }
}

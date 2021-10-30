use std::path::PathBuf;

use super::defaults::{OUTPUT, TEMPLATE};
use super::opt::Opt;

#[derive(Debug)]
pub struct AppConfig {
    pub output: PathBuf,
    pub template: PathBuf,
    pub backup: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            output: OUTPUT.to_path_buf(),
            template: TEMPLATE.to_path_buf(),
            backup: false,
        }
    }
}

impl From<Opt> for AppConfig {
    fn from(opt: Opt) -> Self {
        AppConfig {
            output: opt.output.unwrap_or_else(|| OUTPUT.to_path_buf()),
            template: opt.template.unwrap_or_else(|| TEMPLATE.to_path_buf()),
            backup: opt.backup,
        }
    }
}

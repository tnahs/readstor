pub mod app;
pub mod config;
pub mod defaults;
pub mod utils;

use std::path::PathBuf;
use std::result::Result;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub options: Options,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Parser)]
pub struct Options {
    /// Set the output directory [default: ~/.readstor]
    #[arg(short, long, value_name = "PATH", value_parser(validate_path_exists))]
    pub output_directory: Option<PathBuf>,

    /// Set a custom databses directory
    #[arg(short, long, value_name = "PATH", value_parser(validate_path_exists))]
    pub databases_directory: Option<PathBuf>,

    /// Run even if Apple Books is open
    #[arg(short, long)]
    pub force: bool,

    /// Silence output messages
    #[arg(short, long = "quiet")]
    pub is_quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Export Apple Books' data as JSON
    Export {
        #[clap(flatten)]
        preprocessor_options: PreProcessorOptions,
    },

    /// Render Apple Books' data via templates
    Render {
        #[clap(flatten)]
        template_options: TemplateOptions,

        #[clap(flatten)]
        preprocessor_options: PreProcessorOptions,

        #[clap(flatten)]
        postprocessor_options: PostProcessorOptions,
    },

    /// Back-up Apple Books' databases
    Backup,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct TemplateOptions {
    /// Set a custom templates directory
    #[arg(
        short = 'd',
        long,
        value_name = "PATH",
        value_parser(validate_path_exists)
    )]
    pub templates_directory: Option<PathBuf>,

    /// Render specified template groups
    #[arg(short = 'g', long = "template-group", value_name = "GROUP")]
    pub template_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Default, Parser)]
#[allow(clippy::struct_excessive_bools)]
pub struct PreProcessorOptions {
    /// Extract #tags from annotation notes
    #[arg(short = 'e', long)]
    pub extract_tags: bool,

    /// Normalize whitespace in annotation body
    #[arg(short = 'n', long)]
    pub normalize_whitespace: bool,

    /// Convert all Unicode characters to ASCII
    #[arg(
        short = 'a',
        long = "ascii-all",
        conflicts_with = "convert_symbols_to_ascii"
    )]
    pub convert_all_to_ascii: bool,

    /// Convert "smart" Unicode symbols to ASCII
    #[arg(
        short = 's',
        long = "ascii-symbols",
        conflicts_with = "convert_all_to_ascii"
    )]
    pub convert_symbols_to_ascii: bool,
}

#[derive(Debug, Clone, Copy, Default, Parser)]
pub struct PostProcessorOptions {
    /// Trim any blocks left after rendering
    #[arg(short = 't', long)]
    pub trim_blocks: bool,
}

pub fn validate_path_exists(value: &str) -> Result<PathBuf, String> {
    std::fs::canonicalize(value).map_err(|_| "path does not exist".into())
}

impl From<TemplateOptions> for lib::templates::manager::TemplateOptions {
    fn from(options: TemplateOptions) -> Self {
        Self {
            templates_directory: options.templates_directory,
            template_groups: options.template_groups,
        }
    }
}

impl From<PreProcessorOptions> for lib::processor::PreProcessorOptions {
    fn from(options: PreProcessorOptions) -> Self {
        Self {
            extract_tags: options.extract_tags,
            normalize_whitespace: options.normalize_whitespace,
            convert_all_to_ascii: options.convert_all_to_ascii,
            convert_symbols_to_ascii: options.convert_symbols_to_ascii,
        }
    }
}

impl From<PostProcessorOptions> for lib::processor::PostProcessorOptions {
    fn from(options: PostProcessorOptions) -> Self {
        Self {
            trim_blocks: options.trim_blocks,
        }
    }
}

use std::path::PathBuf;

use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    disable_help_subcommand = true,
    after_help = "See the documentation for more information: https://tnahs.github.io/readstor",
    styles = styles(),
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Render annotations via templates
    Render {
        platform: Platform,

        #[clap(flatten)]
        render_options: RenderOptions,

        #[clap(flatten)]
        filter_options: FilterOptions,

        #[clap(flatten)]
        preprocess_options: PreProcessOptions,

        #[clap(flatten)]
        postprocess_options: PostProcessOptions,

        #[clap(flatten)]
        global_options: GlobalOptions,
    },

    /// Export data as json
    Export {
        platform: Platform,

        #[clap(flatten)]
        export_options: ExportOptions,

        #[clap(flatten)]
        filter_options: FilterOptions,

        #[clap(flatten)]
        preprocess_options: PreProcessOptions,

        #[clap(flatten)]
        global_options: GlobalOptions,
    },

    /// Back-up Apple Books data
    Backup {
        platform: Platform,

        #[clap(flatten)]
        backup_options: BackupOptions,

        #[clap(flatten)]
        global_options: GlobalOptions,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Platform {
    #[value(name = "macos")]
    MacOs,

    #[value(name = "ios")]
    IOs,
}

#[derive(Debug, Clone, Parser)]
pub struct GlobalOptions {
    /// Set a custom output directory
    #[arg(
        short = 'o',
        long,
        value_name = "PATH",
        value_parser(validate_path_exists),
        help_heading = "Global Options"
    )]
    pub output_directory: Option<PathBuf>,

    /// Set a custom input data directory
    #[arg(
        short = 'd',
        long,
        value_name = "PATH",
        value_parser(validate_path_exists),
        help_heading = "Global Options"
    )]
    pub data_directory: Option<PathBuf>,

    /// Run command even if Apple Books is currently running
    #[arg(short = 'F', long = "force", help_heading = "Global Options")]
    pub is_force: bool,

    /// Silence output messages
    #[arg(short = 'q', long = "quiet", help_heading = "Global Options")]
    pub is_quiet: bool,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct RenderOptions {
    /// Set a custom templates directory
    #[arg(
        short = 't',
        long,
        value_name = "PATH",
        value_parser(validate_path_exists)
    )]
    pub templates_directory: Option<PathBuf>,

    /// Render specified template-group(s)
    #[arg(short = 'g', long = "template-group", value_name = "GROUP")]
    pub template_groups: Vec<String>,

    /// Overwrite existing files
    #[arg(short = 'O', long)]
    pub overwrite_existing: bool,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct ExportOptions {
    /// Set the output directory template
    #[arg(short = 't', long, value_name = "TEMPLATE")]
    pub directory_template: Option<String>,

    /// Overwrite existing files
    #[arg(short = 'O', long)]
    pub overwrite_existing: bool,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct BackupOptions {
    /// Set the output directory template
    #[arg(short = 't', long, value_name = "TEMPLATE")]
    pub directory_template: Option<String>,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct FilterOptions {
    /// Filter books/annotations before outputting
    #[arg(
        short = 'f',
        long = "filter",
        value_name = "[OP]{FIELD}:{QUERY}",
        help_heading = "Filter"
    )]
    pub filter_types: Vec<super::filter::FilterType>,

    /// Auto-confirm filter results
    #[arg(
        short = 'A', // Capital lettes for critical options
        long = "auto-confirm-filter",
        requires = "filter_types",
        help_heading = "Filter"
    )]
    pub auto_confirm: bool,
}

#[derive(Debug, Clone, Copy, Default, Parser)]
#[allow(clippy::struct_excessive_bools)]
pub struct PreProcessOptions {
    /// Extract #tags from annotation notes
    #[arg(short = 'e', long, help_heading = "Pre-process")]
    pub extract_tags: bool,

    /// Normalize whitespace in annotation body
    #[arg(short = 'n', long, help_heading = "Pre-process")]
    pub normalize_whitespace: bool,

    /// Convert all Unicode characters to ASCII
    #[arg(
        short = 'a',
        long = "ascii-all",
        conflicts_with = "convert_symbols_to_ascii",
        help_heading = "Pre-process"
    )]
    pub convert_all_to_ascii: bool,

    /// Convert "smart" Unicode symbols to ASCII
    #[arg(
        short = 's',
        long = "ascii-symbols",
        conflicts_with = "convert_all_to_ascii",
        help_heading = "Pre-process"
    )]
    pub convert_symbols_to_ascii: bool,
}

#[derive(Debug, Clone, Copy, Default, Parser)]
pub struct PostProcessOptions {
    /// Trim any blocks left after rendering
    #[arg(short = 'b', long, help_heading = "Post-process")]
    pub trim_blocks: bool,

    /// Wrap text to a maximum character width
    #[arg(short = 'w', long, value_name = "WIDTH", help_heading = "Post-process")]
    pub wrap_text: Option<usize>,
}

fn styles() -> Styles {
    Styles::styled()
        .usage(AnsiColor::Green.on_default())
        .header(AnsiColor::Green.on_default())
        .literal(AnsiColor::Cyan.on_default())
        .placeholder(AnsiColor::Yellow.on_default())
}

pub fn validate_path_exists(value: &str) -> std::result::Result<PathBuf, String> {
    std::fs::canonicalize(value).map_err(|_| "path does not exist".into())
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacOs => write!(f, "macOS"),
            Self::IOs => write!(f, "iOS"),
        }
    }
}

impl From<Platform> for lib::applebooks::Platform {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::MacOs => Self::MacOs,
            Platform::IOs => Self::IOs,
        }
    }
}

impl From<RenderOptions> for lib::render::renderer::RenderOptions {
    fn from(options: RenderOptions) -> Self {
        Self {
            templates_directory: options.templates_directory,
            template_groups: options.template_groups,
            overwrite_existing: options.overwrite_existing,
        }
    }
}

impl From<ExportOptions> for lib::export::ExportOptions {
    fn from(options: ExportOptions) -> Self {
        Self {
            directory_template: options.directory_template,
            overwrite_existing: options.overwrite_existing,
        }
    }
}

impl From<BackupOptions> for lib::backup::BackupOptions {
    fn from(options: BackupOptions) -> Self {
        Self {
            directory_template: options.directory_template,
        }
    }
}

impl From<PreProcessOptions> for lib::process::pre::PreProcessOptions {
    fn from(options: PreProcessOptions) -> Self {
        Self {
            extract_tags: options.extract_tags,
            normalize_whitespace: options.normalize_whitespace,
            convert_all_to_ascii: options.convert_all_to_ascii,
            convert_symbols_to_ascii: options.convert_symbols_to_ascii,
        }
    }
}

impl From<PostProcessOptions> for lib::process::post::PostProcessOptions {
    fn from(options: PostProcessOptions) -> Self {
        Self {
            trim_blocks: options.trim_blocks,
            wrap_text: options.wrap_text,
        }
    }
}

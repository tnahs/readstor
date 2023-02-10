pub mod app;
pub mod config;
pub mod data;
pub mod defaults;
pub mod utils;

use std::path::PathBuf;
use std::result::Result;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use once_cell::sync::Lazy;
use regex::Regex;

static RE_FILTER_QUERY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<operator>[?*=]?)(?P<field>\w*):(?P<query>.*)$").unwrap()
    //            └───┬──────────────┘└───────────┬┘ └───┬───────┘
    //                │                           │      │
    // operator ──────┘                           │      │
    //   Captures a single char representing the  │      │
    //   filter operator. Can be one of:          │      │
    //     - "?" -> any                           │      │
    //     - "*" -> all                           │      │
    //     - "=" -> exact                         │      │
    //                                            │      │
    // field ─────────────────────────────────────┘      │
    //   The field used to run filtering.                │
    //                                                   │
    // query ────────────────────────────────────────────┘
    //   The query string.
});

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    after_help = "See the documentation for more information: https://tnahs.github.io/readstor"
)]
pub struct Cli {
    #[clap(flatten)]
    pub options: Options,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Parser)]
pub struct Options {
    /// Set the output directory [default: ~/.readstor]
    #[arg(
        short = 'o',
        long,
        global = true,
        value_name = "PATH",
        value_parser(validate_path_exists),
        help_heading = "Global"
    )]
    pub output_directory: Option<PathBuf>,

    /// Set the directory containing macOS's Apple Books databases
    #[arg(
        short = 'd',
        long,
        global = true,
        value_name = "PATH",
        value_parser(validate_path_exists),
        help_heading = "Global"
    )]
    pub databases_directory: Option<PathBuf>,

    /// Set the directory containing iOS's Apple Books plists
    #[arg(
        short = 'p',
        long,
        global = true,
        value_name = "PATH",
        value_parser(validate_path_exists),
        help_heading = "Global"
    )]
    pub plists_directory: Option<PathBuf>,

    /// Run even if Apple Books is open
    #[arg(short = 'F', long, global = true, help_heading = "Global")]
    pub force: bool,

    /// Silence output messages
    #[arg(short = 'q', long = "quiet", global = true, help_heading = "Global")]
    pub is_quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Render data via templates
    Render {
        #[clap(flatten)]
        filter_options: FilterOptions,

        #[clap(flatten)]
        template_options: RenderOptions,

        #[clap(flatten)]
        preprocess_options: PreProcessOptions,

        #[clap(flatten)]
        postprocess_options: PostProcessOptions,
    },

    /// Export data as JSON
    Export {
        #[clap(flatten)]
        filter_options: FilterOptions,

        #[clap(flatten)]
        export_options: ExportOptions,

        #[clap(flatten)]
        preprocess_options: PreProcessOptions,
    },

    /// Back-up macOS's Apple Books databases
    Backup {
        #[clap(flatten)]
        backup_options: BackupOptions,
    },
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
    // TODO: Rename option?
    #[arg(short = 't', long, value_name = "TEMPLATE")]
    pub directory_template: Option<String>,

    /// Overwrite existing files
    #[arg(short = 'O', long)]
    pub overwrite_existing: bool,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct BackupOptions {
    /// Set the output directory template
    // TODO: Rename option?
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
    filter_types: Vec<FilterType>,

    /// Auto-confirm filter results
    #[arg(
        short = 'A',
        long = "auto-confirm-filter",
        requires = "filter_types",
        help_heading = "Filter"
    )]
    auto_confirm: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FilterType {
    /// Filter books by their title
    Title {
        query: Vec<String>,
        operator: FilterOperator,
    },

    /// Filter books by their author
    Author {
        query: Vec<String>,
        operator: FilterOperator,
    },

    /// Filter annotations by their #tags
    Tags {
        query: Vec<String>,
        operator: FilterOperator,
    },
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum FilterOperator {
    /// Match any of the query strings
    #[default]
    Any,

    /// Match all of the query strings
    All,

    /// Match the exact query string
    Exact,
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

    /// Wrap text to a maximum character width.
    #[arg(short = 'w', long, value_name = "WIDTH", help_heading = "Post-process")]
    pub wrap_text: Option<usize>,
}

pub fn validate_path_exists(value: &str) -> Result<PathBuf, String> {
    std::fs::canonicalize(value).map_err(|_| "path does not exist".into())
}

impl From<RenderOptions> for lib::render::templates::RenderOptions {
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

impl From<FilterOperator> for lib::filter::FilterOperator {
    fn from(filter_operator: FilterOperator) -> Self {
        match filter_operator {
            FilterOperator::Any => Self::Any,
            FilterOperator::All => Self::All,
            FilterOperator::Exact => Self::Exact,
        }
    }
}

impl From<FilterType> for lib::filter::FilterType {
    fn from(filter_type: FilterType) -> Self {
        match filter_type {
            FilterType::Title { query, operator } => Self::Title {
                query,
                operator: operator.into(),
            },
            FilterType::Author { query, operator } => Self::Author {
                query,
                operator: operator.into(),
            },
            FilterType::Tags { query, operator } => Self::Tags {
                query,
                operator: operator.into(),
            },
        }
    }
}

impl From<PreProcessOptions> for lib::process::PreProcessOptions {
    fn from(options: PreProcessOptions) -> Self {
        Self {
            extract_tags: options.extract_tags,
            normalize_whitespace: options.normalize_whitespace,
            convert_all_to_ascii: options.convert_all_to_ascii,
            convert_symbols_to_ascii: options.convert_symbols_to_ascii,
        }
    }
}

impl From<PostProcessOptions> for lib::process::PostProcessOptions {
    fn from(options: PostProcessOptions) -> Self {
        Self {
            trim_blocks: options.trim_blocks,
            wrap_text: options.wrap_text,
        }
    }
}

impl FromStr for FilterType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_FILTER_QUERY.captures(s);

        let Some(captures) = captures else {
            return Err("filters must follow the format '[op]{field}:{query}'".into());
        };

        // These unwraps are safe as they will only panic if the capture-group
        // name does not exist. These are all defined above.
        let operator = captures.name("operator").unwrap().as_str();
        let field = captures.name("field").unwrap().as_str().to_lowercase();
        let query = captures.name("query").unwrap();

        let operator = if operator.is_empty() {
            FilterOperator::default()
        } else {
            operator.parse()?
        };

        let query = query
            .as_str()
            .split(' ')
            .map(std::string::ToString::to_string)
            .collect();

        let filter_by = match field.as_str() {
            "title" => Self::Title { query, operator },
            "author" => Self::Author { query, operator },
            "tags" | "tag" => Self::Tags { query, operator },
            _ => return Err(format!("invalid field: '{field}'")),
        };

        Ok(filter_by)
    }
}

impl FromStr for FilterOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let filter_type = match s {
            "?" => Self::Any,
            "*" => Self::All,
            "=" => Self::Exact,
            _ => return Err(format!("invalid operator: '{s}'")),
        };

        Ok(filter_type)
    }
}

#[cfg(test)]
mod test_cli {

    use super::*;

    mod parse_filter {

        use super::*;

        // Title

        #[test]
        fn test_title_any() {
            assert_eq!(
                FilterType::from_str("?title:art think").unwrap(),
                FilterType::Title {
                    query: vec!["art".to_string(), "think".to_string()],
                    operator: FilterOperator::Any,
                }
            );
        }

        #[test]
        fn test_title_all() {
            assert_eq!(
                FilterType::from_str("*title:joking feynman").unwrap(),
                FilterType::Title {
                    query: vec!["joking".to_string(), "feynman".to_string()],
                    operator: FilterOperator::All,
                }
            );
        }

        #[test]
        fn test_title_exact() {
            assert_eq!(
                FilterType::from_str("=title:the art spirit").unwrap(),
                FilterType::Title {
                    query: vec!["the".to_string(), "art".to_string(), "spirit".to_string()],
                    operator: FilterOperator::Exact,
                }
            );
        }

        // Author

        #[test]
        fn test_author_any() {
            assert_eq!(
                FilterType::from_str("?author:robert richard").unwrap(),
                FilterType::Author {
                    query: vec!["robert".to_string(), "richard".to_string()],
                    operator: FilterOperator::Any,
                }
            );
        }

        #[test]
        fn test_author_all() {
            assert_eq!(
                FilterType::from_str("*author:richard feynman").unwrap(),
                FilterType::Author {
                    query: vec!["richard".to_string(), "feynman".to_string()],
                    operator: FilterOperator::All,
                }
            );
        }

        #[test]
        fn test_author_exact() {
            assert_eq!(
                FilterType::from_str("=author:richard p. feynman").unwrap(),
                FilterType::Author {
                    query: vec![
                        "richard".to_string(),
                        "p.".to_string(),
                        "feynman".to_string(),
                    ],
                    operator: FilterOperator::Exact,
                }
            );
        }

        // Tags

        #[test]
        fn test_tags_any() {
            assert_eq!(
                FilterType::from_str("?tags:#artist #death").unwrap(),
                FilterType::Tags {
                    query: vec!["#artist".to_string(), "#death".to_string()],
                    operator: FilterOperator::Any,
                }
            );
        }

        #[test]
        fn test_tags_all() {
            assert_eq!(
                FilterType::from_str("*tags:#death #impermanence").unwrap(),
                FilterType::Tags {
                    query: vec!["#death".to_string(), "#impermanence".to_string()],
                    operator: FilterOperator::All,
                }
            );
        }

        #[test]
        fn test_tags_exact() {
            assert_eq!(
                FilterType::from_str("=tags:#artist #being").unwrap(),
                FilterType::Tags {
                    query: vec!["#artist".to_string(), "#being".to_string()],
                    operator: FilterOperator::Exact,
                }
            );
        }
    }
}

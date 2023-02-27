use std::io::Write;
use std::path::Path;

use color_eyre::eyre::WrapErr;

use lib::backup::BackupRunner;
use lib::export::ExportRunner;
use lib::filter::FilterRunner;
use lib::models::entry::Entries;
use lib::process::{PostProcessRunner, PreProcessRunner};
use lib::render::templates::Templates;

use crate::cli;

use super::config::{Config, DataDirectory};
use super::data::Data;
use super::Command;

pub type Result<T> = color_eyre::Result<T>;

/// The main application struct.
#[derive(Debug)]
pub struct App {
    config: Config,
    data: Data,
}

impl App {
    /// Creates a new instance of [`App`].
    pub fn new(config: Config) -> Self {
        Self {
            config,
            data: Data::default(),
        }
    }

    /// Runs the application based off of the given [`Command`].
    pub fn run(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Render {
                filter_options,
                template_options,
                preprocess_options,
                postprocess_options,
            } => {
                self.print("-> Initializing data");
                self.init_data()?;

                self.print("-> Running pre-processes");
                Self::run_preprocesses(&mut self.data, preprocess_options);

                if !filter_options.filter_types.is_empty() {
                    self.print("-> Running filters");
                    Self::run_filters(&mut self.data, filter_options.filter_types);

                    // Show filter confirmation prompt...
                    if !filter_options.auto_confirm {
                        // ...and exit if the user does not confirm.
                        if !self.confirm_filter_results() {
                            return Ok(());
                        }
                    }
                }

                self.print("-> Initializing templates");
                let mut templates = Self::init_templates(template_options)?;

                self.print("-> Rendering templates");
                Self::render_templates(&mut templates, &mut self.data)?;

                self.print("-> Running post-processes");
                Self::run_postprocesses(&mut templates, postprocess_options);

                self.print("-> Writing templates");
                Self::write_templates(&templates, &self.config.output_directory)?;

                let count_templates = templates.count_templates();
                let count_renders = templates.count_renders();

                let summary = format!(
                    "-> Rendered {count_templates} template{} into {count_renders} file{} to {}",
                    if count_templates == 1 { "" } else { "s" },
                    if count_renders == 1 { "" } else { "s" },
                    self.config.output_directory.display()
                );

                self.print(&summary);
            }
            Command::Export {
                filter_options,
                preprocess_options,
                export_options,
            } => {
                self.print("-> Initializing data");
                self.init_data()?;

                self.print("-> Running pre-processes");
                Self::run_preprocesses(&mut self.data, preprocess_options);

                if !filter_options.filter_types.is_empty() {
                    self.print("-> Running filters");
                    Self::run_filters(&mut self.data, filter_options.filter_types);

                    // Show filter confirmation prompt...
                    if !filter_options.auto_confirm {
                        // ...and exit if the user does not confirm.
                        if !self.confirm_filter_results() {
                            return Ok(());
                        }
                    }
                }

                self.print("-> Exporting data");
                Self::export_data(
                    &mut self.data,
                    &self.config.output_directory,
                    export_options,
                )?;

                let count_books = self.data.count_books();
                let count_annotations = self.data.count_annotations();

                let summary = format!(
                    "-> Exported {count_annotations} annotation{} from {count_books} book{} to {}",
                    if count_annotations == 1 { "" } else { "s" },
                    if count_books == 1 { "" } else { "s" },
                    self.config.output_directory.display()
                );

                self.print(&summary);
            }
            Command::Backup { backup_options } => {
                self.print("-> Backing-up databases");

                // TODO: It might be nice to eventually support this.
                let DataDirectory::Macos(databases) = &self.config.data_directory else {
                    println!("Backing-up iOS's Apple Books plists is currently unsupported.");
                    return Ok(());
                };

                Self::backup_databases(databases, &self.config.output_directory, backup_options)?;

                let summary = &format!(
                    "-> Backed-up databases to {}",
                    self.config.output_directory.display()
                );

                self.print(summary);
            }
        }

        Ok(())
    }

    /// Initializes the application's data.
    fn init_data(&mut self) -> Result<()> {
        let error_ios = "Failed while initializing iOS's Apple Books plists data";
        let error_macos = "Failed while initializing macOS's Apple Books databases data";

        match &self.config.data_directory {
            DataDirectory::Macos(path) => {
                self.data.init_macos(path).wrap_err(error_macos)?;
            }
            DataDirectory::Ios(path) => {
                self.data.init_ios(path).wrap_err(error_ios)?;
            }
            DataDirectory::Both {
                path_macos,
                path_ios,
            } => {
                self.data.init_macos(path_macos).wrap_err(error_macos)?;
                self.data.init_ios(path_ios).wrap_err(error_ios)?;
            }
        }

        Ok(())
    }

    /// Renders templates.
    ///
    /// # Arguments
    ///
    /// * `templates` - The templates to render with.
    /// * `entries` - The [`Entry`][entry]s to render.
    ///
    /// [entry]: lib::models::entry::Entry
    fn render_templates(templates: &mut Templates, entries: &mut Entries) -> Result<()> {
        entries.values_mut().try_for_each(|entry| {
            templates
                .render(entry)
                .wrap_err("Failed while rendering template(s)")
        })
    }

    /// Writes templates to disk.
    ///
    /// # Arguments
    ///
    /// * `templates` - The templates to write.
    /// * `path` - The ouput directory.
    fn write_templates(templates: &Templates, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path)?;

        templates
            .write(path)
            .wrap_err("Failed while writing template(s)")
    }

    /// Exports data to disk.
    ///
    /// # Arguments
    ///
    /// * `entries` - The [`Entry`][entry]s to export.
    /// * `path` - The ouput directory.
    /// * `options` - The export options.
    ///
    /// [entry]: lib::models::entry::Entry
    fn export_data(entries: &mut Entries, path: &Path, options: cli::ExportOptions) -> Result<()> {
        ExportRunner::run(entries, path, options).wrap_err("Failed while exporting data")?;

        Ok(())
    }

    /// Backs-up macOS's Apple Books databases to disk.
    ///
    /// # Arguments
    ///
    /// * `databases` - The directory to back-up.
    /// * `output` - The ouput directory.
    /// * `options` - The back-up options.
    fn backup_databases(
        databases: &Path,
        output: &Path,
        options: cli::BackupOptions,
    ) -> Result<()> {
        BackupRunner::run(databases, output, options)
            .wrap_err("Failed while backing-up databases")?;

        Ok(())
    }

    /// Initializes templates.
    ///
    /// # Arguments
    ///
    /// * `options` - The [`Templates`]' options.
    fn init_templates(options: cli::RenderOptions) -> Result<Templates> {
        let mut templates = Templates::new(options, super::defaults::TEMPLATE.into());

        templates
            .init()
            .wrap_err("Failed while initializing template(s)")?;

        Ok(templates)
    }

    /// Runs pre-processes on all [`Entry`][entry]s.
    ///
    /// # Arguments
    ///
    /// * `entries` - The [`Entry`][entry]s to run pre-processors on.
    /// * `options` - The pre-process options.
    ///
    /// [entry]: lib::models::entry::Entry
    fn run_preprocesses(entries: &mut Entries, options: cli::PreProcessOptions) {
        PreProcessRunner::run(entries, options);
    }

    /// Runs filters on all [`Entry`][entry]s.
    ///
    /// # Arguments
    ///
    /// * `entries` - The [`Entry`][entry]s to run filters on.
    /// * `filter_types` - The filters to run.
    ///
    /// [entry]: lib::models::entry::Entry
    fn run_filters(entries: &mut Entries, filter_types: Vec<cli::FilterType>) {
        for filter_type in filter_types {
            FilterRunner::run(filter_type, entries);
        }
    }

    /// Runs post-processes on all [`TemplateRender`][template-render]s.
    ///
    /// # Arguments
    ///
    /// * `entries` - The [`Entry`][entry]s to run post-processors on.
    /// * `options` - The post-process options.
    ///
    /// [entry]: lib::models::entry::Entry
    /// [template-render]: lib::render::template::TemplateRender
    fn run_postprocesses(templates: &mut Templates, options: cli::PostProcessOptions) {
        PostProcessRunner::run(templates.renders_mut().collect(), options);
    }

    /// Prompts the user to confirm the filter results. Returns a boolean
    /// representing whether or not to continue.
    fn confirm_filter_results(&self) -> bool {
        let indent = " ".repeat(3);
        let line = "-".repeat(64);

        println!("{indent}{line}");

        let count_books = self.data.count_books();

        if count_books == 0 {
            println!("{indent}No annotations found.");
            println!("{indent}{line}");
            return false;
        }

        let count_annotations = self.data.count_annotations();

        println!(
            "{indent}Found {count_annotations} annotation{} from {count_books} book{}:",
            if count_annotations == 1 { "" } else { "s" },
            if count_books == 1 { "" } else { "s" },
        );

        for book in self.data.iter_books() {
            println!("{indent} • {} by {}", book.title, book.author);
        }

        println!("{indent}{line}");

        print!("{indent}Continue? [y/N]: ");

        let mut confirm = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut confirm).unwrap();

        println!();

        matches!(confirm.trim().to_lowercase().as_str(), "y" | "yes")
    }

    /// Prints to the terminal. Allows muting.
    fn print(&self, message: &str) {
        if !self.config.is_quiet {
            println!("{message}");
        }
    }
}

#[cfg(test)]
mod test_app {

    use crate::cli::config::tests::TestConfig;

    use super::*;

    mod macos {

        use super::*;

        // Tests that empty databases return zero books and zero annotations.
        #[test]
        fn test_empty() {
            let config = TestConfig::macos_empty();
            let mut app = App::new(config);

            app.init_data().unwrap();

            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that databases with un-annotated books return zero books and
        // zero annotations.
        #[test]
        fn test_books_new() {
            let config = TestConfig::macos_new();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Un-annotated books are filtered out.
            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that databases with annotated books return non-zero books and
        // non-zero annotations.
        #[test]
        fn test_books_annotated() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            assert_eq!(app.data.iter_books().count(), 3);
            assert_eq!(app.data.iter_annotations().count(), 10);
        }

        // Tests that annotations are sorted in the correct order.
        #[test]
        fn test_annotations_order() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor sorts the annotations.
            App::run_preprocesses(&mut app.data, cli::PreProcessOptions::default());

            for entry in app.data.values() {
                for annotations in entry.annotations.windows(2) {
                    assert!(annotations[0] < annotations[1]);
                }
            }
        }
    }

    mod ios {

        use super::*;

        // Tests that empty plist files return zero books and zero annotations.
        #[test]
        fn test_empty() {
            let config = TestConfig::ios_empty();
            let mut app = App::new(config);

            app.init_data().unwrap();

            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that plist files with un-annotated books return zero books and
        // zero annotation.
        #[test]
        fn test_books_new() {
            let config = TestConfig::ios_new();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Un-annotated books are filtered out.
            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that plist files with annotated books return non-zero books and
        // non-zero annotations.
        #[test]
        fn test_books_annotated() {
            let config = TestConfig::ios_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            assert_eq!(app.data.iter_books().count(), 3);
            assert_eq!(app.data.iter_annotations().count(), 7);
        }

        // Tests that annotations are sorted in the correct order.
        #[test]
        fn test_annotations_order() {
            let config = TestConfig::ios_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor sorts the annotations.
            App::run_preprocesses(&mut app.data, cli::PreProcessOptions::default());

            for entry in app.data.values() {
                for annotations in entry.annotations.windows(2) {
                    assert!(annotations[0] < annotations[1]);
                }
            }
        }
    }

    mod both {

        use super::*;

        // Tests that empty databases and plist files return zero books and
        // zero annotations.
        #[test]
        fn test_empty() {
            let config = TestConfig::both_empty();
            let mut app = App::new(config);

            app.init_data().unwrap();

            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that databses and plist files with un-annotated books return
        // zero books and zero annotation.
        #[test]
        fn test_books_new() {
            let config = TestConfig::both_new();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Un-annotated books are filtered out.
            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that databases and plist files with annotated books return non-
        // zero books and non-zero annotations.
        #[test]
        fn test_books_annotated() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            assert_eq!(app.data.iter_books().count(), 6);
            assert_eq!(app.data.iter_annotations().count(), 17);
        }

        // Tests that annotations are sorted in the correct order.
        #[test]
        fn test_annotations_order() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor sorts the annotations.
            App::run_preprocesses(&mut app.data, cli::PreProcessOptions::default());

            for entry in app.data.values() {
                for annotations in entry.annotations.windows(2) {
                    assert!(annotations[0] < annotations[1]);
                }
            }
        }
    }

    mod filter {

        use super::*;

        // Title

        #[test]
        fn test_title_any() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "?title:art think"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Title {
                    query: vec!["art".to_string(), "think".to_string()],
                    operator: cli::FilterOperator::Any,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 2);
            assert_eq!(app.data.iter_annotations().count(), 9);
        }

        #[test]
        fn test_title_all() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "*title:joking feynman"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Title {
                    query: vec!["joking".to_string(), "feynman".to_string()],
                    operator: cli::FilterOperator::All,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        #[test]
        fn test_title_exact() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "=title:the art spirit"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Title {
                    query: vec!["the".to_string(), "art".to_string(), "spirit".to_string()],
                    operator: cli::FilterOperator::Exact,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 4);
        }

        // Author

        #[test]
        fn test_author_any() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "?author:robert richard"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Author {
                    query: vec!["robert".to_string(), "richard".to_string()],
                    operator: cli::FilterOperator::Any,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 2);
            assert_eq!(app.data.iter_annotations().count(), 5);
        }

        #[test]
        fn test_author_all() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "*author:richard feynman"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Author {
                    query: vec!["richard".to_string(), "feynman".to_string()],
                    operator: cli::FilterOperator::All,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        #[test]
        fn test_author_exact() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "=author:richard p. feynman"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Author {
                    query: vec![
                        "richard".to_string(),
                        "p.".to_string(),
                        "feynman".to_string(),
                    ],
                    operator: cli::FilterOperator::Exact,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        // Tags

        #[test]
        fn test_tags_any() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor extracts the tags.
            App::run_preprocesses(
                &mut app.data,
                cli::PreProcessOptions {
                    extract_tags: true,
                    ..Default::default()
                },
            );

            // Filter string: "?tags:#artist #death"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Tags {
                    query: vec!["#artist".to_string(), "#death".to_string()],
                    operator: cli::FilterOperator::Any,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 2);
            assert_eq!(app.data.iter_annotations().count(), 2);
        }

        #[test]
        fn test_tags_all() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor extracts the tags.
            App::run_preprocesses(
                &mut app.data,
                cli::PreProcessOptions {
                    extract_tags: true,
                    ..Default::default()
                },
            );

            // Filter string: "*tags:#death #impermanence"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Tags {
                    query: vec!["#death".to_string(), "#impermanence".to_string()],
                    operator: cli::FilterOperator::All,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        #[test]
        fn test_tags_exact() {
            let config = TestConfig::both_annotated();
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor extracts the tags.
            App::run_preprocesses(
                &mut app.data,
                cli::PreProcessOptions {
                    extract_tags: true,
                    ..Default::default()
                },
            );

            // Filter string: "=tags:#artist #being"
            App::run_filters(
                &mut app.data,
                vec![cli::FilterType::Tags {
                    query: vec!["#artist".to_string(), "#being".to_string()],
                    operator: cli::FilterOperator::Exact,
                }],
            );

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }
    }

    mod backup {
        use std::path::PathBuf;

        use lib::result::Error;

        use crate::cli::defaults::MockDatabases;
        use crate::cli::BackupOptions;

        use super::*;

        // Tests that a valid template returns no error.
        #[test]
        fn valid_template() {
            let options = BackupOptions {
                directory_template: Some("{{ now }}".to_string()),
            };

            let databases: PathBuf = MockDatabases::Empty.into();
            let output = crate::cli::defaults::TEMP_OUTPUT_DIRECTORY.join("tests");

            BackupRunner::run(&databases, &output, options).unwrap();
        }

        // Tests that an invalid template returns an error.
        #[test]
        fn invalid_template() {
            let options = BackupOptions {
                directory_template: Some("{{ invalid }}".to_string()),
            };

            let databases: PathBuf = MockDatabases::Empty.into();
            let output = crate::cli::defaults::TEMP_OUTPUT_DIRECTORY.join("tests");

            let result = BackupRunner::run(&databases, &output, options);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }
    }

    mod export {
        use std::collections::HashMap;

        use lib::result::Error;

        use crate::cli::ExportOptions;

        use super::*;

        // Tests that a valid template returns no error.
        #[test]
        fn valid_template() {
            let options = ExportOptions {
                directory_template: Some("{{ book.author }} - {{ book.title }}".to_string()),
                ..Default::default()
            };

            let mut entries = HashMap::new();
            let path = crate::cli::defaults::TEMP_OUTPUT_DIRECTORY.join("tests");

            ExportRunner::run(&mut entries, &path, options).unwrap();
        }

        // Tests that an invalid template returns an error.
        #[test]
        fn invalid_template() {
            let options = ExportOptions {
                directory_template: Some("{{ invalid }}".to_string()),
                ..Default::default()
            };

            let mut entries = HashMap::new();
            let path = crate::cli::defaults::TEMP_OUTPUT_DIRECTORY.join("tests");

            let result = ExportRunner::run(&mut entries, &path, options);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }
    }
}

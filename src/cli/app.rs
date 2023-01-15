use std::io::Write;
use std::path::Path;

use color_eyre::eyre::WrapErr;

use lib::backup::BackupRunner;
use lib::export::ExportRunner;
use lib::filter::FilterRunner;
use lib::models::data::{Data, Entries};
use lib::process::{PostProcessRunner, PreProcessRunner};
use lib::render::templates::Templates;

use crate::cli;

use super::config::Config;
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

                #[rustfmt::skip]
                let summary = format!(
                    "-> Rendered {} template{} into {} file{} to {}",
                    templates.count_templates(),
                    if templates.count_templates() == 1 { "" } else { "s" },
                    templates.count_renders(),
                    if templates.count_renders() == 1 { "" } else { "s" },
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

                #[rustfmt::skip]
                let summary = format!(
                    "-> Exported {} annotation{} from {} book{} to {}",
                    self.data.annotations().count(),
                    if self.data.annotations().count() == 1 { "" } else { "s" },
                    self.data.books().count(),
                    if self.data.books().count() == 1 { "" } else { "s" },
                    self.config.output_directory.display()
                );

                self.print(&summary);
            }
            Command::Backup { backup_options } => {
                self.print("-> Backing-up databases");
                Self::backup_databases(
                    &self.config.databases_directory,
                    &self.config.output_directory,
                    backup_options,
                )?;

                let summary = &format!(
                    "-> Backed-up databases to {}",
                    self.config.output_directory.display()
                );

                self.print(summary);
            }
        }

        Ok(())
    }

    /// Initializes the application's data from the Apple Books databases.
    fn init_data(&mut self) -> Result<()> {
        self.data
            .init(&self.config.databases_directory)
            .wrap_err("Failed while initializing data")
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

    /// Exports Apple Books' data to disk.
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

    /// Backs-up Apple Books' databases to disk.
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

        if self.data.annotations().count() == 0 {
            println!("{indent}No annotations found.");
            println!("{indent}{line}");
            return false;
        }

        #[rustfmt::skip]
        println!(
            "{indent}Found {} annotation{} from {} book{}:",
            self.data.annotations().count(),
            if self.data.annotations().count() == 1 { "" } else { "s" },
            self.data.books().count(),
            if self.data.books().count() == 1 { "" } else { "s" },
        );

        for book in self.data.books() {
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

    use crate::cli::config::Config;

    use super::*;

    // Tests that an empty database returns zero books and zero annotations.
    #[test]
    fn test_databases_empty() {
        let config = Config::test("empty");
        let mut app = App::new(config);

        app.init_data().unwrap();

        assert_eq!(app.data.books().count(), 0);
        assert_eq!(app.data.annotations().count(), 0);
    }

    // Tests that a database with un-annotated books returns zero books and
    // zero annotations.
    #[test]
    fn test_databases_books_new() {
        let config = Config::test("books-new");
        let mut app = App::new(config);

        app.init_data().unwrap();

        // Un-annotated books are filtered out.
        assert_eq!(app.data.books().count(), 0);
        assert_eq!(app.data.annotations().count(), 0);
    }

    // Tests that a database with annotated books returns non-zero books and
    // non-zero annotations.
    #[test]
    fn test_databases_books_annotated() {
        let config = Config::test("books-annotated");
        let mut app = App::new(config);

        app.init_data().unwrap();

        assert_eq!(app.data.books().count(), 3);
        assert_eq!(app.data.annotations().count(), 10);
    }

    // Tests that the annotations are sorted in the correct order.
    #[test]
    fn test_annotations_order() {
        let config = Config::test("books-annotated");
        let mut app = App::new(config);

        app.init_data().unwrap();

        // The pre-processor sorts the annotations.
        App::run_preprocesses(&mut app.data, cli::PreProcessOptions::default());

        for entry in app.data.entries() {
            for annotations in entry.annotations.windows(2) {
                assert!(annotations[0] < annotations[1]);
            }
        }
    }

    mod filter {

        use super::*;

        // Title

        #[test]
        fn test_title_any() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 2);
            assert_eq!(app.data.annotations().count(), 9);
        }

        #[test]
        fn test_title_all() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }

        #[test]
        fn test_title_exact() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 4);
        }

        // Author

        #[test]
        fn test_author_any() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 2);
            assert_eq!(app.data.annotations().count(), 5);
        }

        #[test]
        fn test_author_all() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }

        #[test]
        fn test_author_exact() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }

        // Tags

        #[test]
        fn test_tags_any() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 2);
            assert_eq!(app.data.annotations().count(), 2);
        }

        #[test]
        fn test_tags_all() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }

        #[test]
        fn test_tags_exact() {
            let config = Config::test("books-annotated");
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

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }
    }
}

use std::fs;
use std::io::Write;

use color_eyre::eyre::WrapErr;
use lib::filters::FilterRunner;

use crate::cli;
use lib::applebooks::database::ABDatabaseName;
use lib::applebooks::utils::APPLEBOOKS_VERSION;
use lib::models::data::Data;
use lib::processor::{PostProcessor, PreProcessor};
use lib::templates::Templates;
use lib::utils;

use super::config::Config;
use super::Command;
use super::FilterType;

pub type Result<T> = color_eyre::Result<T>;

/// The main application struct.
///
/// Contains a single public method to run the application. A provided
/// [`Config`] is used to change its behavior.
#[derive(Debug)]
pub struct App {
    config: Config,
    data: Data,
    templates: Option<Templates>,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            data: Data::default(),
            templates: None,
        }
    }

    /// Runs the application based off of the given [`Command`].
    pub fn run(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Export {
                filter_options,
                preprocessor_options,
            } => {
                self.print("-> Initializing data");
                self.init_data()?;

                self.print("-> Running pre-processors");
                self.run_preprocessors(preprocessor_options);

                if !filter_options.filters.is_empty() {
                    self.print("-> Running filters");
                    self.run_filters(filter_options.filters);

                    // Show filter confirmation prompt...
                    if !filter_options.auto_confirm {
                        // ...and exit if the user does not confirm.
                        if !self.confirm_filter_results() {
                            return Ok(());
                        }
                    }
                }

                self.print("-> Exporting data");
                self.export_data().wrap_err("Failed while exporting data")?;

                self.print_export_summary();
            }
            Command::Render {
                filter_options,
                template_options,
                preprocessor_options,
                postprocessor_options,
            } => {
                self.print("-> Initializing data");
                self.init_data()?;

                self.print("-> Running pre-processors");
                self.run_preprocessors(preprocessor_options);

                if !filter_options.filters.is_empty() {
                    self.print("-> Running filters");
                    self.run_filters(filter_options.filters);

                    // Show filter confirmation prompt...
                    if !filter_options.auto_confirm {
                        // ...and exit if the user does not confirm.
                        if !self.confirm_filter_results() {
                            return Ok(());
                        }
                    }
                }

                self.print("-> Initializing templates");
                self.init_templates(template_options)?;

                self.print("-> Rendering templates");
                self.render_templates()?;

                self.print("-> Running post-processors");
                self.run_postprocessors(postprocessor_options);

                self.print("-> Writing templates");
                self.write_templates()?;

                self.print_render_summary();
            }
            Command::Backup => {
                self.print("-> Backing-up databases");
                self.backup_databases()
                    .wrap_err("Failed while backing-up databases")?;

                self.print_backup_summary();
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

    /// Initializes templates.
    fn init_templates(&mut self, options: cli::TemplateOptions) -> Result<()> {
        let mut templates = Templates::new(options, super::defaults::TEMPLATE.into());

        templates
            .init()
            .wrap_err("Failed while initializing template(s)")?;

        self.templates = Some(templates);

        Ok(())
    }

    /// Runs pre-processors on all [`Entry`][entry]s.
    ///
    /// [entry]: lib::models::entry::Entry
    fn run_preprocessors(&mut self, options: cli::PreProcessorOptions) {
        self.data
            .entries_mut()
            .for_each(|entry| PreProcessor::run(options, entry));
    }

    /// Runs filters on all [`Entry`][entry]s.
    ///
    /// [entry]: lib::models::entry::Entry
    fn run_filters(&mut self, filters: Vec<FilterType>) {
        for filter in filters {
            FilterRunner::run(filter, &mut self.data);
        }
    }

    /// Runs post-processors on all [`TemplateRender`][template-render]s.
    ///
    /// [template-render]: lib::templates::template::TemplateRender
    fn run_postprocessors(&mut self, options: cli::PostProcessorOptions) {
        self.templates
            .as_mut()
            .expect("attempted to access un-initialized templates")
            .renders_mut()
            .for_each(|render| PostProcessor::run(options, render));
    }

    /// Renders templates.
    fn render_templates(&mut self) -> Result<()> {
        let templates = self
            .templates
            .as_mut()
            .expect("attempted to access un-initialized templates");

        self.data.entries().try_for_each(|entry| {
            templates
                .render(entry)
                .wrap_err("Failed while rendering template(s)")
        })
    }

    /// Writes templates to disk.
    fn write_templates(&self) -> Result<()> {
        let templates = self
            .templates
            .as_ref()
            .expect("attempted to access un-initialized templates");

        // -> [ouput-directory]
        let path = &self.config.output_directory;

        fs::create_dir_all(path)?;

        templates
            .write(path)
            .wrap_err("Failed while writing template(s)")
    }

    /// Exports Apple Books' data with the following structure:
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├─ [author-title]
    ///  │   │
    ///  │   ├─ data
    ///  │   │   ├─ book.json
    ///  │   │   └─ annotations.json
    ///  │   │
    ///  │   └─ resources
    ///  │       ├─ .gitkeep
    ///  │       ├─ [author-title].epub   ─┐
    ///  │       ├─ cover.jpeg             ├─ These are not exported.
    ///  │       └─ ...                   ─┘
    ///  │
    ///  ├─ [author-title]
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    ///
    /// Existing files are left unaffected unless explicitly written to. For
    /// example, the `resources` directory will not be deleted/recreated if it
    /// already exists and/or contains data.
    fn export_data(&self) -> Result<()> {
        // -> [ouput-directory]
        let path = &self.config.output_directory;

        for entry in self.data.entries() {
            // -> [ouput-directory]/[author-title]
            let item = path.join(entry.slug_name());
            // -> [ouput-directory]/[author-title]/data
            let data = item.join("data");
            // -> [ouput-directory]/[author-title]/resources
            let resources = item.join("resources");

            fs::create_dir_all(&item)?;
            fs::create_dir_all(&data)?;
            fs::create_dir_all(&resources)?;

            // -> [ouput-directory]/[author-title]/data/book.json
            let book_json = data.join("book").with_extension("json");
            let book_file = fs::File::create(book_json)?;

            serde_json::to_writer_pretty(&book_file, &entry.book)?;

            // -> [ouput-directory]/[author-title]/data/annotation.json
            let annotations_json = data.join("annotations").with_extension("json");
            let annotations_file = fs::File::create(annotations_json)?;

            serde_json::to_writer_pretty(&annotations_file, &entry.annotations)?;

            // -> [ouput-directory]/[author-title]/resources/.gitkeep
            let gitkeep = resources.join(".gitkeep");
            fs::File::create(gitkeep)?;
        }

        Ok(())
    }

    /// Backs up Apple Books' databases with the following structure:
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├─ [YYYY-MM-DD-HHMMSS-VERSION]
    ///  │   │
    ///  │   ├─ AEAnnotation
    ///  │   │   ├─ AEAnnotation*.sqlite
    ///  │   │   └─ ...
    ///  │   │
    ///  │   └─ BKLibrary
    ///  │       ├─ BKLibrary*.sqlite
    ///  │       └─ ...
    ///  │
    ///  │─ [YYYY-MM-DD-HHMMSS-VERSION]
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    fn backup_databases(&self) -> Result<()> {
        // -> [ouput-directory]
        let path = &self.config.output_directory;

        // -> [YYYY-MM-DD-HHMMSS]-[VERSION]
        let today = format!("{}-{}", utils::today(), *APPLEBOOKS_VERSION);

        // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]
        let destination_root = path.join(today);

        // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]/BKLibrary
        let destination_books = destination_root.join(ABDatabaseName::Books.to_string());

        // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]/AEAnnotation
        let destination_annotations =
            destination_root.join(ABDatabaseName::Annotations.to_string());

        // -> [DATABASES]/BKLibrary
        let source_books = &self
            .config
            .databases_directory
            .join(ABDatabaseName::Books.to_string());

        // -> [DATABASES]/AEAnnotation
        let source_annotations = &self
            .config
            .databases_directory
            .join(ABDatabaseName::Annotations.to_string());

        utils::copy_dir(source_books, destination_books)?;
        utils::copy_dir(source_annotations, destination_annotations)?;

        Ok(())
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

    /// Prints the export summary.
    fn print_export_summary(&self) {
        self.print(&format!(
            "-> Exported {} annotation(s) from {} book(s) to {}",
            self.data.annotations().count(),
            self.data.books().count(),
            self.config.output_directory.display()
        ));
    }

    /// Prints the render summary.
    fn print_render_summary(&self) {
        let templates = self
            .templates
            .as_ref()
            .expect("attempted to access un-initialized templates.");

        self.print(&format!(
            "-> Rendered {} template(s) into {} file(s) to {}",
            templates.count_templates(),
            templates.count_renders(),
            self.config.output_directory.display()
        ));
    }

    /// Prints the back-up summary.
    fn print_backup_summary(&self) {
        self.print(&format!(
            "-> Backed-up databases to {}",
            self.config.output_directory.display()
        ));
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
        app.run_preprocessors(cli::PreProcessorOptions::default());

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
            app.run_filters(vec![cli::FilterType::Title {
                query: vec!["art".to_string(), "think".to_string()],
                operator: cli::FilterOperator::Any,
            }]);

            assert_eq!(app.data.books().count(), 2);
            assert_eq!(app.data.annotations().count(), 9);
        }

        #[test]
        fn test_title_all() {
            let config = Config::test("books-annotated");
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "*title:joking feynman"
            app.run_filters(vec![cli::FilterType::Title {
                query: vec!["joking".to_string(), "feynman".to_string()],
                operator: cli::FilterOperator::All,
            }]);

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }

        #[test]
        fn test_title_exact() {
            let config = Config::test("books-annotated");
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "=title:the art spirit"
            app.run_filters(vec![cli::FilterType::Title {
                query: vec!["the".to_string(), "art".to_string(), "spirit".to_string()],
                operator: cli::FilterOperator::Exact,
            }]);

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
            app.run_filters(vec![cli::FilterType::Author {
                query: vec!["robert".to_string(), "richard".to_string()],
                operator: cli::FilterOperator::Any,
            }]);

            assert_eq!(app.data.books().count(), 2);
            assert_eq!(app.data.annotations().count(), 5);
        }

        #[test]
        fn test_author_all() {
            let config = Config::test("books-annotated");
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "*author:richard feynman"
            app.run_filters(vec![cli::FilterType::Author {
                query: vec!["richard".to_string(), "feynman".to_string()],
                operator: cli::FilterOperator::All,
            }]);

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }

        #[test]
        fn test_author_exact() {
            let config = Config::test("books-annotated");
            let mut app = App::new(config);

            app.init_data().unwrap();

            // Filter string: "=author:richard p. feynman"
            app.run_filters(vec![cli::FilterType::Author {
                query: vec![
                    "richard".to_string(),
                    "p.".to_string(),
                    "feynman".to_string(),
                ],
                operator: cli::FilterOperator::Exact,
            }]);

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
            app.run_preprocessors(cli::PreProcessorOptions {
                extract_tags: true,
                ..Default::default()
            });

            // Filter string: "?tags:#artist #death"
            app.run_filters(vec![cli::FilterType::Tags {
                query: vec!["#artist".to_string(), "#death".to_string()],
                operator: cli::FilterOperator::Any,
            }]);

            assert_eq!(app.data.books().count(), 2);
            assert_eq!(app.data.annotations().count(), 2);
        }

        #[test]
        fn test_tags_all() {
            let config = Config::test("books-annotated");
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor extracts the tags.
            app.run_preprocessors(cli::PreProcessorOptions {
                extract_tags: true,
                ..Default::default()
            });

            // Filter string: "*tags:#death #impermanence"
            app.run_filters(vec![cli::FilterType::Tags {
                query: vec!["#death".to_string(), "#impermanence".to_string()],
                operator: cli::FilterOperator::All,
            }]);

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }

        #[test]
        fn test_tags_exact() {
            let config = Config::test("books-annotated");
            let mut app = App::new(config);

            app.init_data().unwrap();

            // The pre-processor extracts the tags.
            app.run_preprocessors(cli::PreProcessorOptions {
                extract_tags: true,
                ..Default::default()
            });

            // Filter string: "=tags:#artist #being"
            app.run_filters(vec![cli::FilterType::Tags {
                query: vec!["#artist".to_string(), "#being".to_string()],
                operator: cli::FilterOperator::Exact,
            }]);

            assert_eq!(app.data.books().count(), 1);
            assert_eq!(app.data.annotations().count(), 1);
        }
    }
}

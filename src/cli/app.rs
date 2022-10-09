use std::fs;

use color_eyre::eyre::WrapErr;

use crate::lib::applebooks::database::ABDatabaseName;
use crate::lib::applebooks::utils::APPLEBOOKS_VERSION;
use crate::lib::models::data::Data;
use crate::lib::processor::{self, Processor};
use crate::lib::templates::manager::TemplateManager;
use crate::lib::utils;

use super::config::Config;
use super::{Command, PreProcessOptions, TemplateOptions};

pub type Result<T> = color_eyre::Result<T>;

/// The main application struct.
///
/// Contains a single public method to run the application. A provided
/// [`Config`] is used to change its behavior.
#[derive(Debug)]
pub struct App {
    config: Config,
    data: Data,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            data: Data::default(),
        }
    }

    /// Runs the application based off of the given [`Command`].
    pub fn run(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Export { preprocess_options } => {
                self.print("-> Building data");
                self.init_data()?;
                self.print("-> Running pre-processor");
                self.run_preprocess(preprocess_options);
                self.print("-> Exporting data");
                self.export_data().wrap_err("Failed while exporting data")?;
                self.print_summary();
            }
            Command::Render {
                template_options,
                preprocess_options,
            } => {
                self.print("-> Building data");
                self.init_data()?;
                self.print("-> Running pre-processor");
                self.run_preprocess(preprocess_options);
                self.print("-> Rendering templates");
                self.render_templates(template_options)?;
                self.print_summary();
            }
            Command::Backup => {
                self.print("-> Backing-up databases");
                self.backup_databases()
                    .wrap_err("Failed while backing up databases")?;
            }
        }

        Ok(())
    }

    /// Builds the application's data from the Apple Books databases.
    fn init_data(&mut self) -> Result<()> {
        self.data
            .build(&self.config.databases_directory)
            .wrap_err("Failed while building data")
    }

    /// Runs pre-proces on all [`Entry`][entry]s.
    ///
    /// [entry]: crate::lib::models::entry::Entry
    fn run_preprocess(&mut self, options: PreProcessOptions) {
        let options = processor::PreProcessOptions::from(options);
        for entry in self.data.entries_mut() {
            Processor::preprocess(options, entry);
        }
    }

    /// Prints export/render summary.
    fn print_summary(&self) {
        self.print(&format!(
            "-> Saved {} annotations from {} books to {}",
            self.data.count_annotations(),
            self.data.count_books(),
            self.config.output_directory.display()
        ));
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

    /// Renders all registered templates.
    fn render_templates(&self, options: TemplateOptions) -> Result<()> {
        // -> [ouput-directory]
        let path = &self.config.output_directory;

        fs::create_dir_all(path)?;

        let mut template_manager =
            TemplateManager::new(options.into(), super::defaults::TEMPLATE.into());

        template_manager
            .build_templates()
            .wrap_err("Failed while building template(s)")?;

        // Renders each `Entry` i.e. a `Book` and its `Annotation`s.
        for entry in self.data.entries() {
            template_manager
                .render(entry, path)
                .wrap_err("Failed while rendering template(s)")?;
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
        let destination_root = path.join(&today);

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

        utils::copy_dir(source_books, &destination_books)?;
        utils::copy_dir(source_annotations, &destination_annotations)?;

        Ok(())
    }

    /// Prints to the terminal. Allows muting.
    fn print(&self, message: &str) {
        if !self.config.is_quiet {
            println!("{}", message);
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

        assert_eq!(app.data.count_books(), 0);
        assert_eq!(app.data.count_annotations(), 0);
    }

    // Tests that a database with un-annotated books returns zero books and
    // zero annotations.
    #[test]
    fn test_databases_books_new() {
        let config = Config::test("books-new");
        let mut app = App::new(config);

        app.init_data().unwrap();

        // Un-annotated books are filtered out.
        assert_eq!(app.data.count_books(), 0);
        assert_eq!(app.data.count_annotations(), 0);
    }

    // Tests that a database with annotated books returns non-zero books and
    // non-zero annotations.
    #[test]
    fn test_databases_books_annotated() {
        let config = Config::test("books-annotated");
        let mut app = App::new(config);

        app.init_data().unwrap();

        assert_eq!(app.data.count_books(), 3);
        assert_eq!(app.data.count_annotations(), 10);
    }

    // Tests that the annotations are sorted in the correct order.
    #[test]
    fn test_annotations_order() {
        let config = Config::test("books-annotated");
        let mut app = App::new(config);

        app.init_data().unwrap();
        app.run_preprocess(PreProcessOptions::default());

        for entry in app.data.entries() {
            for annotations in entry.annotations.windows(2) {
                assert!(annotations[0] < annotations[1]);
            }
        }
    }
}

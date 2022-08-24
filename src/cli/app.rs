use std::fs;

use color_eyre::eyre::WrapErr;

use crate::lib::applebooks::database::ABDatabaseName;
use crate::lib::applebooks::utils::APPLEBOOKS_VERSION;
use crate::lib::models::data::Data;
use crate::lib::templates::TemplateManager;
use crate::lib::utils;

use super::args::ArgCommand;
use super::config::Config;

pub type AppResult<T> = color_eyre::Result<T>;

/// The main application struct.
///
/// Contains a single public method to run the application. A provided
/// [`Config`] is used to change its behavior.
#[derive(Debug)]
pub struct App {
    data: Data,
    config: Box<dyn Config>,
    template_manager: TemplateManager,
}

impl App {
    pub fn new(config: Box<dyn Config>) -> Self {
        Self {
            data: Data::default(),
            config,
            template_manager: TemplateManager::default(),
        }
    }

    /// Runs the application with a specified command represented by an
    /// [`ArgCommand`].
    pub fn run(&mut self, command: ArgCommand) -> AppResult<()> {
        self.print_msg("• Building data...");
        self.build_data()?;

        match command {
            ArgCommand::Export => {
                self.print_msg("• Exporting data...");
                self.export_data().wrap_err("Failed while exporting data")?;
            }
            ArgCommand::Render => {
                self.print_msg("• Building templates...");
                self.build_templates()?;

                self.print_msg("• Rendering template...");
                self.render_templates()
                    .wrap_err("Failed while rendering template(s)")?;
            }
            ArgCommand::Backup => {
                self.print_msg("• Backing up databases...");
                self.backup_databases()
                    .wrap_err("Failed while backing up databases")?;
            }
        }

        self.print_msg(&format!(
            "• Saved {} annotations from {} books to `{}`",
            self.data.count_annotations(),
            self.data.count_books(),
            self.config.options().output().display()
        ));

        Ok(())
    }

    /// Verifies, builds and registers all templates for rendering.
    fn build_templates(&mut self) -> AppResult<()> {
        self.template_manager
            .build(self.config.options().templates())
            .wrap_err("Failed while building template(s)")?;

        Ok(())
    }

    /// Builds the application's data from the Apple Books databases.
    fn build_data(&mut self) -> AppResult<()> {
        self.data
            .build(self.config.options().databases())
            .wrap_err("Failed while building data")
    }

    /// Exports Apple Books' data with the following structure:
    ///
    /// ```plaintext
    /// [output]
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
    fn export_data(&self) -> AppResult<()> {
        // -> [output]
        let path = self.config.options().output().join("data");

        for entry in self.data.entries() {
            // -> [output]/[author-title]
            let item = path.join(entry.name());
            // -> [output]/[author-title]/data
            let data = item.join("data");
            // -> [output]/[author-title]/resources
            let resources = item.join("resources");

            std::fs::create_dir_all(&item)?;
            std::fs::create_dir_all(&data)?;
            std::fs::create_dir_all(&resources)?;

            // -> [output]/[author-title]/data/book.json
            let book_json = data.join("book").with_extension("json");
            let book_file = fs::File::create(book_json)?;

            serde_json::to_writer_pretty(&book_file, &entry.book)?;

            // -> [output]/[author-title]/data/annotation.json
            let annotations_json = data.join("annotations").with_extension("json");
            let annotations_file = fs::File::create(annotations_json)?;

            serde_json::to_writer_pretty(&annotations_file, &entry.annotations)?;

            // -> [output]/[author-title]/resources/.gitkeep
            let gitkeep = resources.join(".gitkeep");
            fs::File::create(gitkeep)?;
        }

        Ok(())
    }

    /// Renders templates in one of two ways: `single` or `multi`.
    ///
    /// Single:
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  ├─ [template-name]
    ///  │   ├─ [author-title].[template-ext]
    ///  │   ├─ [author-title].[template-ext]
    ///  │   └─ ...
    ///  │
    ///  ├─ [template-name]
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    ///
    /// Multi:
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  ├─ [template-name]
    ///  │   │
    ///  │   ├─ [author-title]
    ///  │   │   ├─ [author-title].[template.ext]
    ///  │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[template.ext]
    ///  │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[template.ext]
    ///  │   │   └─ ...
    ///  │   │
    ///  │   ├─ [author-title]
    ///  │   │   └─ ...
    ///  │   └─ ...
    ///  │
    ///  ├─ [template-name]
    ///  │   │
    ///  │   ├─ [author-title]
    ///  │   │   ├─ [author-title].[template-ext]
    ///  │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[template.ext]
    ///  │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[template.ext]
    ///  │   │   └─ ...
    ///  │   │
    ///  │   ├─ [author-title]
    ///  │   │   └─ ...
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    fn render_templates(&mut self) -> AppResult<()> {
        // -> [output]
        let path = self.config.options().output();

        std::fs::create_dir_all(&path)?;

        // Renders each `Entry` i.e. a `Book` and its `Annotation`s.
        for entry in self.data.entries() {
            self.template_manager
                .render(entry, path)
                .wrap_err("Failed while rendering template(s)")?;
        }

        Ok(())
    }

    /// Backs up Apple Books' databases with the following structure:
    ///
    /// ```plaintext
    /// [output]
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
    fn backup_databases(&self) -> AppResult<()> {
        // -> [output]
        let path = self.config.options().output();

        // -> [YYYY-MM-DD-HHMMSS]-[VERSION]
        let today = format!("{}-{}", utils::today(), *APPLEBOOKS_VERSION);

        // -> [output]/[YYYY-MM-DD-HHMMSS]-[VERSION]
        let destination_root = path.join(&today);

        // -> [output]/[YYYY-MM-DD-HHMMSS]-[VERSION]/BKLibrary
        let destination_books = destination_root.join(ABDatabaseName::Books.to_string());

        // -> [output]/[YYYY-MM-DD-HHMMSS]-[VERSION]/AEAnnotation
        let destination_annotations =
            destination_root.join(ABDatabaseName::Annotations.to_string());

        // -> [DATABASES]/BKLibrary
        let source_books = &self
            .config
            .options()
            .databases()
            .join(ABDatabaseName::Books.to_string());

        // -> [DATABASES]/AEAnnotation
        let source_annotations = &self
            .config
            .options()
            .databases()
            .join(ABDatabaseName::Annotations.to_string());

        utils::copy_dir(source_books, &destination_books)?;
        utils::copy_dir(source_annotations, &destination_annotations)?;

        Ok(())
    }

    /// Prints a message to the terminal.
    fn print_msg(&self, message: &str) {
        if !self.config.options().is_quiet() {
            println!("{}", message);
        }
    }
}

#[cfg(test)]
mod test_app {

    use crate::cli::config::test::TestConfig;

    use super::*;

    /// Tests that an empty database returns zero books and zero annotations.
    #[test]
    fn test_databases_empty() {
        let config = TestConfig::new("empty");
        let mut app = App::new(Box::new(config));

        app.build_data().unwrap();

        assert_eq!(app.data.count_books(), 0);
        assert_eq!(app.data.count_annotations(), 0);
    }

    /// Tests that a database with un-annotated books returns zero books and
    /// zero annotations.
    #[test]
    fn test_databases_books_new() {
        let config = TestConfig::new("books-new");
        let mut app = App::new(Box::new(config));

        app.build_data().unwrap();

        // Un-annotated books are filtered out.
        assert_eq!(app.data.count_books(), 0);
        assert_eq!(app.data.count_annotations(), 0);
    }

    /// Tests that a database with annotated books returns non-zero books and
    /// non-zero annotations.
    #[test]
    fn test_databases_books_annotated() {
        let config = TestConfig::new("books-annotated");
        let mut app = App::new(Box::new(config));

        app.build_data().unwrap();

        assert_eq!(app.data.count_books(), 3);
        assert_eq!(app.data.count_annotations(), 10);
    }

    /// Tests that the annotations are sorted in the correct order.
    #[test]
    fn test_annotations_order() {
        let config = TestConfig::new("books-annotated");
        let mut app = App::new(Box::new(config));

        app.build_data().unwrap();

        for entry in app.data.entries() {
            for annotations in entry.annotations.windows(2) {
                assert!(annotations[0] < annotations[1]);
            }
        }
    }
}

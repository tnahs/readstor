use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::WrapErr;

use crate::cli::args::Command;
use crate::cli::config::Configuration;
use crate::lib::applebooks::database::ABDatabaseName;
use crate::lib::applebooks::utils::APPLEBOOKS_VERSION;
use crate::lib::models::stor::Stor;
use crate::lib::templates::{Template, Templates};
use crate::lib::utils;

use super::args::Args;

pub type AppResult<T> = color_eyre::Result<T>;

#[derive(Debug)]
pub struct App {
    config: Box<dyn Configuration>,
    stor: Stor,
    templates: Templates,
}

impl App {
    pub fn new(config: Box<dyn Configuration>) -> Self {
        Self {
            config,
            stor: Stor::default(),
            templates: Templates::default(),
        }
    }

    /// TODO Document
    fn init(&mut self) -> AppResult<()> {
        self.stor.build(self.config.databases()).map_err(From::from)
    }

    pub fn run(&mut self, args: &Args) -> AppResult<()> {
        println!("• Building stor...");

        self.init().wrap_err("failed while building stor")?;

        match args.command {
            Command::Export => {
                println!("• Exporting data...");
                self.export_data().wrap_err("failed while exporting data")?;
            }
            Command::Render { ref template } => {
                println!("• Rendering template...");
                self.render_templates(template.as_ref())
                    .wrap_err("failed while rendering template")?;
            }
            Command::Backup => {
                println!("• Backing up databases...");
                self.backup_databases()
                    .wrap_err("failed while backing up databases")?;
            }
        }

        let message = format!(
            "• Saved {} annotations from {} books to `{}`",
            self.stor.count_annotations(),
            self.stor.count_books(),
            self.config.output().display()
        );

        log::debug!("{}", message);

        Ok(())
    }

    /// Exports Apple Books' data with the following structure:
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  └─ data
    ///      │
    ///      ├─ Author - Title
    ///      │   │
    ///      │   ├─ data
    ///      │   │   ├─ book.json
    ///      │   │   └─ annotations.json
    ///      │   │
    ///      │   └─ resources
    ///      │       ├─ .gitkeep
    ///      │       ├─ Author - Title.epub   ─┐
    ///      │       ├─ cover.jpeg             ├─ These are not exported.
    ///      │       └─ ...                   ─┘
    ///      │
    ///      ├─ Author - Title
    ///      │   └─ ...
    ///      │
    ///      └─ ...
    /// ```
    ///
    /// Existing files are left unaffected unless explicitly written to. For
    /// example, the `resources` directory will not be deleted/recreated if it
    /// already exists and/or contains data.
    fn export_data(&self) -> AppResult<()> {
        // -> [output]/data/
        let root = self.config.output().join("data");

        for stor_item in self.stor.values() {
            // -> [output]/data/Author - Title
            let item = root.join(stor_item.name());
            // -> [output]/data/Author - Title/data
            let data = item.join("data");
            // -> [output]/data/Author - Title/resources
            let resources = item.join("resources");

            std::fs::create_dir_all(&item)?;
            std::fs::create_dir_all(&data)?;
            std::fs::create_dir_all(&resources)?;

            // -> [output]/data/Author - Title/data/book.json
            let book_json = data.join("book").with_extension("json");
            let book_file = fs::File::create(book_json)?;

            serde_json::to_writer_pretty(&book_file, &stor_item.book)?;

            // -> [output]/data/Author - Title/data/annotation.json
            let annotations_json = data.join("annotations").with_extension("json");
            let annotations_file = fs::File::create(annotations_json)?;

            serde_json::to_writer_pretty(&annotations_file, &stor_item.annotations)?;

            // -> [output]/data/Author - Title/resources/.gitkeep
            let gitkeep = resources.join(".gitkeep");
            fs::File::create(gitkeep)?;
        }

        Ok(())
    }

    /// Exports annotations with the following structure:
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  └─ exports
    ///      │
    ///      ├─ default ── [template-name]
    ///      │   │
    ///      │   ├─ Author - Title.[template-ext]
    ///      │   ├─ Author - Title.txt
    ///      │   ├─ Author - Title.txt
    ///      │   └─ ...
    ///      └─ ...
    /// ```
    ///
    /// See [`Templates::render()`] for more information.
    fn render_templates(&mut self, template: Option<&PathBuf>) -> AppResult<()> {
        // TODO Move template initialization into its own function when default
        // template directories are implemented. For now, this should be fine
        // as we're only dealing with a single template.
        if let Some(template) = template {
            self.templates
                .add(Template::from(template))
                .wrap_err("failed while parsing template")?;
        }

        // -> [output]/exports/
        let root = self.config.output().join("exports");

        std::fs::create_dir_all(&root)?;

        // Renders each `StorItem` aka a book.
        for stor_item in self.stor.values() {
            self.templates
                .render(stor_item, &root)
                .wrap_err("failed while rendering template")?;
        }

        Ok(())
    }

    /// Backs up Apple Books' databases with the following structure:
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  └─ backups
    ///      │
    ///      ├─ 2021-01-01-000000 v3.2-2217 ── [YYYY-MM-DD-HHMMSS VERSION]
    ///      │   │
    ///      │   ├─ AEAnnotation
    ///      │   │  ├─ AEAnnotation*.sqlite
    ///      │   │  └─ ...
    ///      │   │
    ///      │   └─ BKLibrary
    ///      │      ├─ BKLibrary*.sqlite
    ///      │      └─ ...
    ///      │
    ///      │─ 2021-01-02-000000 v3.2-2217
    ///      │   └─ ...
    ///      │
    ///      └─ ...
    /// ```
    fn backup_databases(&self) -> AppResult<()> {
        // -> [output]/backups/
        let root = self.config.output().join("backups");

        // -> [YYYY-MM-DD-HHMMSS] [VERSION]
        let today = format!(
            "{} {}",
            utils::today_format("%Y-%m-%d-%H%M%S"),
            APPLEBOOKS_VERSION.to_owned()
        );

        // -> [output]/backups/[YYYY-MM-DD-HHMMSS] [VERSION]
        let destination_root = root.join(&today);

        // -> [output]/backups/[YYYY-MM-DD-HHMMSS] [VERSION]/BKLibrary
        let destination_books = destination_root.join(ABDatabaseName::Books.to_string());

        // -> [output]/backups/[YYYY-MM-DD-HHMMSS] [VERSION]/AEAnnotation
        let destination_annotations =
            destination_root.join(ABDatabaseName::Annotations.to_string());

        // -> [DATABASES]/BKLibrary
        let source_books = &self
            .config
            .databases()
            .join(ABDatabaseName::Books.to_string());

        // -> [DATABASES]/AEAnnotation
        let source_annotations = &self
            .config
            .databases()
            .join(ABDatabaseName::Annotations.to_string());

        utils::copy_dir(source_books, &destination_books)?;
        utils::copy_dir(source_annotations, &destination_annotations)?;

        Ok(())
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

        app.init().unwrap();

        assert_eq!(app.stor.count_books(), 0);
        assert_eq!(app.stor.count_annotations(), 0);
    }

    /// Tests that a database with un-annotated books returns zero books and
    /// zero annotations.
    #[test]
    fn test_databases_books_new() {
        let config = TestConfig::new("books-new");
        let mut app = App::new(Box::new(config));

        app.init().unwrap();

        // Un-annotated books are filtered out.
        assert_eq!(app.stor.count_books(), 0);
        assert_eq!(app.stor.count_annotations(), 0);
    }

    /// Tests that a database with annotated books returns non-zero books and
    /// non-zero annotations.
    #[test]
    fn test_databases_books_annotated() {
        let config = TestConfig::new("books-annotated");
        let mut app = App::new(Box::new(config));

        app.init().unwrap();

        assert_eq!(app.stor.count_books(), 3);
        assert_eq!(app.stor.count_annotations(), 10);
    }

    /// Tests that the annotations are sorted in the correct order.
    #[test]
    fn test_annotations_order() {
        let config = TestConfig::new("books-annotated");
        let mut app = App::new(Box::new(config));

        app.init().unwrap();

        for stor_item in app.stor.values() {
            for annotations in stor_item.annotations.windows(2) {
                assert!(annotations[0] < annotations[1]);
            }
        }
    }
}

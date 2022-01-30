use std::fs;
use std::path::PathBuf;

use anyhow::Context;

use crate::cli::config::Config;
use crate::lib::applebooks::database::ABDatabaseName;
use crate::lib::applebooks::utils::APPLEBOOKS_VERSION;
use crate::lib::models::stor::Stor;
use crate::lib::result::Result;
use crate::lib::templates::{Template, Templates};
use crate::lib::utils;

pub type AnyhowResult<T> = anyhow::Result<T>;

#[derive(Default)]
pub struct App {
    stor: Stor,
    config: Config,
    templates: Templates,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            stor: Stor::default(),
            config,
            templates: Templates::default(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.stor.build(self.config.databases())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn stor(&self) -> &Stor {
        &self.stor
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
    ///      │   └─ assets
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
    /// example, the `assets` directory will not be deleted/recreated if it
    /// already exists and/or contains data.
    pub fn export_data(&self) -> Result<()> {
        // -> [output]/data/
        let root = self.config.output().join("data");

        for stor_item in self.stor.values() {
            // -> [output]/data/Author - Title
            let item = root.join(stor_item.name());
            // -> [output]/data/Author - Title/data
            let data = item.join("data");
            // -> [output]/data/Author - Title/assets
            let assets = item.join("assets");

            std::fs::create_dir_all(&item)?;
            std::fs::create_dir_all(&data)?;
            std::fs::create_dir_all(&assets)?;

            // -> [output]/data/Author - Title/data/book.json
            let book_json = data.join("book").with_extension("json");
            let book_file = fs::File::create(book_json)?;

            serde_json::to_writer_pretty(&book_file, &stor_item.book)?;

            // -> [output]/data/Author - Title/data/annotation.json
            let annotations_json = data.join("annotations").with_extension("json");
            let annotations_file = fs::File::create(annotations_json)?;

            serde_json::to_writer_pretty(&annotations_file, &stor_item.annotations)?;

            // -> [output]/data/Author - Title/assets/.gitkeep
            let gitkeep = assets.join(".gitkeep");
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
    pub fn render_templates(&mut self, template: Option<&PathBuf>) -> AnyhowResult<()> {
        // TODO Move template initialization into its own function when default
        // template directories are implemented. For now, this should be fine
        // as we're only dealing with a single template.
        if let Some(template) = template {
            self.templates
                .add(Template::from(template))
                .context("ReadStor failed while parsing template")?;
        }

        // -> [output]/exports/
        let root = self.config.output().join("exports");

        std::fs::create_dir_all(&root)?;

        // Renders each `StorItem` aka a book.
        for stor_item in self.stor.values() {
            self.templates.render(stor_item, &root)?;
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
    pub fn backup_databases(&self) -> Result<()> {
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
mod tests {

    use super::*;
    use crate::cli::defaults as cli_defaults;

    #[test]
    /// Tests that an empty database returns zero books and zero annotations.
    fn test_databases_empty() {
        let databases = cli_defaults::DEV_DATABASES.join("empty");

        let mut app = App::default();

        // Mimicking what happens in the [`App::init()`] method.
        app.stor.build(&databases).unwrap();

        assert_eq!(app.stor.count_books(), 0);
        assert_eq!(app.stor.count_annotations(), 0);
    }

    #[test]
    /// Tests that a database with un-annotated books returns zero books and
    /// zero annotations.
    fn test_databases_books_new() {
        let databases = cli_defaults::DEV_DATABASES.join("books-new");

        let mut app = App::default();

        // Mimicking what happens in the [`App::init()`] method.
        app.stor.build(&databases).unwrap();

        // Un-annotated books are filtered out.
        assert_eq!(app.stor.count_books(), 0);
        assert_eq!(app.stor.count_annotations(), 0);
    }

    #[test]
    /// Tests that a database with annotated books returns non-zero books and
    /// non-zero annotations.
    fn test_databases_books_annotated() {
        let databases = cli_defaults::DEV_DATABASES.join("books-annotated");

        let mut app = App::default();

        // Mimicking what happens in the [`App::init()`] method.
        app.stor.build(&databases).unwrap();

        assert_eq!(app.stor.count_books(), 3);
        assert_eq!(app.stor.count_annotations(), 10);
    }
}

use std::fs;

use anyhow::Context;

use crate::cli::config::AppConfig;
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
    config: AppConfig,
    templates: Templates,
}

impl App {
    pub fn new(config: AppConfig) -> AnyhowResult<Self> {
        let mut templates = Templates::default();

        if let Some(template) = &config.template {
            templates
                .add(Template::from(template))
                .context("ReadStor failed while parsing template")?;
        }

        Ok(Self {
            stor: Stor::default(),
            config,
            templates,
        })
    }

    pub fn run(&mut self) -> AnyhowResult<()> {
        println!("* Building stor...");

        self.stor
            .build(&self.config.databases)
            .context("ReadStor failed while building stor")?;

        println!("* Saving items...");

        self.save_items()
            .context("ReadStor failed while saving items")?;

        self.export_templates()
            .context("ReadStor failed while exporting to templates")?;

        if self.config.backup {
            println!("* Backing up databases...");

            self.backup_databases()
                .context("ReadStor failed while backing up databases")?;
        }

        println!(
            "* Saved {} annotations from {} books.",
            self.stor.count_annotations(),
            self.stor.count_books()
        );

        Ok(())
    }

    /// Saves Apple Books' data with the following structure:
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  └─ items
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
    pub fn save_items(&self) -> Result<()> {
        // -> [output]/items/
        let root = self.config.output.join("items");

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
    /// See [`Templates::render`] for more information.
    pub fn export_templates(&self) -> Result<()> {
        // -> [output]/exports/
        let root = self.config.output.join("exports");

        std::fs::create_dir_all(&root)?;

        // `StorItem` aka a book.
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
        let root = self.config.output.join("backups");

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

        // -> [output]/backups/[YYYY-MM-DD-HHMMSS] [VERSION]/AEAnnotations
        let destination_annotations =
            destination_root.join(ABDatabaseName::Annotations.to_string());

        // -> [DATABASES]/BKLibrary
        let source_books = &self
            .config
            .databases
            .join(ABDatabaseName::Books.to_string());

        // -> [DATABASES]/AEAnnotations
        let source_annotations = &self
            .config
            .databases
            .join(ABDatabaseName::Annotations.to_string());

        utils::copy_dir(source_books, &destination_books)?;
        utils::copy_dir(source_annotations, &destination_annotations)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::super::defaults::DATABASES_DEV;
    use super::*;

    #[test]
    fn test_databases_empty() {
        let databases = DATABASES_DEV.join("empty");

        let mut app = App::default();

        // Mimicking what happens in the [`App::run`] method.
        app.stor.build(&databases).unwrap();

        assert_eq!(app.stor.count_books(), 0);
        assert_eq!(app.stor.count_annotations(), 0);
    }

    #[test]
    fn test_databases_books_new() {
        let databases = DATABASES_DEV.join("books-new");

        let mut app = App::default();

        // Mimicking what happens in the [`App::run`] method.
        app.stor.build(&databases).unwrap();

        // Although there are books in the database, the books are not
        // annotated therefore they are filtered out of the `Stor`.
        assert_eq!(app.stor.count_books(), 0);
        assert_eq!(app.stor.count_annotations(), 0);
    }

    #[test]
    fn test_databases_books_annotated() {
        let databases = DATABASES_DEV.join("books-annotated");

        let mut app = App::default();

        // Mimicking what happens in the [`App::run`] method.
        app.stor.build(&databases).unwrap();

        assert_eq!(app.stor.count_books(), 3);
        assert_eq!(app.stor.count_annotations(), 10);
    }
}

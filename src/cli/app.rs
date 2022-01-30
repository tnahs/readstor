use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::WrapErr;

use crate::cli::config::Config;
use crate::lib::applebooks::database::ABDatabaseName;
use crate::lib::applebooks::utils::APPLEBOOKS_VERSION;
use crate::lib::models::stor::Stor;
use crate::lib::templates::{Template, Templates};
use crate::lib::utils;

pub type AppResult<T> = color_eyre::Result<T>;

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

    pub fn init(&mut self) -> AppResult<()> {
        self.stor
            .build(self.config.databases())
            .wrap_err("failed while building stor")
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
    pub fn export_data(&self) -> AppResult<()> {
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
    pub fn render_templates(&mut self, template: Option<&PathBuf>) -> AppResult<()> {
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
    pub fn backup_databases(&self) -> AppResult<()> {
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

    /// Creates an app from a database found in `tests/data/databases/`.
    fn app_from_test_db(name: &str) -> App {
        let databases = cli_defaults::DEV_DATABASES.join(name);

        let mut app = App::default();

        // Mimicking what happens in the [`App::init()`] method.
        app.stor.build(&databases).unwrap();

        app
    }

    /// Tests that an empty database returns zero books and zero annotations.
    #[test]
    fn test_databases_empty() {
        let app = app_from_test_db("empty");

        assert_eq!(app.stor().count_books(), 0);
        assert_eq!(app.stor().count_annotations(), 0);
    }

    /// Tests that a database with un-annotated books returns zero books and
    /// zero annotations.
    #[test]
    fn test_databases_books_new() {
        let app = app_from_test_db("books-new");

        // Un-annotated books are filtered out.
        assert_eq!(app.stor().count_books(), 0);
        assert_eq!(app.stor().count_annotations(), 0);
    }

    /// Tests that a database with annotated books returns non-zero books and
    /// non-zero annotations.
    #[test]
    fn test_databases_books_annotated() {
        let app = app_from_test_db("books-annotated");

        assert_eq!(app.stor().count_books(), 3);
        assert_eq!(app.stor().count_annotations(), 10);
    }

    /// Tests that the annotations are sorted in the correct order.
    #[test]
    fn test_annotations_order() {
        let app = app_from_test_db("books-annotated");

        for stor_item in app.stor().values() {
            for annotations in stor_item.annotations.windows(2) {
                assert!(annotations[0] < annotations[1]);
            }
        }
    }
}

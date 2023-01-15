//! Defines types for exporting data.

use std::fs::File;
use std::path::Path;

use serde::Serialize;

use crate::contexts::book::BookContext;
use crate::models::data::Entries;
use crate::models::entry::Entry;
use crate::result::Result;

/// The default export directory template.
///
/// Outputs `[author] - [book]` e.g. `Robert Henri - The Art Spirit`.
const DIRECTORY_TEMPLATE: &str = "{{ book.author }} - {{ book.title }}";

/// A struct for running export tasks.
#[derive(Debug, Copy, Clone)]
pub struct ExportRunner;

impl ExportRunner {
    /// Runs the export task.
    ///
    /// # Arguments
    ///
    /// * `entries` - The [`Entry`][entry]s to export.
    /// * `path` - The ouput directory.
    /// * `options` - The export options.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * Any IO errors are encountered.
    /// * [`serde_json`][serde-json] encounters any errors.
    ///
    /// [entry]: crate::models::entry::Entry
    /// [serde-json]: https://docs.rs/serde_json/latest/serde_json/
    pub fn run<O>(entries: &mut Entries, path: &Path, options: O) -> Result<()>
    where
        O: Into<ExportOptions>,
    {
        let options: ExportOptions = options.into();

        Self::export(entries, path, options)?;

        Ok(())
    }

    /// Exports Apple Books' data as JSON.
    ///
    /// # Arguments
    ///
    /// * `entries` - The [`Entry`][entry]s to export.
    /// * `path` - The ouput directory.
    /// * `options` - The export options.
    ///
    /// The output strucutre is as follows:
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
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * Any IO errors are encountered.
    /// * [`serde_json`][serde-json] encounters any errors.
    ///
    /// [entry]: crate::models::entry::Entry
    /// [serde-json]: https://docs.rs/serde_json/latest/serde_json/
    fn export(entries: &mut Entries, path: &Path, options: ExportOptions) -> Result<()> {
        let directory_template = options
            .directory_template
            .unwrap_or_else(|| DIRECTORY_TEMPLATE.to_string());

        for entry in entries.values() {
            // -> [author-title]
            let directory_name = Self::render_directory_name(&directory_template, entry)?;
            // -> [ouput-directory]/[author-title]
            let item = path.join(directory_name);
            // -> [ouput-directory]/[author-title]/data
            let data = item.join("data");
            // -> [ouput-directory]/[author-title]/resources
            let resources = item.join("resources");

            std::fs::create_dir_all(&item)?;
            std::fs::create_dir_all(&data)?;
            std::fs::create_dir_all(&resources)?;

            // -> [ouput-directory]/[author-title]/data/book.json
            let book_json = data.join("book").with_extension("json");

            // -> [ouput-directory]/[author-title]/data/annotation.json
            let annotations_json = data.join("annotations").with_extension("json");

            // -> [ouput-directory]/[author-title]/resources/.gitkeep
            let gitkeep = resources.join(".gitkeep");

            if !options.overwrite_existing && book_json.exists() {
                log::debug!("skipped writing {}", book_json.display());
            } else {
                let book_json = File::create(book_json)?;
                serde_json::to_writer_pretty(&book_json, &entry.book)?;
            }

            if !options.overwrite_existing && annotations_json.exists() {
                log::debug!("skipped writing {}", annotations_json.display());
            } else {
                let annotations_json = File::create(annotations_json)?;
                serde_json::to_writer_pretty(&annotations_json, &entry.annotations)?;
            }

            if !options.overwrite_existing && gitkeep.exists() {
                log::debug!("skipped writing {}", gitkeep.display());
            } else {
                File::create(gitkeep)?;
            }
        }

        Ok(())
    }

    fn render_directory_name(template: &str, entry: &Entry) -> Result<String> {
        let context = BookContext::from(&entry.book);
        let context = ExportContext::Directory { book: &context };
        crate::utils::render_and_sanitize(template, context)
    }
}

/// A struct representing options for the [`ExportRunner`] struct.
#[derive(Debug)]
pub struct ExportOptions {
    /// The template to use for rendering the export's ouput directories.
    pub directory_template: Option<String>,

    /// Toggles whether or not to overwrite existing files.
    pub overwrite_existing: bool,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ExportContext<'a> {
    Directory { book: &'a BookContext<'a> },
}

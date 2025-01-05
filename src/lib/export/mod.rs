//! Defines types for exporting data.

use std::fs::File;
use std::path::Path;

use serde::Serialize;

use crate::contexts::book::BookContext;
use crate::models::entry::{Entries, Entry};
use crate::result::Result;
use crate::strings;

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
    /// * `entries` - The entries to export.
    /// * `path` - The ouput directory.
    /// * `options` - The export options.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * Any IO errors are encountered.
    /// * [`serde_json`][serde-json] encounters any errors.
    ///
    /// [serde-json]: https://docs.rs/serde_json/latest/serde_json/
    pub fn run<O>(entries: &mut Entries, path: &Path, options: O) -> Result<()>
    where
        O: Into<ExportOptions>,
    {
        let options: ExportOptions = options.into();

        Self::export(entries, path, options)?;

        Ok(())
    }

    /// Exports data as JSON.
    ///
    /// # Arguments
    ///
    /// * `entries` - The entries to export.
    /// * `path` - The ouput directory.
    /// * `options` - The export options.
    ///
    /// The output strucutre is as follows:
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├── [author-title]
    ///  │    ├── book.json
    ///  │    └── annotations.json
    ///  │
    ///  ├── [author-title]
    ///  │    └── ...
    ///  └── ...
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * Any IO errors are encountered.
    /// * [`serde_json`][serde-json] encounters any errors.
    ///
    /// [serde-json]: https://docs.rs/serde_json/latest/serde_json/
    fn export(entries: &Entries, path: &Path, options: ExportOptions) -> Result<()> {
        let directory_template = if let Some(template) = options.directory_template {
            Self::validate_template(&template)?;
            template
        } else {
            DIRECTORY_TEMPLATE.to_string()
        };

        for entry in entries.values() {
            // -> [author-title]
            let directory_name = Self::render_directory_name(&directory_template, entry)?;

            // -> [ouput-directory]/[author-title]
            let item = path.join(directory_name);
            // -> [ouput-directory]/[author-title]/book.json
            let book_json = item.join("book").with_extension("json");
            // -> [ouput-directory]/[author-title]/annotation.json
            let annotations_json = item.join("annotations").with_extension("json");

            std::fs::create_dir_all(&item)?;

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
        }

        Ok(())
    }

    /// Validates a template by rendering it.
    ///
    /// The template is rendered and an empty [`Result`] is returned.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string to validate.
    fn validate_template(template: &str) -> Result<()> {
        let entry = Entry::dummy();
        Self::render_directory_name(template, &entry).map(|_| ())
    }

    /// Renders the directory name from a template string and an [`Entry`].
    ///
    /// # Arguments
    ///
    /// * `template` - The template string to render.
    /// * `entry` - The [`Entry`] providing the template context.
    fn render_directory_name(template: &str, entry: &Entry) -> Result<String> {
        let context = BookContext::from(&entry.book);
        let context = ExportContext::from(&context);
        strings::render_and_sanitize(template, context)
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

/// An struct represening the template context for exports.
///
/// This is primarily used for generating directory names.
#[derive(Debug, Serialize)]
struct ExportContext<'a> {
    book: &'a BookContext<'a>,
}

impl<'a> From<&'a BookContext<'a>> for ExportContext<'a> {
    fn from(book: &'a BookContext<'a>) -> Self {
        Self { book }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::defaults::test::TemplatesDirectory;
    use crate::models::book::Book;
    use crate::render::engine::RenderEngine;
    use crate::utils;

    // Tests that the default template returns no error.
    #[test]
    fn default_template() {
        let book = Book::default();
        let context = BookContext::from(&book);
        let context = ExportContext { book: &context };

        RenderEngine::default()
            .render_str(DIRECTORY_TEMPLATE, context)
            .unwrap();
    }

    // Tests that all valid context fields return no errors.
    #[test]
    fn valid_context() {
        let template =
            utils::testing::load_template_str(TemplatesDirectory::ValidContext, "valid-export.txt");

        let book = Book::default();
        let context = BookContext::from(&book);
        let context = ExportContext::from(&context);

        RenderEngine::default()
            .render_str(&template, context)
            .unwrap();
    }

    // Tests that an invalid context field returns an error.
    #[test]
    #[should_panic(expected = "Failed to render '__tera_one_off'")]
    fn invalid_context() {
        let template = utils::testing::load_template_str(
            TemplatesDirectory::InvalidContext,
            "invalid-export.txt",
        );

        let book = Book::default();
        let context = BookContext::from(&book);
        let context = ExportContext::from(&context);

        RenderEngine::default()
            .render_str(&template, context)
            .unwrap();
    }
}

//! Defines types for exporting.

use std::{fs, path::Path};

use crate::models::data::Entries;
use crate::result::Result;

/// Exports Apple Books' data as JSON.
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
/// Existing files are left unaffected unless explicitly written to. For
/// example, the `resources` directory will not be deleted/recreated if it
/// already exists and/or contains data.
///
/// # Arguments
///
/// * `entries` - The [`Entry`][entry]s to export.
/// * `path` - The ouput directory.
///
/// # Errors
///
/// Will return `Err` if:
/// * Any IO errors are encountered.
/// * [`serde_json`][serde-json] encounters any errors.
///
/// [entry]: crate::models::entry::Entry
/// [serde-json]: https://docs.rs/serde_json/latest/serde_json/
pub fn export_data(entries: &mut Entries, path: &Path) -> Result<()> {
    for entry in entries.values() {
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
        let book_json = fs::File::create(book_json)?;

        serde_json::to_writer_pretty(&book_json, &entry.book)?;

        // -> [ouput-directory]/[author-title]/data/annotation.json
        let annotations_json = data.join("annotations").with_extension("json");
        let annotations_json = fs::File::create(annotations_json)?;

        serde_json::to_writer_pretty(&annotations_json, &entry.annotations)?;

        // -> [ouput-directory]/[author-title]/resources/.gitkeep
        let gitkeep = resources.join(".gitkeep");
        fs::File::create(gitkeep)?;
    }

    Ok(())
}

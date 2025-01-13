//! Defines utilities for working with templates.

use walkdir::DirEntry;

/// Helper function for [`walkdir`][walkdir]. Filter "hidden" entries e.g. `.hidden`.
///
/// [walkdir]: https://docs.rs/walkdir/latest/walkdir/
#[must_use]
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| !s.starts_with('.'))
}

/// Helper function for [`walkdir`][walkdir]. Filter normal templates.
///
/// [walkdir]: https://docs.rs/walkdir/latest/walkdir/
#[must_use]
pub fn is_normal_template(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| !s.starts_with('_'))
}

/// Helper function for [`walkdir`][walkdir]. Filter partial templates.
///
/// [walkdir]: https://docs.rs/walkdir/latest/walkdir/
#[must_use]
pub fn is_partial_template(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('_'))
}

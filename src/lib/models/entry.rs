//! Defines the [`Entry`] struct.

use std::collections::HashMap;

use serde::Serialize;

use super::annotation::Annotation;
use super::book::Book;

/// A type alias represening how [`Entry`]s are organized.
///
/// [`Entries`] is a `HashMap` composed of `key:value` pairs of where the value
/// is an [`Entry`] and the key is the unique id of its [`Book`], taken from
/// the [`BookMetadata::id`][book-metadata-id] field.
///
/// For example:
///
/// ```plaintext
/// Entries
///  │
///  ├── ID: Entry
///  ├── ID: Entry
///  └── ...
/// ```
///
/// [book-metadata-id]: crate::models::book::BookMetadata::id
pub type Entries = HashMap<String, Entry>;

/// A container struct that stores a [`Book`] and its respective [`Annotation`]s.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Entry {
    /// The entry's [`Book`].
    pub book: Book,

    /// The entry's [`Annotation`]s.
    pub annotations: Vec<Annotation>,
}

impl From<Book> for Entry {
    /// Constructs an instance of [`Entry`] via a [`Book`] object. This is the
    /// primary way [`Entry`]s are created.
    fn from(book: Book) -> Self {
        Self {
            book,
            annotations: Vec::new(),
        }
    }
}

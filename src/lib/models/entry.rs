//! Defines the [`Entry`] struct. A container type that stores a [`Book`] and
//! its respective [`Annotation`]s.

use serde::Serialize;

use super::annotation::Annotation;
use super::book::Book;

/// A container type that stores a [`Book`] and its respective [`Annotation`]s.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Entry {
    /// The entry's [`Book`].
    pub book: Book,

    /// The entry's [`Annotation`]s.
    pub annotations: Vec<Annotation>,
}

impl Entry {
    /// Formats an [`Entry`]' into a slugified string.
    #[must_use]
    pub fn slug_name(&self) -> String {
        format!("{}-{}", self.book.slug_title(), self.book.slug_author())
    }
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

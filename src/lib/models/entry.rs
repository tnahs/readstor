use serde::Serialize;

use crate::lib::utils;

use super::annotation::Annotation;
use super::book::Book;

/// A container representing a [`Book`] and its respective [`Annotation`]s.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Entry {
    pub book: Book,
    pub annotations: Vec<Annotation>,
}

impl Entry {
    /// Formats a [`Entry`] into a friendly-human-readable string. Primarily
    /// used for naming files or directories for its respective [`Book`].
    #[must_use]
    pub fn name(&self) -> String {
        utils::to_safe_string(
            &format!("{} - {}", self.book.author, self.book.title),
            &['!', '@', '#', '$', '%', '&', '(', ')', '-', ',', '.', '?'],
        )
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

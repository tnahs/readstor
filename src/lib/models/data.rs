use std::collections::HashMap;
use std::path::Path;

use serde::Serialize;

use crate::lib::applebooks::database::{ABDatabase, ABDatabaseName};
use crate::lib::result::LibResult;
#[allow(unused_imports)] // For docs.
use crate::lib::templates::Templates;

use super::annotation::Annotation;
use super::book::Book;

/// Defines the `Entries` type alias.
///
/// A `Entries` is a `HashMap` composed of `key:value` pairs of `ID:Entry`.
/// For example:
///
/// ```plaintext
/// Entries
///  │
///  ├─ ID: Entry
///  ├─ ID: Entry
///  └─ ...
/// ```
///
/// The `ID` for each `Entry` is taken from its `Book`'s `BookMetadata::id`.
/// field. See [`From<Book> for Entry`](struct@Entry#impl-From<Book>)
/// for more information.
type Entries = HashMap<String, Entry>;

/// Defines a thin wrapper around the [`Entries`] type alias.
///
/// Allows default `HashMap` interaction via `Deref` and `DerefMut`, but also
/// provides a few specific convenience methods.
///
/// The basic structure is as follows:
/// ```plaintext
/// Data
///  │
///  └─ Entries
///      │
///      ├─ ID: Entry
///      │       ├─ Book
///      │       └─ Annotation
///      ├─ ID: Entry
///      │       ├─ Book
///      │       └─ Annotation
///      └─ ...
/// ```
#[derive(Debug, Default)]
pub struct Data(Entries);

impl Data {
    /// TODO Document
    #[must_use]
    pub fn entries(&self) -> impl IntoIterator<Item = &Entry> {
        self.0.values()
    }

    /// Builds [`Entries`] from the Apple Books database.
    ///
    /// # Errors
    ///
    /// See [`ABDatabase::query()`] for information on errors as these are the
    /// only sources of possible errors.
    pub fn build(&mut self, path: &Path) -> LibResult<()> {
        let books = ABDatabase::query::<Book>(path)?;
        let annotations = ABDatabase::query::<Annotation>(path)?;

        log::debug!(
            "Found {} book(s) in {}.",
            books.len(),
            ABDatabaseName::Books.to_string()
        );

        log::debug!(
            "Found {} annotation(s) in {}.",
            annotations.len(),
            ABDatabaseName::Annotations.to_string()
        );

        // `Entry`s are created from `Book`s. Note that `book.metadata.id`
        // is set as the key for each entry into the `Data`. This is later used
        // to compare with each `Annotation` to determine if the `Annotation`
        // belongs to `Book` therefore its `Entry`.
        //
        // See https://stackoverflow.com/q/69274529/16968574
        let mut data: Entries = books
            .into_iter()
            .map(|book| (book.metadata.id.clone(), Entry::from(book)))
            .collect();

        // `Annotation`s are pushed onto a `Entry` based on its `book_id`.
        for annotation in annotations {
            if let Some(entry) = data.get_mut(&annotation.metadata.book_id) {
                entry.annotations.push(annotation);
            }
        }

        // Remove `Entry`s that have no `Annotation`s.
        data.retain(|_, entry| !entry.annotations.is_empty());

        // Sort `Annotation`s by their `location`s.
        for entry in data.values_mut() {
            entry.annotations.sort();
        }

        self.0 = data;

        log::debug!("Created {} book(s).", self.count_books());
        log::debug!("Created {} annotation(s).", self.count_annotations());

        Ok(())
    }

    /// Returns the number of books.
    #[must_use]
    pub fn count_books(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of annotations.
    #[must_use]
    pub fn count_annotations(&self) -> usize {
        self.0
            .iter()
            .map(|(_, entry)| entry.annotations.len())
            .sum()
    }
}

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
        format!("{} - {}", self.book.author, self.book.title)
    }
}

impl From<Book> for Entry {
    /// Constructs an instance of [`Entry`] via a [`Book`] object. This is
    /// the primary way [`Entry`]s are created.
    fn from(book: Book) -> Self {
        Self {
            book,
            annotations: Vec::new(),
        }
    }
}

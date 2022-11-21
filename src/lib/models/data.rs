//! Defines the [`Data`] struct. A type that compiles and stores [`Entry`]s.

use std::collections::HashMap;
use std::path::Path;

use crate::applebooks::database::{ABDatabase, ABDatabaseName};
use crate::result::Result;

use super::annotation::Annotation;
use super::book::Book;
use super::entry::Entry;

/// Defines the `Entries` type alias.
///
/// A `Entries` is a `HashMap` composed of `key:value` pairs of `ID:Entry`.
///
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
/// field. See [`From<Book> for Entry`](struct@Entry#impl-From<Book>) for more
/// information.
type Entries = HashMap<String, Entry>;

/// Defines a thin wrapper around the [`Entries`] type alias.
///
/// Allows default `HashMap` interaction via `Deref` and `DerefMut`, but also
/// provides a few specific convenience methods.
///
/// The basic structure is as follows:
///
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
    /// Builds [`Entry`]s from the Apple Books databases.
    ///
    /// # Errors
    ///
    /// See [`ABDatabase::query()`] for information on errors as these are the
    /// only sources of possible errors.
    pub fn init(&mut self, path: &Path) -> Result<()> {
        let books = ABDatabase::query::<Book>(path)?;
        let annotations = ABDatabase::query::<Annotation>(path)?;

        log::debug!(
            "found {} book(s) in {}.",
            books.len(),
            ABDatabaseName::Books.to_string()
        );

        log::debug!(
            "found {} annotation(s) in {}.",
            annotations.len(),
            ABDatabaseName::Annotations.to_string()
        );

        // `Entry`s are created from `Book`s. Note that `book.metadata.id` is
        // set as the key for each entry into the `Data`. This is later used to
        // compare with each `Annotation` to determine if the `Annotation`
        // belongs to a `Book` and therefore its `Entry`.
        //
        // See https://stackoverflow.com/q/69274529/16968574
        let mut data: Entries = books
            .into_iter()
            .map(|book| (book.metadata.id.clone(), Entry::from(book)))
            .collect();

        // `Annotation`s are pushed onto an `Entry` based on their `book_id`.
        for annotation in annotations {
            if let Some(entry) = data.get_mut(&annotation.metadata.book_id) {
                entry.annotations.push(annotation);
            }
        }

        // Remove `Entry`s that have no `Annotation`s.
        data.retain(|_, entry| !entry.annotations.is_empty());

        self.0 = data;

        log::debug!("created {} Book's", self.count_books());
        log::debug!("created {} Annotation's", self.count_annotations());

        Ok(())
    }

    /// Returns an iterator over all [`Entry`]s.
    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.0.values()
    }

    /// Returns a mutable iterator over all [`Entry`]s.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = &mut Entry> {
        self.0.values_mut()
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

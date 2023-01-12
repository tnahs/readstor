//! Defines the [`Data`] struct.

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use crate::applebooks::database::{ABDatabase, ABDatabaseName};
use crate::filter::filters;
use crate::result::Result;

use super::annotation::Annotation;
use super::book::Book;
use super::entry::Entry;

/// A type alias represening the inner type of [`Data`].
///
/// [`Entries`] is a `HashMap` composed of `key:value` pairs of where the value
/// is an [`Entry`] and the key is the unique id of its [`Book`], taken from
/// the [`BookMetadata::id`][id] field.
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
/// [id]: crate::models::book::BookMetadata::id
pub type Entries = HashMap<String, Entry>;

/// A container struct for storing and managing [`Entry`]s.
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
            "found {} book(s) in {}",
            books.len(),
            ABDatabaseName::Books.to_string()
        );

        log::debug!(
            "found {} annotation(s) in {}",
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
        filters::contains_no_annotations(&mut data);

        self.0 = data;

        log::debug!("created {} Book's", self.books().count());
        log::debug!("created {} Annotation's", self.annotations().count());

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

    /// Returns an iterator over all [`Book`]s.
    pub fn books(&self) -> impl Iterator<Item = &Book> {
        self.0.values().map(|entry| &entry.book)
    }

    /// Returns a mutable iterator over all [`Book`]s.
    pub fn books_mut(&mut self) -> impl Iterator<Item = &Book> {
        self.0.values().map(|entry| &entry.book)
    }

    /// Returns an iterator over all [`Annotation`]s.
    pub fn annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.0.values().flat_map(|entry| &entry.annotations)
    }

    /// Returns a mutable iterator over all [`Annotation`]s.
    pub fn annotations_mut(&mut self) -> impl Iterator<Item = &Annotation> {
        self.0.values().flat_map(|entry| &entry.annotations)
    }
}

impl Deref for Data {
    type Target = Entries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Entries {
        &mut self.0
    }
}

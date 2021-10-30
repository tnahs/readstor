use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use serde::Serialize;

use super::annotation::Annotation;
use super::book::Book;
use crate::lib::applebooks::database::ABDatabase;
use crate::lib::result::Result;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::defaults::APPLEBOOKS_DATABASES;
#[allow(unused_imports)] // For docs.
use crate::lib::templates::Templates;

/// Defines the `StorData` type alias.
///
/// A `StorData` is a `HashMap` composed of `key:value` pairs of `ID:StorItem`.
/// For example:
///
/// ```plaintext
/// StorData
///  │
///  ├─ ID: StorItem
///  ├─ ID: StorItem
///  └─ ...
/// ```
///
/// The `ID` for each `StorItem` is taken from its `Book`'s `BookMetadata::id`.
/// field. See [`From<Book> for StorItem`](struct@StorItem#impl-From<Book>)
/// for more information.
type StorData = HashMap<String, StorItem>;

/// Defines a thin wrapper around the [`StorData`] type alias.
///
/// Allows default `HashMap` interaction via `Deref` and `DerefMut`, but also
/// provides a few specific convenience methods.
///
/// The basic structure is as follows:
/// ```plaintext
/// Stor
///  │
///  └─ StorData
///      │
///      ├─ ID: StorItem
///      │       ├─ Book
///      │       └─ Annotation
///      ├─ ID: StorItem
///      │       ├─ Book
///      │       └─ Annotation
///      └─ ...
/// ```
#[derive(Default)]
pub struct Stor {
    data: StorData,
}

impl Stor {
    /// Builds the [`StorData`] from the Apple Books database.
    ///
    /// Locating the database path is delegated to the [`ABDatabase`] and the
    /// [`static@APPLEBOOKS_DATABASES`] static variable.
    pub fn build(&mut self) -> Result<()> {
        let books = ABDatabase::query::<Book>()?;
        let annotations = ABDatabase::query::<Annotation>()?;

        log::debug!("Found {} book(s) in database.", books.len());
        log::debug!("Found {} annotation(s) in database.", annotations.len());

        // See https://stackoverflow.com/q/69274529/16968574

        // `StorItem`s are created from `Book`s. Note that `book.metadata.id`
        // is set as the key for each entry into the `Stor`. This is later used
        // to compare with each `Annotation` to determine if the `Annotation`
        // belongs to `Book` therefore its `StorItem`.
        let mut data: StorData = books
            .into_iter()
            .map(|book| (book.metadata.id.clone(), StorItem::from(book)))
            .collect();

        // `Annotation`s are pushed onto a `StorItem` based on its `book_id`.
        annotations.into_iter().for_each(|annotation| {
            if let Some(stor_item) = data.get_mut(&annotation.metadata.book_id) {
                stor_item.annotations.push(annotation);
            }
        });

        // Remove `StorItem`s that have no `Annotation`s.
        data.retain(|_, stor_item| !stor_item.annotations.is_empty());

        // Sort `Annotation`s by their `location`s.
        data.values_mut().for_each(|stor_item| {
            stor_item
                .annotations
                // <https://stackoverflow.com/a/56106352/16968574>
                .sort_by(|annotation_a, annotation_b| {
                    annotation_a
                        .metadata
                        .location
                        .cmp(&annotation_b.metadata.location)
                });
        });

        self.data = data;

        Ok(())
    }

    /// Returns the number of books.
    pub fn count_books(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of annotations.
    pub fn count_annotations(&self) -> usize {
        self.data
            .iter()
            .map(|(_, stor_item)| stor_item.annotations.len())
            .sum()
    }
}

// https://stackoverflow.com/a/31497621
impl Deref for Stor {
    type Target = StorData;

    fn deref(&self) -> &StorData {
        &self.data
    }
}

impl DerefMut for Stor {
    fn deref_mut(&mut self) -> &mut StorData {
        &mut self.data
    }
}

/// A container representing a [`Book`] and its respective [`Annotation`]s.
#[derive(Debug, Default, Clone, Serialize)]
pub struct StorItem {
    pub book: Book,
    pub annotations: Vec<Annotation>,
}

impl StorItem {
    /// Formats a [`StorItem`] into a friendly-human-readable string. Primarily
    /// used for naming files or directories for its respective [`Book`].
    pub fn name(&self) -> String {
        format!("{} - {}", self.book.author, self.book.title)
    }
}

impl From<Book> for StorItem {
    /// Constructs an instance of [`StorItem`] via a [`Book`] object. This is
    /// the primary way [`StorItem`]s are created.
    fn from(book: Book) -> Self {
        Self {
            book,
            annotations: Vec::new(),
        }
    }
}

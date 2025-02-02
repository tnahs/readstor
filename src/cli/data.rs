use std::ops::{Deref, DerefMut};
use std::path::Path;

use lib::applebooks::ios::{ABIOs, ABPlist};
use lib::applebooks::macos::{ABDatabase, ABMacOs};
use lib::filter::filters;
use lib::models::annotation::Annotation;
use lib::models::book::Book;
use lib::models::entry::{Entries, Entry};

use super::CliResult;

/// A container struct for storing and managing [`Entry`]s.
#[derive(Debug, Default)]
pub struct Data(Entries);

impl Data {
    /// Builds [`Book`]s and [`Annotation`]s from macOS's Apple Books databases, converts them to
    /// [`Entry`]s and appends them to the data model.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing macOS's Apple Books databases.
    ///
    /// See [`ABMacOs`] for more information on how the databases directory should be structured.
    ///
    /// # Errors
    ///
    /// See [`ABMacOs::extract_books()`] and [`ABMacOs::extract_annotations()`] for information as
    /// these are the only sources of possible errors.
    pub fn init_macos(&mut self, path: &Path) -> CliResult<()> {
        let books = ABMacOs::extract_books(path)?;
        let annotations = ABMacOs::extract_annotations(path)?;

        log::debug!(
            "found {} book(s) in {}",
            books.len(),
            ABDatabase::Books.to_string()
        );

        log::debug!(
            "found {} annotation(s) in {}",
            annotations.len(),
            ABDatabase::Annotations.to_string()
        );

        let entries = Self::build_entries(books, annotations);

        self.0.extend(entries);

        Ok(())
    }

    /// Builds [`Book`]s and [`Annotation`]s from iOS's Apple Books plists, converts them to
    /// [`Entry`]s and appends them to the data model.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing iOS's Apple Books plists.
    ///
    /// See [`ABIOs`] for more information on how the plists directory should be structured.
    ///
    /// # Errors
    ///
    /// See [`ABIOs::extract_books()`] and [`ABIOs::extract_annotations()`] for information as these
    /// are the only sources of possible errors.
    pub fn init_ios(&mut self, path: &Path) -> CliResult<()> {
        let books = ABIOs::extract_books(path)?;
        let annotations = ABIOs::extract_annotations(path)?;

        log::debug!(
            "found {} book(s) in {}",
            books.len(),
            ABPlist::Books.to_string()
        );

        log::debug!(
            "found {} annotation(s) in {}",
            annotations.len(),
            ABPlist::Annotations.to_string()
        );

        let entries = Self::build_entries(books, annotations);

        self.0.extend(entries);

        Ok(())
    }

    /// Converts [`Book`]s and [`Annotation`]s to [`Entry`]s, then sorts and filters them before
    /// adding them to the data model.
    fn build_entries(books: Vec<Book>, annotations: Vec<Annotation>) -> Entries {
        // `Entry`s are created from `Book`s. Note that `book.metadata.id` is set as the key for
        // each entry into the `Data`. This is later used to compare with each `Annotation` to
        // determine if the `Annotation` belongs to a `Book` and therefore its `Entry`.
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

        let count_books = Self::iter_books_inner(&data).count();
        let count_annotations = Self::iter_annotations_inner(&data).count();

        log::debug!("created {count_books} Book(s)",);
        log::debug!("created {count_annotations} Annotation(s)",);

        data
    }

    /// Returns the number of books within [`Data`].
    pub fn count_books(&self) -> usize {
        self.iter_books().count()
    }

    /// Returns the number of annotations within [`Data`].
    pub fn count_annotations(&self) -> usize {
        self.iter_annotations().count()
    }

    /// Returns an iterator over all [`Book`]s.
    pub fn iter_books(&self) -> impl Iterator<Item = &Book> {
        Self::iter_books_inner(&self.0)
    }

    /// Returns an iterator over all [`Annotation`]s.
    pub fn iter_annotations(&self) -> impl Iterator<Item = &Annotation> {
        Self::iter_annotations_inner(&self.0)
    }

    /// Returns an iterator over all [`Annotation`]s given an [`Entries`] type.
    fn iter_annotations_inner(entries: &Entries) -> impl Iterator<Item = &Annotation> {
        entries.values().flat_map(|entry| &entry.annotations)
    }

    /// Returns an iterator over all [`Book`]s given an [`Entries`] type.
    fn iter_books_inner(entries: &Entries) -> impl Iterator<Item = &Book> {
        entries.values().map(|entry| &entry.book)
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

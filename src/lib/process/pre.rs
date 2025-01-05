//! Defines the pre-processor type.
//!
//! Pre-processors are used to mutate fields within an [`Entry`].

use crate::models::entry::{Entries, Entry};
use crate::strings;

/// A struct for pre-processing [`Entry`]s.
#[derive(Debug, Clone, Copy)]
pub struct PreProcessor;

impl PreProcessor {
    /// Runs all pre-strings on an [`Entry`].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`]s to process.
    /// * `options` - The pre-process options.
    pub fn run<O>(entries: &mut Entries, options: O)
    where
        O: Into<PreProcessOptions>,
    {
        let options: PreProcessOptions = options.into();

        for entry in entries.values_mut() {
            Self::sort_annotations(entry);

            if options.extract_tags {
                Self::extract_tags(entry);
            }

            if options.normalize_whitespace {
                Self::normalize_whitespace(entry);
            }

            if options.convert_all_to_ascii {
                Self::convert_all_to_ascii(entry);
            }

            if options.convert_symbols_to_ascii {
                Self::convert_symbols_to_ascii(entry);
            }
        }
    }

    /// Sort annotations by [`AnnotationMetadata::location`][location].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [location]: crate::models::annotation::AnnotationMetadata::location
    pub fn sort_annotations(entry: &mut Entry) {
        entry.annotations.sort();
    }

    /// Extracts `#tags` from [`Annotation::notes`][annotation-notes] and places
    /// them into [`Annotation::tags`][annotation-tags]. The `#tags` are removed from
    /// [`Annotation::notes`][annotation-notes].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [annotation-notes]: crate::models::annotation::Annotation::notes
    /// [annotation-tags]: crate::models::annotation::Annotation::tags
    fn extract_tags(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.tags = strings::extract_tags(&annotation.notes);
            annotation.notes = strings::remove_tags(&annotation.notes);
        }
    }

    /// Normalizes whitespace in [`Annotation::body`][body].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [body]: crate::models::annotation::Annotation::body
    fn normalize_whitespace(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.body = strings::normalize_whitespace(&annotation.body);
        }
    }

    /// Converts all Unicode characters found in [`Annotation::body`][body], [`Book::title`][title]
    /// and [`Book::author`][author] to their ASCII equivalents.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [author]: crate::models::book::Book::author
    /// [body]: crate::models::annotation::Annotation::body
    /// [title]: crate::models::book::Book::title
    fn convert_all_to_ascii(entry: &mut Entry) {
        entry.book.title = strings::convert_all_to_ascii(&entry.book.title);
        entry.book.author = strings::convert_all_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = strings::convert_all_to_ascii(&annotation.body);
        }
    }

    /// Converts a subset of "smart" Unicode symbols found in [`Annotation::body`][body],
    /// [`Book::title`][title] and [`Book::author`][author] to their ASCII equivalents.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [author]: crate::models::book::Book::author
    /// [body]: crate::models::annotation::Annotation::body
    /// [title]: crate::models::book::Book::title
    fn convert_symbols_to_ascii(entry: &mut Entry) {
        entry.book.title = strings::convert_symbols_to_ascii(&entry.book.title);
        entry.book.author = strings::convert_symbols_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = strings::convert_symbols_to_ascii(&annotation.body);
        }
    }
}

/// A struct representing options for the [`PreProcessor`] struct.
#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct PreProcessOptions {
    /// Toggles running `#tag` extraction from notes.
    pub extract_tags: bool,

    /// Toggles running whitespace normalization.
    pub normalize_whitespace: bool,

    /// Toggles converting all Unicode characters to ASCII.
    pub convert_all_to_ascii: bool,

    /// Toggles converting "smart" Unicode symbols to ASCII.
    pub convert_symbols_to_ascii: bool,
}

#[cfg(test)]
mod test {

    use super::*;

    mod tags {

        use super::*;

        use crate::models::annotation::Annotation;
        use crate::models::book::Book;

        // Tests that tags are properly extracted from `Annotation::notes`, placed into the
        // `Annotation::tags` field.
        #[test]
        fn extract() {
            let mut entry = Entry {
                book: Book::default(),
                annotations: vec![
                    Annotation {
                        notes: "#tag01 #tag02".to_string(),
                        ..Default::default()
                    },
                    Annotation {
                        notes: "#tag02 #tag03".to_string(),
                        ..Default::default()
                    },
                    Annotation {
                        notes: "#tag03 #tag01".to_string(),
                        ..Default::default()
                    },
                ],
            };

            PreProcessor::extract_tags(&mut entry);

            for annotation in entry.annotations {
                assert_eq!(annotation.tags.len(), 2);
                assert!(annotation.notes.is_empty());
            }
        }
    }
}

//! Defines the [`Processor`] struct used for pre- and post-processing of
//! template data.

pub mod process;

use crate::lib::models::entry::Entry;

/// A struct containing pre- and post-processing methods for [`Entry`]s and
/// templates.
#[derive(Debug, Clone, Copy)]
pub struct Processor {}

impl Processor {
    /// Runs all pre-processors on an `Entry`.
    ///
    /// # Arguments
    ///
    /// * `options` - An instance of [`PreprocessOptions`].
    /// * `entry` - The [`Entry`] to process.
    pub fn preprocess(options: PreprocessOptions, entry: &mut Entry) {
        if options.extract_tags {
            Self::extract_tags(entry);
        }

        if options.normalize_linebreaks {
            Self::normalize_linebreaks(entry);
        }

        if options.convert_all_to_ascii {
            Self::convert_all_to_ascii(entry);
        }

        if options.convert_symbols_to_ascii {
            Self::convert_symbols_to_ascii(entry);
        }
    }

    /// Temporary placeholder for post-processing methods.
    #[must_use]
    pub fn postprocess(string: &str) -> String {
        process::trim_blocks(string)
    }

    /// Normalizes line breaks in `Annotation.body`.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    fn normalize_linebreaks(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.body = process::normalize_linebreaks(&annotation.body);
        }
    }

    /// Extracts `#tags` from `Annotation.notes` and places them into
    /// `Annotation.tags`. The `#tags` are removed from `Annotation.notes`.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    fn extract_tags(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.tags = process::extract_tags(&annotation.notes);
            annotation.notes = process::remove_tags(&annotation.notes);
        }
    }

    /// Converts all Unicode characters found in `Annotation.body`, `Book.title`
    /// and `Book.author` to their ASCII equivalents.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    fn convert_all_to_ascii(entry: &mut Entry) {
        entry.book.title = process::convert_all_to_ascii(&entry.book.title);
        entry.book.author = process::convert_all_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = process::convert_all_to_ascii(&annotation.body);
        }
    }

    /// Converts a subset of "smart: Unicode symbols found in `Annotation.body`,
    /// `Book.title` and `Book.author` to their ASCII equivalents.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    fn convert_symbols_to_ascii(entry: &mut Entry) {
        entry.book.title = process::convert_symbols_to_ascii(&entry.book.title);
        entry.book.author = process::convert_symbols_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = process::convert_symbols_to_ascii(&annotation.body);
        }
    }
}

/// A struct represting pre-process options for the [`Processor`] struct.
#[derive(Debug, Default, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct PreprocessOptions {
    /// Enable running `#tag` extraction from notes.
    pub extract_tags: bool,

    /// Enable running linebreak normalization.
    pub normalize_linebreaks: bool,

    /// Enable converting all Unicode characters ASCII.
    pub convert_all_to_ascii: bool,

    /// Enable converting "smart" Unicode symbols to ASCII.
    // TODO: Add link to documentation here.
    pub convert_symbols_to_ascii: bool,
}

use crate::lib::models::entry::Entry;
use crate::lib::models::utils;

struct Preprocessor {}

impl Preprocessor {
    /// Normalizes line breaks in `Annotation.body`.
    pub fn normalize_linebreaks(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.body = utils::normalize_linebreaks(&annotation.body);
        }
    }

    /// Extracts `#tags` from `Annotation.notes` and places them into
    /// `Annotation.tags`. The `#tags` are removed from `Annotation.notes`.
    pub fn extract_tags(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.tags = utils::extract_tags(&annotation.notes);
            annotation.notes = utils::remove_tags(&annotation.notes);
        }
    }

    /// Converts all Unicode characters found in `Annotation.body` to their
    /// ASCII equivalents.
    pub fn convert_all_to_ascii(entry: &mut Entry) {
        entry.book.title = utils::convert_all_to_ascii(&entry.book.title);
        entry.book.author = utils::convert_all_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = utils::convert_all_to_ascii(&annotation.body);
        }
    }

    /// Converts a subset of "smart" Unicode symbols found in `Annotation.body`
    /// to their ASCII equivalents.
    pub fn convert_symbols_to_ascii(entry: &mut Entry) {
        entry.book.title = utils::convert_symbols_to_ascii(&entry.book.title);
        entry.book.author = utils::convert_symbols_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = utils::convert_symbols_to_ascii(&annotation.body);
        }
    }
}

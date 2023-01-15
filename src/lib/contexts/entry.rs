//! Defines the context for [`Entry`] data.

use serde::Serialize;

use crate::models::entry::Entry;

use super::annotation::AnnotationContext;
use super::book::BookContext;

/// A struct representing an [`Entry`] within a template context.
///
/// See [`Entry`] for undocumented fields.
#[derive(Debug, Serialize)]
pub struct EntryContext<'a> {
    #[allow(missing_docs)]
    pub book: BookContext<'a>,
    #[allow(missing_docs)]
    pub annotations: Vec<AnnotationContext<'a>>,
}

impl<'a> From<&'a Entry> for EntryContext<'a> {
    fn from(entry: &'a Entry) -> Self {
        Self {
            book: BookContext::from(&entry.book),
            annotations: entry
                .annotations
                .iter()
                .map(std::convert::Into::into)
                .collect(),
        }
    }
}

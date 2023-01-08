//! Defines the [`BookContext`] struct.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::models::book::{Book, BookMetadata};

/// A struct representing a [`Book`] within a template context.
///
/// See [`Book`] for undocumented fields.
#[derive(Debug, Serialize)]
#[allow(missing_docs)]
pub struct BookContext<'a> {
    pub title: &'a str,
    pub author: &'a String,
    pub tags: &'a BTreeSet<String>,
    pub metadata: &'a BookMetadata,

    /// An [`Book`]s slugified strings.
    pub slugs: BookSlugs,
}

impl<'a> From<&'a Book> for BookContext<'a> {
    fn from(book: &'a Book) -> Self {
        Self {
            title: &book.title,
            author: &book.author,
            tags: &book.tags,
            metadata: &book.metadata,
            slugs: BookSlugs {
                title: crate::utils::to_slug_string(&book.title, '-'),
                author: crate::utils::to_slug_string(&book.author, '-'),
                metadata: BookMetadataSlugs {
                    last_opened: crate::utils::to_slug_date(&book.metadata.last_opened),
                },
            },
        }
    }
}

/// A struct representing an [`Annotation`]'s slugified strings.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize)]
pub struct BookSlugs {
    pub title: String,
    pub author: String,
    pub metadata: BookMetadataSlugs,
}

/// A struct representing an [`BookMetadata`]'s slugified strings.
///
/// See [`BookMetadata`] for undocumented fields.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize)]
pub struct BookMetadataSlugs {
    pub last_opened: String,
}

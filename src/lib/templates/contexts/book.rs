//! Defines the context for [`Book`] data.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::models::book::{Book, BookMetadata};

/// A struct representing a [`Book`] within a template context.
///
/// See [`Book`] for undocumented fields.
#[derive(Debug, Serialize)]
pub struct BookContext<'a> {
    #[allow(missing_docs)]
    pub title: &'a str,
    #[allow(missing_docs)]
    pub author: &'a String,
    #[allow(missing_docs)]
    pub tags: &'a BTreeSet<String>,
    #[allow(missing_docs)]
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

/// A struct representing a [`Book`]'s slugified strings.
#[derive(Debug, Default, Clone, Serialize)]
pub struct BookSlugs {
    #[allow(missing_docs)]
    pub title: String,
    #[allow(missing_docs)]
    pub author: String,
    #[allow(missing_docs)]
    pub metadata: BookMetadataSlugs,
}

/// A struct representing an [`BookMetadata`]'s slugified strings.
///
/// See [`BookMetadata`] for undocumented fields.
#[derive(Debug, Default, Clone, Serialize)]
pub struct BookMetadataSlugs {
    #[allow(missing_docs)]
    pub last_opened: String,
}

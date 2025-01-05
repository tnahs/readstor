//! Defines the context for [`Book`] data.

use serde::Serialize;

use crate::models::book::{Book, BookMetadata};
use crate::strings;

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
    pub metadata: &'a BookMetadata,

    /// A [`Book`]s slugified strings.
    pub slugs: BookSlugs,
}

impl<'a> From<&'a Book> for BookContext<'a> {
    fn from(book: &'a Book) -> Self {
        let last_opened = if let Some(date) = &book.metadata.last_opened {
            strings::to_slug_date(date)
        } else {
            String::new()
        };

        Self {
            title: &book.title,
            author: &book.author,
            metadata: &book.metadata,
            slugs: BookSlugs {
                title: strings::to_slug(&book.title, true),
                author: strings::to_slug(&book.author, true),
                metadata: BookMetadataSlugs { last_opened },
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

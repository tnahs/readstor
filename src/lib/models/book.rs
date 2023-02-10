//! Defines the [`Book`] struct.

use std::collections::BTreeSet;

use rusqlite::Row;
use serde::Serialize;

use crate::applebooks::ios::models::BookRaw;
use crate::applebooks::macos::ABQuery;

use super::datetime::DateTimeUtc;

/// A struct represening a book and its metadata.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Book {
    /// The title of the book.
    pub title: String,

    /// The author of the book.
    pub author: String,

    /// The book's `#tags` compiled from its [`Annotation`][annotation]s.
    ///
    /// [annotation]: crate::models::annotation::Annotation
    pub tags: BTreeSet<String>,

    /// The book's metadata.
    pub metadata: BookMetadata,
}

impl ABQuery for Book {
    const QUERY: &'static str = {
        "SELECT
            ZBKLIBRARYASSET.ZTITLE,        -- 0 title
            ZBKLIBRARYASSET.ZAUTHOR,       -- 1 author
            ZBKLIBRARYASSET.ZASSETID,      -- 2 id
            ZBKLIBRARYASSET.ZLASTOPENDATE  -- 3 last_opened
        FROM ZBKLIBRARYASSET
        ORDER BY ZBKLIBRARYASSET.ZTITLE;"
    };

    fn from_row(row: &Row<'_>) -> Self {
        let last_opened: f64 = row.get_unwrap(3);

        Self {
            title: row.get_unwrap(0),
            author: row.get_unwrap(1),
            tags: BTreeSet::new(),
            metadata: BookMetadata {
                id: row.get_unwrap(2),
                last_opened: DateTimeUtc::from(last_opened),
            },
        }
    }
}

impl From<BookRaw> for Book {
    fn from(book: BookRaw) -> Self {
        Self {
            title: book.title,
            author: book.author,
            tags: BTreeSet::new(),
            metadata: BookMetadata {
                id: book.id,
                // FIXME: How can we find the `last_opened` date?
                last_opened: DateTimeUtc::default(),
            },
        }
    }
}

/// A struct representing a book's metadata.
#[derive(Debug, Default, Clone, Serialize)]
pub struct BookMetadata {
    /// The book's unique id.
    pub id: String,

    /// The date the book was last opened.
    pub last_opened: DateTimeUtc,
}

//! Defines the [`Book`] struct and its trait implementations.

use rusqlite::Row;
use serde::Serialize;

use crate::lib::applebooks::database::{ABDatabaseName, ABQuery};
use crate::lib::utils;

use super::datetime::DateTimeUtc;

/// A struct represening a book and its metadata.
#[derive(Debug, Default, Clone)]
pub struct Book {
    /// The title of the book.
    pub title: String,

    /// The author of the book.
    pub author: String,

    /// The book's metadata.
    pub metadata: BookMetadata,
}

impl Book {
    ///Returns a slugified string of the title.
    #[must_use]
    pub fn slug_title(&self) -> String {
        utils::to_slug_string(&self.title, '-')
    }

    ///Returns a slugified string of the author.
    #[must_use]
    pub fn slug_author(&self) -> String {
        utils::to_slug_string(&self.author, '-')
    }
}

impl serde::Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct _Slugs {
            title: String,
            author: String,
        }

        #[derive(Serialize)]
        struct _Book<'a> {
            title: &'a str,
            author: &'a str,
            metadata: &'a BookMetadata,
            slugs: _Slugs,
        }

        let book = _Book {
            title: &self.title,
            author: &self.author,
            metadata: &self.metadata,
            slugs: _Slugs {
                title: self.slug_title(),
                author: self.slug_author(),
            },
        };

        book.serialize(serializer)
    }
}

impl ABQuery for Book {
    const DATABASE_NAME: ABDatabaseName = ABDatabaseName::Books;

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
        // It's necessary to explicitly type all these variables as `rusqlite`
        // needs the type information to convert the column value to `T`. If the
        // types do not match `rusqlite` will return an `InvalidColumnType` when
        // calling `get_unwrap`. Therefore it should be safe to call
        // `get_unwrap` as we know both the types match and we can see the
        // column indices in the `query` method below.

        let last_opened: f64 = row.get_unwrap(3);

        Self {
            title: row.get_unwrap(0),
            author: row.get_unwrap(1),
            metadata: BookMetadata {
                id: row.get_unwrap(2),
                last_opened: last_opened.into(),
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

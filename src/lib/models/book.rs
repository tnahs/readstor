use rusqlite::Row;
use serde::Serialize;

use crate::lib::applebooks::database::{ABDatabaseName, ABQueryable};
use crate::lib::utils::DateTimeUTC;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub metadata: BookMetadata,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct BookMetadata {
    pub id: String,
    pub last_opened: DateTimeUTC,
}

impl ABQueryable for Book {
    const DATABASE_NAME: ABDatabaseName = ABDatabaseName::BKLibrary;

    const QUERY: &'static str = {
        "SELECT
            ZBKLIBRARYASSET.ZTITLE,        -- 0 title
            ZBKLIBRARYASSET.ZAUTHOR,       -- 1 author
            ZBKLIBRARYASSET.ZASSETID,      -- 2 id
            ZBKLIBRARYASSET.ZLASTOPENDATE  -- 3 last_opened

        FROM ZBKLIBRARYASSET

        ORDER BY ZBKLIBRARYASSET.ZTITLE;"
    };

    fn from_row(row: &Row) -> Self {
        // It's necessary to explicitly type all these variables as `rusqlite`
        // needs the type information to convert the column value to `T`. If
        // the types do not match `rusqlite` will return an `InvalidColumnType`
        // when calling `get_unwrap`. Therefore it should be safe to call
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

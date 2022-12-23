//! Defines the [`Annotation`] struct and its trait implementations.

use std::cmp::Ordering;

use rusqlite::Row;
use serde::Serialize;

use crate::applebooks::database::{ABDatabaseName, ABQuery};

use super::datetime::DateTimeUtc;
use super::epubcfi;

/// A struct representing an annotation and its metadata.
#[derive(Debug, Default, Clone, Eq, Serialize)]
pub struct Annotation {
    /// The body of the annotation.
    pub body: String,

    /// The annotation's highlight style.
    ///
    /// Possible values are: `green`, `blue`, `yellow`, `pink` `purple` or `underline`.
    pub style: String,

    /// The annotation's notes.
    pub notes: String,

    /// The annotation's `#tags`.
    pub tags: Vec<String>,

    /// The annotation's metadata.
    pub metadata: AnnotationMetadata,
}

impl Annotation {
    /// Returns a style/color string from Apple Books' integer representation.
    fn int_to_style(int: u8) -> String {
        let style = match int {
            0 => "underline",
            1 => "green",
            2 => "blue",
            3 => "yellow",
            4 => "pink",
            5 => "purple",
            _ => "",
        };

        style.to_owned()
    }
}

impl ABQuery for Annotation {
    const DATABASE_NAME: ABDatabaseName = ABDatabaseName::Annotations;

    const QUERY: &'static str = {
        "SELECT
            ZANNOTATIONSELECTEDTEXT,           -- 0 body
            ZANNOTATIONNOTE,                   -- 1 notes
            ZANNOTATIONSTYLE,                  -- 2 style
            ZANNOTATIONUUID,                   -- 3 id
            ZAEANNOTATION.ZANNOTATIONASSETID,  -- 4 book_id
            ZANNOTATIONCREATIONDATE,           -- 5 created
            ZANNOTATIONMODIFICATIONDATE,       -- 6 modified
            ZANNOTATIONLOCATION                -- 7 location
        FROM ZAEANNOTATION
        WHERE ZANNOTATIONSELECTEDTEXT IS NOT NULL
            AND ZANNOTATIONDELETED = 0
        ORDER BY ZANNOTATIONASSETID;"
    };

    fn from_row(row: &Row<'_>) -> Self {
        let notes: Option<String> = row.get_unwrap(1);
        let style: u8 = row.get_unwrap(2);
        let created: f64 = row.get_unwrap(5);
        let modified: f64 = row.get_unwrap(6);
        let epubcfi: String = row.get_unwrap(7);

        Self {
            body: row.get_unwrap(0),
            style: Self::int_to_style(style),
            notes: notes.unwrap_or_default(),
            tags: Vec::new(),
            metadata: AnnotationMetadata {
                id: row.get_unwrap(3),
                book_id: row.get_unwrap(4),
                created: created.into(),
                modified: modified.into(),
                location: epubcfi::parse(&epubcfi),
                epubcfi,
            },
        }
    }
}

impl Ord for Annotation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.metadata.cmp(&other.metadata)
    }
}

impl PartialOrd for Annotation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.metadata.partial_cmp(&other.metadata)
    }
}

impl PartialEq for Annotation {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
    }
}

/// A struct representing an annotation's metadata.
///
/// This is all the data that is not directly editable by the user.
#[derive(Debug, Default, Clone, Eq)]
pub struct AnnotationMetadata {
    /// The annotation's unique id.
    pub id: String,

    /// The book id this annotation belongs to.
    pub book_id: String,

    /// The date the annotation was created.
    pub created: DateTimeUtc,

    /// The date the annotation was last modified.
    pub modified: DateTimeUtc,

    /// A location string used for sorting annotations into their order of
    /// appearance inside their respective book. This string is generated from
    /// the annotation's `epubcfi`.
    pub location: String,

    /// The annotation's raw `epubcfi`.
    pub epubcfi: String,
}

impl AnnotationMetadata {
    ///Returns a slugified string of the creation date.
    #[must_use]
    pub fn slug_created(&self) -> String {
        self.created
            .format(crate::defaults::DATE_FORMAT)
            .to_string()
    }

    ///Returns a slugified string of the modification date.
    #[must_use]
    pub fn slug_modified(&self) -> String {
        self.created
            .format(crate::defaults::DATE_FORMAT)
            .to_string()
    }
}

impl serde::Serialize for AnnotationMetadata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct _Slugs {
            created: String,
            modified: String,
        }

        #[derive(Serialize)]
        struct _AnnotationMetadata<'a> {
            id: &'a str,
            book_id: &'a str,
            created: &'a DateTimeUtc,
            modified: &'a DateTimeUtc,
            location: &'a str,
            epubcfi: &'a str,
            slugs: _Slugs,
        }

        let metadata = _AnnotationMetadata {
            id: &self.id,
            book_id: &self.book_id,
            created: &self.created,
            modified: &self.modified,
            location: &self.location,
            epubcfi: &self.epubcfi,
            slugs: _Slugs {
                created: self.slug_created(),
                modified: self.slug_modified(),
            },
        };

        metadata.serialize(serializer)
    }
}

impl Ord for AnnotationMetadata {
    fn cmp(&self, other: &Self) -> Ordering {
        self.location.cmp(&other.location)
    }
}

impl PartialOrd for AnnotationMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.location.partial_cmp(&other.location)
    }
}

impl PartialEq for AnnotationMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

#[cfg(test)]
mod test_annotations {

    use super::*;

    // TODO: Base function to start testing annotation order using `<` and `>`.
    #[test]
    fn test_cmp_annotations() {
        let mut a1 = Annotation::default();
        a1.metadata.location = epubcfi::parse("epubcfi(/6/10[c01]!/4/10/3,:335,:749)");

        let mut a2 = Annotation::default();
        a2.metadata.location = epubcfi::parse("epubcfi(/6/12[c02]!/4/26/3,:68,:493)");

        assert!(a1 < a2);
    }
}

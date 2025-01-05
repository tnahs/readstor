//! Defines the [`Annotation`] struct.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use rusqlite::Row;
use serde::Serialize;

use crate::applebooks::ios::models::AnnotationRaw;
use crate::applebooks::macos::ABQuery;

use super::datetime::DateTimeUtc;
use super::epubcfi;

/// A struct representing an annotation and its metadata.
#[derive(Debug, Default, Clone, Eq, Serialize)]
pub struct Annotation {
    /// The body of the annotation.
    pub body: String,

    /// The annotation's highlight style.
    pub style: AnnotationStyle,

    /// The annotation's notes.
    pub notes: String,

    /// The annotation's `#tags`.
    pub tags: BTreeSet<String>,

    /// The annotation's metadata.
    pub metadata: AnnotationMetadata,
}

// For macOS.
impl ABQuery for Annotation {
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
            style: AnnotationStyle::from(style as usize),
            notes: notes.unwrap_or_default(),
            tags: BTreeSet::new(),
            metadata: AnnotationMetadata {
                id: row.get_unwrap(3),
                book_id: row.get_unwrap(4),
                created: DateTimeUtc::from(created),
                modified: DateTimeUtc::from(modified),
                location: epubcfi::parse(&epubcfi),
                epubcfi,
            },
        }
    }
}

// For iOS.
impl From<AnnotationRaw> for Annotation {
    fn from(annotation: AnnotationRaw) -> Self {
        Self {
            body: annotation.body,
            style: AnnotationStyle::from(annotation.style),
            notes: annotation.notes.unwrap_or_default(),
            tags: BTreeSet::new(),
            metadata: AnnotationMetadata {
                id: annotation.id,
                book_id: annotation.book_id,
                created: DateTimeUtc::from(annotation.created),
                modified: DateTimeUtc::from(annotation.modified),
                location: epubcfi::parse(&annotation.epubcfi),
                epubcfi: annotation.epubcfi,
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
        Some(self.metadata.cmp(&other.metadata))
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
#[derive(Debug, Default, Clone, Eq, Serialize)]
pub struct AnnotationMetadata {
    /// The annotation's unique id.
    pub id: String,

    /// The book id this annotation belongs to.
    pub book_id: String,

    /// The date the annotation was created.
    pub created: DateTimeUtc,

    /// The date the annotation was last modified.
    pub modified: DateTimeUtc,

    /// A location string used for sorting annotations into their order of appearance inside their
    /// respective book. This string is generated from the annotation's `epubcfi`.
    pub location: String,

    /// The annotation's raw `epubcfi`.
    pub epubcfi: String,
}

impl Ord for AnnotationMetadata {
    fn cmp(&self, other: &Self) -> Ordering {
        self.location.cmp(&other.location)
    }
}

impl PartialOrd for AnnotationMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.location.cmp(&other.location))
    }
}

impl PartialEq for AnnotationMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

/// An enum represening all possible annotation highlight styles.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationStyle {
    #[default]
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    Underline,
    #[allow(missing_docs)]
    Green,
    #[allow(missing_docs)]
    Blue,
    #[allow(missing_docs)]
    Yellow,
    #[allow(missing_docs)]
    Red,
    #[allow(missing_docs)]
    Purple,
}

impl From<usize> for AnnotationStyle {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Underline,
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Yellow,
            4 => Self::Red,
            5 => Self::Purple,
            _ => Self::None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    // Tests that annotation ordering is properly evaluated from an `epubcfi` string.
    // TODO: Base function to start testing annotation order using `<` and `>`.
    #[test]
    fn cmp_annotations() {
        let mut a1 = Annotation::default();
        a1.metadata.location = epubcfi::parse("epubcfi(/6/10[c01]!/4/10/3,:335,:749)");

        let mut a2 = Annotation::default();
        a2.metadata.location = epubcfi::parse("epubcfi(/6/12[c02]!/4/26/3,:68,:493)");

        assert!(a1 < a2);
    }
}

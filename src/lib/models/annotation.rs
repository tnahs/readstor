use std::cmp::Ordering;

use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::Row;
use serde::Serialize;

use crate::lib::applebooks::database::{ABDatabaseName, ABQuery};
use crate::lib::parser;
use crate::lib::utils::DateTimeUTC;

/// Captures a `#tag`.
static RE_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[^\s#]+").unwrap());

#[derive(Debug, Default, Clone, Eq, Serialize)]
pub struct Annotation {
    pub body: Vec<String>,
    pub style: String,
    pub notes: String,
    pub tags: Vec<String>,
    pub metadata: AnnotationMetadata,
}

impl Annotation {
    /// Returns `Vec<String>` representing the a split and trimmed paragraph.
    fn process_body(body: &str) -> Vec<String> {
        body.lines()
            // Remove empty paragraphs.
            .filter(|&s| !s.is_empty())
            // Trim whitespace.
            .map(str::trim)
            .map(ToOwned::to_owned)
            .collect()
    }

    /// Returns a `String` with all `#tag`s removed.
    fn process_notes(notes: &Option<String>) -> String {
        let notes = match notes {
            Some(notes) => notes.clone(),
            None => return "".to_owned(),
        };

        RE_TAG
            // Remove all occurrences of `#tag`.
            .replace_all(&notes, "")
            // Trim whitespace.
            .trim()
            .to_owned()
    }

    /// Returns a `Vec<String>` of `#tag`s extracted from the text.
    fn process_tags(notes: &Option<String>) -> Vec<String> {
        let notes = match notes {
            Some(notes) => notes,
            None => return Vec::new(),
        };

        RE_TAG
            // Find all occurrences of `#tag`.
            .find_iter(notes)
            .map(|m| m.as_str())
            .map(ToOwned::to_owned)
            .collect()
    }

    /// Returns a style/color string from Apple Books' id representation.
    fn style_from_id(int: u8) -> String {
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
        // It's necessary to explicitly type all these variables as `rusqlite`
        // needs the type information to convert the column value to `T`. If
        // the types do not match `rusqlite` will return an `InvalidColumnType`
        // when calling `get_unwrap`. Therefore it should be safe to call
        // `get_unwrap` as we know both the types match and we can see the
        // column indices in the `query` method below.

        let body: String = row.get_unwrap(0);
        let r_notes: Option<String> = row.get_unwrap(1);
        let style: u8 = row.get_unwrap(2);
        let created: f64 = row.get_unwrap(5);
        let modified: f64 = row.get_unwrap(6);
        let epubcfi: String = row.get_unwrap(7);

        let body = Self::process_body(&body);
        let style = Self::style_from_id(style);
        let notes = Self::process_notes(&r_notes);
        let tags = Self::process_tags(&r_notes);
        let location = parser::parse_epubcfi(&epubcfi);

        Self {
            body,
            style,
            notes,
            tags,
            metadata: AnnotationMetadata {
                id: row.get_unwrap(3),
                book_id: row.get_unwrap(4),
                created: created.into(),
                modified: modified.into(),
                location,
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

/// Represents the data that is not directly editable by the user.
#[derive(Debug, Default, Clone, Eq, Serialize)]
pub struct AnnotationMetadata {
    pub id: String,
    pub book_id: String,
    pub created: DateTimeUTC,
    pub modified: DateTimeUTC,
    pub location: String,
    pub epubcfi: String,
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

    /// TODO Base function to start testing annotation order using `<` and `>`.
    #[test]
    fn test_cmp_annotations() {
        let mut a1 = Annotation::default();
        a1.metadata.location =
            parser::parse_epubcfi("epubcfi(/6/10[c01]!/4/10/3,:335,:749)");

        let mut a2 = Annotation::default();
        a2.metadata.location =
            parser::parse_epubcfi("epubcfi(/6/12[c02]!/4/26/3,:68,:493)");

        assert!(a1 < a2);
    }
}

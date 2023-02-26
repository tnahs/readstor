//! Defines dummy implementations for template validation.

use std::collections::BTreeSet;

use uuid::Uuid;

use crate::models::annotation::{Annotation, AnnotationMetadata, AnnotationStyle};
use crate::models::book::{Book, BookMetadata};
use crate::models::datetime::DateTimeUtc;
use crate::models::entry::Entry;

impl Entry {
    #[must_use]
    pub(super) fn dummy() -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            book: Book::dummy(id),
            annotations: vec![
                Annotation::dummy(id),
                Annotation::dummy(id),
                Annotation::dummy(id),
            ],
        }
    }
}

impl Book {
    #[must_use]
    pub(super) fn dummy(id: Uuid) -> Self {
        Self {
            title: "Excepteur Sit Commodo".to_string(),
            author: "Laborum Cillum".to_string(),
            tags: BTreeSet::from_iter(["#laboris", "#magna", "#nisi"].map(String::from)),
            metadata: BookMetadata {
                id: id.to_string(),
                last_opened: DateTimeUtc::default(),
            },
        }
    }
}

impl Annotation {
    #[must_use]
    pub(super) fn dummy(book_id: Uuid) -> Self {
        Self {
            body: "Elit consequat pariatur incididunt excepteur mollit.".to_string(),
            style: AnnotationStyle::Underline,
            notes: "Dolor ipsum officia non cillum.".to_string(),
            tags: BTreeSet::from_iter(["#laboris", "#magna", "#nisi"].map(String::from)),
            metadata: AnnotationMetadata {
                id: Uuid::new_v4().to_string(),
                book_id: book_id.to_string(),
                created: DateTimeUtc::default(),
                modified: DateTimeUtc::default(),
                location: String::new(),
                epubcfi: String::new(),
            },
        }
    }
}

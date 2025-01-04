//! Defines the context for [`Annotation`] data.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::models::annotation::{Annotation, AnnotationMetadata, AnnotationStyle};
use crate::strings;

/// A struct representing an [`Annotation`] within a template context.
///
/// See [`Annotation`] for undocumented fields.
#[derive(Debug, Serialize)]
pub struct AnnotationContext<'a> {
    #[allow(missing_docs)]
    pub body: &'a str,
    #[allow(missing_docs)]
    pub style: &'a AnnotationStyle,
    #[allow(missing_docs)]
    pub notes: &'a str,
    #[allow(missing_docs)]
    pub tags: &'a BTreeSet<String>,
    #[allow(missing_docs)]
    pub metadata: &'a AnnotationMetadata,

    /// An [`Annotation`]s slugified strings.
    pub slugs: AnnotationSlugs,
}

impl<'a> From<&'a Annotation> for AnnotationContext<'a> {
    fn from(annotation: &'a Annotation) -> Self {
        Self {
            body: &annotation.body,
            style: &annotation.style,
            notes: &annotation.notes,
            tags: &annotation.tags,
            metadata: &annotation.metadata,
            slugs: AnnotationSlugs {
                metadata: AnnotationMetadataSlugs {
                    created: strings::to_slug_date(&annotation.metadata.created),
                    modified: strings::to_slug_date(&annotation.metadata.modified),
                },
            },
        }
    }
}

/// A struct representing an [`Annotation`]'s slugified strings.
#[derive(Debug, Serialize)]
pub struct AnnotationSlugs {
    #[allow(missing_docs)]
    metadata: AnnotationMetadataSlugs,
}

/// A struct representing an [`AnnotationMetadata`]'s slugified strings.
///
/// See [`AnnotationMetadata`] for undocumented fields.
#[derive(Debug, Serialize)]
pub struct AnnotationMetadataSlugs {
    #[allow(missing_docs)]
    created: String,
    #[allow(missing_docs)]
    modified: String,
}

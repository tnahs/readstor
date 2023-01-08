//! Defines the [`AnnotationContext`] struct.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::models::annotation::{Annotation, AnnotationMetadata};

/// A struct representing an [`Annotation`] within a template context.
///
/// See [`Annotation`] for undocumented fields.
#[derive(Debug, Serialize)]
#[allow(missing_docs)]
pub struct AnnotationContext<'a> {
    pub body: &'a str,
    pub style: &'a str,
    pub notes: &'a str,
    pub tags: &'a BTreeSet<String>,
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
                    created: crate::utils::to_slug_date(&annotation.metadata.created),
                    modified: crate::utils::to_slug_date(&annotation.metadata.modified),
                },
            },
        }
    }
}

/// A struct representing an [`Annotation`]'s slugified strings.
#[allow(missing_docs)]
#[derive(Debug, Serialize)]
pub struct AnnotationSlugs {
    metadata: AnnotationMetadataSlugs,
}

/// A struct representing an [`AnnotationMetadata`]'s slugified strings.
///
/// See [`AnnotationMetadata`] for undocumented fields.
#[allow(missing_docs)]
#[derive(Debug, Serialize)]
pub struct AnnotationMetadataSlugs {
    created: String,
    modified: String,
}

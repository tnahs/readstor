//! Defines the context injected directly into a template.

use serde::Serialize;

use super::annotation::AnnotationContext;
use super::book::BookContext;
use super::entry::EntryContext;
use super::names::NamesContext;

/// An enum representing all possible template contexts.
///
/// This primarily used to shuffle data to fit a certain shape before it's
/// injected into a template.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum TemplateContext<'a> {
    /// Used when rendering both a [`Book`][book] and its
    /// [`Annotation`][annotation]s in a template. Includes all the output
    /// filenames and the nested directory name.
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    Book {
        #[allow(missing_docs)]
        book: &'a BookContext<'a>,
        #[allow(missing_docs)]
        annotations: &'a [AnnotationContext<'a>],
        #[allow(missing_docs)]
        names: &'a NamesContext,
    },
    /// Used when rendering a single [`Annotation`][annotation] in a template.
    /// Includes all the output filenames and the nested directory name.
    ///
    /// [annotation]: crate::models::annotation::Annotation
    Annotation {
        #[allow(missing_docs)]
        book: &'a BookContext<'a>,
        #[allow(missing_docs)]
        annotation: &'a AnnotationContext<'a>,
        #[allow(missing_docs)]
        names: &'a NamesContext,
    },
    /// Used when rendering the output filename for a template with
    /// [`ContextMode::Book`][book].
    ///
    /// [book]: super::super::ContextMode::Book
    NameBook {
        #[allow(missing_docs)]
        book: &'a BookContext<'a>,
        #[allow(missing_docs)]
        annotations: &'a [AnnotationContext<'a>],
    },
    /// Used when rendering the output filename for a template with
    /// [`ContextMode::Annotation`][annotation].
    ///
    /// [annotation]: super::super::ContextMode::Annotation
    NameAnnotation {
        #[allow(missing_docs)]
        book: &'a BookContext<'a>,
        #[allow(missing_docs)]
        annotation: &'a AnnotationContext<'a>,
    },
}

#[allow(missing_docs)]
impl<'a> TemplateContext<'a> {
    #[must_use]
    pub fn book(entry: &'a EntryContext<'a>, names: &'a NamesContext) -> Self {
        Self::Book {
            book: &entry.book,
            annotations: &entry.annotations,
            names,
        }
    }

    #[must_use]
    pub fn annotation(
        book: &'a BookContext<'a>,
        annotation: &'a AnnotationContext<'a>,
        names: &'a NamesContext,
    ) -> Self {
        Self::Annotation {
            book,
            annotation,
            names,
        }
    }

    #[must_use]
    pub fn name_book(entry: &'a EntryContext<'a>) -> Self {
        Self::NameBook {
            book: &entry.book,
            annotations: &entry.annotations,
        }
    }

    #[must_use]
    pub fn name_annotation(
        book: &'a BookContext<'a>,
        annotation: &'a AnnotationContext<'a>,
    ) -> Self {
        Self::NameAnnotation { book, annotation }
    }
}

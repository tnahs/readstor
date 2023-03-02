//! Defines types for pre- and post-processing.

pub mod processors;

use std::collections::BTreeSet;

use crate::models::entry::{Entries, Entry};
use crate::render::template::TemplateRender;

/// A struct for pre-processing [`Entry`]s.
#[derive(Debug, Clone, Copy)]
pub struct PreProcessRunner;

impl PreProcessRunner {
    /// Runs all pre-processors on an [`Entry`].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`]s to process.
    /// * `options` - The pre-process options.
    pub fn run<O>(entries: &mut Entries, options: O)
    where
        O: Into<PreProcessOptions>,
    {
        let options: PreProcessOptions = options.into();

        for entry in entries.values_mut() {
            Self::sort_annotations(entry);

            if options.extract_tags {
                Self::extract_tags(entry);
            }

            if options.normalize_whitespace {
                Self::normalize_whitespace(entry);
            }

            if options.convert_all_to_ascii {
                Self::convert_all_to_ascii(entry);
            }

            if options.convert_symbols_to_ascii {
                Self::convert_symbols_to_ascii(entry);
            }
        }
    }

    /// Sort annotations by [`AnnotationMetadata::location`][location].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [location]: crate::models::annotation::AnnotationMetadata::location
    pub fn sort_annotations(entry: &mut Entry) {
        entry.annotations.sort();
    }

    /// Extracts `#tags` from [`Annotation::notes`][annotation-notes] and
    /// places them into [`Annotation::tags`][annotation-tags]. Additionally,
    /// compiles all `#tags` and places them into [`Book::tags`][book-tags].
    /// The `#tags` are removed from [`Annotation::notes`][annotation-notes].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [annotation-notes]: crate::models::annotation::Annotation::notes
    /// [annotation-tags]: crate::models::annotation::Annotation::tags
    /// [book-tags]: crate::models::book::Book::tags
    fn extract_tags(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.tags = processors::extract_tags(&annotation.notes);
            annotation.notes = processors::remove_tags(&annotation.notes);
        }

        // Compile/insert unique list of `#tags` into `Book::tags`.
        let mut tags = entry
            .annotations
            .iter()
            .flat_map(|annotation| annotation.tags.clone())
            .collect::<Vec<String>>();

        tags.sort();

        entry.book.tags = BTreeSet::from_iter(tags);
    }

    /// Normalizes whitespace in [`Annotation::body`][body].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [body]: crate::models::annotation::Annotation::body
    fn normalize_whitespace(entry: &mut Entry) {
        for annotation in &mut entry.annotations {
            annotation.body = processors::normalize_whitespace(&annotation.body);
        }
    }

    /// Converts all Unicode characters found in [`Annotation::body`][body],
    /// [`Book::title`][title] and [`Book::author`][author] to their ASCII
    /// equivalents.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [author]: crate::models::book::Book::author
    /// [body]: crate::models::annotation::Annotation::body
    /// [title]: crate::models::book::Book::title
    fn convert_all_to_ascii(entry: &mut Entry) {
        entry.book.title = processors::convert_all_to_ascii(&entry.book.title);
        entry.book.author = processors::convert_all_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = processors::convert_all_to_ascii(&annotation.body);
        }
    }

    /// Converts a subset of "smart" Unicode symbols found in
    /// [`Annotation::body`][body], [`Book::title`][title] and
    /// [`Book::author`][author] to their ASCII equivalents.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to process.
    ///
    /// [author]: crate::models::book::Book::author
    /// [body]: crate::models::annotation::Annotation::body
    /// [title]: crate::models::book::Book::title
    fn convert_symbols_to_ascii(entry: &mut Entry) {
        entry.book.title = processors::convert_symbols_to_ascii(&entry.book.title);
        entry.book.author = processors::convert_symbols_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = processors::convert_symbols_to_ascii(&annotation.body);
        }
    }
}

/// A struct representing options for the [`PreProcessRunner`] struct.
#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct PreProcessOptions {
    /// Toggles running `#tag` extraction from notes.
    pub extract_tags: bool,

    /// Toggles running whitespace normalization.
    pub normalize_whitespace: bool,

    /// Toggles converting all Unicode characters to ASCII.
    pub convert_all_to_ascii: bool,

    /// Toggles converting "smart" Unicode symbols to ASCII.
    pub convert_symbols_to_ascii: bool,
}

/// A struct for post-processing [`TemplateRender`]s.
#[derive(Debug, Clone, Copy)]
pub struct PostProcessRunner;

impl PostProcessRunner {
    /// Runs all post-processors on an [`TemplateRender`].
    ///
    /// # Arguments
    ///
    /// * `renders` - The [`TemplateRender`]s to process.
    /// * `options` - The post-process options.
    pub fn run<O>(renders: Vec<&mut TemplateRender>, options: O)
    where
        O: Into<PostProcessOptions>,
    {
        let options: PostProcessOptions = options.into();

        for render in renders {
            if options.trim_blocks {
                Self::trim_blocks(render);
            }

            if let Some(width) = options.wrap_text {
                Self::wrap_text(render, width);
            }
        }
    }

    /// Trims any blocks left after rendering.
    ///
    /// # Arguments
    ///
    /// * `render` - The [`TemplateRender`] to process.
    fn trim_blocks(render: &mut TemplateRender) {
        render.contents = processors::trim_blocks(&render.contents);
    }

    /// Wraps text to a maximum character width.
    ///
    /// Maximum line length is not guaranteed as long words are not broken if
    /// their length exceeds the maximum. Hyphenation is not used, however,
    /// an existing hyphen can be split on to insert a line-break.
    ///
    /// # Arguments
    ///
    /// * `render` - The [`TemplateRender`] to process.
    /// * `width` - The maximum character width.
    fn wrap_text(render: &mut TemplateRender, width: usize) {
        let options = textwrap::Options::new(width).break_words(false);
        render.contents = textwrap::fill(&render.contents, options);
    }
}

/// A struct representing options for the [`PostProcessRunner`] struct.
#[derive(Debug, Default, Clone, Copy)]
pub struct PostProcessOptions {
    /// Toggles trimming blocks left after rendering.
    pub trim_blocks: bool,

    /// Toggles wrapping text to a maximum character width.
    pub wrap_text: Option<usize>,
}

#[cfg(test)]
mod test_processes {

    use super::*;

    mod tags {

        use super::*;

        use std::collections::BTreeSet;

        use crate::models::annotation::Annotation;
        use crate::models::book::Book;

        // Tests that tags are properly extracted from `Annotation::notes`, placed into the
        // `Annotation::tags` field and finally compiled, deduped and placed into their respective
        // book's `Book::tags` field.
        #[test]
        fn test_extracted_tags() {
            let mut entry = Entry {
                book: Book::default(),
                annotations: vec![
                    Annotation {
                        notes: "#tag01 #tag02".to_string(),
                        ..Default::default()
                    },
                    Annotation {
                        notes: "#tag02 #tag03".to_string(),
                        ..Default::default()
                    },
                    Annotation {
                        notes: "#tag03 #tag01".to_string(),
                        ..Default::default()
                    },
                ],
            };

            PreProcessRunner::extract_tags(&mut entry);

            for annotation in entry.annotations {
                assert_eq!(annotation.tags.len(), 2);
                assert!(annotation.notes.is_empty());
            }

            assert_eq!(
                entry.book.tags,
                BTreeSet::from(["#tag01", "#tag02", "#tag03"].map(String::from))
            );
        }
    }
}

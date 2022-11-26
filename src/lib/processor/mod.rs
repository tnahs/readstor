//! Defines the [`PreProcessor`] and [`PostProcessor`] structs used for pre- and
//! post-processing of [`Entry`]s and [`TemplateRender`]s respectively.

pub mod process;

use crate::models::entry::Entry;
use crate::templates::template::TemplateRender;

/// A struct for pre-processing [`Entry`]s.
#[derive(Debug, Clone, Copy)]
pub struct PreProcessor;

impl PreProcessor {
    /// Runs all pre-processors on an [`Entry`].
    ///
    /// # Arguments
    ///
    /// * `options` - The pre-processor's options.
    /// * `entry` - The [`Entry`] to process.
    pub fn run<O>(options: O, entry: &mut Entry)
    where
        O: Into<PreProcessorOptions>,
    {
        let options = options.into();

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
            annotation.tags = process::extract_tags(&annotation.notes);
            annotation.notes = process::remove_tags(&annotation.notes);
        }

        // Compile/insert all `#tags` into `Book::tags`.
        entry.book.tags = entry
            .annotations
            .iter()
            .flat_map(|a| a.tags.clone())
            .collect();
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
            annotation.body = process::normalize_whitespace(&annotation.body);
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
        entry.book.title = process::convert_all_to_ascii(&entry.book.title);
        entry.book.author = process::convert_all_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = process::convert_all_to_ascii(&annotation.body);
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
        entry.book.title = process::convert_symbols_to_ascii(&entry.book.title);
        entry.book.author = process::convert_symbols_to_ascii(&entry.book.author);

        for annotation in &mut entry.annotations {
            annotation.body = process::convert_symbols_to_ascii(&annotation.body);
        }
    }
}

/// A struct represting options for the [`PreProcessor`] struct.
#[derive(Debug, Default, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct PreProcessorOptions {
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
pub struct PostProcessor;

impl PostProcessor {
    /// Runs all post-processors on an [`TemplateRender`].
    ///
    /// # Arguments
    ///
    /// * `options` - The post-processor's options.
    /// * `render` - The [`TemplateRender`] to process.
    pub fn run<O>(options: O, render: &mut TemplateRender)
    where
        O: Into<PostProcessorOptions>,
    {
        let options = options.into();

        if options.trim_blocks {
            Self::trim_blocks(render);
        }

        if let Some(width) = options.wrap_text {
            Self::wrap_text(render, width);
        }
    }

    /// Trims any blocks left after rendering.
    ///
    /// # Arguments
    ///
    /// * `render` - The [`TemplateRender`] to process.
    fn trim_blocks(render: &mut TemplateRender) {
        render.contents = process::trim_blocks(&render.contents);
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

/// A struct represting options for the [`PostProcessor`] struct.
#[derive(Debug, Default, Clone, Copy)]
pub struct PostProcessorOptions {
    /// Toggles trimming blocks left after rendering.
    pub trim_blocks: bool,

    /// Toggles wrapping text to a maximum character width.
    pub wrap_text: Option<usize>,
}

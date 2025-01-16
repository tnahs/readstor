//! Defines types for post-processing.
//!
//! Post-processors are used mutate fields within a [`Render`].

use crate::render::template::Render;
use crate::strings;

/// Runs post-processes on [`Render`]s.
///
/// # Arguments
///
/// * `renders` - The [`Render`]s to process.
/// * `options` - The post-process options.
pub fn run<O>(renders: Vec<&mut Render>, options: O)
where
    O: Into<PostProcessOptions>,
{
    let options: PostProcessOptions = options.into();

    for render in renders {
        if options.trim_blocks {
            self::trim_blocks(render);
        }

        if let Some(width) = options.wrap_text {
            self::wrap_text(render, width);
        }
    }
}

/// Trims any blocks left after rendering.
///
/// # Arguments
///
/// * `render` - The [`Render`] to process.
fn trim_blocks(render: &mut Render) {
    render.contents = strings::trim_blocks(&render.contents);
}

/// Wraps text to a maximum character width.
///
/// Maximum line length is not guaranteed as long words are not broken if their length exceeds
/// the maximum. Hyphenation is not used, however, an existing hyphen can be split on to insert
/// a line-break.
///
/// # Arguments
///
/// * `render` - The [`Render`] to process.
/// * `width` - The maximum character width.
fn wrap_text(render: &mut Render, width: usize) {
    let options = textwrap::Options::new(width).break_words(false);
    render.contents = textwrap::fill(&render.contents, options);
}

/// A struct representing options for running post-processes.
#[derive(Debug, Default, Clone, Copy)]
pub struct PostProcessOptions {
    /// Toggles trimming blocks left after rendering.
    pub trim_blocks: bool,

    /// Toggles wrapping text to a maximum character width.
    pub wrap_text: Option<usize>,
}

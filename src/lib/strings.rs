//! Defines functions for string creation/manipulation.

use std::collections::BTreeSet;
use std::ffi::OsStr;

use chrono::DateTime;
use chrono::Utc;
use deunicode::deunicode;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

use super::result::Result;
use crate::render::engine::RenderEngine;

/// Captures a `#tag`. Tags *must* start with a hash symbol `#` followed by a letter in `[a-zA-Z]`
/// and then a series of any characters. A tag ends when a space or another `#` is encountered.
static RE_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[a-zA-Z][^\s#]+\s?").unwrap());

/// Captures three or more consecutive linebreaks.
static RE_BLOCKS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n{3,}").unwrap());

/// Strips a string of a set of characters.
///
/// # Arguments
///
/// * `string` - The input string.
/// * `chars` - Characters to strip out.
#[must_use]
pub fn strip(string: &str, chars: &str) -> String {
    let mut stripped = string.to_string();

    stripped.retain(|char| !chars.contains(char));

    stripped
}

/// Removes/replaces problematic characters from a string.
///
/// # Arguments
///
/// * `string` - The string to sanitize.
#[must_use]
pub fn sanitize(string: &str) -> String {
    // These characters can potentially cause problems in filenames.
    let remove = &['\n', '\r', '\0'];
    let replace = &['/', ':'];

    let sanitized: String = string
        .chars()
        .filter(|c| !remove.contains(c))
        .map(|c| if replace.contains(&c) { '_' } else { c })
        .collect();

    let sanitized = OsStr::new(&sanitized);
    let sanitized = sanitized.to_string_lossy().to_string();

    if sanitized != string {
        log::warn!("the string '{}' contained invalid characters", string);
    };

    sanitized
}

/// Slugifies a string.
///
/// Re-implementation of: <https://github.com/Stebalien/slug-rs/> but with an additional argument to
/// toggle whether or not to drop the case of the slugified string.
///
/// # Arguments
///
/// * `string` - The input string.
/// * `lowercase` - Toggle dropping the case of the string.
#[must_use]
pub fn to_slug(string: &str, lowercase: bool) -> String {
    let mut slug = String::with_capacity(string.len());

    // Start `true` to avoid any leading dashes.
    let mut prev_is_dash = true;

    {
        let mut push_char = |mut char: u8| match char {
            b'a'..=b'z' | b'0'..=b'9' => {
                prev_is_dash = false;
                slug.push(char.into());
            }
            b'A'..=b'Z' => {
                prev_is_dash = false;

                char = if lowercase { char - b'A' + b'a' } else { char };

                slug.push(char.into());
            }
            _ => {
                if !prev_is_dash {
                    slug.push('-');
                    prev_is_dash = true;
                }
            }
        };

        for char in string.chars() {
            if char.is_ascii() {
                (push_char)(char as u8);
            } else {
                for &byte in deunicode::deunicode_char(char).unwrap_or("-").as_bytes() {
                    (push_char)(byte);
                }
            }
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    slug.shrink_to_fit();

    slug
}

/// Slugifies a date.
///
/// # Arguments
///
/// * `date` - The date to slugify.
#[must_use]
pub fn to_slug_date(date: &DateTime<Utc>) -> String {
    date.format(crate::defaults::DATE_FORMAT).to_string()
}

/// Renders a one-off template string with a context and sanitizes the output string.
///
/// # Errors
///
/// Will return `Err` if the render engine encounters any errors.
pub fn render_and_sanitize<C>(template: &str, context: C) -> Result<String>
where
    C: Serialize,
{
    let string = RenderEngine::default().render_str(template, context)?;

    Ok(sanitize(&string))
}

/// Builds a filename from a file stem and extension and sanitizes the output string.
///
/// This is a helper method to replace `PathBuf::set_extension()` as some file stems might include
/// a period `.`. If we used `PathBuf::set_extension()`, the text after the last period would be
/// replaced with the extension.
///
/// # Arguments
///
/// * `file_stem` - The file stem.
/// * `extension` - The file extension.
#[must_use]
pub fn build_filename_and_sanitize(file_stem: &str, extension: &str) -> String {
    let filename = format!("{file_stem}.{extension}");

    sanitize(&filename)
}

/// Trims whitespace and replaces all linebreaks with: `\n\n`.
///
/// # Arguments
///
/// * `string` - The string to normalize.
#[must_use]
pub fn normalize_whitespace(string: &str) -> String {
    string
        .lines()
        .filter(|&s| !s.is_empty())
        .map(str::trim)
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Extracts all `#tags` from a string.
///
/// # Arguments
///
/// * `string` - The string to extract from.
#[must_use]
pub fn extract_tags(string: &str) -> BTreeSet<String> {
    let mut tags = RE_TAG
        .find_iter(string)
        .map(|t| t.as_str())
        .map(str::trim)
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>();

    tags.sort();

    BTreeSet::from_iter(tags)
}

/// Removes all `#tags` from a string.
///
/// # Arguments
///
/// * `string` - The string to remove from.
#[must_use]
pub fn remove_tags(string: &str) -> String {
    RE_TAG.replace_all(string, "").trim().to_owned()
}

/// Converts all Unicode characters to their ASCII equivalent.
///
/// # Arguments
///
/// * `string` - The string to convert.
#[must_use]
pub fn convert_all_to_ascii(string: &str) -> String {
    deunicode(string)
}

/// Converts a subset of "smart" Unicode symbols to their ASCII equivalents.
///
/// See [`UNICODE_TO_ASCII_SYMBOLS`][symbols] for list of symbols and their ASCII equivalents.
///
/// # Arguments
///
/// * `string` - The string to convert.
///
/// [symbols]: crate::defaults::UNICODE_TO_ASCII_SYMBOLS
#[must_use]
pub fn convert_symbols_to_ascii(string: &str) -> String {
    let mut string = string.to_owned();

    for (from, to) in &*crate::defaults::UNICODE_TO_ASCII_SYMBOLS {
        string = string.replace(*from, to);
    }

    string
}

/// Normalizes linebreaks by replacing three or more consecutive linebreaks with two consecutive
/// linebreaks while leaving a single trailing linebreak.
///
/// NOTE: This is a temporary solution that naively mimicks what [`tera`][tera] would do if/when it
/// adds [`trim_blocks`][github-tera]. It is by no means smart and will just normalize whitespace
/// regardless of what the template requested.
///
/// # Arguments
///
/// * `string` - The string to normalize.
///
/// [github-tera]: https://github.com/Keats/tera/issues/637
/// [tera]: https://docs.rs/tera/latest/tera/
#[must_use]
pub fn trim_blocks(string: &str) -> String {
    let string = RE_BLOCKS.replace_all(string, "\n\n");
    let mut string = string.trim_end().to_string();

    string.push('\n');

    string
}

// TODO: Add tests for other functions.
#[cfg(test)]
mod test_strings {

    use super::*;

    #[test]
    fn strip() {
        assert_eq!(
            super::strip("Lorem ipsum. Aedipisicing culpa!?", " .!?"),
            "LoremipsumAedipisicingculpa"
        );
        assert_eq!(
            super::strip("Lorem ipsum.\n   Aedipisicing culpa!?", " .!?\n"),
            "LoremipsumAedipisicingculpa"
        );
        assert_eq!(
            super::strip("--Lorem--ipsum. Aedipisicing   -culpa-", " .-"),
            "LoremipsumAedipisicingculpa"
        );
        assert_eq!(
            super::strip("Lorem & Ipsúm. Ædipisicing culpa!?", " &.!?"),
            "LoremIpsúmÆdipisicingculpa"
        );
    }

    #[test]
    fn slugify_original() {
        assert_eq!(
            super::to_slug("Lorem ipsum. Aedipisicing culpa!?", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
        assert_eq!(
            super::to_slug("Lorem ipsum.\n   Aedipisicing culpa!?", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
        assert_eq!(
            super::to_slug("--Lorem--ipsum. Aedipisicing   -culpa-", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
        assert_eq!(
            super::to_slug("Lorem & Ipsúm. Ædipisicing culpa!?", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
    }

    #[test]
    fn slugify_with_lowercase() {
        assert_eq!(
            super::to_slug("Lorem ipsum. Aedipisicing culpa!?", false),
            "Lorem-ipsum-Aedipisicing-culpa"
        );
        assert_eq!(
            super::to_slug("Lorem ipsum.\n   Aedipisicing culpa!?", false),
            "Lorem-ipsum-Aedipisicing-culpa"
        );
        assert_eq!(
            super::to_slug("--Lorem--ipsum. Aedipisicing   -culpa-", false),
            "Lorem-ipsum-Aedipisicing-culpa"
        );
        assert_eq!(
            super::to_slug("Lorem & Ipsúm. Ædipisicing culpa!?", false),
            "Lorem-Ipsum-AEdipisicing-culpa"
        );
    }

    // https://stackoverflow.com/a/34666891/16968574
    macro_rules! remove_and_extract_tags {
        ($($name:ident: ($input:tt, $tags_removed_expected:tt, $tags_expected:tt),)*) => {
            $(
                #[test]
                fn $name() {
                    let tags_extracted = super::extract_tags($input);
                    let tags_expected: BTreeSet<String> = $tags_expected
                        .into_iter()
                        .map(|t: &str| t.to_string())
                        .collect();

                    let tags_removed = super::remove_tags($input);

                    assert_eq!(tags_extracted, tags_expected);
                    assert_eq!(tags_removed, $tags_removed_expected.to_string());
                }
            )*
        }
    }

    // Tests that extracting and removing tags from a string produces the expected results. Only
    // tags, e.g. contigious strings starting with a hashtag, should be extracted and removed
    // from the original string.
    //
    // "Lorem ipsum. #tag",  // Input string
    // "Lorem ipsum.",       // Expected: tags removed
    // ["#tag"]              // Expected: tags extracted
    remove_and_extract_tags! {
        // Tests no tags in string.
        process_tags_00: (
            "Lorem ipsum.",
            "Lorem ipsum.",
            []
        ),
        // Tests tags at end of a string.
        process_tags_01: (
            "Lorem ipsum. #tag01 #tag02",
            "Lorem ipsum.",
            ["#tag01", "#tag02"]
        ),
        // Tests tags in the middle of a string.
        process_tags_02: (
            "Lorem ipsum. #tag01 #tag02 Adipisicing culpa.",
            "Lorem ipsum. Adipisicing culpa.",
            ["#tag01", "#tag02"]
        ),
        // Tests tags at beginning of a string.
        process_tags_03: (
            "#tag01 #tag02 Lorem ipsum. Adipisicing culpa.",
            "Lorem ipsum. Adipisicing culpa.",
            ["#tag01", "#tag02"]
        ),
        // Tests tags with extra whitespace.
        process_tags_04: (
            "Lorem ipsum.  #tag01  #tag02  ",
            "Lorem ipsum.",
            ["#tag01", "#tag02"]
        ),
        // Tests tags without spacing.
        process_tags_05: (
            "Lorem ipsum.#tag01#tag02",
            "Lorem ipsum.",
            ["#tag01", "#tag02"]
         ),
        // Tests that tags must start with letter.
        process_tags_06: (
            "#tag01 #TAG01 #Tag01 #1 #999",
            "#1 #999",
            ["#tag01", "#TAG01", "#Tag01"]
        ),
        // Tests that a string with only tags ends up empty.
        process_tags_07: (
            "#tag01 #tag02",
            "",
            ["#tag01", "#tag02"]
        ),
        // Tests that tags are deduped.
        process_tags_08: (
            "#tag01 #tag01 #tag01",
            "",
            ["#tag01"]
        ),
        // Tests that extra hashtags are ignored.
        process_tags_09: (
            "###tag01##tag02",
            "###",
            ["#tag01", "#tag02"]
        ),
    }
}

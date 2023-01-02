//! Pre-processors for manipulating [`Entry`][entry], [`Book`][book] and
//! [`Annotation`][annotation] types before they are rendered/exported and
//! post-processors for manipulating a template after it's been rendered.
//!
//! [annotation]: crate::models::annotation::Annotation
//! [book]: crate::models::book::Book
//! [entry]: crate::models::entry::Entry

use std::collections::BTreeSet;

use deunicode::deunicode;
use once_cell::sync::Lazy;
use regex::Regex;

/// Captures a `#tag`. Tags *must* start with a hash symbol `#` followed by a
/// letter `[a-zA-Z]`.
static RE_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[a-zA-Z][^\s#]+\s?").unwrap());

/// Captures three or more consecutive linebreaks.
static RE_BLOCKS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n{3,}").unwrap());

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
/// See [`UNICODE_TO_ASCII_SYMBOLS`][symbols] for list of symbols and their
/// ASCII equivalents.
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

/// Normalizes linebreaks by replacing three or more consecutive linebreaks
/// with two consecutive linebreaks while leaving a single trailing linebreak.
///
/// NOTE: This is a temporary solution that naively mimicks what [`tera`][tera]
/// would do if/when it adds [`trim_blocks`][github-tera]. It is by no means
/// smart and will just normalize whitespace regardless of what the template
/// requested.
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

#[cfg(test)]
mod test_processes {

    use super::*;

    // https://stackoverflow.com/a/34666891/16968574
    macro_rules! test_tags {
        ($($name:ident: ($input:tt, $tags_removed_expected:tt, $tags_expected:tt),)*) => {
            $(
                #[test]
                fn $name() {
                    let tags_extracted = extract_tags($input);
                    let tags_expected: BTreeSet<String> = $tags_expected
                        .into_iter()
                        .map(|t: &str| t.to_string())
                        .collect();

                    let tags_removed = remove_tags($input);

                    assert_eq!(tags_extracted, tags_expected);
                    assert_eq!(tags_removed, $tags_removed_expected.to_string());
                }
            )*
        }
    }

    test_tags! {
        // ...
        // "Lorem ipsum. #tag",  // Input string
        // "Lorem ipsum.",       // Expected: tags removed
        // ["#tag"]              // Expected: tags extracted
        // ...
        test_extract_tags_00: (
            "Lorem ipsum.",
            "Lorem ipsum.",
            []
        ),
        test_extract_tags_01: (
            "Lorem ipsum. #tag01 #tag02",
            "Lorem ipsum.",
            ["#tag01", "#tag02"]
        ),
        test_extract_tags_02: (
            "Lorem ipsum. #tag01 #tag02 ",
            "Lorem ipsum.",
            ["#tag01", "#tag02"]
        ),
        test_extract_tags_03: (
            "Lorem ipsum.  #tag01  #tag02",
            "Lorem ipsum.",
            ["#tag01", "#tag02"]
         ),
        test_extract_tags_04: (
            "#tag01 #02 #03",
            "#02 #03",
            ["#tag01"]
        ),
        test_extrt_tags_05: (
            "#tag01 #tag01 #tag01",
            "",
            ["#tag01"]
        ),
    }
}

//! Defines template filters.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use tera::{Result, Value};

/// A mapping of `name:filter`. For registering template filters.
pub static FILTER_MAPPING: Lazy<HashMap<&'static str, FilterFn>> = Lazy::new(|| {
    let mut filters: HashMap<&str, FilterFn> = HashMap::new();
    filters.insert("strip", filter_strip);
    filters.insert("slugify", filter_slugify);
    filters
});

/// Tera filter function signature.
type FilterFn = fn(&Value, &HashMap<String, Value>) -> Result<Value>;

#[allow(clippy::implicit_hasher)]
fn filter_strip(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let input = value
        .as_str()
        .ok_or("Expected input value to be a string")?;

    let chars = args.get("chars").and_then(Value::as_str).unwrap_or(" ");

    Ok(Value::String(strip(input, chars)))
}

#[allow(clippy::implicit_hasher)]
fn filter_slugify(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let input = value
        .as_str()
        .ok_or("Expected input value to be a string")?;

    let lowercase = args
        .get("lowercase")
        .and_then(Value::as_bool)
        .unwrap_or(true);

    let replaced = slugify(input, lowercase);

    Ok(Value::String(replaced))
}

/// Strips a string of a set of characters.
///
/// # Arguments
///
/// * `string` - The input string.
/// * `chars` - Characters to strip out.
fn strip(string: &str, chars: &str) -> String {
    let mut stripped = string.to_string();

    stripped.retain(|char| !chars.contains(char));

    stripped
}

/// Slugify a string.
///
/// Re-implementation of: <https://github.com/Stebalien/slug-rs/> but with an additional argument to
/// toggle whether or not to drop the case of the slugified string.
///
/// # Arguments
///
/// * `string` - The input string.
/// * `lowercase` - Toggle dropping the case of the string.
///
fn slugify(string: &str, lowercase: bool) -> String {
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_strip() {
        assert_eq!(
            strip("Lorem ipsum. Aedipisicing culpa!?", " .!?"),
            "LoremipsumAedipisicingculpa"
        );
        assert_eq!(
            strip("Lorem ipsum.\n   Aedipisicing culpa!?", " .!?\n"),
            "LoremipsumAedipisicingculpa"
        );
        assert_eq!(
            strip("--Lorem--ipsum. Aedipisicing   -culpa-", " .-"),
            "LoremipsumAedipisicingculpa"
        );
        assert_eq!(
            strip("Lorem & Ipsúm. Ædipisicing culpa!?", " &.!?"),
            "LoremIpsúmÆdipisicingculpa"
        );
    }

    #[test]
    fn test_slugify_original() {
        assert_eq!(
            slugify("Lorem ipsum. Aedipisicing culpa!?", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
        assert_eq!(
            slugify("Lorem ipsum.\n   Aedipisicing culpa!?", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
        assert_eq!(
            slugify("--Lorem--ipsum. Aedipisicing   -culpa-", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
        assert_eq!(
            slugify("Lorem & Ipsúm. Ædipisicing culpa!?", true),
            "lorem-ipsum-aedipisicing-culpa"
        );
    }

    #[test]
    fn test_slugify_with_lowercase() {
        assert_eq!(
            slugify("Lorem ipsum. Aedipisicing culpa!?", false),
            "Lorem-ipsum-Aedipisicing-culpa"
        );
        assert_eq!(
            slugify("Lorem ipsum.\n   Aedipisicing culpa!?", false),
            "Lorem-ipsum-Aedipisicing-culpa"
        );
        assert_eq!(
            slugify("--Lorem--ipsum. Aedipisicing   -culpa-", false),
            "Lorem-ipsum-Aedipisicing-culpa"
        );
        assert_eq!(
            slugify("Lorem & Ipsúm. Ædipisicing culpa!?", false),
            "Lorem-Ipsum-AEdipisicing-culpa"
        );
    }
}

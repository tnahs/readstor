//! Utilities for working with templates.

use ::std::hash::BuildHasher;
use std::collections::HashMap;

use serde::de;
use serde::ser::{self, SerializeSeq};
use serde::{Deserialize, Serialize};

use walkdir::DirEntry;

/// Helper function for [`walkdir`][walkdir]. Filter "hidden" entries e.g. `.hidden`.
///
/// [walkdir]: https://docs.rs/walkdir/latest/walkdir/
#[must_use]
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| !s.starts_with('.'))
}

/// Helper function for [`walkdir`][walkdir]. Filter normal templates.
///
/// [walkdir]: https://docs.rs/walkdir/latest/walkdir/
#[must_use]
pub fn is_normal_template(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| !s.starts_with('_'))
}

/// Helper function for [`walkdir`][walkdir]. Filter partial templates.
///
/// [walkdir]: https://docs.rs/walkdir/latest/walkdir/
#[must_use]
pub fn is_partial_template(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('_'))
}

/// Custom deserialization method to deserialize and sanitize a string.
#[allow(clippy::missing_errors_doc)]
pub fn deserialize_and_sanitize<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(crate::utils::sanitize_string(s))
}

/// Custom serialization method to convert a `HashMap<K, V>` to `Vec<V>`.
// https://rust-lang.github.io/rust-clippy/master/index.html
#[allow(clippy::missing_errors_doc)]
pub fn serialize_hashmap_to_vec<S, K, V, B>(
    map: &HashMap<K, V, B>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: ser::Serializer,
    V: Serialize,
    B: BuildHasher,
{
    let values: Vec<&V> = map.values().collect();
    let mut seq = serializer.serialize_seq(Some(values.len()))?;
    for value in values {
        seq.serialize_element(value)?;
    }
    seq.end()
}

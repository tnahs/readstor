//! Defines utilities for this crate.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::hash::BuildHasher;
use std::io;
use std::path::Path;

use serde::ser::SerializeSeq;
use serde::{de, ser, Deserialize, Serialize};

use super::strings;

/// Recursively copies all files from one directory into another.
///
/// # Arguments
///
/// * `source` - The source directory.
/// * `destination` - The destination directory.
///
/// # Errors
///
/// Will return `Err` if any IO errors are encountered.
//
// <https://stackoverflow.com/a/65192210/16968574>
pub fn copy_dir<S, D>(source: S, destination: D) -> io::Result<()>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    let source = source.as_ref();
    let destination = destination.as_ref();

    std::fs::create_dir_all(destination)?;

    for entry in std::fs::read_dir(source)? {
        let entry = entry?;

        if entry.path().is_dir() {
            copy_dir(entry.path(), destination.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }

    log::debug!(
        "copied directory {} to {}",
        &source.display(),
        &destination.display(),
    );

    Ok(())
}

/// Returns the file extension of a path.
///
/// # Arguments
///
/// * `path` - The path to extract the file extension from.
///
/// Returns `None` if the `source` path terminates in `..` or is `/`.
#[must_use]
pub fn get_file_extension<P>(path: &P) -> Option<&str>
where
    P: AsRef<Path>,
{
    path.as_ref().extension().and_then(OsStr::to_str)
}

/// Returns the filename of a path.
///
/// # Arguments
///
/// * `path` - The path to extract the filename from.
///
/// Returns `None` if the `source` path terminates in `..` or is `/`.
#[must_use]
pub fn get_filename<P>(path: &P) -> Option<&str>
where
    P: AsRef<Path>,
{
    path.as_ref().file_name().and_then(OsStr::to_str)
}

/// Returns the file stem of a path.
///
/// # Arguments
///
/// * `path` - The path to extract the file stem from.
///
/// Returns `None` if the `source` path terminates in `..` or is `/`.
#[must_use]
pub fn get_file_stem<P>(path: &P) -> Option<&str>
where
    P: AsRef<Path>,
{
    path.as_ref().file_stem().and_then(OsStr::to_str)
}

/// Custom deserialization method to deserialize and sanitize a string.
#[allow(clippy::missing_errors_doc)]
pub fn deserialize_and_sanitize<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(strings::sanitize(s))
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

/// Loads a test template from the [`TEST_TEMPLATES`][test-templates] directory.
///
/// # Arguments
///
/// * `directory` - The template directory.
/// * `filename` - The template filename.
///
/// [test-templates]: crate::defaults::TEST_TEMPLATES
#[cfg(test)]
#[allow(clippy::missing_panics_doc)]
pub fn load_test_template_str(directory: &str, filename: &str) -> String {
    let path = crate::defaults::TEST_TEMPLATES
        .join(directory)
        .join(filename);

    std::fs::read_to_string(path).unwrap()
}

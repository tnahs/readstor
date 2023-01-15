//! Defines utilities for this crate.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::hash::BuildHasher;
use std::io;
use std::path::Path;

use chrono::DateTime;
use chrono::Local;
use chrono::Utc;
use deunicode::deunicode;
use serde::ser::SerializeSeq;
use serde::{de, ser, Deserialize, Serialize};
use tera::Tera;

use super::result::Result;

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

/// Returns today's date using the default `strftime` format string.
#[must_use]
pub fn today() -> String {
    Local::now()
        .format(crate::defaults::DATE_FORMAT)
        .to_string()
}

/// Returns today's date using a custom `strftime` format string.
///
/// # Arguments
///
/// * `format` - An `strftime` format string.
#[must_use]
pub fn today_format(format: &str) -> String {
    Local::now().format(format).to_string()
}

/// Removes/replaces problematic characters from a string.
///
/// # Arguments
///
/// * `string` - The string to sanitize.
#[must_use]
pub fn sanitize_string(string: &str) -> String {
    // These characters can potentially cause problems in filenames.
    let remove = &['\n', '\r', '\0'];
    let replace = &['/', ':'];

    let sanitized: String = string
        .chars()
        .filter(|c| !remove.contains(c))
        .map(|c| if replace.contains(&c) { '_' } else { c })
        .collect();

    if sanitized != string {
        log::warn!("the string '{}' contained invalid characters", string);
    };

    sanitized
}

/// Slugifies a string.
///
/// # Arguments
///
/// * `string` - The string to slugify.
/// * `delimeter` - The slug delimeter.
#[must_use]
pub fn to_slug_string(string: &str, delimiter: char) -> String {
    let slug = deunicode(string)
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
        .to_lowercase()
        .replace(' ', &delimiter.to_string());

    slug.chars()
        .filter(|c| c.is_alphanumeric() || c == &delimiter)
        .collect()
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

/// Renders a template string with a context and sanitizes the output string.
///
/// # Errors
///
/// Will return `Err` if Tera encounters an error.
pub fn render_and_sanitize<C>(template: &str, context: C) -> Result<String>
where
    C: Serialize,
{
    let context = &tera::Context::from_serialize(context)?;

    let string = Tera::one_off(template, context, false)?;

    Ok(sanitize_string(&string))
}

/// Custom deserialization method to deserialize and sanitize a string.
#[allow(clippy::missing_errors_doc)]
pub fn deserialize_and_sanitize<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(sanitize_string(s))
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

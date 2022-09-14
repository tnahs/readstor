//! Defines utilities for working with this library.

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

use chrono::Local;
use deunicode::deunicode;
use walkdir::DirEntry;

use crate::lib;

/// Helper function for `walkdir`. Filters out hidden directories e.g. `.hidden`.
#[must_use]
pub fn entry_is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| !s.starts_with('.'))
}

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

    fs::create_dir_all(&destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;

        if entry.path().is_dir() {
            copy_dir(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }

    log::debug!(
        "Copied directory `{}` to `{}`",
        &source.display(),
        &destination.display(),
    );

    Ok(())
}

/// Returns the file extension from a path.
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

/// Returns the file name from a path.
///
/// # Arguments
///
/// * `path` - The path to extract the file name from.
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
    Local::now().format(lib::defaults::DATE_FORMAT).to_string()
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

/// Converts a string to ASCII.
///
/// # Arguments
///
/// * `string` - The string to convert.
#[must_use]
pub fn to_safe_string(string: &str) -> String {
    // These characters can potentially cause problems in filenames.
    let deny = &['/', ':', '\n', '\r', '\0'];

    // TODO: Maybe we should allow unicode characters here...
    // deunicode(string)
    string
        .chars()
        .map(|c| if deny.contains(&c) { '_' } else { c })
        .collect()
}

/// Slugifies a string.
///
/// # Arguments
///
/// * `string` - The string to slugify.
/// * `delimeter` - Allow list for non-alphanumeric characters.
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

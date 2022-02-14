use std::ffi::OsStr;
use std::io;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::Serialize;

#[allow(unused_imports)] // For docs.
use super::models::data::Entry;
#[allow(unused_imports)] // For docs.
use super::templates::Templates;

/// Thin wrapper around `chrono`s `DateTime<Utc>` to allow for a `Default`
/// implementation.
///
/// Why do we need a `Default` implementation? When a new template is added to
/// the [`Templates`] registry it needs to be validates both for its syntax
/// and for the fields that its variables reference. In order to achieve the
/// latter, a dummy [`Entry`] struct---its `Default` implementation---is
/// passed to validate the template's variables. Seeing as `DateTime` does not
/// have a `Default` implementation, it was either we implementation a hand
/// written `Default` of [`Entry`] which would include multiple nested
/// structs or wrap `DateTime<Utc>` and provide a `Default` implementation.
///
/// See [`Templates::add()`] for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DateTimeUTC(DateTime<Utc>);

impl Default for DateTimeUTC {
    fn default() -> Self {
        Self(DateTime::<Utc>::from(UNIX_EPOCH))
    }
}

impl Deref for DateTimeUTC {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl DerefMut for DateTimeUTC {
    fn deref_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.0
    }
}

/// Converts a `Core Data` timestamp (f64) to `DateTime`.
///
/// A `Core Data` timestamp is the number of seconds (or nanoseconds) since
/// midnight, January 1, 2001, GMT. The difference between a `Core Data`
/// timestamp and a Unix timestamp (seconds since 1/1/1970) is
/// 978307200 seconds.
///
/// <https://www.epochconverter.com/coredata>
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<f64> for DateTimeUTC {
    fn from(f: f64) -> Self {
        // Add the `Core Data` timestamp offset
        let timestamp = f + 978_307_200_f64;

        let seconds = timestamp.trunc() as i64;
        let nanoseconds = timestamp.fract() * 1_000_000_000.0;
        let datetime = NaiveDateTime::from_timestamp(seconds, nanoseconds as u32);

        DateTimeUTC(DateTime::from_utc(datetime, Utc))
    }
}

/// Helper function for [`walkdir`]. Filters out hidden/private directories.
#[must_use]
pub fn entry_is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| !s.starts_with(|c| c == '_' || c == '.'))
}

/// Returns an iterator over all the directories in a path without recursing.
///
/// # Arguments
///
/// * `path` - The path to a directory to iterate.
pub fn iter_dir<P>(path: &P) -> impl Iterator<Item = PathBuf>
where
    P: AsRef<Path>,
{
    walkdir::WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(entry_is_hidden)
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().to_owned())
}

/// Recursively copies all files in a directory.
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

    std::fs::create_dir_all(&destination)?;

    for entry in std::fs::read_dir(source)? {
        let entry = entry?;

        if entry.path().is_dir() {
            copy_dir(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.join(entry.file_name()))?;
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
/// Returns `None` if the `source` path terminates in `..` or is `/`.
#[must_use]
pub fn get_file_name<P>(path: &P) -> Option<&str>
where
    P: AsRef<Path>,
{
    path.as_ref().file_name().and_then(OsStr::to_str)
}

/// Returns the file stem of a path.
///
/// Returns `None` if the `source` path terminates in `..` or is `/`.
#[must_use]
pub fn get_file_stem<P>(path: &P) -> Option<&str>
where
    P: AsRef<Path>,
{
    path.as_ref().file_stem().and_then(OsStr::to_str)
}

/// Returns today's date as a string.
#[must_use]
pub fn today_format(format: &str) -> String {
    Local::now().format(format).to_string()
}

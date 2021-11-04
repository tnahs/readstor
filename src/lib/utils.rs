use std::ffi::OsStr;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{fs, io};

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::Serialize;

#[allow(unused_imports)] // For docs.
use super::models::stor::StorItem;
#[allow(unused_imports)] // For docs.
use super::templates::Templates;

/// Thin wrapper around `chrono`s `DateTime<Utc>` to allow for a `Default`
/// implementation.
///
/// Why do we need a `Default` implementation? When a new template is added to
/// the [`Templates`] registry it needs to be validates both for its syntax
/// and for the fields that its variables reference. In order to achieve the
/// latter, a dummy [`StorItem`] struct---its `Default` implementation---is
/// passed to validate the template's variables. Seeing as `DateTime` does not
/// have a `Default` implementation, it was either we implementation a hand
/// written `Default` of [`StorItem`] which would include multiple nested
/// structs or wrap `DateTime<Utc>` and provide a `Default` implementation.
///
/// See [`Templates::add`] for more information.
#[derive(Debug, Clone, Serialize)]
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

/// Recursively copies all files in a directory.
///
/// # Errors
///
/// Will return `Err` if any IO errors are encountered.
//
// <https://stackoverflow.com/a/65192210/16968574>
pub fn copy_dir(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(&destination)?;

    log::debug!(
        "Copying `{}` to `{}`",
        &source.display(),
        &destination.display(),
    );

    for entry in fs::read_dir(source)? {
        let entry = entry?;

        if entry.path().is_dir() {
            copy_dir(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }

    Ok(())
}

/// Returns the file extension of a path.
#[must_use]
pub fn get_file_extension(path: &Path) -> Option<&str> {
    path.extension().and_then(OsStr::to_str)
}

/// Returns the file name of a path.
#[must_use]
pub fn get_file_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(OsStr::to_str)
}

/// Returns the file stem of a path.
#[must_use]
pub fn get_file_stem(path: &Path) -> Option<&str> {
    path.file_stem().and_then(OsStr::to_str)
}

/// Returns today's date as a string.
#[must_use]
pub fn today_format(format: &str) -> String {
    Local::now().format(format).to_string()
}

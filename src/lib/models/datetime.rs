//! Defines the [`DateTimeUtc`] struct. A newtype to help when working with datetimes.

use std::ops::{Deref, DerefMut};
use std::time::UNIX_EPOCH;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

#[allow(unused_imports)] // For docs.
use super::entry::Entry;
#[allow(unused_imports)] // For docs.
use crate::lib::templates::TemplateManager;

/// Thin wrapper around [`chrono`]'s [`DateTime<Utc>`] to allow for a [`Default`] implementation.
///
/// Why do we need a [`Default`] implementation?
///
/// When a new template is added to the [`TemplateManager`] it needs to be validated both for its
/// syntax and for the fields that its variables reference. In order to achieve the latter, a dummy
/// [`Entry`] struct---its [`Default`] implementation---is passed to validate the template's
/// variables. Seeing as `DateTime` does not have a [`Default`] implementation, it was either we
/// implementation a hand written [`Default`] of [`Entry`] which would include multiple nested
/// structs or wrap [`DateTime<Utc>`] and provide a [`Default`] implementation.
///
/// See [`TemplateManager::validate_template()`] for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DateTimeUtc(DateTime<Utc>);

impl Default for DateTimeUtc {
    fn default() -> Self {
        Self(DateTime::<Utc>::from(UNIX_EPOCH))
    }
}

impl Deref for DateTimeUtc {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl DerefMut for DateTimeUtc {
    fn deref_mut(&mut self) -> &mut DateTime<Utc> {
        &mut self.0
    }
}

/// Converts a `Core Data` timestamp (f64) to `DateTime`.
///
/// A `Core Data` timestamp is the number of seconds (or nanoseconds) since midnight, January 1,
/// 2001, GMT. The difference between a `Core Data` timestamp and a Unix timestamp (seconds since
/// 1/1/1970) is 978307200 seconds.
///
/// <https://www.epochconverter.com/coredata>
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<f64> for DateTimeUtc {
    fn from(f: f64) -> Self {
        // Add the `Core Data` timestamp offset
        let timestamp = f + 978_307_200_f64;

        let seconds = timestamp.trunc() as i64;
        let nanoseconds = timestamp.fract() * 1_000_000_000.0;
        let datetime = NaiveDateTime::from_timestamp(seconds, nanoseconds as u32);

        DateTimeUtc(DateTime::from_utc(datetime, Utc))
    }
}

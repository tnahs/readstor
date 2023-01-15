//! Defines the [`DateTimeUtc`] struct.

use std::ops::{Deref, DerefMut};
use std::time::UNIX_EPOCH;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

/// A newtype around [`chrono`]'s [`DateTime<Utc>`] to allow implementation of
/// the [`Default`] trait.
///
/// Why do we need a Default implementation?
///
/// When a new template is added to the [`Templates`][templates] struct, it
/// needs to be validated both for its syntax and for the fields that its
/// variables reference. In order to achieve the latter, a dummy
/// [`Entry`][entry] struct ---its Default implementation---is passed to
/// validate the template's variables. Seeing as `DateTime` does not have a
/// Default implementation, it was either we implement a hand written Default
/// of [`Entry`][entry] which would include multiple nested structs or wrap
/// [`DateTime<Utc>`] and provide a Default implementation.
///
/// See [`Templates::validate_template()`][validate-template] for more
/// information.
///
/// [entry]: crate::models::entry::Entry
/// [templates]: crate::render::templates::Templates
/// [validate-template]: crate::render::templates::Templates::validate_template()
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
/// A `Core Data` timestamp is the number of seconds (or nanoseconds) since
/// midnight, January 1, 2001, GMT. The difference between a `Core Data`
/// timestamp and a Unix timestamp (seconds since 1/1/1970) is 978307200
/// seconds.
///
/// <https://www.epochconverter.com/coredata>
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<f64> for DateTimeUtc {
    fn from(f: f64) -> Self {
        // Add the `Core Data` timestamp offset
        let timestamp = f + 978_307_200_f64;

        let seconds = timestamp.trunc() as i64;
        let nanoseconds = timestamp.fract() * 1_000_000_000.0;
        // Unwrap should be safe here as the timestamps are coming from the OS.
        let datetime = NaiveDateTime::from_timestamp_opt(seconds, nanoseconds as u32).unwrap();

        DateTimeUtc(DateTime::from_utc(datetime, Utc))
    }
}

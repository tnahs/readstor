//! Defines the [`DateTimeUtc`] struct.

use std::ops::{Deref, DerefMut};
use std::time::UNIX_EPOCH;

use chrono::{DateTime, Utc};
use serde::Serialize;

/// A newtype around [`chrono`]'s [`DateTime<Utc>`] to allow implementation of the [`Default`] trait.
///
/// Why do we need a Default implementation?
///
/// When a template is registered, it's validated to make sure it contains no syntax errors
/// or variables that reference non-existent fields. In order to achieve the latter, a dummy
/// [`Entry`][entry] struct---its Default implementation---is passed to validate the template's
/// variables.
///
/// See [`Renderer`][renderer] and [`dummy`][dummy] for more information.
///
/// [dummy]: crate::models::dummy
/// [entry]: crate::models::entry::Entry
/// [renderer]: crate::render::renderer::Renderer
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
        // Unwrap should be safe here as the timestamps are coming from the OS.
        let datetime = DateTime::from_timestamp(seconds, nanoseconds as u32).unwrap();

        DateTimeUtc(datetime)
    }
}

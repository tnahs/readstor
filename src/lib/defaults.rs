//! Defines defaults for working with this library.

use std::path::PathBuf;

use once_cell::sync::Lazy;

/// Defines the crates's root directory.
pub static CRATE_ROOT: Lazy<PathBuf> = Lazy::new(|| env!("CARGO_MANIFEST_DIR").into());

/// Defines the user's home directory.
//
// Unwrap should be safe here. It would only fail if the user is deleted after the process has
// started. Which is highly unlikely, and would be okay to panic if that was the case.
pub static HOME: Lazy<PathBuf> = Lazy::new(|| home::home_dir().unwrap());

/// Defines the default format string. Translates to: `YYYY-MM-DD-HHMMSS` i.e. 1970-01-01-120000.
pub const DATE_FORMAT: &str = "%Y-%m-%d-%H%M%S";

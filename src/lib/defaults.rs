use std::path::PathBuf;

use once_cell::sync::Lazy;

/// Defines the crates's root directory.
pub static ROOT: Lazy<PathBuf> = Lazy::new(|| env!("CARGO_MANIFEST_DIR").into());

/// Defines the user's home directory.
//
// Unwrap should be safe here. It would only fail if the user is deleted after
// the process has started. Which is highly unlikely, and would be okay to
// panic if that was the case.
pub static HOME: Lazy<PathBuf> = Lazy::new(|| home::home_dir().unwrap());

/// Defines the environment variable key used to determine weather testing is
/// happening or not.
///
/// When testing, the official Apple Books database path is bypassed and
/// redirected to the local testing database.
pub const READSTOR_TESTING: &str = "READSTOR_TESTING";

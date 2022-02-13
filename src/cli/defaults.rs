use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::lib::defaults as lib_defaults;

/// Defines the default output directory.
///
/// The full path:
/// ```plaintext
/// /users/[user]/.readstor
/// ```
pub static OUTPUT: Lazy<PathBuf> = Lazy::new(|| lib_defaults::HOME.join(".readstor"));

/// Defines the environment variable key used to determine whether the
/// application is being worked on. If so, the Apple Books database path is
/// bypassed and redirected to a local testing/dev database.
pub const READSTOR_DEV: &str = "READSTOR_DEV";

/// TODO Document
pub const READSTOR_LOG: &str = "READSTOR_LOG";

/// Defines the path to the mock databases.
pub static MOCK_DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib_defaults::CRATE_ROOT.to_owned();
    path.extend(["tests", "data", "databases"].iter());
    path
});

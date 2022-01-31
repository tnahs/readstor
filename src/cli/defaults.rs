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
pub const DEV_READSTOR: &str = "DEV_READSTOR";

/// Defines the path to the testing/dev databases.
pub static DEV_DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib_defaults::CRATE_ROOT.to_owned();
    path.extend(["tests", "data", "databases"].iter());
    path
});

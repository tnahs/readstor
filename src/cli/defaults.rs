use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::lib;

pub const CLI_HELP_TEXT: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/cli-help.txt"));

/// Defines the default output directory.
///
/// The full path:
/// ```plaintext
/// /users/[user]/.readstor
/// ```
pub static OUTPUT: Lazy<PathBuf> = Lazy::new(|| lib::defaults::HOME.join(".readstor"));

/// Defines the environment variable key used to determine whether the
/// application is being developed on or not. If so, the Apple Books databases
/// path is bypassed and redirected to a local testing/dev database.
pub const READSTOR_DEV: &str = "READSTOR_DEV";

/// Defines the environment variable key used to set the application's log
/// level. Valid values are: `error`, `warn`, `info`, `debug` and `trace`.
pub const READSTOR_LOG: &str = "READSTOR_LOG";

/// Defines the root path to the mock databases. These are used when
pub static MOCK_DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "databases"].iter());
    path
});

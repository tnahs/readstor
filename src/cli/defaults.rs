use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::lib;

/// Defines the environment variable key used to determine whether the
/// application is being developed on or not. If so, the Apple Books databases
/// path is bypassed and redirected to a local testing/dev database.
pub const READSTOR_DEV: &str = "READSTOR_DEV";

/// Defines the environment variable key used to set the application's log
/// level. Valid values are: `error`, `warn`, `info`, `debug` and `trace`.
pub const READSTOR_LOG: &str = "READSTOR_LOG";

/// Defines the default output directory.
///
/// The full path:
/// ```plaintext
/// /users/[user]/.readstor
/// ```
pub static OUTPUT: Lazy<PathBuf> = Lazy::new(|| lib::defaults::HOME.join(".readstor"));

/// Defines the default template string. This is used as a fallback if the user
/// doesn't supply a templates directory.
pub static TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/flat/template.jinja2"
));

/// Defines the root path to the mock databases. These are used when
pub static MOCK_DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "databases"].iter());
    path
});

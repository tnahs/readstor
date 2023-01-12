//! Defines defaults for working with Apple Books.

use std::collections::HashSet;
use std::path::PathBuf;

use once_cell::sync::Lazy;

/// The root databases directory.
///
/// This assembles the full path to Apple Books' directory containing
/// `BKLibrary*.sqlite` and `AEAnnotation*.sqlite` databases.
///
/// The full path:
/// ```plaintext
/// /users/[user]/Library/Containers/com.apple.iBooksX/Data/Documents
/// ```
pub static DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = crate::defaults::HOME.to_owned();
    path.extend(
        [
            "Library",
            "Containers",
            "com.apple.iBooksX",
            "Data",
            "Documents",
        ]
        .iter(),
    );
    path
});

/// A set of all the variations of the Apple Books application name.
pub static APPLEBOOKS_NAMES: Lazy<HashSet<String>> = Lazy::new(|| {
    ["Books", "iBooks", "Apple Books", "AppleBooks"]
        .into_iter()
        .map(ToOwned::to_owned)
        .collect()
});

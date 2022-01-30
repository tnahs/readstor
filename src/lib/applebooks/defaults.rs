use std::collections::HashSet;
use std::path::PathBuf;

use once_cell::sync::Lazy;

#[allow(unused_imports)] // For docs.
use super::database::ABDatabase;
use crate::lib::defaults as lib_defaults;
#[allow(unused_imports)] // For docs.
use crate::lib::models::annotation::Annotation;
#[allow(unused_imports)] // For docs.
use crate::lib::models::book::Book;

/// Defines the root databases directory.
///
/// This assembles the full path to Apple Books' directory containing
/// `BKLibrary*.sqlite` and `AEAnnotation*.sqlite` databases.
///
/// The full path:
/// ```plaintext
/// /users/[user]/Library/Containers/com.apple.iBooksX/Data/Documents.
/// ```
pub static DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib_defaults::HOME.to_owned();
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

/// Defines all the variants of the Apple Books application name.
pub static APPLEBOOKS_NAMES: Lazy<HashSet<String>> = Lazy::new(|| {
    ["Books", "iBooks", "Apple Books", "AppleBooks"]
        .into_iter()
        .map(ToOwned::to_owned)
        .collect()
});

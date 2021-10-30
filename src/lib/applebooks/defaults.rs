use std::collections::HashSet;
use std::path::PathBuf;

use once_cell::sync::Lazy;

#[allow(unused_imports)] // For docs.
use super::database::ABDatabase;
use crate::lib::defaults::{HOME, READSTOR_TESTING, ROOT};
#[allow(unused_imports)] // For docs.
use crate::lib::models::annotation::Annotation;
#[allow(unused_imports)] // For docs.
use crate::lib::models::book::Book;

/// Defines the root databases directory.
///
/// When testing, the official Apple Books database path is bypassed and
/// redirected to the local testing database. Otherwise, this assembles the
/// full path to Apple Books' directory containing `BKLibrary*.sqlite` and
/// `AEAnnotation*.sqlite` databases. See [`ABDatabase::get_database`] for more
/// information.
///
/// The full path:
/// ```plaintext
/// /users/[user]/Library/Containers/com.apple.iBooksX/Data/Documents.
/// ```
pub static APPLEBOOKS_DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path: PathBuf;

    if std::env::var_os(READSTOR_TESTING).is_some() {
        path = ROOT.to_owned();
        path.extend(["tests", "data", "databases"]);
    } else {
        path = HOME.to_owned();
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
    }

    log::debug!("Using databases at: `{}`", path.display());

    path
});

/// Defines all the variants of the Apple Books application name.
pub static APPLEBOOKS_NAMES: Lazy<HashSet<String>> = Lazy::new(|| {
    ["Books", "iBooks", "Apple Books", "AppleBooks"]
        .iter()
        .map(|s| s.to_string())
        .collect()
});

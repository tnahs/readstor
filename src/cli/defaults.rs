use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::lib::defaults::{HOME, ROOT};

/// Defines the default output directory.
///
/// The full path:
/// ```plaintext
/// /users/[user]/.readstor
/// ```
pub static OUTPUT: Lazy<PathBuf> = Lazy::new(|| HOME.join(".readstor"));

/// Defines the path to the default template.
pub static TEMPLATE: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = ROOT.to_owned();
    path.extend(["templates", "default.txt"].iter());
    path
});

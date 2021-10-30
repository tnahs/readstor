use std::collections::HashSet;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use plist::Value;
use sysinfo::{ProcessExt, System, SystemExt};

use super::defaults::APPLEBOOKS_NAMES;
#[allow(unused_imports)] // For docs.
use crate::lib::models::annotation::Annotation;
#[allow(unused_imports)] // For docs.
use crate::lib::models::book::Book;

/// Returns Apple Books' version as `v[short]-[long]` e.g. `v3.2-2217`.
///
/// Returns `v?` if the Apple Books application cannot be found and returns
/// `v[short]-?`, `v?-[long]` or `v?-?` for possible partial versions.
pub static APPLEBOOKS_VERSION: Lazy<String> = Lazy::new(|| {
    let path: PathBuf = [
        "/",
        "System",
        "Applications",
        "Books.app",
        "Contents",
        "Info.plist",
    ]
    .iter()
    .collect();

    let value = match Value::from_file(&path) {
        Ok(value) => value,
        Err(_) => {
            // This can happen if the user is on a non-macOS device.
            log::warn!(
                "Could not determine Apple Books version. `Info.plist` not found at `{}`",
                &path.display()
            );
            return "v?".to_owned();
        }
    };

    // -> 3.2
    let version_short = value
        .as_dictionary()
        .and_then(|d| d.get("CFBundleShortVersionString"))
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| {
            log::warn!("Could not determine `CFBundleShortVersionString`");
            "?"
        });

    // -> 2217
    let version = value
        .as_dictionary()
        .and_then(|d| d.get("CFBundleVersion"))
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| {
            log::warn!("Could not determine `CFBundleVersion`");
            "?"
        });

    // v3.2-2217
    format!("v{}-{}", version_short, version)
});

/// Returns boolean based on if Apple Books is running or not.
pub fn applebooks_is_running() -> bool {
    let process_names: HashSet<String> = System::new_all()
        .processes()
        .values()
        .map(|p| p.name())
        .map(String::from)
        .collect();

    // "Returns true if self has no elements in common with other. This is
    // equivalent to checking for an empty intersection."
    //
    // https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.is_disjoint
    !APPLEBOOKS_NAMES.is_disjoint(&process_names)
}

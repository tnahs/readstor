//! Utilities for working with Apple Books.

use std::collections::HashSet;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use plist::Value;
use sysinfo::{System, SystemExt};

use crate::lib::applebooks;

/// Returns Apple Books' version as `v[short]-[long]` e.g. `v3.2-2217`.
///
/// * Returns `v?` if the Apple Books application cannot be found.
/// * Returns `v[short]-?`, `v?-[long]` or `v?-?` depending on what version
/// numbers can be located.
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

    let value = if let Ok(value) = Value::from_file(&path) {
        value
    } else {
        // This can happen if the user is on a non-macOS device.
        log::warn!("could not determine Apple Books version");
        return "v?".to_owned();
    };

    // -> 3.2
    let version_short = value
        .as_dictionary()
        .and_then(|d| d.get("CFBundleShortVersionString"))
        .and_then(plist::Value::as_string)
        .unwrap_or_else(|| {
            log::warn!("could not determine 'CFBundleShortVersionString'");
            "?"
        });

    // -> 2217
    let version_long = value
        .as_dictionary()
        .and_then(|d| d.get("CFBundleVersion"))
        .and_then(plist::Value::as_string)
        .unwrap_or_else(|| {
            log::warn!("could not determine 'CFBundleVersion'");
            "?"
        });

    // v3.2-2217
    format!("v{}-{}", version_short, version_long)
});

/// Returns a boolean based on if Apple Books is running or not.
#[must_use]
pub fn applebooks_is_running() -> bool {
    let process_names: HashSet<String> = System::new_all()
        .processes()
        .values()
        .map(sysinfo::ProcessExt::name)
        .map(String::from)
        .collect();

    // "Returns true if self has no elements in common with other. This is
    // equivalent to checking for an empty intersection."
    //
    // https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.is_disjoint
    !applebooks::defaults::APPLEBOOKS_NAMES.is_disjoint(&process_names)
}

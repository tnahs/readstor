//! Defines defaults for working with Apple Books for iOS.

use std::path::PathBuf;

use once_cell::sync::Lazy;

/// The root plists directory on an iOS device.
///
/// This is the full path to Apple Books data directory on an iOS device. The directory contains
/// both the `Books.plist` and `com.apple.ibooks-sync.plist` files.
///
/// The full path:
/// ```plaintext
/// /Books
/// ```
pub static DATA_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("Books"));

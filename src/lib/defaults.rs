use std::path::PathBuf;

use once_cell::sync::Lazy;

/// Defines the crates's root directory.
pub static CRATE_ROOT: Lazy<PathBuf> = Lazy::new(|| env!("CARGO_MANIFEST_DIR").into());

/// Defines the user's home directory.
//
//Unwrap should be safe here. It would only fail if the user is deleted after
//the process has started. Which is highly unlikely, and would be okay to panic
//if that was the case.
pub static HOME: Lazy<PathBuf> = Lazy::new(|| home::home_dir().unwrap());

/// TODO Document `YYYY-MM-DD-HHMMSS`
pub const DATE_FORMAT: &str = "%Y-%m-%d-%H%M%S";

/// Defines the default single template name.
pub const DEFAULT_TEMPLATE_FLAT_NAME: &str = "DEFAULT_TEMPLATE_FLAT_NAME";

/// Defines the default multi template name.
pub const DEFAULT_TEMPLATE_SPLIT_NAME: &str = "DEFAULT_TEMPLATE_SPLIT_NAME";

/// Defines the default single template.
pub const DEFAULT_TEMPLATE_FLAT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/default-single.md"
));

/// Defines the default multi template.
pub const DEFAULT_TEMPLATE_SPLIT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/default-multi.md"
));

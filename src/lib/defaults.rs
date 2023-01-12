//! Defines the defaults for this crate.

use std::path::PathBuf;

use once_cell::sync::Lazy;

/// The crates's root directory.
pub static CRATE_ROOT: Lazy<PathBuf> = Lazy::new(|| env!("CARGO_MANIFEST_DIR").into());

/// The user's home directory.
//
// Unwrap should be safe here. It would only fail if the user is deleted after
// the process has started. Which is highly unlikely, and would be okay to panic
// if that was the case.
pub static HOME: Lazy<PathBuf> = Lazy::new(|| home::home_dir().unwrap());

/// The default date format string. Translates to: `YYYY-MM-DD-HHMMSS` i.e.
/// `1970-01-01-120000`.
pub const DATE_FORMAT: &str = "%Y-%m-%d-%H%M%S";

/// Defines the root path to the default templates.
#[cfg(test)]
pub static EXAMPLE_TEMPLATES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = CRATE_ROOT.to_owned();
    path.push("templates");
    path
});

/// Defines the root path to the testing templates.
#[cfg(test)]
pub static TEST_TEMPLATES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = CRATE_ROOT.to_owned();
    path.extend(["data", "templates"].iter());
    path
});

/// A list of "smart" Unicode symbols and their ASCII eqivalents.
///
/// Based on the following:
///
/// * [Daring Fireball - SmartyPants](https://daringfireball.net/projects/smartypants/)
/// * [Python-Markdown - SmartyPants](https://python-markdown.github.io/extensions/smarty/)
pub static UNICODE_TO_ASCII_SYMBOLS: Lazy<Vec<(char, &str)>> = Lazy::new(|| {
    [
        ('‘', "'"),
        ('’', "'"),
        ('“', "\""),
        ('”', "\""),
        ('»', "<<"),
        ('«', ">>"),
        ('…', "..."),
        ('–', "--"),
        ('—', "---"),
    ]
    .into_iter()
    .collect()
});

//! Defines the defaults for this crate.

use std::path::PathBuf;

use once_cell::sync::Lazy;

/// The name of this package.
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// The crates's root directory.
pub static CRATE_ROOT: Lazy<PathBuf> = Lazy::new(|| env!("CARGO_MANIFEST_DIR").into());

/// The user's home directory.
//
// Unwrap should be safe here. It would only fail if the user is deleted after the process has
// started. Which is highly unlikely, and would be okay to panic if that was the case.
pub static HOME_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
    let path = std::env::var_os("HOME").expect("could not determine home directory");
    PathBuf::from(path)
});

/// Returns a path to a temp directory to use for reading and writing data during development/testing.
///
/// Internally this returns the value of the TMPDIR environment variable if it is set, otherwise it
/// returns `/tmp`. See [`std::env::temp_dir()`] for more information.
///
/// The full path:
///
/// ```plaintext
/// [temp_dir]/readstor/[name]
/// ```
///
/// For example:
///
/// ```plaintext
/// /var/folders/58/8yrgg8897ld633zt0qg9ww680000gn/T/readstor/
/// ```
pub static TEMP_OUTPUT_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| std::env::temp_dir().join(NAME));

/// Date format string for slugs. Translates to: `YYYY-MM-DD-HHMMSS` i.e. `1970-01-01-120000`.
pub const DATE_FORMAT_SLUG: &str = "%Y-%m-%d-%H%M%S";

/// Date format string for template `date` filter. Translates to: `YYYY-MM-DD` i.e. `1970-01-01`.
pub const DATE_FORMAT_TEMPLATE: &str = "%Y-%m-%d";

/// A list of "smart" Unicode symbols and their ASCII eqivalents.
///
/// Based on the following:
///
/// * [Daring Fireball - SmartyPants](https://daringfireball.net/projects/smartypants/)
/// * [Python-Markdown - SmartyPants](https://python-markdown.github.io/extensions/smarty/)
#[allow(clippy::doc_markdown)]
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

#[cfg(test)]
pub(crate) mod test {

    use super::*;

    /// Defines the root path to the example templates.
    pub static EXAMPLE_TEMPLATES_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = CRATE_ROOT.to_owned();
        path.push("templates");
        path
    });

    /// Defines the root path to the testing templates.
    ///
    /// The test templates are located at: [crate-root]/data/templates/[directory]/[filename]
    pub static TEST_TEMPLATES_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = CRATE_ROOT.to_owned();
        path.extend(["data", "templates"].iter());
        path
    });

    /// Defines the root path to the testing templates.
    #[derive(Debug, Copy, Clone)]
    #[allow(missing_docs)]
    pub enum TemplatesDirectory {
        ValidConfig,
        ValidContext,
        ValidFilter,
        ValidSyntax,
        InvalidConfig,
        InvalidContext,
        InvalidFilter,
        InvalidSyntax,
    }

    impl std::fmt::Display for TemplatesDirectory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ValidConfig => write!(f, "valid-config"),
                Self::ValidContext => write!(f, "valid-context"),
                Self::ValidFilter => write!(f, "valid-filter"),
                Self::ValidSyntax => write!(f, "valid-syntax"),
                Self::InvalidConfig => write!(f, "invalid-config"),
                Self::InvalidContext => write!(f, "invalid-context"),
                Self::InvalidFilter => write!(f, "invalid-filter"),
                Self::InvalidSyntax => write!(f, "invalid-syntax"),
            }
        }
    }

    impl From<TemplatesDirectory> for PathBuf {
        fn from(directory: TemplatesDirectory) -> Self {
            TEST_TEMPLATES_DIRECTORY.join(directory.to_string())
        }
    }
}

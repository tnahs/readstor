//! Defines the result and error types for this crate.

/// A generic result type.
pub type Result<T> = std::result::Result<T, Error>;

/// An enum representing all possible library errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error returned when the default Apple Books database cannot be found.
    #[error("missing default Apple Books databases")]
    MissingDefaultDatabase,

    /// Error returned when there are issues connecting to a database.
    #[error("unable to connect to '{name}*.sqlite' at {path}")]
    DatabaseConnection {
        /// The basename of the database: `BKLibrary` or `AEAnnotation`.
        name: String,
        /// The path to the database.
        path: String,
    },

    /// Error returned when the currently installed version of Apple Books for
    /// macOS is unsupported.
    ///
    /// This most likely means that the database schema is different than
    /// the one the query has been designed for. In that case, the currently
    /// installed version of Apple Books is considered unsupported.
    #[error("unsupported version of Apple Books for macOS: {version}")]
    UnsupportedMacosVersion {
        /// The currently installed Apple Books for macOS version number.
        version: String,
        /// The source error string.
        error: String,
    },

    /// Error returned when the currently installed version of Apple Books for
    /// iOS is unsupported.
    ///
    /// This most likely means that the plist schema is different than
    /// the one used for deserialization. In that case, the currently
    /// installed version of Apple Books for iOS  is considered unsupported.
    // TODO: Is there a way to retrieve the iOS version from the device?
    #[error("unsupported version of Apple Books for iOS: {error}")]
    UnsupportedIosVersion {
        /// The source error string.
        error: String,
    },

    /// Error returned when a syntax error is detected in how a template's
    /// config block is defined. This does not include YAML syntax error.
    #[error("cannot read config for: {path}")]
    InvalidTemplateConfig {
        /// The partial path to the template e.g. `nested/template.md`.
        path: String,
    },

    /// Error returned when a requested template-group does not exist.
    #[error("no template-group named: '{name}'")]
    NonexistentTemplateGroup {
        /// The name of the template-group.
        name: String,
    },

    /// Error returned if [`tera`][tera] encounters any errors.
    ///
    /// [tera]: https://docs.rs/tera/latest/tera/
    #[error(transparent)]
    InvalidTemplate(#[from] tera::Error),

    /// Error returned if [`serde_json`][serde-json] encounters any errors
    /// during serialization.
    ///
    /// [serde-json]: https://docs.rs/serde_json/latest/serde_json/
    #[error(transparent)]
    JsonSerializationError(#[from] serde_json::Error),

    /// Error returned if [`plist`][plist] encounters any errors during
    /// deserialization.
    ///
    /// [plist]: https://docs.rs/plist/latest/plist/
    #[error(transparent)]
    PlistDeserializationError(#[from] plist::Error),

    /// Error returned if [`serde_yaml`][serde-yaml] encounters any errors
    /// during deserialization.
    ///
    /// [serde-yaml]: https://docs.rs/serde_yaml/latest/serde_yaml/
    #[error(transparent)]
    YamlDeserializationError(#[from] serde_yaml::Error),

    /// Error returned if any other IO errors are encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

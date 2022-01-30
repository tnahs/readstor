pub type LibResult<T> = std::result::Result<T, LibError>;

#[derive(Debug, thiserror::Error)]
pub enum LibError {
    #[error("missing `{name}*.sqlite` in `{path}`")]
    DatabaseMissing { name: String, path: String },

    #[error("cannot resolve path to `{name}*.sqlite` in `{path}`")]
    DatabaseUnresolvable { name: String, path: String },

    #[error("unable to connect to `{name}*.sqlite` at `{path}`")]
    DatabaseConnection { name: String, path: String },

    #[error("Apple Books {version} unsupported")]
    UnsupportedVersion { version: String },

    #[error(transparent)]
    Template(#[from] tera::Error),

    #[error(transparent)]
    Serialization(#[from] serde_json::error::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

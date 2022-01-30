pub type Result<T> = std::result::Result<T, ApplicationError>;

#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("missing `{name}*.sqlite` in `{path}`")]
    DatabaseMissing { name: String, path: String },

    #[error("cannot resolve path to `{name}*.sqlite` in `{path}`")]
    DatabaseUnresolvable { name: String, path: String },

    #[error("unable to connect to `{name}*.sqlite` at `{path}`")]
    DatabaseConnection { name: String, path: String },

    #[error("database unsupported: Apple Books {version}")]
    DatabaseUnsupported { version: String },

    #[error(transparent)]
    Template(#[from] tera::Error),

    #[error(transparent)]
    Serialization(#[from] serde_json::error::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

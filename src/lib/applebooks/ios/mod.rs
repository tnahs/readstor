//! Defines types for interacting with iOS's Apple Books plists.

pub mod models;

use std::path::Path;

use crate::result::{Error, Result};

use self::models::{AnnotationRaw, AnnotationsPlist, BookRaw, BooksPlist};

/// A struct for interacting with iOS's Apple Books plists.
///
/// A directory containing iOS's Apple Books plists should conform to the
/// following structure:
///
/// ```plaintext
/// [plists]
///  │
///  ├── Books.plist
///  ├── com.apple.ibooks-sync.plist
///  └── ...
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ABIos;

impl ABIos {
    /// Extracts data from the books plist and converts them into `T`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing iOS's Apple Books plists.
    ///
    /// See [`ABIos`] for more information on how the databases directory
    /// should be structured.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The plist cannot be found/opened.
    /// * Any deserialization errors are encountered.
    /// * The version of Apple Books is unsupported.
    pub fn extract_books<T>(path: &Path) -> Result<Vec<T>>
    where
        T: From<BookRaw>,
    {
        let path = path.join(ABPlist::Books.to_string());

        let data: BooksPlist = match plist::from_file(path) {
            Ok(data) => data,
            Err(error) => {
                return Err(Error::UnsupportedIosVersion {
                    error: error.to_string(),
                })
            }
        };

        let books = data.books;

        Ok(books.into_iter().map(T::from).collect())
    }

    /// Extracts data from the annotations plist and converts them into `T`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing iOS's Apple Books plists.
    ///
    /// See [`ABIos`] for more information on how the databases directory
    /// should be structured.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The plist cannot be found/opened.
    /// * Any deserialization errors are encountered.
    /// * The version of Apple Books is unsupported.
    #[allow(clippy::missing_panics_doc)]
    pub fn extract_annotations<T>(path: &Path) -> Result<Vec<T>>
    where
        T: From<AnnotationRaw>,
    {
        let path = path.join(ABPlist::Annotations.to_string());

        let data: AnnotationsPlist = match plist::from_file(path) {
            Ok(data) => data,
            Err(error) => {
                return Err(Error::UnsupportedIosVersion {
                    error: error.to_string(),
                })
            }
        };

        // This should be safe as the structure of the incoming data is enforced
        // by `serde`. Therefore guaranteeing that the unwrap is safe. `serde`
        // would return an error in the previous block if the structure of the
        // plist didn't match the model used for deserializing it.
        let mut annotations = data.into_values().next().unwrap().bookmarks;

        // Filter out any deleted annotations.
        annotations.retain(|annotation| annotation.is_deleted == 0);

        Ok(annotations.into_iter().map(T::from).collect())
    }
}

/// An enum representing iOS's Apple Books plists.
#[derive(Debug, Clone, Copy)]
pub enum ABPlist {
    /// The books plist.
    Books,

    /// The annotations plist.
    Annotations,
}

impl std::fmt::Display for ABPlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ABPlist::Books => write!(f, "Books.plist"),
            ABPlist::Annotations => write!(f, "com.apple.ibooks-sync.plist"),
        }
    }
}

//! Defines types for deserializing iOS's Apple Books plists data.

use serde::Deserialize;
use std::collections::HashMap;

/// A struct representing the data structure of the book plist file.
///
/// The structure is as follows:
///
/// ```plaintext
/// "Books"
///  │
///  ├── Book
///  ├── Book
///  └── ...
/// ```
///
/// And represented as JSON:
///
/// ```json
/// {
///   "Books": [
///     {
///       "Package Hash": "07B4BBF8CB409B439C0B5F14622C32F4",
///       ...
///     },
///     {
///       "Package Hash": "9E6143AA0FAC031691359779729F9B37",
///       ...
///     },
///     ...
///   ]
/// }
/// ```
#[derive(Debug, Deserialize)]
pub(super) struct BooksPlist {
    #[serde(alias = "Books")]
    pub books: Vec<BookRaw>,
}

/// A struct representing a book and its metadata in the books plist file.
#[derive(Debug, Deserialize)]
pub struct BookRaw {
    #[serde(alias = "Artist")]
    #[allow(missing_docs)]
    pub author: String,

    #[serde(alias = "Name")]
    #[allow(missing_docs)]
    pub title: String,

    #[serde(alias = "Package Hash")]
    #[allow(missing_docs)]
    pub id: String,
}

/// A type alias representing the data structure of the annotations plist file.
///
/// The structure is as follows:
///
/// ```plaintext
/// "Bookmark-Container-XXXXXXXX"
///  │
///  ├── "Bookmarks"
///  │   ├── Annotation
///  │   ├── Annotation
///  │   └── ...
///  └── "Generation"
/// ```
///
/// And represented as JSON:
///
/// ```json
/// {
///   "Bookmark-Container-XXXXXXXX": {
///     "Bookmarks": [
///       {
///         "annotationUuid": "2244252D-6496-4BC2-87D0-72D5B3D86600",
///         ...
///       },
///       {
///         "annotationUuid": "BFE9ABA9-2F0E-42A8-8320-E1A30A79C5A8",
///         ...
///       },
///       ...
///     ],
///     "Generation": XXXXXXXXXX
///   }
/// }
/// ```
pub(super) type AnnotationsPlist = HashMap<String, Bookmarks>;

/// A struct representing an inner structure of the annotations plist file.
#[derive(Debug, Deserialize)]
pub(super) struct Bookmarks {
    #[serde(alias = "Bookmarks")]
    pub bookmarks: Vec<AnnotationRaw>,
}

/// A struct representing an annotation and its metadata in the annotations plist file.
#[derive(Debug, Deserialize)]
pub struct AnnotationRaw {
    #[serde(alias = "annotationSelectedText")]
    #[allow(missing_docs)]
    pub body: String,

    #[serde(alias = "annotationStyle")]
    #[allow(missing_docs)]
    pub style: usize,

    #[serde(alias = "annotationNote")]
    #[allow(missing_docs)]
    pub notes: Option<String>,

    #[serde(alias = "annotationUuid")]
    #[allow(missing_docs)]
    pub id: String,

    #[serde(alias = "annotationAssetID")]
    #[allow(missing_docs)]
    pub book_id: String,

    #[serde(alias = "annotationCreationDate")]
    #[allow(missing_docs)]
    pub created: f64,

    #[serde(alias = "annotationModificationDate")]
    #[allow(missing_docs)]
    pub modified: f64,

    #[serde(alias = "annotationLocation")]
    #[allow(missing_docs)]
    pub epubcfi: String,

    #[serde(alias = "annotationDeleted")]
    #[allow(missing_docs)]
    pub is_deleted: usize,
}

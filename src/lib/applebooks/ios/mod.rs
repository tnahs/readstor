//! Defines types for interacting with iOS's Apple Books plists.

pub mod defaults;
pub mod models;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use rusty_libimobiledevice::{idevice, services::afc::AfcFileMode};

use crate::result::{Error, Result};

use self::models::{AnnotationRaw, AnnotationsPlist, BookRaw, BooksPlist};

/// A struct for interacting with iOS's Apple Books plists.
///
/// A directory containing iOS's Apple Books plists should conform to the following structure:
///
/// ```plaintext
/// [plists]
///  │
///  ├── Books.plist
///  ├── com.apple.ibooks-sync.plist
///  └── ...
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ABIOs;

impl ABIOs {
    /// Extracts data from the books plist and converts them into `T`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing iOS's Apple Books plists.
    ///
    /// See [`ABIOs`] for more information on how the databases directory should be structured.
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
                return Err(Error::IOsUnsupportedAppleBooksVersion {
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
    /// See [`ABIOs`] for more information on how the databases directory should be structured.
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
                return Err(Error::IOsUnsupportedAppleBooksVersion {
                    error: error.to_string(),
                })
            }
        };

        // This should be safe as the structure of the incoming data is enforced by `serde`.
        // Therefore guaranteeing that the unwrap is safe. `serde` would return an error in
        // the previous block if the structure of the plist didn't match the model used for
        // deserializing it.
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

impl ABPlist {
    /// Copies iOS's Apple Books plists to a destination directory.
    ///
    /// # Arguments
    ///
    /// * `destination` - Where to copy the plists to.
    /// * `source` - An optional source plists directory. If no source is provided, this function
    ///   will attempt to access a connected iOS device and copy it from the default data location.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * Any IO errors are encountered.
    /// * There are any errors finding/reading the iOS device.
    pub fn save_to(destination: &Path, source: Option<&Path>) -> Result<()> {
        if let Some(source) = source {
            Self::save_from_disk(source, destination)?;
        } else {
            Self::save_from_device(destination, None)?;
            // TODO(feat): Implement UDID ------^^^^
        }

        log::debug!("saved iOS plists to: {destination:?}");

        Ok(())
    }

    /// Copies iOS's Apple Books plists from the host filesystem to a destination directory.
    ///
    /// # Arguments
    ///
    /// * `source` - Where to copy the plists from.
    /// * `destination` - Where to copy the plists to.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    fn save_from_disk(source: &Path, destination: &Path) -> Result<()> {
        for variant in &[Self::Books, Self::Annotations] {
            let name = variant.to_string();

            // -> [plists-directory]/[name]
            let item_source = source.join(&name);

            // -> [output-directory]/[name]
            let item_destination = destination.join(&name);

            std::fs::copy(item_source, item_destination)?;
        }

        Ok(())
    }

    /// Copies iOS's Apple Books plists from an iOS device filesystem to a destination directory.
    ///
    /// # Arguments
    ///
    /// * `destination` - Where to copy the plists to.
    /// * `udid` - An optional UDID to connect to a specific iOS device.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there are any errors finding/reading the iOS device.
    //
    // TODO(feat): Allow users to pass UDID from the CLI.
    fn save_from_device(destination: &Path, udid: Option<String>) -> Result<()> {
        let device = if let Some(udid) = udid {
            idevice::get_device(&udid).map_err(|_| Error::IOsDeviceNotFoundWithUdid { udid })?
        } else {
            idevice::get_first_device().map_err(|_| Error::IOsDeviceNotFound)?
        };

        let afc_client = device
            .new_afc_client(crate::defaults::NAME)
            .map_err(|error| Error::IOsDeviceReadError { error })?;

        std::fs::create_dir_all(destination)?;

        for variant in &[Self::Books, Self::Annotations] {
            let name = variant.to_string();

            let device_path = defaults::DATA_DIRECTORY.join(&name);
            let device_path = device_path.to_string_lossy().to_string();

            let file_handle = afc_client
                .file_open(&device_path, AfcFileMode::ReadOnly)
                .map_err(|error| Error::IOsDeviceReadError { error })?;

            let file_size = {
                let file_info = afc_client
                    .get_file_info(&device_path)
                    .map_err(|error| Error::IOsDeviceReadError { error })?;

                let size = file_info.get("st_size").ok_or_else(|| Error::OtherError {
                    error: "Unable to find 'st_size' field".to_owned(),
                })?;

                let size = size.parse::<u32>().map_err(|_| Error::OtherError {
                    error: "Failed to parse file size".to_owned(),
                })?;

                Ok::<u32, Error>(size)
            }
            .unwrap_or(u32::MAX);

            let file_contents = afc_client
                .file_read(file_handle, file_size)
                .map_err(|error| Error::IOsDeviceReadError { error })?;

            let host_path = destination.join(&name);

            let mut file = File::create(&host_path)?;

            file.write_all(&file_contents)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ABPlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Books => write!(f, "Books.plist"),
            Self::Annotations => write!(f, "com.apple.ibooks-sync.plist"),
        }
    }
}

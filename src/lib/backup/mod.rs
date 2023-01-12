//! Defines types for backing-up Apple Books' databases.

use std::path::Path;

use crate::applebooks::database::ABDatabaseName;
use crate::applebooks::utils::APPLEBOOKS_VERSION;
use crate::result::Result;

/// Backs-up Apple Books' databases to disk.
///
/// The output strucutre is as follows:
///
/// ```plaintext
/// [ouput-directory]
///  │
///  ├─ [YYYY-MM-DD-HHMMSS-VERSION]
///  │   │
///  │   ├─ AEAnnotation
///  │   │   ├─ AEAnnotation*.sqlite
///  │   │   └─ ...
///  │   │
///  │   └─ BKLibrary
///  │       ├─ BKLibrary*.sqlite
///  │       └─ ...
///  │
///  │─ [YYYY-MM-DD-HHMMSS-VERSION]
///  │   └─ ...
///  └─ ...
/// ```
/// # Arguments
///
/// * `databases` - The databases directory to back-up.
/// * `output` - The ouput directory.
///
/// See [`ABDatabase::get_database()`][abdatabase] for information on how the
/// `databases` directory should be structured.
///
/// # Errors
///
/// Will return `Err` if any IO errors are encountered.
///
/// [abdatabase]: crate::applebooks::database::ABDatabase
pub fn backup_databases(databases: &Path, output: &Path) -> Result<()> {
    // -> [YYYY-MM-DD-HHMMSS]-[VERSION]
    let today = format!("{}-{}", crate::utils::today(), *APPLEBOOKS_VERSION);

    // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]
    let destination_root = output.join(today);

    for name in &[
        ABDatabaseName::Books.to_string(),
        ABDatabaseName::Annotations.to_string(),
    ] {
        // -> [databases-directory]/[name]
        let source = databases.join(name.clone());
        // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]/[name]
        let destination = destination_root.join(name);

        crate::utils::copy_dir(source, destination)?;
    }

    Ok(())
}

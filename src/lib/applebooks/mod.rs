//! Defines types for interacting the Apple Books' data.

pub mod ios;
pub mod macos;

/// An enum representing the two platforms Apple Books is available on.
#[derive(Debug, Clone, Copy)]
pub enum Platform {
    /// macOS
    MacOs,

    /// iOS
    IOs,
}

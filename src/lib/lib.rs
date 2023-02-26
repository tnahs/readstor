//! Defines types used for interacting with Apple Books.

#![warn(
    clippy::all,
    clippy::pedantic,
    // clippy::missing_docs_in_private_items,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
)]
#![allow(
    rustdoc::private_intra_doc_links,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions
)]

pub mod applebooks;
pub mod backup;
pub mod contexts;
pub mod defaults;
pub mod export;
pub mod filter;
pub mod models;
pub mod process;
pub mod render;
pub mod result;
pub mod utils;

//! Defines a set of structs, traits and functions used to interact with,
//! extract and export and render data from Apple Books.

#![warn(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility
)]
#![allow(
    rustdoc::private_intra_doc_links,
    clippy::module_name_repetitions,
    // Produces some false positives in docs.
    clippy::doc_markdown,
    // TODO: How is this fixed?
    clippy::multiple_crate_versions,
)]

pub mod applebooks;
pub mod defaults;
pub mod filter;
pub mod models;
pub mod processor;
pub mod result;
pub mod templates;
pub mod utils;

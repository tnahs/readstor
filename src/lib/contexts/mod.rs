//! Defines types injected into templates.
//!
//! The primary purpose for these types is to provide a clean separation
//! between the data that is stored and the data that is injected into a
//! template. This allows us to append or derive extra data onto each type that
//! is specific only to the template context. For example, slugified strings
//! are added to the [`Book`][book] and [`Annotation`][annotation] types for
//! use within templates.
//!
//! [annotation]: crate::models::annotation::Annotation
//! [book]: crate::models::book::Book

pub mod annotation;
pub mod book;
pub mod entry;

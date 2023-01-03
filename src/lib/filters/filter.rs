//! Defines functions for filtering [`Entry`][entry]s based on a criteria.
//!
//! [entry]: crate::models::entry::Entry

use std::collections::BTreeSet;

use crate::models::data::Entries;

/// Filters out [`Entry`][entry]s which have no [`Annotation`][annotation]s.
///
/// # Arguments
///
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [annotation]: crate::models::annotation::Annotation
/// [entry]: crate::models::entry::Entry
pub fn contains_no_annotations(entries: &mut Entries) {
    entries.retain(|_, entry| !entry.annotations.is_empty());
}

/// Filters out [`Entry`][entry]s where their [`Book::title`][book] doesn't
/// match any of the queries.
///
/// # Arguments
///
/// * `queries`: A list of strings to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [book]: crate::models::book::Book::title
/// [entry]: crate::models::entry::Entry
pub fn by_title_any(queries: &[String], entries: &mut Entries) {
    entries.retain(|_, entry| {
        queries
            .iter()
            .any(|query| entry.book.title.to_lowercase().contains(query))
    });
}

/// Filters out [`Entry`][entry]s where their [`Book::title`][book] doesn't
/// match all of the queries.
///
/// # Arguments
///
/// * `queries`: A list of strings to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [book]: crate::models::book::Book::title
/// [entry]: crate::models::entry::Entry
pub fn by_title_all(queries: &[String], entries: &mut Entries) {
    entries.retain(|_, entry| {
        queries
            .iter()
            .all(|query| entry.book.title.to_lowercase().contains(query))
    });
}

/// Filters out [`Entry`][entry]s where their [`Book::title`][book] does't
/// exactly match the query.
///
/// # Arguments
///
/// * `query`: A strings to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [book]: crate::models::book::Book::title
/// [entry]: crate::models::entry::Entry
pub fn by_title_exact(query: &str, entries: &mut Entries) {
    entries.retain(|_, entry| entry.book.title.to_lowercase() == query);
}

/// Filters out [`Entry`][entry]s where their [`Book::author`][author] doesn't
/// match any of the queries.
///
/// # Arguments
///
/// * `queries`: A list of strings to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [author]: crate::models::book::Book::author
/// [entry]: crate::models::entry::Entry
pub fn by_author_any(query: &[String], entries: &mut Entries) {
    entries.retain(|_, entry| {
        query
            .iter()
            .any(|q| entry.book.author.to_lowercase().contains(q))
    });
}

/// Filters out [`Entry`][entry]s where their [`Book::author`][author] doesn't
/// match all of the queries.
///
/// # Arguments
///
/// * `queries`: A list of strings to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [author]: crate::models::book::Book::author
/// [entry]: crate::models::entry::Entry
pub fn by_author_all(query: &[String], entries: &mut Entries) {
    entries.retain(|_, entry| {
        query
            .iter()
            .all(|q| entry.book.author.to_lowercase().contains(q))
    });
}

/// Filters out [`Entry`][entry]s where their [`Book::author`][author] does't
/// exactly match the query.
///
/// # Arguments
///
/// * `query`: A strings to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [author]: crate::models::book::Book::author
/// [entry]: crate::models::entry::Entry
pub fn by_author_exact(query: &str, entries: &mut Entries) {
    entries.retain(|_, entry| entry.book.author.to_lowercase() == query);
}

/// Filters out [`Annotation`][annotation]s where their [`tags`][tags] don't
/// match any of the target `#tags`.
///
/// # Arguments
///
/// * `tags`: A list of `#tags` to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [annotation]: crate::models::annotation::Annotation
/// [entry]: crate::models::entry::Entry
/// [tags]: crate::models::annotation::Annotation::tags
pub fn by_tags_any(tags: &BTreeSet<&String>, entries: &mut Entries) {
    for entry in entries.values_mut() {
        entry
            .annotations
            .retain(|annotation| tags.iter().any(|tag| annotation.tags.contains(*tag)));
    }
}

/// Filters out [`Annotation`][annotation]s where their [`tags`][tags] don't
/// match all of the target `#tags`.
///
/// # Arguments
///
/// * `tags`: A list of `#tags` to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [annotation]: crate::models::annotation::Annotation
/// [entry]: crate::models::entry::Entry
/// [tags]: crate::models::annotation::Annotation::tags
pub fn by_tags_all(tags: &BTreeSet<&String>, entries: &mut Entries) {
    for entry in entries.values_mut() {
        entry
            .annotations
            .retain(|annotation| tags.iter().all(|tag| annotation.tags.contains(*tag)));
    }
}

/// Filters out [`Annotation`][annotation]s where their [`tags`][tags] don't
/// exactly match the target `#tags`.
///
/// # Arguments
///
/// * `tags`: A list of `#tags` to filter against.
/// * `entries`: The [`Entry`][entry]s to filter.
///
/// [annotation]: crate::models::annotation::Annotation
/// [entry]: crate::models::entry::Entry
/// [tags]: crate::models::annotation::Annotation::tags
pub fn by_tags_exact(tags: &BTreeSet<&String>, entries: &mut Entries) {
    let tags = tags.iter().map(std::string::ToString::to_string).collect();

    for entry in entries.values_mut() {
        entry
            .annotations
            .retain(|annotation| annotation.tags == tags);
    }
}

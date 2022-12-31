//! Defines types for filtering out [`Entry`][entry]s.
//!
//! [entry]: crate::models::entry::Entry

use crate::models::data::Entries;

/// A struct for filtering [`Entry`][entry]s.
///
/// [entry]: crate::models::entry::Entry
#[derive(Debug, Clone, Copy)]
pub struct FilterRunner;

impl FilterRunner {
    /// Runs filters on all [`Entry`][entry]s.
    ///
    /// # Arguments
    ///
    /// * `filter_type` - The type of filter to run.
    /// * `entries` - The [`Entry`][entry]s to filter.
    ///
    /// [entry]: crate::models::entry::Entry
    pub fn run<F>(filter_type: F, entries: &mut Entries)
    where
        F: Into<FilterType>,
    {
        let filter_type = filter_type.into();

        match filter_type {
            FilterType::Title { queries, operator } => {
                FilterRunner::filter_by_title(&queries, operator, entries);
            }
            FilterType::Author { queries, operator } => {
                FilterRunner::filter_by_author(&queries, operator, entries);
            }
            FilterType::Tags { queries, operator } => {
                FilterRunner::filter_by_tags(&queries, operator, entries);
            }
        }

        // Remove `Entry`s that have no `Annotation`s.
        entries.retain(|_, entry| !entry.annotations.is_empty());
    }

    fn filter_by_title(queries: &[String], operator: FilterOperator, entries: &mut Entries) {
        match operator {
            FilterOperator::Any => {
                entries.retain(|_, entry| {
                    queries
                        .iter()
                        .any(|query| entry.book.title.to_lowercase().contains(query))
                });
            }
            FilterOperator::All => {
                entries.retain(|_, entry| {
                    queries
                        .iter()
                        .all(|query| entry.book.title.to_lowercase().contains(query))
                });
            }
            FilterOperator::Exact => {
                entries.retain(|_, entry| entry.book.title.to_lowercase() == queries.join(" "));
            }
        }
    }

    fn filter_by_author(queries: &[String], operator: FilterOperator, entries: &mut Entries) {
        match operator {
            FilterOperator::Any => {
                entries.retain(|_, entry| {
                    queries
                        .iter()
                        .any(|query| entry.book.author.to_lowercase().contains(query))
                });
            }
            FilterOperator::All => {
                entries.retain(|_, entry| {
                    queries
                        .iter()
                        .all(|query| entry.book.author.to_lowercase().contains(query))
                });
            }
            FilterOperator::Exact => {
                entries.retain(|_, entry| entry.book.author.to_lowercase() == queries.join(" "));
            }
        }
    }

    fn filter_by_tags(queries: &[String], operator: FilterOperator, entries: &mut Entries) {
        let tags = queries;

        match operator {
            FilterOperator::Any => {
                for entry in entries.values_mut() {
                    entry
                        .annotations
                        .retain(|annotation| tags.iter().any(|tag| annotation.tags.contains(tag)));
                }
            }
            FilterOperator::All => {
                for entry in entries.values_mut() {
                    entry
                        .annotations
                        .retain(|annotation| tags.iter().all(|tag| annotation.tags.contains(tag)));
                }
            }
            FilterOperator::Exact => {
                for entry in entries.values_mut() {
                    entry
                        .annotations
                        .retain(|annotation| annotation.tags == tags);
                }
            }
        }
    }
}

/// An enum representing possible filters.
///
/// A filter generally consists of three elements: (1) the field to use for
/// filtering, (2) a list of queries and (3) a [`FilterOperator`] to determine
/// how to handle the queries.
#[derive(Debug, Clone)]
pub enum FilterType {
    /// Sets the filter to use the [`Book::title`][book] field for filtering.
    ///
    /// [book]: crate::models::book::Book::title
    Title {
        /// The filter queries.
        queries: Vec<String>,
        /// The filter operator.
        operator: FilterOperator,
    },

    /// Sets the filter to use the [`Book::author`][book] field for filtering.
    ///
    /// [book]: crate::models::book::Book::author
    Author {
        /// The filter queries.
        queries: Vec<String>,
        /// The filter operator.
        operator: FilterOperator,
    },

    /// Sets the filter to use the [`Annotation::tags`][annotation] field for
    /// filtering.
    ///
    /// [annotation]: crate::models::annotation::Annotation::tags
    Tags {
        /// The filter queries.
        queries: Vec<String>,
        /// The filter operator.
        operator: FilterOperator,
    },
}

/// An enum representing possible filter operators.
///
/// See [`FilterType`] for more information.
#[derive(Debug, Clone, Copy, Default)]
pub enum FilterOperator {
    /// Sets the filter to return `true` if any of the query strings match.
    #[default]
    Any,

    /// Sets the filter to return `true` if all of the query strings match.
    All,

    /// Sets the filter to return `true` if the query string is an exact match.
    Exact,
}

#[cfg(test)]
mod test_filters {

    use std::collections::HashMap;

    use crate::models::annotation::Annotation;
    use crate::models::book::Book;
    use crate::models::entry::Entry;

    use super::*;

    fn create_test_entries() -> Entries {
        let annotations = vec![
            Annotation {
                tags: vec!["#tag01".to_string()],
                ..Default::default()
            },
            Annotation {
                tags: vec!["#tag02".to_string()],
                ..Default::default()
            },
            Annotation {
                tags: vec!["#tag03".to_string()],
                ..Default::default()
            },
            Annotation {
                tags: vec![
                    "#tag01".to_string(),
                    "#tag02".to_string(),
                    "#tag03".to_string(),
                ],
                ..Default::default()
            },
        ];

        let entry_00 = Entry {
            book: Book {
                title: "Book One".to_string(),
                author: "Author One".to_string(),
                ..Default::default()
            },
            annotations: annotations.clone(),
        };

        let entry_01 = Entry {
            book: Book {
                title: "Book Two: The Return".to_string(),
                author: "Author No. Two".to_string(),
                ..Default::default()
            },
            annotations,
        };

        let mut data = HashMap::new();
        data.insert("00".to_string(), entry_00);
        data.insert("01".to_string(), entry_01);

        data
    }

    // Title

    #[test]
    fn test_title_any() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Title {
                queries: vec!["book".to_string()],
                operator: FilterOperator::Any,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 2);
        assert_eq!(annotations, 8);
    }

    #[test]
    fn test_title_all() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Title {
                queries: vec!["two".to_string(), "return".to_string()],
                operator: FilterOperator::All,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 4);
    }

    #[test]
    fn test_title_exact() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Title {
                queries: vec!["book".to_string(), "one".to_string()],
                operator: FilterOperator::Exact,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 4);
    }

    // Author

    #[test]
    fn test_author_any() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Author {
                queries: vec!["author".to_string()],
                operator: FilterOperator::Any,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 2);
        assert_eq!(annotations, 8);
    }

    #[test]
    fn test_author_all() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Author {
                queries: vec!["author".to_string(), "no.".to_string()],
                operator: FilterOperator::All,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 4);
    }

    #[test]
    fn test_author_exact() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Author {
                queries: vec!["author".to_string(), "no.".to_string(), "two".to_string()],
                operator: FilterOperator::Exact,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 4);
    }

    // Tags

    #[test]
    fn test_tags_any() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Tags {
                queries: vec!["#tag01".to_string(), "#tag03".to_string()],
                operator: FilterOperator::Any,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 2);
        assert_eq!(annotations, 6);
    }

    #[test]
    fn test_tags_all() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Tags {
                queries: vec!["#tag01".to_string(), "#tag03".to_string()],
                operator: FilterOperator::All,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 2);
        assert_eq!(annotations, 2);
    }

    #[test]
    fn test_tags_exact() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Tags {
                queries: vec![
                    "#tag01".to_string(),
                    "#tag02".to_string(),
                    "#tag03".to_string(),
                ],
                operator: FilterOperator::Exact,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 2);
        assert_eq!(annotations, 2);
    }

    // Multi

    #[test]
    fn test_multi() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::Title {
                queries: vec!["one".to_string()],
                operator: FilterOperator::Any,
            },
            &mut entries,
        );

        FilterRunner::run(
            FilterType::Author {
                queries: vec!["author".to_string(), "one".to_string()],
                operator: FilterOperator::Exact,
            },
            &mut entries,
        );

        FilterRunner::run(
            FilterType::Tags {
                queries: vec!["#tag02".to_string()],
                operator: FilterOperator::Exact,
            },
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 1);
    }
}

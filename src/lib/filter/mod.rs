//! Defines types for filtering out [`Entry`][entry]s.
//!
//! [entry]: crate::models::entry::Entry

use std::collections::BTreeSet;

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
            FilterType::Title { query, operator } => {
                FilterRunner::filter_by_title(&query, operator, entries);
            }
            FilterType::Author { query, operator } => {
                FilterRunner::filter_by_author(&query, operator, entries);
            }
            FilterType::Tags { query, operator } => {
                FilterRunner::filter_by_tags(&query, operator, entries);
            }
        }

        // Remove `Entry`s that have no `Annotation`s.
        entries.retain(|_, entry| !entry.annotations.is_empty());
    }

    fn filter_by_title(query: &[String], operator: FilterOperator, entries: &mut Entries) {
        match operator {
            FilterOperator::Any => {
                entries.retain(|_, entry| {
                    query
                        .iter()
                        .any(|q| entry.book.title.to_lowercase().contains(q))
                });
            }
            FilterOperator::All => {
                entries.retain(|_, entry| {
                    query
                        .iter()
                        .all(|q| entry.book.title.to_lowercase().contains(q))
                });
            }
            FilterOperator::Exact => {
                entries.retain(|_, entry| entry.book.title.to_lowercase() == query.join(" "));
            }
        }
    }

    fn filter_by_author(query: &[String], operator: FilterOperator, entries: &mut Entries) {
        match operator {
            FilterOperator::Any => {
                entries.retain(|_, entry| {
                    query
                        .iter()
                        .any(|q| entry.book.author.to_lowercase().contains(q))
                });
            }
            FilterOperator::All => {
                entries.retain(|_, entry| {
                    query
                        .iter()
                        .all(|q| entry.book.author.to_lowercase().contains(q))
                });
            }
            FilterOperator::Exact => {
                entries.retain(|_, entry| entry.book.author.to_lowercase() == query.join(" "));
            }
        }
    }

    fn filter_by_tags(query: &[String], operator: FilterOperator, entries: &mut Entries) {
        let tags: BTreeSet<String> = query.iter().cloned().collect();

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
        /// The filter query.
        query: Vec<String>,
        /// The filter operator.
        operator: FilterOperator,
    },

    /// Sets the filter to use the [`Book::author`][book] field for filtering.
    ///
    /// [book]: crate::models::book::Book::author
    Author {
        /// The filter query.
        query: Vec<String>,
        /// The filter operator.
        operator: FilterOperator,
    },

    /// Sets the filter to use the [`Annotation::tags`][annotation] field for
    /// filtering.
    ///
    /// [annotation]: crate::models::annotation::Annotation::tags
    Tags {
        /// The filter query.
        query: Vec<String>,
        /// The filter operator.
        operator: FilterOperator,
    },
}

#[cfg(test)]
impl FilterType {
    fn title(query: &[&str], operator: FilterOperator) -> Self {
        Self::Title {
            query: query.iter().map(std::string::ToString::to_string).collect(),
            operator,
        }
    }

    fn author(query: &[&str], operator: FilterOperator) -> Self {
        Self::Author {
            query: query.iter().map(std::string::ToString::to_string).collect(),
            operator,
        }
    }

    fn tags(query: &[&str], operator: FilterOperator) -> Self {
        Self::Tags {
            query: query.iter().map(std::string::ToString::to_string).collect(),
            operator,
        }
    }
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
                tags: create_test_tags_from_str(&["#tag01"]),
                ..Default::default()
            },
            Annotation {
                tags: create_test_tags_from_str(&["#tag02"]),
                ..Default::default()
            },
            Annotation {
                tags: create_test_tags_from_str(&["#tag03"]),
                ..Default::default()
            },
            Annotation {
                tags: create_test_tags_from_str(&["#tag01", "#tag02", "#tag03"]),
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

    fn create_test_tags_from_str(tags: &[&str]) -> BTreeSet<String> {
        tags.iter().map(std::string::ToString::to_string).collect()
    }

    // Title

    #[test]
    fn test_title_any() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::title(&["book"], FilterOperator::Any),
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
            FilterType::title(&["two", "return"], FilterOperator::All),
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
            FilterType::title(&["book", "one"], FilterOperator::Exact),
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
            FilterType::author(&["author"], FilterOperator::Any),
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
            FilterType::author(&["author", "no."], FilterOperator::All),
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
            FilterType::author(&["author", "no.", "two"], FilterOperator::Exact),
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
            FilterType::tags(&["#tag01", "#tag03"], FilterOperator::Any),
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
            FilterType::tags(&["#tag01", "#tag03"], FilterOperator::All),
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
            FilterType::tags(&["#tag01", "#tag02", "#tag03"], FilterOperator::Exact),
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
    fn test_tags_exact_different_order() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::tags(&["#tag03", "#tag02", "#tag01"], FilterOperator::Exact),
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
            FilterType::title(&["one"], FilterOperator::Any),
            &mut entries,
        );

        FilterRunner::run(
            FilterType::author(&["author", "one"], FilterOperator::Exact),
            &mut entries,
        );

        FilterRunner::run(
            FilterType::tags(&["#tag02"], FilterOperator::Exact),
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

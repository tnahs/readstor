//! Defines types for filtering.

pub mod filters;

use std::collections::BTreeSet;

use crate::models::entry::Entries;

/// A struct for running filters on [`Entry`][entry]s.
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
        let filter_type: FilterType = filter_type.into();

        match filter_type {
            FilterType::Title { query, operator } => {
                Self::filter_by_title(&query, operator, entries);
            }
            FilterType::Author { query, operator } => {
                Self::filter_by_author(&query, operator, entries);
            }
            FilterType::Tags { query, operator } => {
                Self::filter_by_tags(&query, operator, entries);
            }
        }

        // Remove `Entry`s that have had all their `Annotation`s filtered out.
        filters::contains_no_annotations(entries);
    }

    /// Filters out [`Entry`][entry]s by their [`Book::title`][book].
    ///
    /// # Arguments
    ///
    /// * `query` - A list of strings to filter against.
    /// * `operator` - The [`FilterOperator`] to use.
    /// * `entries` - The [`Entry`][entry]s to filter.
    ///
    /// [book]: crate::models::book::Book::title
    /// [entry]: crate::models::entry::Entry
    fn filter_by_title(query: &[String], operator: FilterOperator, entries: &mut Entries) {
        match operator {
            FilterOperator::Any => filters::by_title_any(query, entries),
            FilterOperator::All => filters::by_title_all(query, entries),
            FilterOperator::Exact => filters::by_title_exact(&query.join(" "), entries),
        }
    }

    /// Filters out [`Entry`][entry]s by their [`Book::author`][book].
    ///
    /// # Arguments
    ///
    /// * `query` - A list of strings to filter against.
    /// * `operator` - The [`FilterOperator`] to use.
    /// * `entries` - The [`Entry`][entry]s to filter.
    ///
    /// [book]: crate::models::book::Book::author
    /// [entry]: crate::models::entry::Entry
    fn filter_by_author(query: &[String], operator: FilterOperator, entries: &mut Entries) {
        match operator {
            FilterOperator::Any => filters::by_author_any(query, entries),
            FilterOperator::All => filters::by_author_all(query, entries),
            FilterOperator::Exact => filters::by_author_exact(&query.join(" "), entries),
        }
    }

    /// Filters out [`Entry`][entry]s by their [`tags`][tags].
    ///
    /// # Arguments
    ///
    /// * `query` - A list of strings to filter against.
    /// * `operator` - The [`FilterOperator`] to use.
    /// * `entries` - The [`Entry`][entry]s to filter.
    ///
    /// [entry]: crate::models::entry::Entry
    /// [tags]: crate::models::annotation::Annotation::tags
    fn filter_by_tags(query: &[String], operator: FilterOperator, entries: &mut Entries) {
        let tags = BTreeSet::from_iter(query);

        match operator {
            FilterOperator::Any => filters::by_tags_any(&tags, entries),
            FilterOperator::All => filters::by_tags_all(&tags, entries),
            FilterOperator::Exact => filters::by_tags_exact(&tags, entries),
        }
    }
}

/// An enum representing possible filter types.
///
/// A filter generally consists of three elements: (1) the field to use for filtering, (2) a list of
/// queries and (3) a [`FilterOperator`] to determine how to handle the queries.
#[derive(Debug, Clone)]
pub enum FilterType {
    /// Sets the filter to use the [`Book::title`][book] field for filtering.
    ///
    /// [book]: crate::models::book::Book::title
    Title {
        #[allow(missing_docs)]
        query: Vec<String>,
        #[allow(missing_docs)]
        operator: FilterOperator,
    },

    /// Sets the filter to use the [`Book::author`][book] field for filtering.
    ///
    /// [book]: crate::models::book::Book::author
    Author {
        #[allow(missing_docs)]
        query: Vec<String>,
        #[allow(missing_docs)]
        operator: FilterOperator,
    },

    /// Sets the filter to use the [`Annotation::tags`][annotation] field for filtering.
    ///
    /// [annotation]: crate::models::annotation::Annotation::tags
    Tags {
        #[allow(missing_docs)]
        query: Vec<String>,
        #[allow(missing_docs)]
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
    /// Sets the filter to check if any of the queries match.
    #[default]
    Any,

    /// Sets the filter to check if all of the queries match.
    All,

    /// Sets the filter to check if the query string is an exact match.
    Exact,
}

#[cfg(test)]
mod test_filter {

    use super::*;

    use std::collections::HashMap;

    use crate::models::annotation::Annotation;
    use crate::models::book::Book;
    use crate::models::entry::Entry;

    fn create_test_entries() -> Entries {
        let annotations = vec![
            Annotation {
                tags: create_test_tags(&["#tag01"]),
                ..Default::default()
            },
            Annotation {
                tags: create_test_tags(&["#tag02"]),
                ..Default::default()
            },
            Annotation {
                tags: create_test_tags(&["#tag03"]),
                ..Default::default()
            },
            Annotation {
                tags: create_test_tags(&["#tag01", "#tag02", "#tag03"]),
                ..Default::default()
            },
        ];

        let entry_00 = Entry {
            book: Book {
                title: "Incididunt Sint".to_string(),
                author: "Quis Sint".to_string(),
                ..Default::default()
            },
            annotations: annotations.clone(),
        };

        // Laboris Incididunt Esse Commodo Do Tempor Ut
        // Lorem aliqua do ex cillum
        let entry_01 = Entry {
            book: Book {
                title: "Laboris Ex Cillum".to_string(),
                author: "Lorem Du Quis".to_string(),
                ..Default::default()
            },
            annotations,
        };

        let mut data = HashMap::new();
        data.insert("00".to_string(), entry_00);
        data.insert("01".to_string(), entry_01);

        data
    }

    fn create_test_tags(tags: &[&str]) -> BTreeSet<String> {
        tags.iter().map(std::string::ToString::to_string).collect()
    }

    // Keeps annotations where their book's title contains "incididunt" or "laboris".
    #[test]
    fn title_any() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::title(&["incididunt", "laboris"], FilterOperator::Any),
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 2);
        assert_eq!(annotations, 8);
    }

    // Keeps annotations where their book's title contains both "laboris" and "cillum".
    #[test]
    fn title_all() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::title(&["laboris", "cillum"], FilterOperator::All),
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 4);
    }

    // Keeps annotations where their book's title is exactly "incididunt sint".
    #[test]
    fn title_exact() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::title(&["incididunt", "sint"], FilterOperator::Exact),
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 4);
    }

    // Keeps annotations where their book's author contains "quis".
    #[test]
    fn author_any() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::author(&["quis"], FilterOperator::Any),
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 2);
        assert_eq!(annotations, 8);
    }

    // Keeps annotations where their book's author contains both "lorem" and "sint".
    #[test]
    fn author_all() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::author(&["lorem", "sint"], FilterOperator::All),
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 0);
        assert_eq!(annotations, 0);
    }

    // Keeps annotations where their book's author is exactly "lorem du quis".
    #[test]
    fn author_exact() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::author(&["lorem", "du", "quis"], FilterOperator::Exact),
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 4);
    }

    // Keeps annotations where their tags contain "#tag01" or "#tag03".
    #[test]
    fn tags_any() {
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

    // Keeps annotations where their tags contain both "#tag01" and "#tag03".
    #[test]
    fn tags_all() {
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

    // Keeps annotations where their tags contain exactly "#tag01", "#tag02" and "#tag03".
    #[test]
    fn tags_exact() {
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

    // Tests that tag declaration order doesn't matter when performing exact match filtering.
    #[test]
    fn tags_exact_different_order() {
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

    // Tests that multiple filters produce the expected result.
    #[test]
    fn multi() {
        let mut entries = create_test_entries();

        FilterRunner::run(
            FilterType::title(&["sint"], FilterOperator::Any),
            &mut entries,
        );

        FilterRunner::run(
            FilterType::author(&["quis", "sint"], FilterOperator::Exact),
            &mut entries,
        );

        FilterRunner::run(
            FilterType::tags(&["#tag02"], FilterOperator::Any),
            &mut entries,
        );

        let annotations = entries
            .values()
            .flat_map(|entry| &entry.annotations)
            .count();

        assert_eq!(entries.len(), 1);
        assert_eq!(annotations, 2);
    }
}

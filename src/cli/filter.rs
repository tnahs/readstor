use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

static RE_FILTER_QUERY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<operator>[?*=]?)(?P<field>\w*):(?P<query>.*)$").unwrap()
    //            └───┬──────────────┘└───────────┬┘ └───┬───────┘
    //                │                           │      │
    // operator ──────┘                           │      │
    //   Captures a single char representing the  │      │
    //   filter operator. Can be one of:          │      │
    //     - "?" -> any                           │      │
    //     - "*" -> all                           │      │
    //     - "=" -> exact                         │      │
    //                                            │      │
    // field ─────────────────────────────────────┘      │
    //   The field used to run filtering.                │
    //                                                   │
    // query ────────────────────────────────────────────┘
    //   The query string.
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FilterType {
    /// Filter books by their title
    Title {
        query: Vec<String>,
        operator: FilterOperator,
    },

    /// Filter books by their author
    Author {
        query: Vec<String>,
        operator: FilterOperator,
    },

    /// Filter annotations by their #tags
    Tags {
        query: Vec<String>,
        operator: FilterOperator,
    },
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum FilterOperator {
    /// Match any of the query strings
    #[default]
    Any,

    /// Match all of the query strings
    All,

    /// Match the exact query string
    Exact,
}

impl FromStr for FilterType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_FILTER_QUERY.captures(s);

        let Some(captures) = captures else {
            return Err("filters must follow the format '[op]{field}:{query}'".into());
        };

        // These unwraps are safe as they will only panic if the capture-group name does not exist.
        // These are all defined above.
        let operator = captures.name("operator").unwrap().as_str();
        let field = captures.name("field").unwrap().as_str().to_lowercase();
        let query = captures.name("query").unwrap();

        let operator = if operator.is_empty() {
            FilterOperator::default()
        } else {
            operator.parse()?
        };

        let query = query
            .as_str()
            .split(' ')
            .map(std::string::ToString::to_string)
            .collect();

        let filter_by = match field.as_str() {
            "title" => Self::Title { query, operator },
            "author" => Self::Author { query, operator },
            "tags" | "tag" => Self::Tags { query, operator },
            _ => return Err(format!("invalid field: '{field}'")),
        };

        Ok(filter_by)
    }
}

impl FromStr for FilterOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let filter_type = match s {
            "?" => Self::Any,
            "*" => Self::All,
            "=" => Self::Exact,
            _ => return Err(format!("invalid operator: '{s}'")),
        };

        Ok(filter_type)
    }
}

impl From<FilterOperator> for lib::filter::FilterOperator {
    fn from(filter_operator: FilterOperator) -> Self {
        match filter_operator {
            FilterOperator::Any => Self::Any,
            FilterOperator::All => Self::All,
            FilterOperator::Exact => Self::Exact,
        }
    }
}

impl From<FilterType> for lib::filter::FilterType {
    fn from(filter_type: FilterType) -> Self {
        match filter_type {
            FilterType::Title { query, operator } => Self::Title {
                query,
                operator: operator.into(),
            },
            FilterType::Author { query, operator } => Self::Author {
                query,
                operator: operator.into(),
            },
            FilterType::Tags { query, operator } => Self::Tags {
                query,
                operator: operator.into(),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    // Tests that strings are properly converted into `FilterType`s.
    mod parse_filter {

        use super::*;

        #[test]
        fn title_any() {
            assert_eq!(
                FilterType::from_str("?title:art think").unwrap(),
                FilterType::Title {
                    query: vec!["art".to_string(), "think".to_string()],
                    operator: FilterOperator::Any,
                }
            );
        }

        #[test]
        fn title_all() {
            assert_eq!(
                FilterType::from_str("*title:joking feynman").unwrap(),
                FilterType::Title {
                    query: vec!["joking".to_string(), "feynman".to_string()],
                    operator: FilterOperator::All,
                }
            );
        }

        #[test]
        fn title_exact() {
            assert_eq!(
                FilterType::from_str("=title:the art spirit").unwrap(),
                FilterType::Title {
                    query: vec!["the".to_string(), "art".to_string(), "spirit".to_string()],
                    operator: FilterOperator::Exact,
                }
            );
        }

        #[test]
        fn author_any() {
            assert_eq!(
                FilterType::from_str("?author:robert richard").unwrap(),
                FilterType::Author {
                    query: vec!["robert".to_string(), "richard".to_string()],
                    operator: FilterOperator::Any,
                }
            );
        }

        #[test]
        fn author_all() {
            assert_eq!(
                FilterType::from_str("*author:richard feynman").unwrap(),
                FilterType::Author {
                    query: vec!["richard".to_string(), "feynman".to_string()],
                    operator: FilterOperator::All,
                }
            );
        }

        #[test]
        fn author_exact() {
            assert_eq!(
                FilterType::from_str("=author:richard p. feynman").unwrap(),
                FilterType::Author {
                    query: vec![
                        "richard".to_string(),
                        "p.".to_string(),
                        "feynman".to_string(),
                    ],
                    operator: FilterOperator::Exact,
                }
            );
        }

        #[test]
        fn tags_any() {
            assert_eq!(
                FilterType::from_str("?tags:#artist #death").unwrap(),
                FilterType::Tags {
                    query: vec!["#artist".to_string(), "#death".to_string()],
                    operator: FilterOperator::Any,
                }
            );
        }

        #[test]
        fn tags_all() {
            assert_eq!(
                FilterType::from_str("*tags:#death #impermanence").unwrap(),
                FilterType::Tags {
                    query: vec!["#death".to_string(), "#impermanence".to_string()],
                    operator: FilterOperator::All,
                }
            );
        }

        #[test]
        fn tags_exact() {
            assert_eq!(
                FilterType::from_str("=tags:#artist #being").unwrap(),
                FilterType::Tags {
                    query: vec!["#artist".to_string(), "#being".to_string()],
                    operator: FilterOperator::Exact,
                }
            );
        }
    }
}

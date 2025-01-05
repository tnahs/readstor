//! Defines the interface to the templating engine.

use std::collections::HashMap;

use chrono::format::{Item, StrftimeItems};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tera::{try_get_value, Tera};

use crate::result::Result;
use crate::strings;

/// Templating engine interface.
#[derive(Debug)]
pub struct RenderEngine(Tera);

impl Default for RenderEngine {
    fn default() -> Self {
        let mut inst = Self(Tera::default());
        inst.register_custom_filters();
        inst
    }
}

impl RenderEngine {
    /// Registers a template into the engine.
    ///
    /// # Arguments
    ///
    /// * `name` - The template's name.
    /// * `contents` - The templates's contents.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the templates contains any errors.
    pub fn register_template(&mut self, name: &str, content: &str) -> Result<()> {
        self.0.add_raw_template(name, content)?;

        Ok(())
    }

    /// Renders a template with a context.
    ///
    /// # Arguments
    ///
    /// * `name` - The template's name.
    /// * `context` - The templates's context.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The template doesn't exist.
    /// * [`serde_json`][serde-json] encounters any errors.
    pub fn render<C>(&self, name: &str, context: C) -> Result<String>
    where
        C: Serialize,
    {
        let context = &tera::Context::from_serialize(context)?;
        let string = self.0.render(name, context)?;

        Ok(string)
    }

    /// Renders a one-off template string with a context.
    ///
    /// # Arguments
    ///
    /// * `template` - The template's contents.
    /// * `context` - The templates's context.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The templates contains any errors.
    /// * [`serde_json`][serde-json] encounters any errors.
    pub fn render_str<C>(&mut self, template: &str, context: C) -> Result<String>
    where
        C: Serialize,
    {
        let context = &tera::Context::from_serialize(context)?;
        let string = self.0.render_str(template, context)?;

        Ok(string)
    }

    /// Registers custom template filters.
    fn register_custom_filters(&mut self) {
        self.0.register_filter("date", filter_date);
        self.0.register_filter("strip", filter_strip);
        self.0.register_filter("slugify", filter_slugify);
    }
}

/// This is a partial reimplementation of `Tera`'s `date` filter that handles empty dates strings.
///
/// Some date fields in the source data might be blank. Instead of throwing a 'type' error (`Tera`s
/// default behaviuor), this function returns a blank string if an empty date is passed to the
/// `date` filter.
///
/// Additionally, this only handles [`DateTime`]'s default serialize format: RFC 3339. As we're
/// using [`DateTime`]s default [`Serialize`] implementation, we can use its default [`FromStr`]
/// to deserialize it.
#[allow(clippy::implicit_hasher)]
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn filter_date(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    if value.is_null() || value.as_str() == Some("") {
        return Ok(tera::Value::String(String::new()));
    }

    let format = match args.get("format") {
        Some(val) => try_get_value!("date", "format", String, val),
        None => crate::defaults::DATE_FORMAT_TEMPLATE.to_string(),
    };

    let errors: Vec<Item<'_>> = StrftimeItems::new(&format)
        .filter(|item| matches!(item, Item::Error))
        .collect();

    if !errors.is_empty() {
        return Err(tera::Error::msg(format!("Invalid date format `{format}`",)));
    }

    let tera::Value::String(date_str) = value else {
        return Err(tera::Error::msg(format!(
            "Filter `date` received an incorrect type for arg `value`: \
             got `{value:?}` but expected String",
        )));
    };

    // This should be safe as we're providing the input string. It's a serialized `DateTime<Utc>`
    // object. An error here would be critical and should fail.
    let date = date_str.parse::<DateTime<Utc>>().unwrap();

    let formatted = date.format(&format).to_string();

    Ok(tera::Value::String(formatted))
}

/// Wraps the `strip` function to interface with the templating engine.
#[allow(clippy::implicit_hasher)]
fn filter_strip(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let input = value
        .as_str()
        .ok_or("Expected input value to be a string")?;

    let chars = args
        .get("chars")
        .and_then(tera::Value::as_str)
        .unwrap_or(" ");

    Ok(tera::Value::String(strings::strip(input, chars)))
}

/// Wraps the `to_slug` function to interface with the templating engine.
#[allow(clippy::implicit_hasher)]
fn filter_slugify(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let input = value
        .as_str()
        .ok_or("Expected input value to be a string")?;

    let lowercase = args
        .get("lowercase")
        .and_then(tera::Value::as_bool)
        .unwrap_or(true);

    let replaced = strings::to_slug(input, lowercase);

    Ok(tera::Value::String(replaced))
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::defaults::test::TemplatesDirectory;
    use crate::utils;

    use std::collections::BTreeMap;

    #[derive(Default, Serialize)]
    struct EmptyContext(BTreeMap<String, String>);

    fn render_test_template(directory: TemplatesDirectory, filename: &str) {
        let mut engine = RenderEngine::default();
        let template = utils::testing::load_template_str(directory, filename);
        engine.register_template(filename, &template).unwrap();
        engine.render(filename, EmptyContext::default()).unwrap();
    }

    mod valid_filter {

        use super::*;

        #[test]
        fn strip() {
            render_test_template(TemplatesDirectory::ValidFilter, "valid-strip.txt");
        }

        #[test]
        fn slugify() {
            render_test_template(TemplatesDirectory::ValidFilter, "valid-slugify.txt");
        }

        #[test]
        fn date() {
            render_test_template(TemplatesDirectory::ValidFilter, "valid-date.txt");
        }
    }

    mod invalid_filter {

        use super::*;

        #[test]
        #[should_panic(expected = "Failed to parse 'invalid-strip-01.txt'")]
        fn strip_01() {
            render_test_template(TemplatesDirectory::InvalidFilter, "invalid-strip-01.txt");
        }

        #[test]
        #[should_panic(expected = "Failed to parse 'invalid-strip-02.txt'")]
        fn strip_02() {
            render_test_template(TemplatesDirectory::InvalidFilter, "invalid-strip-02.txt");
        }

        #[test]
        #[should_panic(expected = "Failed to parse 'invalid-slugify.txt'")]
        fn slugify() {
            render_test_template(TemplatesDirectory::InvalidFilter, "invalid-slugify.txt");
        }

        #[test]
        #[should_panic(
            expected = "called `Result::unwrap()` on an `Err` value: ParseError(TooShort)"
        )]
        fn date() {
            render_test_template(TemplatesDirectory::InvalidFilter, "invalid-date.txt");
        }
    }
}

//! Defines the interface to the templating engine.

use std::collections::HashMap;

use serde::Serialize;
use tera::Tera;

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
        self.0.register_filter("strip", filter_strip);
        self.0.register_filter("slugify", filter_slugify);
    }
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
mod test_engine {

    use super::*;

    use crate::utils;

    use std::collections::BTreeMap;

    #[derive(Default, Serialize)]
    struct EmptyContext(BTreeMap<String, String>);

    fn test_filter(directory: &str, filename: &str) {
        let mut engine = RenderEngine::default();
        let template = utils::load_test_template_str(directory, filename);
        engine.register_template(filename, &template).unwrap();
        engine.render(filename, EmptyContext::default()).unwrap();
    }

    mod valid_filter {

        use super::*;

        const DIRECTORY: &str = "valid-filter";

        #[test]
        fn strip() {
            test_filter(DIRECTORY, "valid-strip.txt");
        }

        #[test]
        fn slugify() {
            test_filter(DIRECTORY, "valid-slugify.txt");
        }
    }

    mod invalid_filter {

        use super::*;

        const DIRECTORY: &str = "invalid-filter";

        #[test]
        #[should_panic(expected = "Failed to parse 'invalid-strip-01.txt'")]
        fn strip_01() {
            test_filter(DIRECTORY, "invalid-strip-01.txt");
        }

        #[test]
        #[should_panic(expected = "Failed to parse 'invalid-strip-02.txt'")]
        fn strip_02() {
            test_filter(DIRECTORY, "invalid-strip-02.txt");
        }

        #[test]
        #[should_panic(expected = "Failed to parse 'invalid-slugify.txt'")]
        fn slugify() {
            test_filter(DIRECTORY, "invalid-slugify.txt");
        }
    }
}

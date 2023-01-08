//! Defines types to represent a template's content and metadata.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::result::{Error, Result};

use super::defaults::{CONFIG_TAG_CLOSE, CONFIG_TAG_OPEN};

/// A struct representing a fully configured template.
#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TemplateRaw {
    /// The template's id.
    ///
    /// This is typically a file path relative to the templates directory. It
    /// serves to identify a template within the registry when rendering. This
    /// is one of two fields that are passed to Tera when registering the
    /// template. The other one being [`Template.contents`].
    ///
    /// ```plaintext
    /// --> /path/to/templates/nested/template.md
    /// -->                    nested/template.md
    /// ```
    #[serde(skip_deserializing)]
    pub id: String,

    /// The unparsed contents of the template.
    ///
    /// This gets parsed and validated during registration. This is one of two
    /// fields that are passed to Tera when registering the template. The other
    /// one being [`Template.id`].
    #[serde(skip_deserializing)]
    pub contents: String,

    /// The template's group name.
    ///
    /// See [`StructureMode::FlatGrouped`] and [`StructureMode::NestedGrouped`]
    /// for more information.
    #[serde(deserialize_with = "super::utils::deserialize_and_sanitize")]
    pub group: String,

    /// The template's context mode i.e what the template intends to render.
    ///
    /// See [`ContextMode`] for more information.
    #[serde(rename = "context")]
    pub context_mode: ContextMode,

    /// The template's structure mode i.e. how the output should be structured.
    ///
    /// See [`StructureMode`] for more information.
    #[serde(rename = "structure")]
    pub structure_mode: StructureMode,

    /// The template's file extension.
    pub extension: String,

    /// The template strings for generating output file and directory names.
    #[serde(default)]
    pub names: Names,
}

impl TemplateRaw {
    /// Creates a new instance of [`TemplateRaw`].
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the template relative to the templates directory.
    /// * `string` - The contents of the template file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The template's opening and closing config tags have syntax errors.
    /// * The tempalte's config has syntax errors or is missing required fields.
    pub fn new<P>(path: P, string: &str) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let (config, contents) = Self::parse(string).ok_or(Error::InvalidTemplateConfig {
            path: path.display().to_string(),
        })?;

        let mut template: Self = serde_yaml::from_str(config)?;

        template.id = path.display().to_string();
        template.contents = contents;

        Ok(template)
    }

    /// Returns a tuple containing the template's configuration and its contents
    /// respectively.
    ///
    /// Returns `None` if the template's config block is formatted incorrectly.
    fn parse(string: &str) -> Option<(&str, String)> {
        // Find where the opening tag starts...
        let mut config_start = string.find(CONFIG_TAG_OPEN)?;

        // (Save the pre-config contents.)
        let pre_config_contents = &string[0..config_start];

        // ...and offset it by the length of the config opening tag.
        config_start += CONFIG_TAG_OPEN.len();

        // Starting from where we found the opening tag, search for a closing
        // tag. If we don't offset the starting point we might find another
        // closing tag located before the opening tag.
        let mut config_end = string[config_start..].find(CONFIG_TAG_CLOSE)?;
        // Remove the offset we just used.
        config_end += config_start;

        let config = &string[config_start..config_end];

        // The template's post-config contents start after the closiong tag.
        let post_config_contents = config_end + CONFIG_TAG_CLOSE.len();
        let mut post_config_contents = &string[post_config_contents..];

        // Trim a single linebreak if its present.
        if post_config_contents.starts_with('\n') {
            post_config_contents = &post_config_contents[1..];
        }

        let contents = format!("{pre_config_contents}{post_config_contents}",);

        Some((config, contents))
    }
}

impl std::fmt::Debug for TemplateRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateRaw")
            .field("id", &self.id)
            .field("group", &self.group)
            .field("context_mode", &self.context_mode)
            .field("structure_mode", &self.structure_mode)
            .finish()
    }
}

/// A struct representing a unconfigured partial template.
///
/// Partial templates get their configuration from the normal templates that
/// `include` them.
#[derive(Clone)]
pub struct TemplatePartialRaw {
    /// The template's id.
    ///
    /// This is typically a file path relative to the templates directory.
    /// It serves to identify a partial template when called in an `include`
    /// tag from within a normal template. This field is passed to Tera when
    /// registering the template.
    ///
    /// ```plaintext
    /// --> /path/to/templates/nested/template.md
    /// -->                    nested/template.md
    /// --> {% include "nested/template.md" %}
    /// ````
    pub id: String,

    /// The unparsed contents of the template.
    ///
    /// This gets parsed and validated only when a normal template that includes
    /// it is being parsed and valiated. This field is passed to Tera when
    /// registering the template.
    pub contents: String,
}

impl TemplatePartialRaw {
    /// Creates a new instance of [`TemplatePartialRaw`].
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the template relative to the templates directory.
    /// * `string` - The contents of the template file.
    pub fn new<P>(path: P, string: &str) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            id: path.as_ref().display().to_string(),
            contents: string.to_owned(),
        }
    }
}

impl std::fmt::Debug for TemplatePartialRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplatePartialRaw")
            .field("id", &self.id)
            .finish()
    }
}

/// A struct representing a rendered template.
#[derive(Default)]
pub struct TemplateRender {
    /// The path to where the template will be written to.
    ///
    /// This path should be relative to the final output directory as this path
    /// is appended to it to determine the the full output path.
    pub path: PathBuf,

    /// The final output filename.
    pub filename: String,

    /// The rendered content.
    pub contents: String,
}

impl TemplateRender {
    /// Creates a new instance of [`TemplateRender`].
    #[must_use]
    pub fn new(path: PathBuf, filename: String, contents: String) -> Self {
        Self {
            path,
            filename,
            contents,
        }
    }
}

impl std::fmt::Debug for TemplateRender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateRender")
            .field("path", &self.path)
            .field("filename", &self.filename)
            .finish()
    }
}

/// An enum representing the ways to structure a template's rendered files.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StructureMode {
    /// When selected, the template is rendered to the output directory without
    /// any structure.
    ///
    /// ```yaml
    /// output-mode: flat
    /// ```
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├─ [template-name-01].[extension]
    ///  ├─ [template-name-01].[extension]
    ///  └─ ...
    /// ```
    Flat,

    /// When selected, the template is rendered to the output directory and
    /// placed inside a directory named after its `group`. This useful if there
    /// are multiple related and unrelated templates being rendered to the same
    /// directory.
    ///
    /// ```yaml
    /// output-mode: flat-grouped
    /// ```
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├─ [template-group-01]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   └─ ...
    ///  │
    ///  ├─ [template-group-02]
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    FlatGrouped,

    /// When selected, the template is rendered to the output directory and
    /// placed inside a directory named after its `nested-directory-template`.
    /// This useful if multiple templates are used to represent a single book
    /// i.e. a book template used to render a book's information to a single
    /// file and an annotation template used to render each annotation to a
    /// separate file.
    ///
    /// ```yaml
    /// output-mode: nested
    /// ```
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├─ [author-title-01]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   └─ ...
    ///  │
    ///  ├─ [author-title-02]
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    Nested,

    /// When selected, the template is rendered to the output directory and
    /// placed inside a directory named after its `group` and another named
    /// after its `nested-directory-template`. This useful if multiple templates
    /// are used to represent a single book i.e. a book template and an
    /// annotation template and there are multiple related and unrelated
    /// templates being rendered to the same directory.
    ///
    ///
    /// ```yaml
    /// output-mode: nested-grouped
    /// ```
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├─ [template-group-01]
    ///  │   │
    ///  │   ├─ [author-title-01]
    ///  │   │   ├─ [template-name-01].[extension]
    ///  │   │   ├─ [template-name-01].[extension]
    ///  │   │   └─ ...
    ///  │   │
    ///  │   ├─ [author-title-02]
    ///  │   │   ├─ [template-name-02].[extension]
    ///  │   │   ├─ [template-name-02].[extension]
    ///  │   │   └─ ...
    ///  │   └─ ...
    ///  │
    ///  ├─ [template-group-02]
    ///  │   ├─ [author-title-01]
    ///  │   │   └─ ...
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    NestedGrouped,
}

/// An enum representing what a template intends to render.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContextMode {
    /// When selected, the template is rendered to a single file containing a
    /// [`Book`][book] and all its [`Annotation`][annotation]s.
    ///
    /// ```yaml
    /// render-context: book
    /// ```
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  └─ [template-name].[extension]
    /// ```
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    Book,

    /// When selected, the template is rendered to multiple files containing a
    /// [`Book`][book] and only one its [`Annotation`][annotation]s.
    ///
    /// ```yaml
    /// render-context: annotation
    /// ```
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  ├─ [template-name].[extension]
    ///  ├─ [template-name].[extension]
    ///  ├─ [template-name].[extension]
    ///  └─ ...
    /// ```
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    Annotation,
}

/// A struct representing the raw template strings for generating output file
/// and directory names.
#[derive(Debug, Clone, Deserialize)]
pub struct Names {
    /// The default template used when generating an output filename for the
    /// template when its context mode is [`ContextMode::Book`].
    #[serde(default = "Names::default_book")]
    pub book: String,

    /// The default template used when generating an output filename for the
    /// template when its context mode is [`ContextMode::Annotation`].
    #[serde(default = "Names::default_annotation")]
    pub annotation: String,

    /// The default template used when generating a nested output directory for
    /// the template when its structure mode is either [`StructureMode::Nested`]
    /// or [`StructureMode::NestedGrouped`].
    #[serde(default = "Names::default_directory")]
    pub directory: String,
}

impl Default for Names {
    fn default() -> Self {
        Self {
            book: Self::default_book(),
            annotation: Self::default_annotation(),
            directory: Self::default_directory(),
        }
    }
}

impl Names {
    fn default_book() -> String {
        super::defaults::FILENAME_TEMPLATE_BOOK.to_owned()
    }

    fn default_annotation() -> String {
        super::defaults::FILENAME_TEMPLATE_ANNOTATION.to_owned()
    }

    fn default_directory() -> String {
        super::defaults::DIRECTORY_TEMPLATE.to_owned()
    }
}

#[cfg(test)]
mod test_templates {

    use crate::defaults::TEST_TEMPLATES;

    use super::*;

    fn load_template_string(directory: &str, filename: &str) -> String {
        let path = TEST_TEMPLATES.join(directory).join(filename);
        std::fs::read_to_string(path).unwrap()
    }

    // https://stackoverflow.com/a/68919527/16968574
    fn test_invalid_template_config(directory: &str, filename: &str) {
        let string = load_template_string(directory, filename);
        let result = TemplateRaw::parse(&string).ok_or(Error::InvalidTemplateConfig {
            path: filename.to_string(),
        });

        assert!(matches!(
            result,
            Err(Error::InvalidTemplateConfig { path: _ })
        ));
    }

    // https://stackoverflow.com/a/68919527/16968574
    fn test_valid_template_config(directory: &str, filename: &str) {
        let string = load_template_string(directory, filename);
        let result = TemplateRaw::parse(&string).ok_or(Error::InvalidTemplateConfig {
            path: filename.to_string(),
        });

        assert!(matches!(result, Ok(_)));
    }

    mod invalid_config {

        use super::*;

        const DIRECTORY: &str = "invalid-config";

        // Tests that a missing config block returns an error.
        #[test]
        fn missing_config() {
            test_invalid_template_config(DIRECTORY, "missing-config.txt");
        }

        // Tests that a missing closing tag returns an error.
        #[test]
        fn missing_closing_tag() {
            test_invalid_template_config(DIRECTORY, "missing-closing-tag.txt");
        }

        // Tests that missing `readstor` in the opening tag returns an error.
        #[test]
        fn incomplete_opening_tag_01() {
            test_invalid_template_config(DIRECTORY, "incomplete-opening-tag-01.txt");
        }

        // Tests that missing the `!` in the opening tag returns an error.
        #[test]
        fn incomplete_opening_tag_02() {
            test_invalid_template_config(DIRECTORY, "incomplete-opening-tag-02.txt");
        }

        // Tests that no linebreak after `readstor` returns an error.
        #[test]
        fn missing_linebreak_01() {
            test_invalid_template_config(DIRECTORY, "missing-linebreak-01.txt");
        }

        // Tests that no linebreak after the config body returns an error.
        #[test]
        fn missing_linebreak_02() {
            test_invalid_template_config(DIRECTORY, "missing-linebreak-02.txt");
        }

        // Tests that no linebreak after the closing tag returns an error.
        #[test]
        fn missing_linebreak_03() {
            test_invalid_template_config(DIRECTORY, "missing-linebreak-03.txt");
        }

        // Tests that no linebreak before the opening tag returns an error.
        #[test]
        fn missing_linebreak_04() {
            test_invalid_template_config(DIRECTORY, "missing-linebreak-04.txt");
        }
    }

    mod valid_config {

        use super::*;

        const DIRECTORY: &str = "valid-config";

        // Test the minimum required keys.
        #[test]
        fn minimum_required_keys() {
            let filename = "minimum-required-keys.txt";
            let string = load_template_string(DIRECTORY, filename);
            let result = TemplateRaw::new(filename, &string);

            assert!(matches!(result, Ok(_)));
        }

        // Tests that a template with pre- and post-config-content returns no error.
        #[test]
        fn pre_and_post_config_content() {
            test_valid_template_config(DIRECTORY, "pre-and-post-config-content.txt");
        }

        // Tests that a template with pre-config-content returns no error.
        #[test]
        fn pre_config_content() {
            test_valid_template_config(DIRECTORY, "pre-config-content.txt");
        }

        // Tests that a template with post-config-content returns no error.
        #[test]
        fn post_config_content() {
            test_valid_template_config(DIRECTORY, "post-config-content.txt");
        }
    }
}

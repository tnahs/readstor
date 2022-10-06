//! Defines the [`TemplateManager`] struct used to build and interact with
//! templates.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use tera::{Context, Tera};
use walkdir::DirEntry;

use crate::lib::models::annotation::Annotation;
use crate::lib::models::book::Book;
use crate::lib::models::entry::Entry;
use crate::lib::processor::Processor;
use crate::lib::result::{LibError, LibResult};

use super::template::{
    ContextMode, Names, PartialTemplate, StructureMode, Template, TemplateContext,
};
use super::utils;

/// A struct providing a simple interface to build and render [`Template`]s.
///
/// Template data is stored in two different locations: the `registry` holds all
/// the parsed templates ready for rendering while `templates` holds each
/// template's config along with the raw template string.
#[derive(Debug, Default)]
pub struct TemplateManager {
    /// An instance of [`Tera`] containing all the parsed templates.
    registry: Tera,

    /// A list of all the registered [`Template`]s.
    templates: Vec<Template>,

    /// A list of all the registered [`PartialTemplate`]s.
    partial_templates: Vec<PartialTemplate>,

    /// The default template to use when no templates directory is specified.
    default_template: String,

    /// An instance of [`TemplateOptions`].
    options: TemplateOptions,
}

impl TemplateManager {
    /// Returns a new instance of [`TemplateManager`].
    ///
    /// # Arguments
    ///
    /// * `options` - An instance [`TemplateOptions`].
    /// * `default_template` - A string representing the contents of a template
    /// to build as the default. Used when no templates directory is specified.
    #[must_use]
    pub fn new(options: TemplateOptions, default_template: String) -> Self {
        Self {
            default_template,
            options,
            ..Default::default()
        }
    }

    /// Builds [`Template`]s depending on whether a templates directory is
    /// provided or not. If none is provided then the default template is built.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that
    /// reference non-existent fields in a [`Book`]/[`Annotation`].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * Any IO errors are encountered.
    pub fn build_templates(&mut self) -> LibResult<()> {
        if let Some(path) = &self.options.templates_directory {
            // FIXME: Cloning here to prevent mutable & immutable borrows.
            self.build_templates_from_directory(&path.clone())?;
        } else {
            self.build_default_template();
        }

        // TODO: Validate that the `self.options.template_groups` doesn't
        // contain non-existing template groups.

        Ok(())
    }

    /// Iterates through all [`Template`]s and renders them based on their
    /// [`StructureMode`] and [`ContextMode`]. See respective enums for more
    /// details.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to be rendered.
    /// * `path` - The path to a directory to save the rendered file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn render(&self, entry: &Entry, path: &Path) -> LibResult<()> {
        for template in self.active_templates() {
            let names = Names::new(entry, template)?;

            let root = match template.structure_mode {
                StructureMode::Flat => {
                    // -> [path]
                    path.to_owned()
                }
                StructureMode::FlatGrouped => {
                    // -> [path]/[template-name]
                    path.join(&template.group)
                }
                StructureMode::Nested => {
                    // -> [path]/[author-title]
                    path.join(&names.directory)
                }

                StructureMode::NestedGrouped => {
                    // -> [path]/[template-name]/[author-title]
                    path.join(&template.group).join(&names.directory)
                }
            };

            fs::create_dir_all(&root)?;

            match template.context_mode {
                ContextMode::Book => {
                    self.render_book(entry, template, &names, &root)?;
                }
                ContextMode::Annotation => {
                    self.render_annotations(entry, template, &names, &root)?;
                }
            }
        }
        Ok(())
    }

    /// Returns an iterator over all the currently active templates. Templates
    /// can be activated through the [`TemplateOptions.template_groups`] field.
    /// All templates are considered active if no groups are specified.
    fn active_templates(&self) -> impl Iterator<Item = &Template> {
        let templates: Vec<&Template> = self.templates.iter().collect();

        if let Some(template_groups) = &self.options.template_groups {
            return templates
                .into_iter()
                .filter(|template| template_groups.contains(&template.group))
                .collect::<Vec<_>>()
                .into_iter();
        }

        templates.into_iter()
    }

    /// Builds and registers [`Template`]s from a directory containing user
    /// created templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that
    /// reference non-existent fields in a [`Book`]/[`Annotation`].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * Any IO errors are encountered.
    fn build_templates_from_directory(&mut self, path: &Path) -> LibResult<()> {
        // When a normal template is registered it's validated to make sure it
        // contains no syntax error or variables that reference non-existent
        // fields. Partial templates however are registered without directly
        // being validation as their validation happens when a normal template
        // includes them. Therefore it's important that partial templates are
        // registered before normal ones.

        for item in Self::iter_templates_directory(&path, TemplateKind::Partial) {
            // Returns the path to the template relative to the root templates
            // directory.
            //
            // --> /path/to/templates/
            // --> /path/to/templates/nested/template.md
            // -->                    nested/template.md
            //
            // This is used to uniquely identify each template.
            //
            // This unwrap is safe seeing as both `item` and `path` should both
            // be absolute paths.
            let path = pathdiff::diff_paths(&item, path).unwrap();

            let partial_template = fs::read_to_string(&item)?;
            let partial_template = PartialTemplate::new(&path, &partial_template);

            self.registry
                .add_raw_template(&partial_template.id, &partial_template.contents)?;

            self.partial_templates.push(partial_template);

            log::debug!("Added partial: `{}`", path.display());
        }

        log::debug!(
            "Currently registed partial templates: {:#?}",
            self.partial_templates
        );

        for item in Self::iter_templates_directory(&path, TemplateKind::Normal) {
            // See above.
            //
            // This unwrap is safe seeing as both `item` and `path` should both
            // be absolute paths.
            let path = pathdiff::diff_paths(&item, path).unwrap();

            let template = fs::read_to_string(&item)?;
            let template = Template::new(&path, &template)?;

            self.registry
                .add_raw_template(&template.id, &template.contents)?;

            // Templates are validated *after* being registered (1) because the
            // registry is used to retrieve templates because (2) this ensures
            // that any partial templates included can also be retrieved.
            self.validate_template(&template)?;

            self.templates.push(template);

            log::debug!("Added template: `{}`", path.display());
        }

        log::debug!("Currently registed templates: {:#?}", self.templates);

        log::debug!(
            "Built {} template(s) from `{}`",
            self.templates.len(),
            path.display()
        );

        Ok(())
    }

    /// Builds and registers the default [`Template`].
    fn build_default_template(&mut self) {
        // This should be safe as were building the default template.
        let template = Template::new("__default", &self.default_template).unwrap();

        self.registry
            .add_raw_template(&template.id, &template.contents)
            // Unwrap should be okay here as were not building a template
            // inheritance chain.
            .unwrap();

        self.templates.push(template);

        log::debug!("Built the default template");
    }

    /// Validates that a template does not contain variables that reference
    /// non-existent fields in an [`Entry`]/[`Book`]/[`Annotation`].
    ///
    /// Tera checks for invalid syntax when a new template is registered however
    /// the template's use of variables can only be checked when a context is
    /// supplied. This method performs a test render with a dummy context to
    /// check for valid use of variables.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template contains variables that reference
    /// non-existent fields in an [`Entry`]/[`Book`]/[`Annotation`].
    fn validate_template(&self, template: &Template) -> Result<(), LibError> {
        // Caching values here to avoid lifetime issues.
        let entry = Entry::default();
        let book = Book::default();
        let annotation = Annotation::default();
        let names = Names::default();

        let context = match template.context_mode {
            ContextMode::Book => TemplateContext::book(&entry, &names),
            ContextMode::Annotation => TemplateContext::annotation(&book, &annotation, &names),
        };

        self.registry
            .render(&template.id, &Context::from_serialize(context)?)?;

        Ok(())
    }

    /// Renders an [`Entry`]'s [`Book`] to disk.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    fn render_book(
        &self,
        entry: &Entry,
        template: &Template,
        names: &Names,
        path: &Path,
    ) -> LibResult<()> {
        let file = &names.book;
        let file = path.join(file);
        let mut file = File::create(&file)?;

        let context = TemplateContext::book(entry, names);

        let mut render = self
            .registry
            .render(&template.id, &Context::from_serialize(context)?)?;

        // TEMP: Temporary solution until Tera implements `trim_blocks`.
        if self.options.trim_blocks {
            render = Processor::postprocess(&render);
        }

        write!(file, "{}", render)?;

        Ok(())
    }

    /// Renders an [`Entry`]'s [`Annotation`]s to disk.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    fn render_annotations(
        &self,
        entry: &Entry,
        template: &Template,
        names: &Names,
        path: &Path,
    ) -> LibResult<()> {
        for annotation in &entry.annotations {
            // This should theoretically never fail as the `Names` instance is
            // created from the `Entry`. This means they contain the same exact
            // keys and it should therefore be safe to unwrap. An error here
            // would be critical and should fail.
            let file = names
                .annotations
                .get(&annotation.metadata.id)
                .expect("`Names` instance missing Annotation present in `Entry`");
            let file = path.join(file);
            let mut file = File::create(&file)?;

            let context = TemplateContext::annotation(&entry.book, annotation, names);

            let mut render = self
                .registry
                .render(&template.id, &Context::from_serialize(context)?)?;

            // TEMP: Temporary solution until Tera implements `trim_blocks`.
            if self.options.trim_blocks {
                render = Processor::postprocess(&render);
            }

            write!(file, "{}", render)?;
        }

        Ok(())
    }

    /// Returns an iterator over all template-like files in a directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to to iterate.
    /// * `kind` - The kind of template the iterator should return.
    fn iter_templates_directory<P>(path: P, kind: TemplateKind) -> impl Iterator<Item = PathBuf>
    where
        P: AsRef<Path>,
    {
        let template_filter: fn(&DirEntry) -> bool = match kind {
            TemplateKind::Normal => utils::is_normal_template,
            TemplateKind::Partial => utils::is_partial_template,
        };

        // let predicate: fn(&DirEntry) -> bool = |entry| !utils::starts_with_underscore(entry);

        // Avoids traversing hidden directories, ignores `.hidden` files,
        // returns non-directory entries and filters the them by whether are
        // normal or partial tempaltes.
        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_entry(utils::is_hidden)
            .filter_map(std::result::Result::ok)
            .filter(|e| !e.path().is_dir())
            .filter(template_filter)
            .map(|e| e.path().to_owned())
    }
}

/// A struct for changing the rendering behavior of the [`TemplateManager`].
#[derive(Debug, Default)]
pub struct TemplateOptions {
    /// The path to a directory containing user-generated templates.
    pub templates_directory: Option<PathBuf>,

    /// A list of template groups to render. If
    pub template_groups: Option<Vec<String>>,

    /// Trim any blocks left after rendering
    pub trim_blocks: bool,
}

/// An enum representing the two different template types.
#[derive(Debug, Clone, Copy)]
enum TemplateKind {
    /// A normal [`Template`]. Requires a configuration block. Should not start
    /// with an underscore.
    Normal,

    /// A [`PartialTemplate`]. Must start with an underscore `_` but does not
    /// require a configuration block.
    Partial,
}

#[cfg(test)]
mod test_manager {

    use crate::lib::defaults::{EXAMPLE_TEMPLATES, TEST_TEMPLATES};

    use super::*;

    fn read_template(directory: &str, filename: &str) -> Template {
        let path = TEST_TEMPLATES.join(directory).join(filename);
        let string = std::fs::read_to_string(path).unwrap();

        Template::new(filename, &string).unwrap()
    }

    fn validate_context(directory: &str, filename: &str) -> LibResult<()> {
        let template = read_template(directory, filename);

        let mut manager = TemplateManager::default();

        manager
            .registry
            .add_raw_template(&template.id, &template.contents)
            .unwrap();

        manager.validate_template(&template)
    }

    fn validate_syntax(directory: &str, filename: &str) -> LibResult<()> {
        let template = read_template(directory, filename);

        let mut manager = TemplateManager::default();

        manager
            .registry
            .add_raw_template(&template.id, &template.contents)?;

        Ok(())
    }

    // https://stackoverflow.com/a/68919527/16968574
    fn test_invalid_context(directory: &str, filename: &str) {
        let result = validate_context(directory, filename);

        assert!(matches!(result, Err(LibError::InvalidTemplate(_))));
    }

    // https://stackoverflow.com/a/68919527/16968574
    fn test_valid_context(directory: &str, filename: &str) {
        let result = validate_context(directory, filename);

        assert!(matches!(result, Ok(_)));
    }

    mod invalid_context {

        use super::*;

        const DIRECTORY: &str = "invalid-context";

        // Tests that an invalid object (`invalid.title`) returns an error.
        #[test]
        fn invalid_object() {
            test_invalid_context(DIRECTORY, "invalid-object.txt");
        }

        // Tests that an invalid attribute (`book.invalid`) returns an error.
        #[test]
        fn invalid_attribute() {
            test_invalid_context(DIRECTORY, "invalid-attribute.txt");
        }
    }

    mod valid_context {

        use super::*;

        const DIRECTORY: &str = "valid-context";

        // Tests that all `Book` fields are valid.
        #[test]
        fn valid_book() {
            test_valid_context(DIRECTORY, "valid-book.txt");
        }

        // Tests that all `Annotation` fields are valid.
        #[test]
        fn valid_annotation() {
            test_valid_context(DIRECTORY, "valid-annotation.txt");
        }
    }

    mod invalid_syntax {

        use super::*;

        const DIRECTORY: &str = "invalid-syntax";

        // Tests that an invalid syntax returns an error.
        #[test]
        fn invalid_syntax() {
            let result = validate_syntax(DIRECTORY, "invalid-syntax.txt");

            assert!(matches!(result, Err(LibError::InvalidTemplate(_))));
        }
    }

    mod valid_syntax {

        use super::*;

        const DIRECTORY: &str = "valid-syntax";

        // Tests that a valid syntax returns no errors.
        #[test]
        fn valid_syntax() {
            let result = validate_syntax(DIRECTORY, "valid-syntax.txt");

            assert!(matches!(result, Ok(_)));
        }
    }

    // Tests that all example templates return no errors.
    mod example_templates {

        use super::*;

        #[test]
        fn examples() {
            let mut manager = TemplateManager::default();
            manager
                .build_templates_from_directory(&EXAMPLE_TEMPLATES)
                .unwrap();
        }
    }
}

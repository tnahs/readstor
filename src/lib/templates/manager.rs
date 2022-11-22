//! Defines the [`Templates`] struct used to build and interact with templates.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use tera::{Context, Tera};
use walkdir::DirEntry;

use crate::models::annotation::Annotation;
use crate::models::book::Book;
use crate::models::entry::Entry;
use crate::result::Result;

use super::template::{
    ContextMode, Names, StructureMode, TemplateContext, TemplatePartialRaw, TemplateRaw,
    TemplateRender,
};
use super::utils;

/// A struct providing a simple interface to build and render [`TemplateRaw`]s.
///
/// Template data is stored in two different locations: the `registry` holds all
/// the parsed templates ready for rendering while `templates` holds each
/// template's config along with the raw template string.
#[derive(Debug, Default)]
pub struct Templates {
    /// A list of all the registered [`TemplateRaw`]s.
    raws: Vec<TemplateRaw>,

    /// A list of all the registered [`TemplatePartialRaw`]s.
    partials: Vec<TemplatePartialRaw>,

    /// A list of all rendered templates.
    renders: Vec<TemplateRender>,

    /// The default template to use when no templates directory is specified.
    default: String,

    /// An instance of [`Tera`] containing all the parsed templates.
    registry: Tera,

    /// An instance of [`TemplateOptions`].
    options: TemplateOptions,
}

impl Templates {
    /// Returns a new instance of [`Templates`].
    ///
    /// # Arguments
    ///
    /// * `options` - An instance [`TemplateOptions`].
    /// * `default` - A string representing the contents of a template to build
    /// as the default. Used when no templates directory is specified.
    #[must_use]
    pub fn new(options: TemplateOptions, default: String) -> Self {
        Self {
            default,
            options,
            ..Default::default()
        }
    }

    /// Initializes [`Templates`] by building [`TemplateRaw`]s depending on
    /// whether a templates directory is provided or not. If none is provided
    /// then the default template is built.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that
    /// reference non-existent fields in a [`Book`]/[`Annotation`].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * Any IO errors are encountered.
    pub fn init(&mut self) -> Result<()> {
        if let Some(path) = &self.options.templates_directory {
            // Cloning here to prevent mutable & immutable borrows.
            self.build_from_directory(&path.clone())?;
        } else {
            self.build_default();
        }

        // TODO: Validate that the `self.options.template_groups` doesn't
        // contain non-existing template groups.

        Ok(())
    }

    /// Iterates through all [`TemplateRaw`]s and renders them based on their
    /// [`StructureMode`] and [`ContextMode`]. See respective enums for more
    /// information.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to be rendered.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn render(&mut self, entry: &Entry) -> Result<()> {
        let mut renders = Vec::new();

        for template in self.iter_active_templates() {
            let names = Names::new(entry, template)?;

            // Builds a path, relative to the [output-directory], to where the
            // the rendered template will be written to.
            let path = match template.structure_mode {
                StructureMode::Flat => {
                    // -> [output-directory]
                    PathBuf::new()
                }
                StructureMode::FlatGrouped => {
                    // -> [output-directory]/[template-name]
                    PathBuf::from(&template.group)
                }
                StructureMode::Nested => {
                    // -> [output-directory]/[author-title]
                    PathBuf::from(&names.directory)
                }

                StructureMode::NestedGrouped => {
                    // -> [output-directory]/[template-name]/[author-title]
                    PathBuf::from(&template.group).join(&names.directory)
                }
            };

            match template.context_mode {
                ContextMode::Book => {
                    renders.push(self.render_book(entry, template, &names, path)?);
                }
                ContextMode::Annotation => {
                    renders.extend(self.render_annotations(entry, template, &names, &path)?);
                }
            }
        }

        self.renders.extend(renders);

        Ok(())
    }

    /// Iterates through all [`TemplateRender`]s and writes them to disk.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the write the rendered templates to. Each
    /// rendered template's path is appened to this path to determine its full
    /// path.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn write(&self, path: &Path) -> Result<()> {
        for render in &self.renders {
            // -> [ouput-directory]/[template-subdirectory]
            let root = path.join(&render.path);

            fs::create_dir_all(&root)?;

            // -> [ouput-directory]/[template-subdirectory]/[template-filename]
            let file = root.join(&render.filename);
            let mut file = File::create(&file)?;

            write!(file, "{}", &render.contents)?;
        }

        Ok(())
    }

    /// Returns an iterator over all [`TemplateRender`]s.
    pub fn renders(&self) -> impl Iterator<Item = &TemplateRender> {
        self.renders.iter()
    }

    /// Returns a mutable iterator over all [`TemplateRender`]s.
    pub fn renders_mut(&mut self) -> impl Iterator<Item = &mut TemplateRender> {
        self.renders.iter_mut()
    }

    /// Returns the number of [`TemplateRaw`]s.
    #[must_use]
    pub fn count_templates(&self) -> usize {
        self.raws.len()
    }

    /// Returns the number of [`TemplateRender`]s.
    #[must_use]
    pub fn count_renders(&self) -> usize {
        self.renders.len()
    }

    /// Returns an iterator over all the currently active templates. Templates
    /// can be activated through the [`TemplateOptions.template_groups`] field.
    /// All templates are considered active if no groups are specified.
    fn iter_active_templates(&self) -> impl Iterator<Item = &TemplateRaw> {
        let templates: Vec<&TemplateRaw> = self.raws.iter().collect();

        if let Some(template_groups) = &self.options.template_groups {
            return templates
                .into_iter()
                .filter(|template| template_groups.contains(&template.group))
                .collect::<Vec<_>>()
                .into_iter();
        }

        templates.into_iter()
    }

    /// Builds and registers [`TemplateRaw`]s from a directory containing user-
    /// generated templates.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to a directory containing user-generated templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that
    /// reference non-existent fields in a [`Book`]/[`Annotation`].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * Any IO errors are encountered.
    fn build_from_directory(&mut self, path: &Path) -> Result<()> {
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
            let partial_template = TemplatePartialRaw::new(&path, &partial_template);

            self.registry
                .add_raw_template(&partial_template.id, &partial_template.contents)?;

            self.partials.push(partial_template);

            log::debug!("added partial template: {}", path.display());
        }

        log::debug!("currently registed partial templates: {:#?}", self.partials);

        for item in Self::iter_templates_directory(&path, TemplateKind::Normal) {
            // See above.
            //
            // This unwrap is safe seeing as both `item` and `path` should both
            // be absolute paths.
            let path = pathdiff::diff_paths(&item, path).unwrap();

            let template = fs::read_to_string(&item)?;
            let template = TemplateRaw::new(&path, &template)?;

            self.registry
                .add_raw_template(&template.id, &template.contents)?;

            // Templates are validated *after* being registered (1) because the
            // registry is used to retrieve templates because (2) this ensures
            // that any partial templates included can also be retrieved.
            self.validate_template(&template)?;

            self.raws.push(template);

            log::debug!("added template: {}", path.display());
        }

        log::debug!("currently registed templates: {:#?}", self.raws);

        log::debug!(
            "built {} template(s) from {}",
            self.raws.len(),
            path.display()
        );

        Ok(())
    }

    /// Builds and registers the default [`TemplateRaw`].
    fn build_default(&mut self) {
        // This should be safe as were building the default template.
        let template = TemplateRaw::new("__default", &self.default).unwrap();

        self.registry
            .add_raw_template(&template.id, &template.contents)
            // Unwrap should be okay here as were not building a template
            // inheritance chain.
            .unwrap();

        self.raws.push(template);

        log::debug!("built the default template");
    }

    /// Validates that a template does not contain variables that reference
    /// non-existent fields in an [`Entry`]/[`Book`]/[`Annotation`].
    ///
    /// Tera checks for invalid syntax when a new template is registered however
    /// the template's use of variables can only be checked when a context is
    /// supplied. This method performs a test render with a dummy context to
    /// check for valid use of variables.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to validate.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template contains variables that reference
    /// non-existent fields in an [`Entry`]/[`Book`]/[`Annotation`].
    fn validate_template(&self, template: &TemplateRaw) -> Result<()> {
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

    /// Renders an [`Entry`]'s [`Book`] to a single [`TemplateRender`].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] containing the [`Book`] to render.
    /// * `template` - The [`TemplateRaw`] to render with.
    /// * `names` - A [`Names`] instance generated from the [`Entry`] and a
    /// [`TemplateRaw`].
    /// * `path` - The path to where the template will be written to. This path
    /// should be relative to the final output directory.
    ///
    /// # Errors
    ///
    /// Will return `Err` if Tera encounters an error.
    fn render_book(
        &self,
        entry: &Entry,
        template: &TemplateRaw,
        names: &Names,
        path: PathBuf,
    ) -> Result<TemplateRender> {
        let filename = names.book.clone();

        let context = TemplateContext::book(entry, names);

        let contents = self
            .registry
            .render(&template.id, &Context::from_serialize(context)?)?;

        Ok(TemplateRender::new(path, filename, contents))
    }

    /// Renders an [`Entry`]'s [`Annotation`]s to multiple [`TemplateRender`]s.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] containing the [`Annotation`]s to render.
    /// * `template` - The [`TemplateRaw`] to render with.
    /// * `names` - A [`Names`] instance generated from the [`Entry`] and a
    /// [`TemplateRaw`].
    /// * `path` - The path to where the template will be written to. This path
    /// should be relative to the final output directory.
    ///
    /// # Errors
    ///
    /// Will return `Err` if Tera encounters an error.
    fn render_annotations(
        &self,
        entry: &Entry,
        template: &TemplateRaw,
        names: &Names,
        path: &Path,
    ) -> Result<Vec<TemplateRender>> {
        let mut renders = Vec::with_capacity(entry.annotations.len());

        for annotation in &entry.annotations {
            // This should theoretically never fail as the `Names` instance is
            // created from the `Entry`. This means they contain the same exact
            // keys and it should therefore be safe to unwrap. An error here
            // would be critical and should fail.
            let filename = names
                .annotations
                .get(&annotation.metadata.id)
                .expect("`Names` instance missing `Annotation` present in `Entry`")
                .clone();

            let context = TemplateContext::annotation(&entry.book, annotation, names);

            let contents = self
                .registry
                .render(&template.id, &Context::from_serialize(context)?)?;

            renders.push(TemplateRender::new(path.to_owned(), filename, contents));
        }

        Ok(renders)
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

/// A struct for changing the rendering behavior of the [`Templates`].
#[derive(Debug, Default)]
pub struct TemplateOptions {
    /// A path to a directory containing user-generated templates.
    pub templates_directory: Option<PathBuf>,

    /// A list of template groups to render. If none is provided, all templates
    /// are rendered.
    pub template_groups: Option<Vec<String>>,
}

/// An enum representing the two different template types.
#[derive(Debug, Clone, Copy)]
enum TemplateKind {
    /// A [`TemplateRaw`] template. Requires a configuration block and should
    /// not start with an underscore.
    Normal,

    /// A [`TemplatePartialRaw`] template. Must start with an underscore `_`
    /// but does not require a configuration block.
    Partial,
}

#[cfg(test)]
mod test_manager {

    use crate::defaults::{EXAMPLE_TEMPLATES, TEST_TEMPLATES};
    use crate::result::Error;

    use super::*;

    fn load_template(directory: &str, filename: &str) -> TemplateRaw {
        let path = TEST_TEMPLATES.join(directory).join(filename);
        let string = std::fs::read_to_string(path).unwrap();

        TemplateRaw::new(filename, &string).unwrap()
    }

    fn validate_context(directory: &str, filename: &str) -> Result<()> {
        let template = load_template(directory, filename);

        let mut templates = Templates::default();

        templates
            .registry
            .add_raw_template(&template.id, &template.contents)
            .unwrap();

        templates.validate_template(&template)
    }

    fn validate_syntax(directory: &str, filename: &str) -> Result<()> {
        let template = load_template(directory, filename);

        let mut templates = Templates::default();

        templates
            .registry
            .add_raw_template(&template.id, &template.contents)?;

        Ok(())
    }

    // https://stackoverflow.com/a/68919527/16968574
    fn test_invalid_context(directory: &str, filename: &str) {
        let result = validate_context(directory, filename);

        assert!(matches!(result, Err(Error::InvalidTemplate(_))));
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

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
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
            let mut templates = Templates::default();

            templates.build_from_directory(&EXAMPLE_TEMPLATES).unwrap();
        }
    }
}

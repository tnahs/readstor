//! Defines types to build and manage templates.

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use tera::Tera;
use walkdir::DirEntry;

use crate::contexts::annotation::AnnotationContext;
use crate::contexts::book::BookContext;
use crate::contexts::entry::EntryContext;
use crate::models::entry::Entry;
use crate::result::{Error, Result};

use super::names::NamesRender;
use super::template::{
    ContextMode, StructureMode, TemplatePartialRaw, TemplateRaw, TemplateRender,
};
use super::utils;

/// A struct providing a simple interface to build and render templates.
#[derive(Debug, Default)]
pub struct Templates {
    /// All the parsed templates ready for rendering.
    registry: Tera,

    /// A list of all registed templates.
    raws: Vec<TemplateRaw>,

    /// A list of all registed partial templates.
    partials: Vec<TemplatePartialRaw>,

    /// A list of all rendered templates.
    renders: Vec<TemplateRender>,

    /// The default template to use when no templates directory is specified.
    default: String,

    /// An instance of [`RenderOptions`].
    options: RenderOptions,
}

impl Templates {
    /// Returns a new instance of [`Templates`].
    ///
    /// # Arguments
    ///
    /// * `options` - The render options.
    /// * `default` - A string representing the contents of a template to build
    /// as the default. Used when no templates directory is specified.
    #[must_use]
    pub fn new<O>(options: O, default: String) -> Self
    where
        O: Into<RenderOptions>,
    {
        Self {
            default,
            options: options.into(),
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
    /// reference non-existent fields in a [`Book`][book]/[`Annotation`][annotation].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * A requested template-group does not exist.
    /// * Any IO errors are encountered.
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    pub fn init(&mut self) -> Result<()> {
        if let Some(path) = &self.options.templates_directory {
            // Cloning here to prevent mutable & immutable borrows.
            self.build_from_directory(&path.clone())?;
        } else {
            self.build_default();
        }

        self.validate_requested_template_groups()?;

        Ok(())
    }

    /// Iterates through all [`TemplateRaw`]s and renders them based on their
    /// [`StructureMode`] and [`ContextMode`]. See respective enums for more
    /// information.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to be rendered.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn render(&mut self, entry: &Entry) -> Result<()> {
        let mut renders = Vec::new();

        let entry = EntryContext::from(entry);

        for template in self.iter_requested_templates() {
            let names = NamesRender::new(&entry, template)?;

            // Builds a path, relative to the [output-directory], to where the
            // the rendered template will be written to.
            let path = match template.structure_mode {
                StructureMode::Flat => {
                    // -> [output-directory]
                    PathBuf::new()
                }
                StructureMode::FlatGrouped => {
                    // -> [output-directory]/[template-group]
                    PathBuf::from(&template.group)
                }
                StructureMode::Nested => {
                    // -> [output-directory]/[author-title]
                    PathBuf::from(&names.directory)
                }
                StructureMode::NestedGrouped => {
                    // -> [output-directory]/[template-group]/[author-title]
                    PathBuf::from(&template.group).join(&names.directory)
                }
            };

            match template.context_mode {
                ContextMode::Book => {
                    renders.push(self.render_book(template, &entry, &names, path)?);
                }
                ContextMode::Annotation => {
                    renders.extend(self.render_annotations(template, &entry, &names, &path)?);
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

            std::fs::create_dir_all(&root)?;

            // -> [ouput-directory]/[template-subdirectory]/[template-filename]
            let file = root.join(&render.filename);

            if !self.options.overwrite_existing && file.exists() {
                log::debug!("skipped writing {}", file.display());
            } else {
                let mut file = File::create(file)?;
                write!(file, "{}", &render.contents)?;
            }
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

    /// Validates that all requested template-groups exist.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a requested template-group does not exist.
    fn validate_requested_template_groups(&self) -> Result<()> {
        if self.options.template_groups.is_empty() {
            return Ok(());
        }

        let available_template_groups: HashSet<&str> = self
            .raws
            .iter()
            .map(|template| template.group.as_str())
            .collect();

        for template_group in &self.options.template_groups {
            if !available_template_groups.contains(template_group.as_str()) {
                return Err(Error::NonexistentTemplateGroup {
                    name: template_group.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Returns an iterator over all the requested templates.
    fn iter_requested_templates(&self) -> impl Iterator<Item = &TemplateRaw> {
        let templates: Vec<&TemplateRaw> = self.raws.iter().collect();

        if self.options.template_groups.is_empty() {
            return templates.into_iter();
        }

        templates
            .into_iter()
            .filter(|template| self.options.template_groups.contains(&template.group))
            .collect::<Vec<_>>()
            .into_iter()
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
    /// reference non-existent fields in a [`Book`][book]/[`Annotation`][annotation].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * Any IO errors are encountered.
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
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

            let partial_template = std::fs::read_to_string(&item)?;
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

            let template = std::fs::read_to_string(&item)?;
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
    /// non-existent fields in an [`Entry`], [`Book`][book], [`Annotation`][annotation],
    /// [`NamesRender`].
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
    /// non-existent fields in an [`Entry`]/[`Book`][book]/[`Annotation`][annotation].
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    fn validate_template(&self, template: &TemplateRaw) -> Result<()> {
        let entry = Entry::dummy();
        let entry = EntryContext::from(&entry);
        let names = NamesRender::new(&entry, template)?;

        match template.context_mode {
            ContextMode::Book => {
                let context = TemplateContext::book(&entry.book, &entry.annotations, &names);

                self.render_template(template, context)?;
            }
            ContextMode::Annotation => {
                // This should be safe as a dummy `Entry` contains three annotations.
                let annotation = &entry.annotations[0];
                let context = TemplateContext::annotation(&entry.book, annotation, &names);

                self.render_template(template, context)?;
            }
        };

        Ok(())
    }

    /// Renders an [`Entry`]'s [`Book`][book] to a single [`TemplateRender`].
    ///
    /// # Arguments
    ///
    /// * `template` - The template to render.
    /// * `entry` - The context to inject into the template.
    /// * `names` - The names to inject into the template context.
    /// * `path` - The path to where the template will be written to. This path
    /// should be relative to the final output directory.
    ///
    /// # Errors
    ///
    /// Will return `Err` if Tera encounters an error.
    ///
    /// [book]: crate::models::book::Book
    fn render_book(
        &self,
        template: &TemplateRaw,
        entry: &EntryContext<'_>,
        names: &NamesRender,
        path: PathBuf,
    ) -> Result<TemplateRender> {
        let filename = names.book.clone();

        let context = TemplateContext::book(&entry.book, &entry.annotations, names);

        let render = self.render_template(template, context)?;
        let render = TemplateRender::new(path, filename, render);

        Ok(render)
    }

    /// Renders an [`Entry`]'s [`Annotation`][annotation]s to multiple [`TemplateRender`]s.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to render.
    /// * `entry` - The context to inject into the template.
    /// * `names` - The names to inject into the template context.
    /// * `path` - The path to where the template will be written to. This path
    /// should be relative to the final output directory.
    ///
    /// # Errors
    ///
    /// Will return `Err` if Tera encounters an error.
    ///
    /// [annotation]: crate::models::annotation::Annotation
    fn render_annotations(
        &self,
        template: &TemplateRaw,
        entry: &EntryContext<'_>,
        names: &NamesRender,
        path: &Path,
    ) -> Result<Vec<TemplateRender>> {
        let mut renders = Vec::with_capacity(entry.annotations.len());

        for annotation in &entry.annotations {
            let filename = names.get_annotation_filename(&annotation.metadata.id);

            let context = TemplateContext::annotation(&entry.book, annotation, names);

            let render = self.render_template(template, context)?;
            let render = TemplateRender::new(path.to_owned(), filename, render);

            renders.push(render);
        }

        Ok(renders)
    }

    /// Renders a single template to a string.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to render.
    /// * `context` - The template's context.
    fn render_template(
        &self,
        template: &TemplateRaw,
        context: TemplateContext<'_>,
    ) -> Result<String> {
        let context = &tera::Context::from_serialize(context)?;
        Ok(self.registry.render(&template.id, context)?)
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

/// A struct representing options for the [`Templates`] struct.
#[derive(Debug, Default)]
pub struct RenderOptions {
    /// A path to a directory containing user-generated templates.
    pub templates_directory: Option<PathBuf>,

    /// A list of template-groups to render. All template-groups are rendered
    /// if none are specified.
    ///
    /// These are considered 'requested' template-groups. If they exist, their
    /// respective templates are considered 'requested' templates and are
    /// set to be rendered.
    pub template_groups: Vec<String>,

    /// Toggles whether or not to overwrite existing files.
    pub overwrite_existing: bool,
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

/// An enum representing all possible template contexts.
///
/// This primarily used to shuffle data to fit a certain shape before it's
/// injected into a template.
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum TemplateContext<'a> {
    /// Used when rendering both a [`Book`][book] and its
    /// [`Annotation`][annotation]s in a template. Includes all the output
    /// filenames and the nested directory name.
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    Book {
        book: &'a BookContext<'a>,
        annotations: &'a [AnnotationContext<'a>],
        names: &'a NamesRender,
    },
    /// Used when rendering a single [`Annotation`][annotation] in a template.
    /// Includes all the output filenames and the nested directory name.
    ///
    /// [annotation]: crate::models::annotation::Annotation
    Annotation {
        book: &'a BookContext<'a>,
        annotation: &'a AnnotationContext<'a>,
        names: &'a NamesRender,
    },
}

impl<'a> TemplateContext<'a> {
    fn book(
        book: &'a BookContext<'a>,
        annotations: &'a [AnnotationContext<'a>],
        names: &'a NamesRender,
    ) -> Self {
        Self::Book {
            book,
            annotations,
            names,
        }
    }

    fn annotation(
        book: &'a BookContext<'a>,
        annotation: &'a AnnotationContext<'a>,
        names: &'a NamesRender,
    ) -> Self {
        Self::Annotation {
            book,
            annotation,
            names,
        }
    }
}

#[cfg(test)]
mod test_templates {

    use super::*;

    use crate::result::Error;

    // Loads a test template from the `TEST_TEMPLATES` directory.
    //
    // The test templates are located at: [crate-root]/data/templates/[directory]/[filename]
    fn load_test_template(directory: &str, filename: &str) -> TemplateRaw {
        let path = crate::defaults::TEST_TEMPLATES
            .join(directory)
            .join(filename);

        let template = std::fs::read_to_string(path).unwrap();

        TemplateRaw::new(filename, &template).unwrap()
    }

    // Validates that a template does not contain variables that reference non-existent fields.
    fn validate_template_context(template: &TemplateRaw) -> Result<()> {
        let mut templates = Templates::default();

        templates
            .registry
            .add_raw_template(&template.id, &template.contents)
            .unwrap();

        templates.validate_template(template)
    }

    // Validates that a template does not contain syntax errors.
    fn validate_template_syntax(template: &TemplateRaw) -> Result<()> {
        let mut templates = Templates::default();

        templates
            .registry
            .add_raw_template(&template.id, &template.contents)?;

        Ok(())
    }

    mod invalid_context {

        use super::*;

        const DIRECTORY: &str = "invalid-context";

        // Tests that an invalid object (`invalid.[attribute]`) returns an error.
        #[test]
        fn invalid_object() {
            let template = load_test_template(DIRECTORY, "invalid-object.txt");
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid attribute (`[object].invalid`) returns an error.
        #[test]
        fn invalid_attribute() {
            let template = load_test_template(DIRECTORY, "invalid-attribute.txt");
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid annotation attribute within a `book` context returns an error.
        #[test]
        fn invalid_book_annotations() {
            let template = load_test_template(DIRECTORY, "invalid-book-annotations.txt");
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid names attribute within a `book` context returns an error.
        #[test]
        fn invalid_book_names() {
            let template = load_test_template(DIRECTORY, "invalid-book-names.txt");
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid names attribute within an `annotation` context returns an error.
        #[test]
        fn invalid_annotation_names() {
            let template = load_test_template(DIRECTORY, "invalid-annotation-names.txt");
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }
    }

    mod valid_context {

        use super::*;

        const DIRECTORY: &str = "valid-context";

        // Tests that all `Book` fields are valid.
        #[test]
        fn valid_book() {
            let template = load_test_template(DIRECTORY, "valid-book.txt");
            let result = validate_template_syntax(&template);

            assert!(matches!(result, Ok(_)));
        }

        // Tests that all `Annotation` fields are valid.
        #[test]
        fn valid_annotation() {
            let template = load_test_template(DIRECTORY, "valid-annotation.txt");
            let result = validate_template_syntax(&template);

            assert!(matches!(result, Ok(_)));
        }
    }

    mod invalid_syntax {

        use super::*;

        const DIRECTORY: &str = "invalid-syntax";

        // Tests that an invalid syntax returns an error.
        #[test]
        fn invalid_syntax() {
            let template = load_test_template(DIRECTORY, "invalid-syntax.txt");
            let result = validate_template_syntax(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }
    }

    mod valid_syntax {

        use super::*;

        const DIRECTORY: &str = "valid-syntax";

        // Tests that a valid syntax returns no errors.
        #[test]
        fn valid_syntax() {
            let template = load_test_template(DIRECTORY, "valid-syntax.txt");
            let result = validate_template_syntax(&template);

            assert!(matches!(result, Ok(_)));
        }
    }

    mod example_templates {

        use super::*;

        // Tests that all example templates return no errors.
        #[test]
        fn example_templates() {
            let mut templates = Templates::default();

            templates
                .build_from_directory(&crate::defaults::EXAMPLE_TEMPLATES)
                .unwrap();
        }
    }
}

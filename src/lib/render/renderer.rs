//! Defines types to build and manage templates.

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::DirEntry;

use crate::contexts::annotation::AnnotationContext;
use crate::contexts::book::BookContext;
use crate::contexts::entry::EntryContext;
use crate::models::entry::Entry;
use crate::result::{Error, Result};

use super::engine::RenderEngine;
use super::names::NamesRender;
use super::template::{ContextMode, Render, StructureMode, Template, TemplatePartial};
use super::utils;

/// A struct providing a simple interface to build and render templates.
#[derive(Debug, Default)]
pub struct Renderer {
    /// The render engine containing the parsed templates ready for rendering.
    engine: RenderEngine,

    /// The default template to use when no templates directory is specified.
    template_default: String,

    /// A list of all registed templates.
    templates: Vec<Template>,

    /// A list of all registed partial templates.
    templates_partial: Vec<TemplatePartial>,

    /// A list of all rendered templates.
    renders: Vec<Render>,

    /// An instance of [`RenderOptions`].
    options: RenderOptions,
}

impl Renderer {
    /// Returns a new instance of [`Renderer`].
    ///
    /// # Arguments
    ///
    /// * `options` - The render options.
    /// * `default` - A string representing the contents of a template to build as the default. Used
    ///   when no templates directory is specified.
    #[must_use]
    pub fn new<O>(options: O, default: String) -> Self
    where
        O: Into<RenderOptions>,
    {
        Self {
            template_default: default,
            options: options.into(),
            ..Default::default()
        }
    }

    /// Initializes [`Renderer`] by building [`Template`]s depending on whether a templates
    /// directory is provided or not. If none is provided then the default template is built.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that reference non-existent
    ///   fields in a [`Book`][book]/[`Annotation`][annotation].
    /// * A template's config block isn't formatted correctly, has syntax errors or is missing
    ///   required fields.
    /// * A requested template-group does not exist.
    /// * Any IO errors are encountered.
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    pub fn init(&mut self) -> Result<()> {
        if let Some(path) = &self.options.templates_directory {
            self.build_from_directory(&path.clone())?;
            // +----------------------^^^^^^^^^^^^^
            // +---- Cloning here to prevent mutable & immutable borrows.
        } else {
            self.build_default()?;
        }

        self.validate_requested_template_groups()?;

        Ok(())
    }

    /// Iterates through all [`Template`]s and renders them based on their [`StructureMode`] and
    /// [`ContextMode`]. See respective enums for more information.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to be rendered.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn render(&mut self, entry: &Entry) -> Result<()> {
        let mut renders = Vec::with_capacity(self.templates.len());

        let entry = EntryContext::from(entry);

        for template in self.iter_requested_templates() {
            let names = NamesRender::new(&entry, template)?;

            // Builds a the template's output path, relative to the [output-directory].
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
                    renders.push(self.render_book(template, &entry, &names, &path)?);
                }
                ContextMode::Annotation => {
                    renders.extend(self.render_annotations(template, &entry, &names, &path)?);
                }
            }
        }

        self.renders.extend(renders);

        Ok(())
    }

    /// Iterates through all [`Render`]s and writes them to disk.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the write the rendered templates to. Each rendered template's path is
    ///   appened to this path to determine its full path.
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

    /// Returns an iterator over all [`Render`]s.
    pub fn templates_rendered(&self) -> impl Iterator<Item = &Render> {
        self.renders.iter()
    }

    /// Returns a mutable iterator over all [`Render`]s.
    pub fn templates_rendered_mut(&mut self) -> impl Iterator<Item = &mut Render> {
        self.renders.iter_mut()
    }

    /// Returns the number of [`Template`]s.
    #[must_use]
    pub fn count_templates(&self) -> usize {
        self.templates.len()
    }

    /// Returns the number of [`Render`]s.
    #[must_use]
    pub fn count_templates_rendered(&self) -> usize {
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
            .templates
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
    fn iter_requested_templates(&self) -> impl Iterator<Item = &Template> {
        let templates: Vec<&Template> = self.templates.iter().collect();

        if self.options.template_groups.is_empty() {
            return templates.into_iter();
        }

        templates
            .into_iter()
            .filter(|template| self.options.template_groups.contains(&template.group))
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Builds and registers [`Template`]s from a directory containing user-generated templates.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to a directory containing user-generated templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that reference non-existent
    ///   fields in a [`Book`][book]/[`Annotation`][annotation].
    /// * A template's config block isn't formatted correctly, has syntax errors or is missing
    ///   required fields.
    /// * Any IO errors are encountered.
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    fn build_from_directory(&mut self, path: &Path) -> Result<()> {
        // When a normal template is registered, it's validated to make sure it contains no syntax
        // errors or variables that reference non-existent fields. Partial templates however are
        // registered without directly being validation as their validation happens when a normal
        // template includes them. Therefore it's important that partial templates are registered
        // before normal ones.

        for item in Self::iter_templates_directory(&path, TemplateKind::Partial) {
            // Returns the path to the template relative to the root templates directory.
            //
            // --> /path/to/templates/
            // --> /path/to/templates/nested/template.md
            // -->                    nested/template.md
            //
            // This is used to uniquely identify each template.
            //
            // This unwrap is safe seeing as both `item` and `path` should both be absolute paths.
            let path = pathdiff::diff_paths(&item, path).unwrap();

            let template = std::fs::read_to_string(&item)?;
            let template = TemplatePartial::new(&path, &template);

            self.engine
                .register_template(&template.id, &template.contents)?;

            self.templates_partial.push(template);

            log::debug!("added partial template: {}", path.display());
        }

        for item in Self::iter_templates_directory(&path, TemplateKind::Normal) {
            // See above.
            //
            // This unwrap is safe seeing as both `item` and `path` should both be absolute paths.
            let path = pathdiff::diff_paths(&item, path).unwrap();

            let template = std::fs::read_to_string(&item)?;
            let template = Template::new(&path, &template)?;

            self.engine
                .register_template(&template.id, &template.contents)?;

            // Templates are validated *after* being registered. The registry handles building
            // template inheritances. We need to register the templates before validating them so
            // ensure that any partial templates they reference are properly resolved.
            self.validate_template(&template)?;

            self.templates.push(template);

            log::debug!("added template: {}", path.display());
        }

        log::debug!("registed partial templates: {:#?}", self.templates_partial);
        log::debug!("registed templates: {:#?}", self.templates);

        log::debug!(
            "built {} template(s) and {} partial template(s) from {}",
            self.templates.len(),
            self.templates_partial.len(),
            path.display()
        );

        Ok(())
    }

    /// Builds and registers the default [`Template`].
    fn build_default(&mut self) -> Result<()> {
        let template = Template::new("__default", &self.template_default)?;

        self.engine
            .register_template(&template.id, &template.contents)?;

        self.templates.push(template);

        log::debug!("built the default template");

        Ok(())
    }

    /// Validates that a template does not contain variables that reference non-existent fields in
    /// an [`Entry`], [`Book`][book], [`Annotation`][annotation] and [`NamesRender`].
    ///
    /// # Arguments
    ///
    /// * `template` - The template to validate.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template contains variables that reference non-existent fields in
    /// an [`Entry`]/[`Book`][book]/[`Annotation`][annotation].
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    fn validate_template(&mut self, template: &Template) -> Result<()> {
        let entry = Entry::dummy();
        let entry = EntryContext::from(&entry);
        let names = NamesRender::new(&entry, template)?;

        match template.context_mode {
            ContextMode::Book => {
                let context = TemplateContext::book(&entry.book, &entry.annotations, &names);

                self.engine.render(&template.id, context)?;
            }
            ContextMode::Annotation => {
                // This should be safe as a dummy `Entry` contains three annotations.
                let annotation = &entry.annotations[0];
                let context = TemplateContext::annotation(&entry.book, annotation, &names);

                self.engine.render(&template.id, context)?;
            }
        };

        Ok(())
    }

    /// Renders an [`Entry`]'s [`Book`][book] to a single [`Render`].
    ///
    /// # Arguments
    ///
    /// * `template` - The template to render.
    /// * `entry` - The context to inject into the template.
    /// * `names` - The names to inject into the template context.
    /// * `path` - The path to where the template will be written to. This path should be relative
    ///   to the final output directory.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template renderer encounters an error.
    ///
    /// [book]: crate::models::book::Book
    fn render_book(
        &self,
        template: &Template,
        entry: &EntryContext<'_>,
        names: &NamesRender,
        path: &Path,
    ) -> Result<Render> {
        let filename = names.book.clone();
        let context = TemplateContext::book(&entry.book, &entry.annotations, names);
        let string = self.engine.render(&template.id, context)?;
        let render = Render::new(path.to_owned(), filename, string);

        Ok(render)
    }

    /// Renders an [`Entry`]'s [`Annotation`][annotation]s to multiple [`Render`]s.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to render.
    /// * `entry` - The context to inject into the template.
    /// * `names` - The names to inject into the template context.
    /// * `path` - The path to where the template will be written to. This path should be relative
    ///   to the final output directory.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template renderer encounters an error.
    ///
    /// [annotation]: crate::models::annotation::Annotation
    fn render_annotations(
        &self,
        template: &Template,
        entry: &EntryContext<'_>,
        names: &NamesRender,
        path: &Path,
    ) -> Result<Vec<Render>> {
        let mut renders = Vec::with_capacity(entry.annotations.len());

        for annotation in &entry.annotations {
            let filename = names.get_annotation_filename(&annotation.metadata.id);
            let context = TemplateContext::annotation(&entry.book, annotation, names);
            let string = self.engine.render(&template.id, context)?;
            let render = Render::new(path.to_owned(), filename, string);

            renders.push(render);
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

        // Avoids traversing hidden directories, ignores `.hidden` files, returns non-directory
        // entries and filters the them by whether are normal or partial tempaltes.
        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_entry(utils::is_hidden)
            .filter_map(std::result::Result::ok)
            .filter(|e| !e.path().is_dir())
            .filter(template_filter)
            .map(|e| e.path().to_owned())
    }
}

/// A struct representing options for the [`Renderer`] struct.
#[derive(Debug, Default)]
pub struct RenderOptions {
    /// A path to a directory containing user-generated templates.
    pub templates_directory: Option<PathBuf>,

    /// A list of template-groups to render. All template-groups are rendered if none are specified.
    ///
    /// These are considered 'requested' template-groups. If they exist, their respective templates
    /// are considered 'requested' templates and are set to be rendered.
    pub template_groups: Vec<String>,

    /// Toggles whether or not to overwrite existing files.
    pub overwrite_existing: bool,
}

/// An enum representing the two different template types.
#[derive(Debug, Clone, Copy)]
enum TemplateKind {
    /// A [`Template`]. Requires a configuration block and should not start with an underscore.
    Normal,

    /// A [`TemplatePartial`]. Must start with an underscore `_` but does not require a configuration block.
    Partial,
}

/// An enum representing all possible template contexts.
///
/// This primarily used to shuffle data to fit a certain shape before it's injected into a template.
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum TemplateContext<'a> {
    /// Used when rendering both a [`Book`][book] and its [`Annotation`][annotation]s in a template.
    /// Includes all the output filenames and the nested directory name.
    ///
    /// [book]: crate::models::book::Book
    /// [annotation]: crate::models::annotation::Annotation
    Book {
        book: &'a BookContext<'a>,
        annotations: &'a [AnnotationContext<'a>],
        names: &'a NamesRender,
    },
    /// Used when rendering a single [`Annotation`][annotation] in a template. Includes all the
    /// output filenames and the nested directory name.
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
mod test {

    use super::*;

    use crate::defaults::test::TemplatesDirectory;
    use crate::result::Error;
    use crate::utils;

    // Validates that a template does not contain variables that reference non-existent fields.
    fn validate_template_context(template: &str) -> Result<()> {
        let template = Template::new("validate_template_context", template).unwrap();

        let mut renderer = Renderer::default();

        renderer
            .engine
            .register_template(&template.id, &template.contents)
            .unwrap();

        renderer.validate_template(&template)
    }

    // Validates that a template does not contain syntax errors.
    fn validate_template_syntax(template: &str) -> Result<()> {
        let template = Template::new("validate_template_syntax", template).unwrap();

        let mut renderer = Renderer::default();

        renderer
            .engine
            .register_template(&template.id, &template.contents)?;

        Ok(())
    }

    mod invalid_context {

        use super::*;

        // Tests that an invalid object (`invalid.[attribute]`) returns an error.
        #[test]
        fn invalid_object() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidContext,
                "invalid-object.txt",
            );
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid attribute (`[object].invalid`) returns an error.
        #[test]
        fn invalid_attribute() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidContext,
                "invalid-attribute.txt",
            );
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid annotation attribute within a `book` context returns an error.
        #[test]
        fn invalid_book_annotations() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidContext,
                "invalid-book-annotations.txt",
            );
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid names attribute within a `book` context returns an error.
        #[test]
        fn invalid_book_names() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidContext,
                "invalid-book-names.txt",
            );
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }

        // Tests that an invalid names attribute within an `annotation` context returns an error.
        #[test]
        fn invalid_annotation_names() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidContext,
                "invalid-annotation-names.txt",
            );
            let result = validate_template_context(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }
    }

    mod valid_context {

        use super::*;

        // Tests that all `Book` fields are valid.
        #[test]
        fn valid_book() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::ValidContext,
                "valid-book.txt",
            );
            let result = validate_template_syntax(&template);

            assert!(result.is_ok());
        }

        // Tests that all `Annotation` fields are valid.
        #[test]
        fn valid_annotation() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::ValidContext,
                "valid-annotation.txt",
            );
            let result = validate_template_syntax(&template);

            assert!(result.is_ok());
        }
    }

    mod invalid_syntax {

        use super::*;

        // Tests that invalid syntax returns an error.
        #[test]
        fn invalid_syntax() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidSyntax,
                "invalid-syntax.txt",
            );
            let result = validate_template_syntax(&template);

            assert!(matches!(result, Err(Error::InvalidTemplate(_))));
        }
    }

    mod valid_syntax {

        use super::*;

        // Tests that valid syntax returns no errors.
        #[test]
        fn valid_syntax() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::ValidSyntax,
                "valid-syntax.txt",
            );
            let result = validate_template_syntax(&template);

            assert!(result.is_ok());
        }
    }

    mod example_templates {

        use super::*;

        // Tests that all example templates return no errors.
        #[test]
        fn example_templates() {
            let mut renderer = Renderer::default();

            renderer
                .build_from_directory(&crate::defaults::test::EXAMPLE_TEMPLATES)
                .unwrap();
        }
    }
}

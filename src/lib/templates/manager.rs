//! Defines the [`TemplateManager`] struct used to build and interact with
//! templates.

use std::fs::{self, File};
use std::path::{Path, PathBuf};

use tera::{Context, Tera};
use walkdir::DirEntry;

use crate::lib::models::annotation::Annotation;
use crate::lib::models::book::Book;
use crate::lib::models::entry::Entry;
use crate::lib::result::{LibError, LibResult};
use crate::lib::utils;

use super::template::{
    ContextMode, Names, PartialTemplate, StructureMode, Template, TemplateContext,
};

/// A struct providing a simple interface to build and render [`Template`]s.
///
/// Template data is stored in two different locations: the `registry` holds all
/// the parsed templates ready for rendering while `templates` holds each
/// template's config along with raw template string.
#[derive(Debug, Default)]
pub struct TemplateManager {
    /// An instance of [`Tera`] containing all the parsed templates.
    registry: Tera,

    /// A list of all the registered [`Template`]s'.
    templates: Vec<Template>,

    /// A list of all the registered [`PartialTemplate`]s. Currently these
    /// are never accessed after they are created and stored.
    partial_templates: Vec<PartialTemplate>,
}

impl TemplateManager {
    /// Builds [`Template`]s depending on whether a templates directory is
    /// provided or not. If none is provided then the default template is built.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to a directory containing templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that
    /// reference non-existent fields in a [`Book`]/[`Annotation`].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * Any IO errors are encountered.
    pub fn build(&mut self, path: &Option<PathBuf>, default_template: &str) -> LibResult<()> {
        if let Some(path) = path {
            self.build_from_directory(path)?;
        } else {
            self.build_default(default_template);
        }

        Ok(())
    }

    /// Iterates through all [`Template`]s and renders them based on their
    /// [`StructureMode`] and [`ContextMode`]. See respective enums for more
    /// details.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to be rendered.
    /// * `path` - A path to a directory to save the rendered file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    // TODO: Make sure `to_safe_string` is doing the right thing and that it
    // won't hammer filenames too badly.
    pub fn render(&self, entry: &Entry, path: &Path) -> LibResult<()> {
        for template in &self.templates {
            let names = Names::new(entry, template)?;

            let root = match template.structure_mode {
                StructureMode::Flat => {
                    // -> [path]
                    path.to_owned()
                }
                StructureMode::FlatGrouped => {
                    // -> [path]/[template-name]
                    path.join(utils::to_safe_string(&template.group))
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

    /// Builds [`Template`]s from a directory containing user created templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template contains either syntax errors or contains variables that
    /// reference non-existent fields in a [`Book`]/[`Annotation`].
    /// * A template's config block isn't formatted correctly, has syntax errors
    /// or is missing required fields.
    /// * Any IO errors are encountered.
    fn build_from_directory(&mut self, path: &Path) -> LibResult<()> {
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
            // This unwrap is safe seeing as both `item` and `path` are absolute.
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
            // This unwrap is safe seeing as both `item` and `path` are absolute.
            let path = pathdiff::diff_paths(&item, path).unwrap();

            let template = fs::read_to_string(&item)?;
            let template = Template::new(&path, &template)?;

            self.registry
                .add_raw_template(&template.id, &template.contents)?;

            // Templates are validated *after* being registered (1) because the
            // registry is used to retrieve templates because (2) this ensures
            // that any partial templates included can also be retrieved.
            // Otherwise, if a one off render is done then partial templates
            // cannot be included.
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

    /// Builds a default [`Template`].
    ///
    /// # Arguments
    ///
    /// * `default_template` - A string representing the contents of a template
    /// to  build as the default. Used when no templates directory is specified.
    fn build_default(&mut self, default_template: &str) {
        // This should be safe as were building the default template.
        let template = Template::new(default_template, "__default").unwrap();

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
        let file = File::create(&file)?;

        let context = TemplateContext::book(entry, names);

        self.registry
            .render_to(&template.id, &Context::from_serialize(context)?, file)?;

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
            // TODO: Document unwrap.
            let file = names.annotations.get(&annotation.metadata.id).unwrap();
            let file = path.join(file);
            let file = File::create(&file)?;

            let context = TemplateContext::annotation(&entry.book, annotation, names);

            self.registry
                .render_to(&template.id, &Context::from_serialize(context)?, file)?;
        }

        Ok(())
    }

    /// Returns an iterator over all the files in a directory.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to a directory to iterate.
    /// * `hidden` - Display only hidden or visible.
    fn iter_templates_directory<P>(path: P, kind: TemplateKind) -> impl Iterator<Item = PathBuf>
    where
        P: AsRef<Path>,
    {
        let predicate: fn(&DirEntry) -> bool = match kind {
            // TODO: Is there a more concise way of doing this?
            TemplateKind::Normal => |entry| !entry.file_name().to_string_lossy().starts_with('_'),
            TemplateKind::Partial => |entry| entry.file_name().to_string_lossy().starts_with('_'),
        };

        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_entry(utils::entry_is_hidden) // Ignore hidden directories/files.
            .filter_map(std::result::Result::ok)
            .filter(|e| !e.path().is_dir()) // Ignore directories.
            .filter(predicate)
            .map(|e| e.path().to_owned())
    }
}

#[derive(Debug, Clone, Copy)]
enum TemplateKind {
    Normal,
    Partial,
}

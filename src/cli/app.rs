use std::io::Write;

use color_eyre::eyre::WrapErr;

use lib::applebooks::Platform;
use lib::render::renderer::Renderer;

use crate::CliResult;

use super::args::{
    BackupOptions, ExportOptions, FilterOptions, PostProcessOptions, PreProcessOptions,
    RenderOptions,
};
use super::config::Config;
use super::data::Data;

/// Extension for an new [`App`].
pub struct ExtNone;

/// Extension for an [`App`] that renders templates.
pub struct ExtRender {
    renderer: Renderer,
}

/// Extension for an [`App`] that exports data.
pub struct ExtExport {
    options: ExportOptions,
}

/// Extension for an [`App`] that backs-up data.
pub struct ExtBackup {
    options: BackupOptions,
}

/// The main application struct.
pub struct App<Ext> {
    /// The application's configuration.
    config: Config,

    /// The application's data.
    data: Data,

    /// The application's capability extension.
    extension: Ext,
}

impl App<ExtNone> {
    /// Creates a new instance of [`App`].
    pub fn new(config: Config) -> CliResult<Self> {
        let mut app = Self {
            config,
            data: Data::default(),
            extension: ExtNone,
        };

        app.init_data()?;

        Ok(app)
    }

    /// Turns the [`App`] into one that renders templates.
    pub fn into_render(self, options: RenderOptions) -> CliResult<App<ExtRender>> {
        let mut renderer = Renderer::new(options, super::defaults::TEMPLATE.into());

        renderer
            .init()
            .wrap_err("Failed while initializing template(s)")?;

        Ok(App {
            config: self.config,
            data: self.data,
            extension: ExtRender { renderer },
        })
    }

    /// Turns the [`App`] into one that exports data.
    pub fn into_export(self, options: ExportOptions) -> App<ExtExport> {
        App {
            config: self.config,
            data: self.data,
            extension: ExtExport { options },
        }
    }

    /// Turns the [`App`] into one that backs-up data.
    pub fn into_backup(self, options: BackupOptions) -> App<ExtBackup> {
        App {
            config: self.config,
            data: self.data,
            extension: ExtBackup { options },
        }
    }

    /// Initializes the application's data.
    fn init_data(&mut self) -> CliResult<()> {
        match &self.config.platform {
            Platform::MacOs => {
                self.data
                    .init_macos(&self.config.data_directory)
                    .wrap_err("Failed while initializing macOS's Apple Books databases data")?;
            }
            Platform::IOs => {
                self.data
                    .init_ios(&self.config.data_directory)
                    .wrap_err("Failed while initializing iOS's Apple Books plists data")?;
            }
        }

        Ok(())
    }
}

/// Implementation of shared methods between different extention types.
impl<Ext> App<Ext> {
    /// Runs filters on all [`Entry`][entry]s.
    ///
    /// [entry]: lib::models::entry::Entry
    pub fn run_filters(&mut self, filter_options: &FilterOptions) {
        // TODO(feat): It might be good to clone `self.data` to allow for filter revisions.
        for filter_type in &filter_options.filter_types {
            // TODO(refactor): Can we qvoid this clone?
            lib::filter::run(filter_type.clone(), &mut self.data);
        }
    }

    /// Runs pre-processes on all [`Entry`][entry]s.
    ///
    /// [entry]: lib::models::entry::Entry
    pub fn run_preprocesses(&mut self, options: PreProcessOptions) {
        lib::process::pre::run(&mut self.data, options);
    }

    /// Prints to the terminal. Allows muting.
    pub fn print<S>(&self, message: S)
    where
        S: AsRef<str>,
    {
        let message: &str = message.as_ref();

        if !self.config.is_quiet {
            println!("{message}");
        }
    }

    // TODO(0.7.0): Redesign this.
    /// Prompts the user to confirm the filter results.
    pub fn confirm_filter_results(&self) -> bool {
        let indent = " ".repeat(3);
        let line = "-".repeat(64);

        println!("{indent}{line}");

        let count_books = self.data.count_books();

        if count_books == 0 {
            println!("{indent}No annotations found.");
            println!("{indent}{line}");
            return false;
        }

        let count_annotations = self.data.count_annotations();

        println!(
            "{indent}Found {count_annotations} annotation{} from {count_books} book{}:",
            if count_annotations == 1 { "" } else { "s" },
            if count_books == 1 { "" } else { "s" },
        );

        for book in self.data.iter_books() {
            println!("{indent} • {} by {}", book.title, book.author);
        }

        println!("{indent}{line}");

        print!("{indent}Continue? [y/N]: ");

        let mut confirm = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut confirm).unwrap();

        println!();

        matches!(confirm.trim().to_lowercase().as_str(), "y" | "yes")
    }
}

impl App<ExtRender> {
    /// Renders templates.
    pub fn render(&mut self) -> CliResult<()> {
        self.data.values_mut().try_for_each(|entry| {
            self.extension
                .renderer
                .render(entry)
                .wrap_err("Failed while rendering template(s)")
        })
    }

    /// Writes templates to disk.
    pub fn write(&self) -> CliResult<()> {
        std::fs::create_dir_all(&self.config.output_directory)?;

        self.extension
            .renderer
            .write(&self.config.output_directory)
            .wrap_err("Failed while writing template(s)")
    }

    /// Runs post-processes on all [`Render`][render]s.
    ///
    /// [render]: lib::render::template::Render
    pub fn run_postprocesses(&mut self, options: PostProcessOptions) {
        lib::process::post::run(
            self.extension.renderer.templates_rendered_mut().collect(),
            options,
        );
    }
}

impl App<ExtExport> {
    /// Exports data to disk.
    pub fn export(&mut self) -> CliResult<()> {
        lib::export::run(
            &mut self.data,
            &self.config.output_directory,
            self.extension.options.clone(),
            // FIXME: Avoid clone? ^^^^^^^
        )
        .wrap_err("Failed while exporting data")?;

        Ok(())
    }
}

impl App<ExtBackup> {
    /// Backs-up source data to disk.
    pub fn backup(&self) -> CliResult<()> {
        lib::backup::run(
            self.config.platform,
            &self.config.data_directory,
            &self.config.output_directory,
            self.extension.options.clone(),
            // FIXME: Avoid clone? ^^^^^^^
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::cli::config::testing::TestConfig;

    // Tests dealing with macOS's Apple Books databases.
    mod macos {

        use super::*;

        // Tests that empty data returns zero books and zero annotations.
        #[test]
        fn test_empty() {
            let config = TestConfig::macos_empty();
            let app = App::new(config).unwrap();

            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that un-annotated books return zero books and zero annotations.
        #[test]
        fn test_books_new() {
            let config = TestConfig::macos_new();
            let app = App::new(config).unwrap();

            // Un-annotated books are filtered out.
            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that annotated books return non-zero books and non-zero annotations.
        #[test]
        fn test_books_annotated() {
            let config = TestConfig::macos_annotated();
            let app = App::new(config).unwrap();

            assert_eq!(app.data.iter_books().count(), 3);
            assert_eq!(app.data.iter_annotations().count(), 10);
        }

        // Tests that annotations are sorted in the correct order.
        #[test]
        fn test_annotations_order() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // The pre-processor sorts the annotations.
            app.run_preprocesses(PreProcessOptions::default());

            for entry in app.data.values() {
                for annotations in entry.annotations.windows(2) {
                    assert!(annotations[0] < annotations[1]);
                }
            }
        }
    }

    // Tests dealing with iOS's Apple Books plists.
    mod ios {

        use super::*;

        // Tests that empty data returns zero books and zero annotations.
        #[test]
        fn test_empty() {
            let config = TestConfig::ios_empty();
            let app = App::new(config).unwrap();

            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that un-annotated books return zero books and zero annotations.
        #[test]
        fn test_books_new() {
            let config = TestConfig::ios_new();
            let app = App::new(config).unwrap();

            // Un-annotated books are filtered out.
            assert_eq!(app.data.iter_books().count(), 0);
            assert_eq!(app.data.iter_annotations().count(), 0);
        }

        // Tests that annotated books return non-zero books and non-zero annotations.
        #[test]
        fn test_books_annotated() {
            let config = TestConfig::ios_annotated();
            let app = App::new(config).unwrap();

            assert_eq!(app.data.iter_books().count(), 3);
            assert_eq!(app.data.iter_annotations().count(), 7);
        }

        // Tests that annotations are sorted in the correct order.
        #[test]
        fn test_annotations_order() {
            let config = TestConfig::ios_annotated();
            let mut app = App::new(config).unwrap();

            // The pre-processor sorts the annotations.
            app.run_preprocesses(PreProcessOptions::default());

            for entry in app.data.values() {
                for annotations in entry.annotations.windows(2) {
                    assert!(annotations[0] < annotations[1]);
                }
            }
        }
    }

    // Tests dealing with filtering annotations.
    mod filter {

        use super::*;

        use crate::cli::filter::{FilterOperator, FilterType};

        // Keeps annotations where their book's title contains either "art" or "think".
        #[test]
        fn test_title_any() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "?title:art think"
            let filter = FilterType::Title {
                query: vec!["art", "think"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::Any,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 2);
            assert_eq!(app.data.iter_annotations().count(), 9);
        }

        // Keeps annotations where their book's title contains both "joking" and "feynman".
        #[test]
        fn test_title_all() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "*title:joking feynman"
            let filter = FilterType::Title {
                query: vec!["joking", "feynman"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::All,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        // Keeps annotations where their book's title exactly matches "the art spirit".
        #[test]
        fn test_title_exact() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "=title:the art spirit"
            let filter = FilterType::Title {
                query: vec!["the", "art", "spirit"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::Exact,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 4);
        }

        // Keeps annotations where their book's author contains either "robert" or "richard".
        #[test]
        fn test_author_any() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "?author:robert richard"
            let filter = FilterType::Author {
                query: vec!["robert", "richard"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::Any,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 2);
            assert_eq!(app.data.iter_annotations().count(), 5);
        }

        // Keeps annotations where their book's author contains both "richard" and "feyman".
        #[test]
        fn test_author_all() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "*author:richard feynman"
            let filter = FilterType::Author {
                query: vec!["richard", "feynman"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::All,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        // Keeps annotations where their book's author exactly matches "richard p. feynman".
        #[test]
        fn test_author_exact() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "=author:richard p. feynman"
            let filter = FilterType::Author {
                query: vec!["richard", "p.", "feynman"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::Exact,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        // Keeps annotations where their tags contain either "#artst" or "#death".
        #[test]
        fn test_tags_any() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "?tags:#artist #death"
            let filter = FilterType::Tags {
                query: vec!["#artist", "#death"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::Any,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            // The pre-processor extracts the tags.
            app.run_preprocesses(PreProcessOptions {
                extract_tags: true,
                ..Default::default()
            });

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 2);
            assert_eq!(app.data.iter_annotations().count(), 2);
        }

        // Keeps annotations where their tags contain both "#death" and "#impermanence".
        #[test]
        fn test_tags_all() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "*tags:#death #impermanence"
            let filter = FilterType::Tags {
                query: vec!["#death", "#impermanence"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::All,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            // The pre-processor extracts the tags.
            app.run_preprocesses(PreProcessOptions {
                extract_tags: true,
                ..Default::default()
            });

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }

        // Keeps annotations where their tags contain exactly "#artist" and "#being".
        #[test]
        fn test_tags_exact() {
            let config = TestConfig::macos_annotated();
            let mut app = App::new(config).unwrap();

            // aka "=tags:#artist #being"
            let filter = FilterType::Tags {
                query: vec!["#artist", "#being"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                operator: FilterOperator::Exact,
            };

            let filter_options = FilterOptions {
                filter_types: vec![filter],
                auto_confirm: true,
            };

            // The pre-processor extracts the tags.
            app.run_preprocesses(PreProcessOptions {
                extract_tags: true,
                ..Default::default()
            });

            app.run_filters(&filter_options);

            assert_eq!(app.data.iter_books().count(), 1);
            assert_eq!(app.data.iter_annotations().count(), 1);
        }
    }
}

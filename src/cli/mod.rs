pub mod app;
pub mod args;
pub mod config;
pub mod data;
pub mod defaults;
pub mod filter;
pub mod utils;

use lib::applebooks::macos::utils::applebooks_is_running;

use app::App;
use args::{Command, Platform};
use config::Config;

pub type CliResult<T> = color_eyre::Result<T>;

pub fn run(command: Command) -> CliResult<()> {
    log::debug!("{:#?}", &command);

    match command {
        Command::Render {
            platform,
            render_options,
            filter_options,
            preprocess_options,
            postprocess_options,
            global_options,
        } => {
            if warn_and_exit(platform, global_options.is_force) {
                return Ok(());
            }

            let config = Config::new(platform.into(), global_options)?;

            let mut app = App::new(config)?.into_render(render_options)?;

            if !filter_options.filter_types.is_empty() {
                app.run_filters(&filter_options);

                if !filter_options.auto_confirm && !app.confirm_filter_results() {
                    return Ok(());
                }
            }

            app.print(format!("Rendering {platform} annotations..."));

            app.run_preprocesses(preprocess_options);
            app.render()?;
            app.run_postprocesses(postprocess_options);
            app.write()?;
        }
        Command::Export {
            platform,
            export_options,
            filter_options,
            preprocess_options,
            global_options,
        } => {
            if warn_and_exit(platform, global_options.is_force) {
                return Ok(());
            }

            let config = Config::new(platform.into(), global_options)?;

            let mut app = App::new(config)?.into_export(export_options);

            if !filter_options.filter_types.is_empty() {
                app.run_filters(&filter_options);

                if !filter_options.auto_confirm && !app.confirm_filter_results() {
                    return Ok(());
                }
            }

            app.print(format!("Exporting {platform} annotations..."));

            app.run_preprocesses(preprocess_options);
            app.export()?;
        }
        Command::Backup {
            platform,
            backup_options,
            global_options,
        } => {
            if warn_and_exit(platform, global_options.is_force) {
                return Ok(());
            }

            let config = Config::new(platform.into(), global_options)?;

            let app = App::new(config)?.into_backup(backup_options);

            app.print(format!("Backing-up {platform} data..."));

            app.backup()?;
        }
    };

    Ok(())
}

fn warn_and_exit(platform: Platform, is_force: bool) -> bool {
    if let Platform::IOs = platform {
        return false;
    }

    if !is_force && applebooks_is_running() {
        println!("Apple Books is currently running. To ignore this, use the `-f, --force` flag.");
        return true;
    }

    false
}

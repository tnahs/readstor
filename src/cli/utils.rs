use crate::cli;

pub fn init_logger() {
    let env = env_logger::Env::default().filter(cli::defaults::READSTOR_LOG);
    env_logger::init_from_env(env);
}

/// Returns a `bool` representing if the application is being developed or not.
/// The state is determined by whether or not an environment variable is set.
/// See [`cli::defaults::READSTOR_DEV`] for more information.
pub fn is_development_env() -> bool {
    match std::env::var_os(cli::defaults::READSTOR_DEV) {
        // Ensures that, if the variable exists but is empty, the function will
        // return false.
        Some(value) => !value.is_empty(),
        None => false,
    }
}

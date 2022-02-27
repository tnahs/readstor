use crate::cli;

/// TODO Document
pub fn init_logger() {
    let env = env_logger::Env::default().filter(cli::defaults::READSTOR_LOG);
    env_logger::init_from_env(env);
}

use super::defaults as cli_defaults;

pub fn init_logger() {
    let env = env_logger::Env::default().filter(cli_defaults::READSTOR_LOG);
    env_logger::init_from_env(env);
}

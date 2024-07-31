use log::LevelFilter;

pub fn init() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init()
}

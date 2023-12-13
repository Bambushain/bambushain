use std::str::FromStr;

use log::Level;

use crate::pages::app::App;
use crate::storage::get_log_level;

mod api;
mod pages;
mod routing;
mod storage;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(
        Level::from_str(get_log_level().unwrap_or(Level::Warn.to_string()).as_str())
            .unwrap_or(Level::Warn),
    ));

    let environment = gloo::utils::window()
        .location()
        .hostname()
        .unwrap_or("development".to_string())
        .split('.')
        .next()
        .unwrap_or("development")
        .to_string();
    let dsn = env!("SENTRY_DSN");

    if !dsn.is_empty() {
        log::info!("Configure glitchtip");
        let _sentry = sentry::init((
            dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                environment: Some(environment.into()),
                ..Default::default()
            },
        ));
    }

    log::info!("Starting the party");
    yew::Renderer::<App>::new().render();
}

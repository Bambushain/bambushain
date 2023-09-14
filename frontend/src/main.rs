use std::str::FromStr;

use log::Level;

use crate::pages::app::App;
use crate::storage::{get_log_level, is_logging_on};

mod api;
mod hooks;
mod pages;
mod routing;
mod storage;

fn main() {
    if is_logging_on() {
        wasm_logger::init(wasm_logger::Config::new(
            Level::from_str(get_log_level().unwrap_or(Level::Warn.to_string()).as_str())
                .unwrap_or(Level::Warn),
        ));
        log::info!("Logging is turned on");
    }

    log::info!("Starting the party");
    yew::Renderer::<App>::new().render();
}

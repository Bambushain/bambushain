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

    log::info!("Starting the party");
    yew::Renderer::<App>::new().render();
}

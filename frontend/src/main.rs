mod app;

use std::str::FromStr;

use log::Level;

use bamboo_frontend_base_storage::get_log_level;

use crate::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(
        Level::from_str(get_log_level().unwrap_or(Level::Warn.to_string()).as_str())
            .unwrap_or(Level::Warn),
    ));

    yew::Renderer::<App>::new().render();
}

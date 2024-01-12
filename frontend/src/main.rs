use std::str::FromStr;

use log::Level;

use bamboo_frontend_base_storage::get_log_level;

use crate::app::App;

mod app;

fn main() {
    console_log::init_with_level(
        Level::from_str(get_log_level().unwrap_or(Level::Warn.to_string()).as_str())
            .unwrap_or(Level::Warn),
    )
    .expect("error initializing log");

    yew::Renderer::<App>::new().render();
}

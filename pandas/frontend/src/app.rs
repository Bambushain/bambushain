use std::str::FromStr;

use log::Level;
use yew::prelude::*;
use yew_cosmo::prelude::CosmoPageLayout;
use yew_router::prelude::*;

use crate::base::routing::AppRoute;
use crate::base::storage::get_log_level;
use crate::sections::layout::switch;

fn format_title(s: AttrValue) -> AttrValue {
    if s.is_empty() {
        AttrValue::from("Bambushain")
    } else {
        AttrValue::from(format!("Bambushain – {s}"))
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <CosmoPageLayout primary_color="#598C79" primary_color_dark="#598C79" default_title="Bambushain" format_title={format_title}>
            <BrowserRouter>
                <Switch<AppRoute> render={switch}/>
            </BrowserRouter>
        </CosmoPageLayout>
    )
}

pub fn start_frontend() {
    console_log::init_with_level(
        Level::from_str(get_log_level().unwrap_or(Level::Warn.to_string()).as_str())
            .unwrap_or(Level::Warn),
    )
    .expect("error initializing log");

    yew::Renderer::<App>::new().render();
}

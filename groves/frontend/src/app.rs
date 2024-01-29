use bamboo_groves_frontend_sections::layout::Layout;
use log::Level;
use yew::prelude::*;
use yew_cosmo::prelude::CosmoPageLayout;
use yew_router::prelude::*;

fn format_title(s: AttrValue) -> AttrValue {
    if s.is_empty() {
        AttrValue::from("Hainverwaltung")
    } else {
        AttrValue::from(format!("Hainverwaltung – {s}"))
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <CosmoPageLayout primary_color="#6275D5" primary_color_dark="#6275D5" default_title="Hainverwaltung" format_title={format_title}>
            <BrowserRouter>
                <Layout />
            </BrowserRouter>
        </CosmoPageLayout>
    )
}

pub fn start_frontend() {
    console_log::init_with_level(Level::Debug).expect("error initializing log");

    yew::Renderer::<App>::new().render();
}

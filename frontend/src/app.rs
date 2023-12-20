use yew::prelude::*;
use yew_cosmo::prelude::CosmoPageLayout;
use yew_router::prelude::*;

use bamboo_frontend_base_routing::AppRoute;
use bamboo_frontend_section_layout::switch;

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

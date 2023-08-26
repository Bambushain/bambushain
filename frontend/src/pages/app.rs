use yew::prelude::*;
use yew_cosmo::prelude::CosmoPageLayout;
use yew_router::prelude::*;

use crate::pages::layout::switch;
use crate::routing::AppRoute;

fn format_title(s: AttrValue) -> AttrValue {
    if s.is_empty() {
        AttrValue::from("Pandaparty")
    } else {
        AttrValue::from(format!("{} – Pandaparty", s))
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <CosmoPageLayout primary_color="#9F2637" primary_color_dark="#9F2637" default_title="Pandaparty" format_title={format_title}>
            <BrowserRouter>
                <Switch<AppRoute> render={switch}/>
            </BrowserRouter>
        </CosmoPageLayout>
    )
}

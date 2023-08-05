use bounce::BounceRoot;
use bounce::helmet::HelmetBridge;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routing::AppRoute;

use crate::pages::layout::switch;

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
        <BounceRoot>
            <HelmetBridge default_title="Pandaparty" format_title={format_title} />
            <BrowserRouter>
                <Switch<AppRoute> render={switch}/>
            </BrowserRouter>
        </BounceRoot>
    )
}

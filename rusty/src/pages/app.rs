use yew::prelude::*;
use yew_router::prelude::*;
use bounce::BounceRoot;
use bounce::helmet::HelmetBridge;
use crate::pages::login::LoginPage;
use crate::pages::sheef::SheefLayout;
use crate::routing::AppRoute;

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => {
            log::debug!("Render login");
            html!(<LoginPage />)
        }
        AppRoute::Sheef | AppRoute::SheefRoot => {
            log::debug!("Render sheef main page");
            html!(<SheefLayout />)
        }
    }
}

fn format_title(s: AttrValue) -> AttrValue {
    if s.is_empty() {
        AttrValue::from("Sheef Planung")
    } else {
        AttrValue::from(format!("{} – Sheef Planung", s))
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <BounceRoot>
            <HelmetBridge default_title="" format_title={format_title} />
            <BrowserRouter>
                <Switch<AppRoute> render={switch}/>
            </BrowserRouter>
        </BounceRoot>
    )
}

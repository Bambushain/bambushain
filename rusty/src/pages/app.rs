use yew::prelude::*;
use yew_router::prelude::*;
use bounce::BounceRoot;
use crate::pages::login::Login;
use crate::pages::sheef::Sheef;
use crate::routing::AppRoute;

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => {
            log::debug!("Render login");
            html!(<Login />)
        },
        AppRoute::Sheef | AppRoute::SheefRoot => {
            log::debug!("Render sheef main page");
            html!(<Sheef />)
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <BounceRoot>
            <BrowserRouter>
                <Switch<AppRoute> render={switch}/>
            </BrowserRouter>
        </BounceRoot>
    )
}

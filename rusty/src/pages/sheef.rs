use bounce::query::use_query_value;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::api::my::Profile;
use crate::pages::calendar::Calendar;
use crate::routing::{AppRoute, SheefRoute};

fn switch(route: SheefRoute) -> Html {
    match route {
        SheefRoute::Calendar => html!(<Calendar />)
    }
}

#[function_component(Sheef)]
pub fn sheef() -> Html {
    let authentication_state_query = use_query_value::<Profile>(().into());

    match authentication_state_query.result() {
        Some(query_result) => match query_result {
            Ok(_) => html!(
                <BrowserRouter>
                    <Switch<SheefRoute> render={switch}/>
                </BrowserRouter>
            ),
            Err(_) => {
                log::debug!("First render, so lets send the request to check if the token is valid and see");
                html!(
                    <Redirect<AppRoute> to={AppRoute::Login} />
                )
            }
        },
        None => html!()
    }
}

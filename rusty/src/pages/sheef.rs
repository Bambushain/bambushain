use bounce::query::use_query_value;
use bounce::use_atom_setter;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::api::authentication::logout;
use crate::api::my::Profile;
use crate::pages::calendar::CalendarPage;
use crate::pages::crew::CrewPage;
use crate::routing::{AppRoute, SheefRoute};
use crate::storage::CurrentUser;

fn switch(route: SheefRoute) -> Html {
    match route {
        SheefRoute::Home => html!(<Redirect<SheefRoute> to={SheefRoute::Calendar} />),
        SheefRoute::Calendar => html!(<CalendarPage />),
        SheefRoute::Crew => html!(<CrewPage />),
    }
}

#[function_component(SheefLayout)]
pub fn sheef_layout() -> Html {
    let authentication_state_query = use_query_value::<Profile>(().into());
    let navigator = use_navigator();
    let on_logout = use_callback(move |evt: MouseEvent, _| {
        evt.prevent_default();
        let navigator = navigator.clone();
        yew::platform::spawn_local(async { logout().await });
        navigator.expect("Navigator should be available").push(&AppRoute::Login);
    }, ());
    let route = use_route::<SheefRoute>().unwrap_or_default();
    let profile_atom_setter = use_atom_setter::<CurrentUser>();
    log::debug!("Current route {}", route);

    match authentication_state_query.result() {
        Some(query_result) => match query_result {
            Ok(profile) => {
                profile_atom_setter(CurrentUser { profile: profile.user.clone() });
                html!(
                    <BrowserRouter>
                        <nav class="container-fluid">
                            <ul>
                                <li><strong>{"Sheef"}</strong></li>
                                <li><Link<SheefRoute> to={SheefRoute::Calendar}>{"Kalender"}</Link<SheefRoute>></li>
                                <li><Link<SheefRoute> to={SheefRoute::Crew}>{"Crew"}</Link<SheefRoute>></li>
                                <li><a href="/crafters">{"Crafters"}</a></li>
                                <li><a href="/fighters">{"Kämpfer"}</a></li>
                                <li><a href="/mounts">{"Mounts"}</a></li>
                                <li><a href="/savage-mounts">{"Savage Mounts"}</a></li>
                                <li><a href="/kills">{"Kills"}</a></li>
                            </ul>
                            <ul>
                                <li role="list" dir="rtl">
                                    <a href="#" aria-haspopup="listbox">{"Mein Sheef"}</a>
                                    <ul role="listbox">
                                        <li><a href="/me">{"Mein Profil"}</a></li>
                                        <li><a href="/my-mounts">{"Meine Mounts"}</a></li>
                                        <li><a href="/my-savage-mounts">{"Meine Savage Mounts"}</a></li>
                                        <li><a href="/my-kills">{"Meine Kills"}</a></li>
                                        <li></li>
                                        <li><a href="/my-password">{"Mein Passwort"}</a></li>
                                        <li><a onclick={on_logout}>{"Abmelden"}</a></li>
                                    </ul>
                                </li>
                            </ul>
                        </nav>
                        <div class="container-fluid">
                            <Switch<SheefRoute> render={switch}/>
                        </div>
                    </BrowserRouter>
                )
            }
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

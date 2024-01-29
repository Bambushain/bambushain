use bounce::helmet::Helmet;
use gloo_utils::window;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::*;

use bamboo_groves_frontend_base::routing::AppRoute;
use bamboo_groves_frontend_section_groves::GrovesPage;
use bamboo_groves_frontend_section_login::LoginPage;

use crate::api;

fn switch_sub_menu(route: AppRoute) -> Html {
    match route {
        AppRoute::Groves => html!(),
        AppRoute::Home => html!(),
    }
}

fn switch_app(route: AppRoute) -> Html {
    match route {
        AppRoute::Groves => html!(
            <>
                <Helmet>
                    <title>{"Haine"}</title>
                </Helmet>
                <GrovesPage />
            </>
        ),
        AppRoute::Home => html!(
            <Redirect<AppRoute> to={AppRoute::Groves} />
        ),
    }
}

fn render_main_menu_entry(
    label: impl Into<AttrValue> + Clone,
    to: AppRoute,
    active: AppRoute,
) -> impl Fn(AppRoute) -> Html {
    move |route| {
        let is_active = route.eq(&active) || route.eq(&to);

        html!(
            <CosmoMainMenuItemLink<AppRoute> to={to.clone()} label={label.clone().into()} is_active={is_active} />
        )
    }
}

fn render_sub_menu_entry<Route: Routable + Clone + 'static>(
    label: impl Into<AttrValue> + Clone,
    to: Route,
) -> impl Fn(Route) -> Html {
    move |route| {
        let is_active = route.eq(&to);

        html!(
            <CosmoSubMenuItemLink<Route> to={to.clone()} label={label.clone().into()} is_active={is_active} />
        )
    }
}

#[autoprops]
#[function_component(Layout)]
pub fn layout() -> Html {
    log::debug!("Render app layout");
    let profile_picture = AttrValue::from("/static/logo.webp");

    let logout = use_callback((), |_, _| {
        let _ = window().location().set_href("/api/logout");
    });
    let authenticated_state =
        use_async(async move { api::check_authentication().await.map(|_| true) });

    {
        let authenticated_state = authenticated_state.clone();

        use_mount(move || {
            log::debug!(
                "First render, so lets send the request to check if the token is valid and see"
            );
            authenticated_state.run();
        });
    }

    html!(
        if authenticated_state.loading {
            <CosmoProgressRing />
        } else if authenticated_state.error.is_some() {
            <>
                <Helmet>
                    <title>{"Anmelden"}</title>
                </Helmet>
                <LoginPage />
            </>
        } else if authenticated_state.data.is_some() {
            <>
                <CosmoTopBar profile_picture={profile_picture} has_right_item={true} right_item_on_click={logout} right_item_label="Abmelden"/>
                <CosmoMenuBar>
                    <CosmoMainMenu>
                        <Switch<AppRoute> render={render_main_menu_entry("Haine", AppRoute::Groves, AppRoute::Groves)} />
                    </CosmoMainMenu>
                    <Switch<AppRoute> render={switch_sub_menu} />
                </CosmoMenuBar>
                <CosmoPageBody>
                    <Switch<AppRoute> render={switch_app} />
               </CosmoPageBody>
            </>
        }
    )
}

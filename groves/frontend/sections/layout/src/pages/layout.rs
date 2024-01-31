use bounce::helmet::Helmet;
use gloo_utils::window;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::*;

use bamboo_groves_frontend_base::routing::AppRoute;
use bamboo_groves_frontend_section_groves::GrovesPage;
use bamboo_groves_frontend_section_login::LoginPage;
use bamboo_groves_frontend_section_users::UsersPage;

use crate::api;

fn switch_app(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html!(
            <Redirect<AppRoute> to={AppRoute::Groves} />
        ),
        AppRoute::Groves => html!(
            <>
                <Helmet>
                    <title>{"Haine"}</title>
                </Helmet>
                <GrovesPage />
            </>
        ),
        AppRoute::Users { grove_id } => html!(
            <>
                <Helmet>
                    <title>{"Benutzer"}</title>
                </Helmet>
                <UsersPage grove_id={grove_id} />
            </>
        ),
    }
}

fn render_groves_route_sub_menu_entry(route: AppRoute) -> Html {
    let is_active = matches!(route, AppRoute::Groves);

    html!(
        <CosmoSubMenuItemLink<AppRoute> to={AppRoute::Groves} is_active={is_active} label="Haine" />
    )
}

fn render_users_route_sub_menu_entry(route: AppRoute) -> Html {
    let is_active = matches!(route, AppRoute::Users { .. });

    html!(
        <CosmoSubMenuItem label="Benutzer" is_active={is_active} />
    )
}

#[function_component(Layout)]
pub fn layout() -> Html {
    log::debug!("Render app layout");
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
                <CosmoTopBar profile_picture="/static/logo.webp" has_right_item={true} right_item_on_click={logout} right_item_label="Abmelden"/>
                <CosmoMenuBar>
                    <CosmoMainMenu>
                         <CosmoMainMenuItemLink<AppRoute> to={AppRoute::Groves} label="Hainverwaltung" is_active={true} />
                    </CosmoMainMenu>
                    <CosmoSubMenuBar>
                        <Switch<AppRoute> render={render_groves_route_sub_menu_entry} />
                        <Switch<AppRoute> render={render_users_route_sub_menu_entry} />
                    </CosmoSubMenuBar>
                </CosmoMenuBar>
                <CosmoPageBody>
                    <Switch<AppRoute> render={switch_app} />
               </CosmoPageBody>
            </>
        }
    )
}

use std::fmt::Display;
use bounce::helmet::Helmet;
use gloo_storage::{SessionStorage, Storage};
use gloo_utils::{head, window};
use url::Url;
use web_sys::Element;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_oauth2::agent::{LoginOptions, OAuth2Operations};
use yew_oauth2::hook::use_latest_access_token;
use yew_oauth2::openid::{Config, OAuth2, use_auth_agent};
use yew_oauth2::prelude::{Authenticated, NotAuthenticated};
use yew_router::prelude::*;

use bamboo_groves_frontend_base::routing::AppRoute;
use bamboo_groves_frontend_section_groves::GrovesPage;
use bamboo_groves_frontend_section_login::LoginPage;
use bamboo_groves_frontend_section_users::UsersPage;

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

#[function_component(StoreToken)]
fn store_token() -> Html {
    let access_token = use_latest_access_token();
    let token_name = "/bamboo/access-token";
    if let Some(token) = access_token {
        let _ = SessionStorage::set(token_name, token.access_token());
    } else {
        let _ = SessionStorage::delete(token_name);
    }

    html!()
}

#[function_component(LayoutInner)]
fn layout_inner() -> Html {
    log::debug!("Render app layout");
    let agent = use_auth_agent().expect("Must be nested inside an OAuth2 component");

    let login = use_callback(agent.clone(), |_, agent| {
        let _ = agent.start_login();
    });
    let logout = use_callback(agent, |_, agent| {
        let _ = agent.logout();
    });

    html!(
        <>
            <NotAuthenticated>
                <Helmet>
                    <title>{"Anmelden"}</title>
                </Helmet>
                <LoginPage on_login={login} />
            </NotAuthenticated>
            <Authenticated>
                <StoreToken/>
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
           </Authenticated>
        </>
    )
}

fn get_meta_attribute(key: impl Into<String> + Display) -> String {
    let error = format!("{key} must be set");
    head()
        .query_selector(format!("meta[name={key}]").as_str())
        .expect(error.as_str())
        .expect(error.as_str())
        .get_attribute("content")
        .expect(error.as_str())
}

fn get_meta_attributes(key: impl Into<String> + Display) -> Vec<String> {
    let error = format!("{key} must be set");
    let nodes = head()
        .query_selector_all(format!("meta[name={key}]").as_str())
        .expect(error.as_str());

    let mut values = vec![];
    for i in 0..nodes.length() {
        let node = nodes.get(i);
        let value = node
            .expect(error.as_str())
            .dyn_ref::<Element>()
            .expect(error.as_str())
            .get_attribute("content")
            .expect(error.as_str());
        values.push(value);
    };

    values
}

#[function_component(Layout)]
pub fn layout() -> Html {
    let redirect_url = window().location().origin().expect("Should have origin");
    let login_options = LoginOptions::new().with_redirect_url(Url::parse(redirect_url.as_str()).unwrap());
    let config = Config::new(get_meta_attribute("bamboo-client-id"), get_meta_attribute("bamboo-issuer-url"))
        .with_additional_trusted_audiences(get_meta_attributes("bamboo-aud"))
        .with_after_logout_url(redirect_url);

    html!(
        <OAuth2 {config} scopes={vec!["openid".to_string(), "profile".to_string()]} login_options={login_options}>
            <LayoutInner />
        </OAuth2>
    )
}

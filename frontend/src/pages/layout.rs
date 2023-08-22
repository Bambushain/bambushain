use bounce::{use_atom_setter, use_atom_value};
use bounce::helmet::{Helmet, HelmetBridge};
use bounce::query::use_query_value;
use stylist::{css, GlobalStyle};
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_router::prelude::*;

use pandaparty_entities::user::UpdateProfile;

use crate::{api, storage};
use crate::pages::final_fantasy::crafter::CrafterPage;
use crate::pages::final_fantasy::fighter::FighterPage;
use crate::pages::login::LoginPage;
use crate::pages::pandaparty::calendar::CalendarPage;
use crate::pages::pandaparty::user::UsersPage;
use crate::routing::{AppRoute, FinalFantasyRoute, PandaPartyRoute};

#[derive(Properties, Clone, PartialEq)]
struct ChangePasswordDialogProps {
    on_close: Callback<()>,
    mods: Vec<AttrValue>,
}

#[derive(Properties, Clone, PartialEq)]
struct UpdateMyProfileDialogProps {
    on_close: Callback<()>,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(<LoginPage />),
        _ => html!(<Layout />)
    }
}

fn switch_sub_menu(route: AppRoute) -> Html {
    match route {
        AppRoute::PandaPartyRoot | AppRoute::PandaParty => html!(
            <CosmoSubMenuBar>
                <Switch<PandaPartyRoute> render={render_sub_menu_entry("Event Kalender".into(), PandaPartyRoute::Calendar)} />
                <Switch<PandaPartyRoute> render={render_sub_menu_entry("Party People".into(), PandaPartyRoute::User)} />
            </CosmoSubMenuBar>
        ),
        AppRoute::FinalFantasyRoot | AppRoute::FinalFantasy => html!(
            <CosmoSubMenuBar>
                <Switch<FinalFantasyRoute> render={render_sub_menu_entry("Meine Crafter".into(), FinalFantasyRoute::Crafter)} />
                <Switch<FinalFantasyRoute> render={render_sub_menu_entry("Meine Kämpfer".into(), FinalFantasyRoute::Fighter)} />
            </CosmoSubMenuBar>
        ),
        _ => {
            log::debug!("Other");
            html!()
        }
    }
}

fn switch_final_fantasy(route: FinalFantasyRoute) -> Html {
    match route {
        FinalFantasyRoute::Crafter => html!(
            <>
                <Helmet>
                    <title>{"Meine Crafter"}</title>
                </Helmet>
                <CrafterPage />
            </>
        ),
        FinalFantasyRoute::Fighter => html!(
            <>
                <Helmet>
                    <title>{"Meine Kämpfer"}</title>
                </Helmet>
                <FighterPage />
            </>
        ),
    }
}

fn switch_panda_party(route: PandaPartyRoute) -> Html {
    match route {
        PandaPartyRoute::Calendar => html!(
            <>
                <Helmet>
                    <title>{"Event Kalender"}</title>
                </Helmet>
                <CalendarPage />
            </>
        ),
        PandaPartyRoute::User => html!(
            <>
                <Helmet>
                    <title>{"Party People"}</title>
                </Helmet>
                <UsersPage />
            </>
        ),
    }
}

fn switch_app(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html!(
            <Redirect<AppRoute> to={AppRoute::PandaPartyRoot} />
        ),
        AppRoute::PandaPartyRoot => html!(
            <Redirect<PandaPartyRoute> to={PandaPartyRoute::Calendar} />
        ),
        AppRoute::PandaParty => html!(
            <>
                <Helmet>
                    <title>{"Pandaparty"}</title>
                </Helmet>
                <Switch<PandaPartyRoute> render={switch_panda_party} />
            </>
        ),
        AppRoute::FinalFantasyRoot => html!(
            <Redirect<FinalFantasyRoute> to={FinalFantasyRoute::Crafter} />
        ),
        AppRoute::FinalFantasy => html!(
            <>
                <Helmet>
                    <title>{"Final Fantasy"}</title>
                </Helmet>
                <Switch<FinalFantasyRoute> render={switch_final_fantasy} />
            </>
        ),
        AppRoute::Login => html!(),
    }
}

fn render_main_menu_entry(label: AttrValue, to: AppRoute, active: AppRoute) -> impl Fn(AppRoute) -> Html {
    move |route| {
        let is_active = route.eq(&active);

        html!(
            <CosmoMainMenuItemLink<AppRoute> to={to.clone()} label={label.clone()} is_active={is_active} />
        )
    }
}

fn render_sub_menu_entry<Route: Routable + Clone + 'static>(label: AttrValue, to: Route) -> impl Fn(Route) -> Html {
    move |route| {
        let is_active = route.eq(&to);

        html!(
            <CosmoSubMenuItemLink<Route> to={to.clone()} label={label.clone()} is_active={is_active} />
        )
    }
}

fn switch_top_bar(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(),
        _ => html!(
            <TopBar />
        )
    }
}

#[function_component(AppLayout)]
fn app_layout() -> Html {
    let initial_loaded_state = use_state_eq(|| false);

    let profile_query = use_query_value::<api::Profile>(().into());
    let profile_atom = use_atom_setter::<storage::CurrentUser>();

    match profile_query.result() {
        None => html!(),
        Some(Ok(res)) => {
            initial_loaded_state.set(true);
            log::debug!("Got user {:?}", res.user.clone());
            profile_atom(res.user.clone().into());
            html!(
                <>
                    <Switch<AppRoute> render={switch_top_bar}/>
                    <CosmoMenuBar>
                        <CosmoMainMenu>
                            <Switch<AppRoute> render={render_main_menu_entry("Pandaparty".into(), AppRoute::PandaPartyRoot, AppRoute::PandaParty)} />
                            <Switch<AppRoute> render={render_main_menu_entry("Final Fantasy".into(), AppRoute::FinalFantasyRoot, AppRoute::FinalFantasy)} />
                        </CosmoMainMenu>
                        <Switch<AppRoute> render={switch_sub_menu} />
                    </CosmoMenuBar>
                    <CosmoPageBody>
                        <Switch<AppRoute> render={switch_app} />
                    </CosmoPageBody>
                </>
            )
        }
        Some(Err(_)) => {
            log::debug!("First render, so lets send the request to check if the token is valid and see");
            html!(
                <Redirect<AppRoute> to={AppRoute::Login} />
            )
        }
    }
}

#[function_component(ChangePasswordDialog)]
fn change_password_dialog(props: &ChangePasswordDialogProps) -> Html {
    log::debug!("Open dialog to change password");
    let navigator = use_navigator();

    let error_state = use_state_eq(|| false);

    let error_header_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let old_password_state = use_state_eq(|| AttrValue::from(""));
    let new_password_state = use_state_eq(|| AttrValue::from(""));

    let update_old_password = use_callback(|value, state| state.set(value), old_password_state.clone());
    let update_new_password = use_callback(|value, state| state.set(value), new_password_state.clone());

    let on_close = props.on_close.clone();
    let on_save = {
        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_header_state = error_header_state.clone();
        let old_password_state = old_password_state.clone();
        let new_password_state = new_password_state.clone();

        Callback::from(move |_| {
            log::debug!("Perform password change");
            let navigator = navigator.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_header_state = error_header_state.clone();
            let old_password_state = old_password_state.clone();
            let new_password_state = new_password_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match api::change_my_password((*old_password_state).to_string(), (*new_password_state).to_string()).await {
                    Ok(_) => {
                        log::debug!("Password change was successful, now logout");
                        api::logout();
                        navigator.expect("Navigator should be available").push(&AppRoute::Login);

                        false
                    }
                    Err(err) => match err.code {
                        api::FORBIDDEN => {
                            log::warn!("The old password is wrong");
                            error_message_state.set("Wenn du möchtest dass es von einem Mod zurückgesetzt wird, einfach anschreiben".into());
                            error_header_state.set("Das alte Passwort ist falsch".into());

                            true
                        }
                        api::NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set("Bitte versuch es erneut um einen Fehler auszuschließen".into());
                            error_header_state.set("Du wurdest scheinbar gelöscht".into());

                            true
                        }
                        _ => {
                            log::warn!("Failed to change the password {err}");
                            error_message_state.set("Leider konnte dein Passwort nicht geändert werden, bitte wende dich an Azami".into());
                            error_header_state.set("Fehler beim ändern".into());

                            true
                        }
                    }
                });
            });
        })
    };

    html!(
        <>
            <Helmet>
                <title>{"Passwort ändern"}</title>
            </Helmet>
            <CosmoModal title="Passwort ändern" is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="Passwort ändern" />
                </>
            )}>
                if *error_state {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} header={(*error_header_state).clone()} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Information} message={format!("Falls du dich an dein altes Passwort nicht erinnern kannst,\nwende dich an einen Mod: {}", props.mods.join(", "))} header="Ändere dein Passwort" />
                }
                <CosmoInputGroup>
                    <CosmoTextBox input_type={CosmoTextBoxType::Password} label="Aktuelles Passwort" on_input={update_old_password} value={(*old_password_state).clone()} required={true} />
                    <CosmoTextBox input_type={CosmoTextBoxType::Password} label="Neues Passwort" on_input={update_new_password} value={(*new_password_state).clone()} required={true} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(UpdateMyProfileDialog)]
fn update_my_profile_dialog(props: &UpdateMyProfileDialogProps) -> Html {
    log::debug!("Open dialog to update profile");
    let authentication_state_query = use_query_value::<api::Profile>(().into());

    let user_atom = use_atom_value::<storage::CurrentUser>();

    let error_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let job_state = use_state_eq(|| AttrValue::from(user_atom.profile.job.clone()));
    let gear_level_state = use_state_eq(|| AttrValue::from(user_atom.profile.gear_level.clone()));
    let discord_name_state = use_state_eq(|| AttrValue::from(user_atom.profile.discord_name.clone()));

    let update_job = use_callback(|value, state| state.set(value), job_state.clone());
    let update_gear_level = use_callback(|value, state| state.set(value), gear_level_state.clone());
    let update_discord_name = use_callback(|value, state| state.set(value), discord_name_state.clone());

    let on_close = props.on_close.clone();
    let on_save = {
        let authentication_state_query = authentication_state_query;

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let job_state = job_state.clone();
        let gear_level_state = gear_level_state.clone();
        let discord_name_state = discord_name_state.clone();

        let on_close = on_close.clone();

        Callback::from(move |_| {
            log::debug!("Perform profile update");
            let authentication_state_query = authentication_state_query.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let job_state = job_state.clone();
            let gear_level_state = gear_level_state.clone();
            let discord_name_state = discord_name_state.clone();

            let on_close = on_close.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match api::update_my_profile(UpdateProfile::new((*job_state).to_string(), (*gear_level_state).to_string(), (*discord_name_state).to_string())).await {
                    Ok(_) => {
                        log::debug!("Profile update successful");
                        let _ = authentication_state_query.refresh().await;

                        on_close.emit(());

                        false
                    }
                    Err(err) => match err.code {
                        api::NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set("Du wurdest scheinbar gelöscht, bitte versuch es erneut um einen Fehler auszuschließen".into());

                            true
                        }
                        _ => {
                            log::warn!("Failed to update the profile {err}");
                            error_message_state.set("Dein Profil konnte leider nicht geändert werden, bitte wende dich an Azami".into());

                            true
                        }
                    }
                });
            });
        })
    };

    html!(
        <>
            <Helmet>
                <title>{"Profil bearbeiten"}</title>
            </Helmet>
            <CosmoModal title="Profil bearbeiten" is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="Profil speichern" />
                </>
            )}>
                if *error_state {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} header="Fehler beim Ändern" />
                }
                <CosmoInputGroup>
                    <CosmoTextBox label="Rolle/Klasse (optional)" on_input={update_job} value={(*job_state).clone()} />
                    <CosmoTextBox label="Gear Level (optional)" on_input={update_gear_level} value={(*gear_level_state).clone()} />
                    <CosmoTextBox label="Discord Name (optional)" on_input={update_discord_name} value={(*discord_name_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(TopBar)]
fn top_bar() -> Html {
    let navigator = use_navigator().expect("Navigator should be available");

    let mods_state = use_state_eq(|| vec![] as Vec<AttrValue>);

    let profile_open_state = use_state_eq(|| false);
    let password_open_state = use_state_eq(|| false);

    let logout = use_callback(|_: (), navigator| {
        api::authentication::logout();
        navigator.push(&AppRoute::Login);
    }, navigator);
    let update_my_profile_click = use_callback(|_, profile_open_state| profile_open_state.set(true), profile_open_state.clone());
    let change_password_click = {
        let password_open_state = password_open_state.clone();

        let mods_state = mods_state.clone();

        Callback::from(move |_| {
            let password_open_state = password_open_state.clone();

            let mods_state = mods_state.clone();

            yew::platform::spawn_local(async move {
                if let Ok(users) = api::get_users().await {
                    mods_state.set(users
                        .into_iter()
                        .filter_map(|user| if user.is_mod {
                            Some(AttrValue::from(user.username))
                        } else {
                            None
                        })
                        .collect::<Vec<AttrValue>>());
                }

                password_open_state.set(true);
            });
        })
    };

    html!(
        <>
            <CosmoTopBar has_right_item={true} right_item_on_click={logout} right_item_label="Abmelden">
                <CosmoTopBarItem label="Mein Profil" on_click={update_my_profile_click} />
                <CosmoTopBarItem label="Passwort ändern" on_click={change_password_click} />
            </CosmoTopBar>
            if *profile_open_state {
                <UpdateMyProfileDialog on_close={move |_| profile_open_state.set(false)} />
            }
            if *password_open_state {
                <ChangePasswordDialog on_close={move |_| password_open_state.set(false)} mods={(*mods_state).clone()} />
            }
        </>
    )
}

fn format_title(s: AttrValue) -> AttrValue {
    if s.is_empty() {
        AttrValue::from("Pandaparty")
    } else {
        AttrValue::from(format!("{} – Pandaparty", s))
    }
}

#[function_component(Layout)]
fn layout() -> Html {
    log::info!("Run layout");
    let global_style = GlobalStyle::new(css!(r#"
body {
    height: 100vh;
    width: 100vw;
    background: var(--background) !important;
    background-size: cover !important;
    background-position-y: bottom !important;
    background-position-x: right !important;
    --background: url("/static/background.webp");
}

@media screen and (prefers-color-scheme: dark) {
    body {
        --background: url("/static/background-dark.webp");
    }
}"#)).expect("Should create global style");

    html!(
        <CosmoPageLayout primary_color="#9F2637" primary_color_dark="#9F2637">
            <HelmetBridge default_title="Pandaparty" format_title={format_title} />
            <Helmet>
                <style>
                    {global_style.get_style_str()}
                </style>
            </Helmet>
            <AppLayout />
        </CosmoPageLayout>
    )
}
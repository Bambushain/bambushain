use std::ops::Deref;

use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use bounce::{use_atom_setter, use_atom_value};
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::use_unmount;
use yew_router::prelude::*;

use bamboo_entities::user::UpdateProfile;

use crate::pages::bamboo::calendar::CalendarPage;
use crate::pages::bamboo::user::UsersPage;
use crate::pages::final_fantasy::character::CharacterPage;
use crate::pages::final_fantasy::settings::SettingsPage;
use crate::pages::login::LoginPage;
use crate::routing::{AppRoute, BambooGroveRoute, FinalFantasyRoute, SupportRoute};
use crate::{api, error, storage};

use super::support::contact::ContactPage;

#[derive(Properties, Clone, PartialEq)]
struct ChangePasswordDialogProps {
    on_close: Callback<()>,
    mods: Vec<AttrValue>,
}

#[derive(Properties, Clone, PartialEq)]
struct UpdateMyProfileDialogProps {
    on_close: Callback<()>,
}

#[derive(Properties, Clone, PartialEq)]
struct EnableTotpDialogProps {
    on_close: Callback<()>,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(<LoginPage />),
        _ => html!(<Layout />),
    }
}

fn switch_sub_menu(route: AppRoute) -> Html {
    match route {
        AppRoute::BambooGroveRoot | AppRoute::BambooGrove => html!(
            <CosmoSubMenuBar>
                <Switch<BambooGroveRoute> render={render_sub_menu_entry("Event Kalender", BambooGroveRoute::Calendar)} />
                <Switch<BambooGroveRoute> render={render_sub_menu_entry("Pandas", BambooGroveRoute::User)} />
            </CosmoSubMenuBar>
        ),
        AppRoute::FinalFantasyRoot | AppRoute::FinalFantasy => html!(
            <CosmoSubMenuBar>
                <Switch<FinalFantasyRoute> render={render_sub_menu_entry("Meine Charaktere", FinalFantasyRoute::Characters)} />
                <Switch<FinalFantasyRoute> render={render_sub_menu_entry("Personalisierung", FinalFantasyRoute::Settings)} />
            </CosmoSubMenuBar>
        ),
        AppRoute::SupportRoot | AppRoute::Support => html!(
            <CosmoSubMenuBar>
                <Switch<SupportRoute> render={render_sub_menu_entry("Kontakt", SupportRoute::Contact)} />
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
        FinalFantasyRoute::Characters => html!(
            <>
                <Helmet>
                    <title>{"Meine Charaktere"}</title>
                </Helmet>
                <CharacterPage />
            </>
        ),
        FinalFantasyRoute::Settings => html!(
            <>
                <Helmet>
                    <title>{"Personalisierung"}</title>
                </Helmet>
                <SettingsPage />
            </>
        ),
    }
}

fn switch_panda_party(route: BambooGroveRoute) -> Html {
    match route {
        BambooGroveRoute::Calendar => html!(
            <>
                <Helmet>
                    <title>{"Event Kalender"}</title>
                </Helmet>
                <CalendarPage />
            </>
        ),
        BambooGroveRoute::User => html!(
            <>
                <Helmet>
                    <title>{"Pandas"}</title>
                </Helmet>
                <UsersPage />
            </>
        ),
    }
}

fn switch_support(route: SupportRoute) -> Html {
    match route {
        SupportRoute::Contact => html!(
            <>
                <Helmet>
                    <title>{"Kontakt"}</title>
                </Helmet>
                <ContactPage />
            </>
        ),
    }
}

fn switch_app(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html!(
            <Redirect<AppRoute> to={AppRoute::BambooGroveRoot} />
        ),
        AppRoute::BambooGroveRoot => html!(
            <Redirect<BambooGroveRoute> to={BambooGroveRoute::Calendar} />
        ),
        AppRoute::BambooGrove => html!(
            <>
                <Helmet>
                    <title>{"Bambushain"}</title>
                </Helmet>
                <Switch<BambooGroveRoute> render={switch_panda_party} />
            </>
        ),
        AppRoute::FinalFantasyRoot => html!(
            <Redirect<FinalFantasyRoute> to={FinalFantasyRoute::Characters} />
        ),
        AppRoute::FinalFantasy => html!(
            <>
                <Helmet>
                    <title>{"Final Fantasy"}</title>
                </Helmet>
                <Switch<FinalFantasyRoute> render={switch_final_fantasy} />
            </>
        ),
        AppRoute::SupportRoot => html!(
            <Redirect<SupportRoute> to={SupportRoute::Contact} />
        ),
        AppRoute::Support => html!(
            <>
                <Helmet>
                    <title>{"Bambussupport"}</title>
                </Helmet>
                <Switch<SupportRoute> render={switch_support} />
            </>
        ),
        AppRoute::Login => html!(),
    }
}

fn render_main_menu_entry(
    label: impl Into<AttrValue> + Clone,
    to: AppRoute,
    active: AppRoute,
) -> impl Fn(AppRoute) -> Html {
    move |route| {
        let is_active = route.eq(&active);

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

fn switch_top_bar(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(),
        _ => html!(
            <TopBar />
        ),
    }
}

#[function_component(AppLayout)]
fn app_layout() -> Html {
    log::debug!("Render app layout");
    let initial_loaded_state = use_state_eq(|| false);

    let profile_query = use_query_value::<api::Profile>(().into());
    let profile_atom = use_atom_setter::<storage::CurrentUser>();

    log::debug!("First render, so lets send the request to check if the token is valid and see");
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
                            <Switch<AppRoute> render={render_main_menu_entry("Bambushain", AppRoute::BambooGroveRoot, AppRoute::BambooGrove)} />
                            <Switch<AppRoute> render={render_main_menu_entry("Final Fantasy", AppRoute::FinalFantasyRoot, AppRoute::FinalFantasy)} />
                            <Switch<AppRoute> render={render_main_menu_entry("Bambussupport", AppRoute::SupportRoot, AppRoute::Support)} />
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
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let error_header_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let old_password_state = use_state_eq(|| AttrValue::from(""));
    let new_password_state = use_state_eq(|| AttrValue::from(""));

    {
        let error_state = error_state.clone();

        use_unmount(move || {
            error_state.set(false);
        })
    }

    let update_old_password =
        use_callback(old_password_state.clone(), |value, state| state.set(value));
    let update_new_password =
        use_callback(new_password_state.clone(), |value, state| state.set(value));

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unknown_error_state.clone()),
        |_, (bamboo_error_state, unknown_error_state)| {
            error::report_unknown_error(
                "layout",
                "change_password_dialog",
                bamboo_error_state.deref().clone(),
            );
            unknown_error_state.set(false);
        },
    );

    let on_close = props.on_close.clone();
    let on_save = {
        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_header_state = error_header_state.clone();
        let old_password_state = old_password_state.clone();
        let new_password_state = new_password_state.clone();

        Callback::from(move |_| {
            log::debug!("Perform password change");
            let navigator = navigator.clone();

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

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
                        unknown_error_state.set(false);

                        false
                    }
                    Err(err) => match err.code {
                        api::FORBIDDEN => {
                            log::warn!("The old password is wrong");
                            error_message_state.set("Wenn du möchtest dass es von einem Mod zurückgesetzt wird, einfach anschreiben".into());
                            error_header_state.set("Das alte Passwort ist falsch".into());
                            unknown_error_state.set(false);

                            true
                        }
                        api::NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set("Bitte versuch es erneut um einen Fehler auszuschließen".into());
                            error_header_state.set("Du wurdest scheinbar gelöscht".into());
                            unknown_error_state.set(false);

                            true
                        }
                        _ => {
                            log::warn!("Failed to change the password {err}");
                            error_message_state.set("Leider konnte dein Passwort nicht geändert werden".into());
                            error_header_state.set("Fehler beim ändern".into());
                            unknown_error_state.set(true);
                            bamboo_error_state.set(err.clone());

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
                    if *unknown_error_state {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} header={(*error_header_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} header={(*error_header_state).clone()} />
                    }
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
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let email_state = use_state_eq(|| AttrValue::from(user_atom.profile.email.clone()));
    let display_name_state =
        use_state_eq(|| AttrValue::from(user_atom.profile.display_name.clone()));
    let discord_name_state =
        use_state_eq(|| AttrValue::from(user_atom.profile.discord_name.clone()));

    {
        let error_state = error_state.clone();

        use_unmount(move || {
            error_state.set(false);
        })
    }

    let update_email = use_callback(email_state.clone(), |value, state| state.set(value));
    let update_display_name =
        use_callback(display_name_state.clone(), |value, state| state.set(value));
    let update_discord_name =
        use_callback(discord_name_state.clone(), |value, state| state.set(value));

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unknown_error_state.clone()),
        |_, (bamboo_error_state, unknown_error_state)| {
            error::report_unknown_error(
                "layout",
                "update_my_profile_dialog",
                bamboo_error_state.deref().clone(),
            );
            unknown_error_state.set(false);
        },
    );

    let on_close = props.on_close.clone();
    let on_save = {
        let authentication_state_query = authentication_state_query;

        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let email_state = email_state.clone();
        let display_name_state = display_name_state.clone();
        let discord_name_state = discord_name_state.clone();

        let on_close = on_close.clone();

        Callback::from(move |_| {
            log::debug!("Perform profile update");
            let authentication_state_query = authentication_state_query.clone();

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let email_state = email_state.clone();
            let display_name_state = display_name_state.clone();
            let discord_name_state = discord_name_state.clone();

            let on_close = on_close.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match api::update_my_profile(UpdateProfile::new((*email_state).to_string(), (*display_name_state).to_string(), (*discord_name_state).to_string())).await {
                    Ok(_) => {
                        log::debug!("Profile update successful");
                        let _ = authentication_state_query.refresh().await;
                        unknown_error_state.set(false);

                        on_close.emit(());

                        false
                    }
                    Err(err) => match err.code {
                        api::NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set("Du wurdest scheinbar gelöscht, bitte versuch es erneut um einen Fehler auszuschließen".into());
                            unknown_error_state.set(false);

                            true
                        }
                        _ => {
                            log::warn!("Failed to update the profile {err}");
                            error_message_state.set("Dein Profil konnte leider nicht geändert werden".into());
                            unknown_error_state.set(true);
                            bamboo_error_state.set(err.clone());

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
                    if *unknown_error_state {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} header="Fehler beim Ändern" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} header="Fehler beim Ändern" />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox label="Email" input_type={CosmoTextBoxType::Email} required={true} on_input={update_email} value={(*email_state).clone()} />
                    <CosmoTextBox label="Name" required={true} on_input={update_display_name} value={(*display_name_state).clone()} />
                    <CosmoTextBox label="Discord Name (optional)" on_input={update_discord_name} value={(*discord_name_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(EnableTotpDialog)]
fn enable_totp_dialog(props: &EnableTotpDialogProps) -> Html {
    log::debug!("Open dialog to enable totp");
    let enable_start_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let profile_query = use_query_value::<api::Profile>(().into());
    let profile_atom = use_atom_setter::<storage::CurrentUser>();

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let code_state = use_state_eq(|| AttrValue::from(""));
    let current_password_state = use_state_eq(|| AttrValue::from(""));
    let qrcode_state = use_state_eq(|| AttrValue::from(""));
    let secret_state = use_state_eq(|| AttrValue::from(""));

    {
        let error_state = error_state.clone();

        use_unmount(move || {
            error_state.set(false);
        })
    }

    let update_code = use_callback(code_state.clone(), |value, state| state.set(value));
    let update_password = use_callback(current_password_state.clone(), |value, state| {
        state.set(value)
    });

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unknown_error_state.clone()),
        |_, (bamboo_error_state, unknown_error_state)| {
            error::report_unknown_error(
                "layout",
                "enable_totp_dialog",
                bamboo_error_state.deref().clone(),
            );
            unknown_error_state.set(false);
        },
    );

    let enable_totp = {
        let enable_start_state = enable_start_state.clone();
        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let qrcode_state = qrcode_state.clone();
        let secret_state = secret_state.clone();
        let error_message_state = error_message_state.clone();

        let profile_query = profile_query;
        let profile_atom = profile_atom.clone();

        Callback::from(move |_: ()| {
            let enable_start_state = enable_start_state.clone();
            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let qrcode_state = qrcode_state.clone();
            let secret_state = secret_state.clone();
            let error_message_state = error_message_state.clone();

            let profile_query = profile_query.clone();
            let profile_atom = profile_atom.clone();

            yew::platform::spawn_local(async move {
                match api::enable_totp().await {
                    Ok(data) => {
                        enable_start_state.set(true);
                        qrcode_state.set(data.qr_code.clone().into());
                        unknown_error_state.set(false);
                        secret_state.set(data.secret.clone().into());
                        log::info!("Here is the secret: {}", data.secret);
                        let _ = profile_query.refresh().await.map(|res| {
                            profile_atom(storage::CurrentUser {
                                profile: res.user.clone(),
                            })
                        });
                    }
                    Err(err) => {
                        log::error!("Failed to enable totp: {err}");
                        error_state.set(true);
                        error_message_state
                            .set("Leider konnte Zwei Faktor per App nicht aktiviert werden".into());

                        unknown_error_state.set(true);
                        bamboo_error_state.set(err.clone());
                    }
                }
            })
        })
    };
    let on_form_submit = {
        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let code_state = code_state.clone();
        let current_password_state = current_password_state.clone();

        let on_close = props.on_close.clone();

        Callback::from(move |_: ()| {
            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let code_state = code_state.clone();
            let current_password_state = current_password_state.clone();

            let on_close = on_close.clone();

            yew::platform::spawn_local(async move {
                unknown_error_state.set(false);

                match api::validate_totp(
                    (*code_state).to_string(),
                    (*current_password_state).to_string(),
                )
                .await
                {
                    Ok(_) => {
                        on_close.emit(());
                        unknown_error_state.set(false);
                    }
                    Err(err) => {
                        log::error!("Failed to validate token: {err}");
                        error_state.set(true);
                        if err.code > api::ErrorCode::from(499) {
                            error_message_state
                                .set("Leider konnte Zwei Faktor nicht aktiviert werden".into());
                            unknown_error_state.set(true);
                            bamboo_error_state.set(err.clone());
                        } else {
                            unknown_error_state.set(false);
                            error_message_state.set(
                                "Der von dir eingegebene Code oder dein Passwort ist ungültig, versuch es nochmal".into(),
                            );
                        }
                    }
                }
            })
        })
    };

    let img_style = use_style!(
        r#"
width: 100%;
height: auto;
object-fit: scale-down;
"#
    );

    html!(
        <>
            <Helmet>
                <title>{"Zwei Faktor per App aktivieren"}</title>
            </Helmet>
            if *enable_start_state {
                <CosmoModal title="Zwei Faktor per App aktivieren" is_form={true} on_form_submit={on_form_submit} buttons={html!(
                    <>
                        <CosmoButton label="Abbrechen" on_click={props.on_close.clone()} />
                        <CosmoButton label="App aktivieren" is_submit={true} />
                    </>
                )}>
                    <img class={img_style} src={format!("data:image/png;base64,{}", (*qrcode_state).clone())} alt={(*secret_state).clone()} />
                    <CosmoInputGroup>
                        <CosmoTextBox input_type={CosmoTextBoxType::Password} label="Aktuelles Passwort" required={true} on_input={update_password} value={(*current_password_state).clone()} />
                        <CosmoTextBox label="Zwei Faktor Code" required={true} on_input={update_code} value={(*code_state).clone()} />
                    </CosmoInputGroup>
                </CosmoModal>
            } else {
                <CosmoConfirm title="Zwei Faktor per App aktivieren" message={r#"Hier kannst du deinen Zwei Faktor Code anpassen, von Haus aus sendet BambooGrove einen Code an deine Emailadresse,
du kannst allerdings auch eine App wie Google Authenticator oder Authy einrichten und dann damit einen Code generieren.
Um eine App einzurichten, musst du unten auf App einrichten klicken.
Anschließend kommt ein QR Code, den musst du scannen und danach einen Code aus deiner App eingeben."#} confirm_label="App einrichten" decline_label="Abbrechen" on_confirm={enable_totp} on_decline={props.on_close.clone()} />
            }
            if *error_state {
                if *unknown_error_state {
                    <CosmoConfirm confirm_type={CosmoModalType::Negative} title="Fehler beim Aktivieren" message={(*error_message_state).clone()} decline_label="Schließen" on_decline={move |_| error_state.set(false)} confirm_label="Fehler melden" on_confirm={report_unknown_error} />
                } else {
                    <CosmoAlert alert_type={CosmoModalType::Negative} title="Fehler beim Aktivieren" message={(*error_message_state).clone()} close_label="Schließen" on_close={move |_| error_state.set(false)} />
                }
            }
        </>
    )
}

#[function_component(TopBar)]
fn top_bar() -> Html {
    log::debug!("Render top bar");
    let profile_atom = use_atom_value::<storage::CurrentUser>();
    let navigator = use_navigator().expect("Navigator should be available");

    let mods_state = use_state_eq(|| vec![] as Vec<AttrValue>);

    let app_two_factor_open_state = use_state_eq(|| false);
    let profile_open_state = use_state_eq(|| false);
    let password_open_state = use_state_eq(|| false);

    let logout = use_callback(navigator, |_: (), navigator| {
        api::authentication::logout();
        navigator.push(&AppRoute::Login);
    });
    let update_my_profile_click =
        use_callback(profile_open_state.clone(), |_, profile_open_state| {
            profile_open_state.set(true)
        });
    let enable_app_two_factor_click = use_callback(
        app_two_factor_open_state.clone(),
        |_, app_two_factor_open_state| app_two_factor_open_state.set(true),
    );
    let change_password_click = {
        let password_open_state = password_open_state.clone();

        let mods_state = mods_state.clone();

        Callback::from(move |_| {
            let password_open_state = password_open_state.clone();

            let mods_state = mods_state.clone();

            yew::platform::spawn_local(async move {
                if let Ok(users) = api::get_users().await {
                    mods_state.set(
                        users
                            .into_iter()
                            .filter_map(|user| {
                                if user.is_mod {
                                    Some(AttrValue::from(user.display_name))
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<AttrValue>>(),
                    );
                }

                password_open_state.set(true);
            });
        })
    };

    html!(
        <>
            <CosmoTopBar profile_picture="/static/logo.webp" has_right_item={true} right_item_on_click={logout} right_item_label="Abmelden">
                <CosmoTopBarItem label="Mein Profil" on_click={update_my_profile_click} />
                <CosmoTopBarItem label="Passwort ändern" on_click={change_password_click} />
                if !profile_atom.profile.app_totp_enabled {
                    <CosmoTopBarItem label="App Zwei Faktor einrichten" on_click={enable_app_two_factor_click} />
                }
            </CosmoTopBar>
            if *profile_open_state {
                <UpdateMyProfileDialog on_close={move |_| profile_open_state.set(false)} />
            }
            if *password_open_state {
                <ChangePasswordDialog on_close={move |_| password_open_state.set(false)} mods={(*mods_state).clone()} />
            }
            if *app_two_factor_open_state {
                <EnableTotpDialog on_close={move |_| app_two_factor_open_state.set(false)} />
            }
        </>
    )
}

#[function_component(Layout)]
fn layout() -> Html {
    log::info!("Run layout");
    html!(
        <AppLayout />
    )
}

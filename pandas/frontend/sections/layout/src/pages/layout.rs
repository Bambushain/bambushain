use std::ops::Deref;

use bounce::helmet::Helmet;
use bounce::{use_atom_setter, use_atom_value};
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount, use_timeout, use_update};
use yew_router::prelude::*;

use bamboo_common::core::entities::user::UpdateProfile;
use bamboo_common::frontend::api::{ApiError, CONFLICT, FORBIDDEN, NOT_FOUND};
use bamboo_pandas_frontend_base::routing::{
    AppRoute, BambooGroveRoute, FinalFantasyRoute, GroveRoute, LegalRoute, LicensesRoute,
    SupportRoute,
};
use bamboo_pandas_frontend_base::{error, storage};
use bamboo_pandas_frontend_section_authentication::LoginPage;
use bamboo_pandas_frontend_section_bamboo::CalendarPage;
use bamboo_pandas_frontend_section_bamboo::UsersPage;
use bamboo_pandas_frontend_section_final_fantasy::CharacterPage;
use bamboo_pandas_frontend_section_final_fantasy::SettingsPage;
use bamboo_pandas_frontend_section_groves::pages::groves::GroveDetails;
use bamboo_pandas_frontend_section_legal::{DataProtectionPage, ImprintPage};
use bamboo_pandas_frontend_section_licenses::{
    BambooGrovePage, FontsPage, ImagesPage, SoftwareLicensesPage,
};
use bamboo_pandas_frontend_section_support::ContactPage;

use crate::api;

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(
            <>
                <Helmet>
                    <title>{"Anmelden"}</title>
                </Helmet>
                <LoginPage />
            </>
        ),
        _ => html!(<Layout />),
    }
}

#[function_component(GrovesMenu)]
fn groves_menus() -> Html {
    let groves_state = use_async(async move { api::get_groves().await });

    {
        let groves_state = groves_state.clone();
        use_mount(move || {
            groves_state.run();
        });
    }

    if let Some(_) = &groves_state.error {
        html!()
    } else if let Some(groves) = &groves_state.data {
        html!(
            {for groves.iter().cloned().map(|grove| {
                let name = grove.name.clone();

                html!(
                    <Switch<GroveRoute> render={render_sub_menu_entry(name.clone(), GroveRoute::Grove { id: grove.id })} />
                )
            })}
        )
    } else {
        html!()
    }
}

#[function_component(GrovesRoot)]
fn groves_root() -> Html {
    let groves_state = use_async(async move { api::get_groves().await });

    {
        let groves_state = groves_state.clone();
        use_mount(move || {
            groves_state.run();
        });
    }

    if groves_state.error.is_some() {
        html!(
            <Redirect<GroveRoute> to={GroveRoute::AddGrove} />
        )
    } else if let Some(groves) = &groves_state.data {
        let to = if let Some(first) = groves.first() {
            GroveRoute::Grove { id: first.id }
        } else {
            GroveRoute::AddGrove
        };

        html!(
            <Redirect<GroveRoute> to={to} />
        )
    } else {
        html!()
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
        AppRoute::GrovesRoot | AppRoute::Groves => html!(
            <CosmoSubMenuBar>
                <GrovesMenu />
                <Switch<GroveRoute> render={render_sub_menu_entry("Neuer Hain", GroveRoute::AddGrove)} />
            </CosmoSubMenuBar>
        ),
        AppRoute::SupportRoot | AppRoute::Support => html!(
            <CosmoSubMenuBar>
                <Switch<SupportRoute> render={render_sub_menu_entry("Kontakt", SupportRoute::Contact)} />
            </CosmoSubMenuBar>
        ),
        AppRoute::LegalRoot | AppRoute::Legal => html!(
            <CosmoSubMenuBar>
                <Switch<LegalRoute> render={render_sub_menu_entry("Impressum", LegalRoute::Imprint)} />
                <Switch<LegalRoute> render={render_sub_menu_entry("Datenschutzerklärung", LegalRoute::DataProtection)} />
            </CosmoSubMenuBar>
        ),
        AppRoute::LicensesRoot | AppRoute::Licenses => html!(
            <CosmoSubMenuBar>
                <Switch<LicensesRoute> render={render_sub_menu_entry("Bambushain Lizenz", LicensesRoute::BambooGrove)} />
                <Switch<LicensesRoute> render={render_sub_menu_entry("Bildlizenzen", LicensesRoute::Images)} />
                <Switch<LicensesRoute> render={render_sub_menu_entry("Schriftlizenzen", LicensesRoute::Fonts)} />
                <Switch<LicensesRoute> render={render_sub_menu_entry("Softwarelizenzen", LicensesRoute::Software)} />
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

fn switch_groves(route: GroveRoute) -> Html {
    match route {
        GroveRoute::AddGrove => html!(
            <>
                <Helmet>
                    <title>{"Neuer Hain"}</title>
                </Helmet>
                <CosmoTitle title="Neuer Hain" />
            </>
        ),
        GroveRoute::Grove { id } => html!(
            <GroveDetails id={id} />
        ),
    }
}

fn switch_bamboo_grove(route: BambooGroveRoute) -> Html {
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

fn switch_legal(route: LegalRoute) -> Html {
    match route {
        LegalRoute::Imprint => html!(
            <>
                <Helmet>
                    <title>{"Impressum"}</title>
                </Helmet>
                <ImprintPage />
            </>
        ),
        LegalRoute::DataProtection => html!(
            <>
                <Helmet>
                    <title>{"Datenschutzerklärung"}</title>
                </Helmet>
                <DataProtectionPage />
            </>
        ),
    }
}

fn switch_licenses(route: LicensesRoute) -> Html {
    match route {
        LicensesRoute::BambooGrove => html!(
            <>
                <Helmet>
                    <title>{"Bambushain Lizenz"}</title>
                </Helmet>
                <BambooGrovePage />
            </>
        ),
        LicensesRoute::Images => html!(
            <>
                <Helmet>
                    <title>{"Bildlizenzen"}</title>
                </Helmet>
                <ImagesPage />
            </>
        ),
        LicensesRoute::Fonts => html!(
            <>
                <Helmet>
                    <title>{"Schriftlizenzen"}</title>
                </Helmet>
                <FontsPage />
            </>
        ),
        LicensesRoute::Software => html!(
            <>
                <Helmet>
                    <title>{"Softwarelizenzen"}</title>
                </Helmet>
                <SoftwareLicensesPage />
            </>
        ),
    }
}

fn switch_app(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html!(
            <Redirect<AppRoute> to={AppRoute::BambooGroveRoot} />
        ),
        AppRoute::BambooGroveRoot | AppRoute::BambooGrove => html!(
            <>
                <Helmet>
                    <title>{"Bambushain"}</title>
                </Helmet>
                <Switch<BambooGroveRoute> render={switch_bamboo_grove} />
            </>
        ),
        AppRoute::FinalFantasyRoot | AppRoute::FinalFantasy => html!(
            <>
                <Helmet>
                    <title>{"Final Fantasy"}</title>
                </Helmet>
                <Switch<FinalFantasyRoute> render={switch_final_fantasy} />
            </>
        ),
        AppRoute::GrovesRoot => html!(
            <GrovesRoot />
        ),
        AppRoute::Groves => html!(
            <>
                <Helmet>
                    <title>{"Meine Haine"}</title>
                </Helmet>
                <Switch<GroveRoute> render={switch_groves} />
            </>
        ),
        AppRoute::SupportRoot | AppRoute::Support => html!(
            <>
                <Helmet>
                    <title>{"Bambussupport"}</title>
                </Helmet>
                <Switch<SupportRoute> render={switch_support} />
            </>
        ),
        AppRoute::LegalRoot | AppRoute::Legal => html!(
            <>
                <Helmet>
                    <title>{"Rechtliches"}</title>
                </Helmet>
                <Switch<LegalRoute> render={switch_legal} />
            </>
        ),
        AppRoute::LicensesRoot | AppRoute::Licenses => html!(
            <>
                <Helmet>
                    <title>{"Lizenz"}</title>
                </Helmet>
                <Switch<LicensesRoute> render={switch_licenses} />
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

fn switch_top_bar(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html!(),
        AppRoute::LegalRoot | AppRoute::Legal | AppRoute::LicensesRoot | AppRoute::Licenses => {
            html!(
                <TopBarLegal />
            )
        }
        _ => html!(
            <TopBar />
        ),
    }
}

fn switch_layout(route: AppRoute) -> Html {
    match route {
        AppRoute::LegalRoot | AppRoute::Legal | AppRoute::LicensesRoot | AppRoute::Licenses => {
            html!(<LegalLayout />)
        }
        _ => html!(<AppLayout />),
    }
}

#[function_component(AppLayout)]
fn app_layout() -> Html {
    log::debug!("Render app layout");
    let profile_atom_setter = use_atom_setter::<storage::CurrentUser>();

    let profile_state = use_async(async move {
        api::get_my_profile().await.map(|user| {
            profile_atom_setter(user.clone().into());
            user
        })
    });

    {
        let profile_state = profile_state.clone();

        use_mount(move || {
            log::debug!(
                "First render, so lets send the request to check if the token is valid and see"
            );
            profile_state.run();
        });
    }

    html!(
        if let Some(_) = &profile_state.error {
            <Redirect<AppRoute> to={AppRoute::Login} />
        } else if profile_state.data.is_some() {
            <>
                <Switch<AppRoute> render={switch_top_bar}/>
                <CosmoMenuBar>
                    <CosmoMainMenu>
                        <Switch<AppRoute> render={render_main_menu_entry("Bambushain", AppRoute::BambooGroveRoot, AppRoute::BambooGrove)} />
                        <Switch<AppRoute> render={render_main_menu_entry("Final Fantasy", AppRoute::FinalFantasyRoot, AppRoute::FinalFantasy)} />
                        <Switch<AppRoute> render={render_main_menu_entry("Meine Haine", AppRoute::GrovesRoot, AppRoute::Groves)} />
                        <Switch<AppRoute> render={render_main_menu_entry("Bambussupport", AppRoute::SupportRoot, AppRoute::Support)} />
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

#[function_component(LegalLayout)]
fn legal_layout() -> Html {
    log::debug!("Render legal layout");
    html!(
        <>
            <Switch<AppRoute> render={switch_top_bar}/>
            <CosmoMenuBar>
                <CosmoMainMenu>
                    <Switch<AppRoute> render={render_main_menu_entry("Rechtliches", AppRoute::LegalRoot, AppRoute::Legal)} />
                    <Switch<AppRoute> render={render_main_menu_entry("Lizenzen", AppRoute::LicensesRoot, AppRoute::Licenses)} />
                </CosmoMainMenu>
                <Switch<AppRoute> render={switch_sub_menu} />
            </CosmoMenuBar>
            <CosmoPageBody>
                <Switch<AppRoute> render={switch_app} />
           </CosmoPageBody>
        </>
    )
}

#[autoprops]
#[function_component(ChangePasswordDialog)]
fn change_password_dialog(on_close: &Callback<()>) -> Html {
    log::debug!("Open dialog to change password");
    let navigator = use_navigator();

    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(ApiError::default);

    let old_password_state = use_state_eq(|| AttrValue::from(""));
    let new_password_state = use_state_eq(|| AttrValue::from(""));

    let update_old_password =
        use_callback(old_password_state.clone(), |value, state| state.set(value));
    let update_new_password =
        use_callback(new_password_state.clone(), |value, state| state.set(value));

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "layout",
                "change_password_dialog",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    let save_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let old_password_state = old_password_state.clone();
        let new_password_state = new_password_state.clone();

        use_async(async move {
            api::change_my_password(
                (*old_password_state).to_string(),
                (*new_password_state).to_string(),
            )
            .await
            .map(|_| {
                api::logout();
                navigator
                    .expect("Navigator should be available")
                    .push(&AppRoute::Login);
                unreported_error_toggle.set(false);
            })
            .map_err(|err| {
                unreported_error_toggle.set(true);
                bamboo_error_state.set(err.clone());
                err
            })
        })
    };

    let on_close = on_close.clone();
    let on_save = use_callback(save_state.clone(), |_, state| state.run());

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
                if let Some(err) = &save_state.error {
                    if err.code == FORBIDDEN {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Falls du dein Passwort vergessen hast, melde dich bitte ab und klicke auf Passwort vergessen" header="Das alte Passwort ist falsch" />
                    } else if err.code == NOT_FOUND {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Bitte versuch es erneut um einen Fehler auszuschließen" header="Du wurdest scheinbar gelöscht" />
                    } else if *unreported_error_toggle {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Leider konnte dein Passwort nicht geändert werden" header="Fehler beim ändern" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Leider konnte dein Passwort nicht geändert werden" header="Fehler beim ändern" />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox input_type={CosmoTextBoxType::Password} label="Aktuelles Passwort" on_input={update_old_password} value={(*old_password_state).clone()} required={true} />
                    <CosmoTextBox input_type={CosmoTextBoxType::Password} label="Neues Passwort" on_input={update_new_password} value={(*new_password_state).clone()} required={true} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[autoprops]
#[function_component(UpdateMyProfileDialog)]
fn update_my_profile_dialog(on_close: &Callback<()>) -> Html {
    log::debug!("Open dialog to update profile");
    let profile_atom_setter = use_atom_setter::<storage::CurrentUser>();
    let profile_atom = use_atom_value::<storage::CurrentUser>();

    let disable_totp_open_toggle = use_bool_toggle(false);
    let app_two_factor_open_toggle = use_bool_toggle(false);
    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(ApiError::default);

    let profile_picture_state = use_state_eq(|| None as Option<web_sys::File>);

    let email_state = use_state_eq(|| AttrValue::from(profile_atom.profile.email.clone()));
    let display_name_state =
        use_state_eq(|| AttrValue::from(profile_atom.profile.display_name.clone()));
    let discord_name_state =
        use_state_eq(|| AttrValue::from(profile_atom.profile.discord_name.clone()));

    let update_email = use_callback(email_state.clone(), |value, state| state.set(value));
    let update_display_name =
        use_callback(display_name_state.clone(), |value, state| state.set(value));
    let update_discord_name =
        use_callback(discord_name_state.clone(), |value, state| state.set(value));
    let select_profile_picture = use_callback(profile_picture_state.clone(), |value, state| {
        state.set(Some(value))
    });

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "layout",
                "update_my_profile_dialog",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    let save_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let profile_atom_setter = profile_atom_setter.clone();

        let email_state = email_state.clone();
        let display_name_state = display_name_state.clone();
        let discord_name_state = discord_name_state.clone();

        let profile_picture_state = profile_picture_state.clone();

        let on_close = on_close.clone();

        use_async(async move {
            let result = api::update_my_profile(UpdateProfile::new(
                (*email_state).to_string(),
                (*display_name_state).to_string(),
                (*discord_name_state).to_string(),
            ))
            .await
            .map(|_| unreported_error_toggle.set(false))
            .map_err(|err| {
                unreported_error_toggle.set(true);
                bamboo_error_state.set(err.clone());
                err
            });
            if result.is_ok() {
                if let Some(profile_picture) = (*profile_picture_state).clone() {
                    let profile_result = api::upload_profile_picture(profile_picture)
                        .await
                        .map(|_| {
                            unreported_error_toggle.set(false);
                            on_close.emit(())
                        })
                        .map_err(|err| {
                            unreported_error_toggle.set(true);
                            bamboo_error_state.set(err.clone());
                            err
                        });

                    if profile_result.is_ok() {
                        if let Ok(profile) = api::get_my_profile().await {
                            profile_atom_setter(profile.into());
                        }
                    }

                    profile_result
                } else {
                    on_close.emit(());
                    result
                }
            } else {
                result
            }
        })
    };
    let disable_totp_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();
        let disable_totp_open_toggle = disable_totp_open_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let profile_atom_setter = profile_atom_setter.clone();

        use_async(async move {
            if let Err(err) = api::disable_totp().await {
                unreported_error_toggle.set(true);
                disable_totp_open_toggle.set(true);
                bamboo_error_state.set(err.clone());

                Err(err)
            } else {
                unreported_error_toggle.set(false);
                disable_totp_open_toggle.set(false);
                if let Ok(profile) = api::get_my_profile().await {
                    profile_atom_setter(profile.into());
                }

                Ok(())
            }
        })
    };

    let on_open_disable_totp =
        use_callback(disable_totp_open_toggle.clone(), |_, state| state.set(true));
    let on_close_disable_totp = use_callback(disable_totp_open_toggle.clone(), |_, state| {
        state.set(false)
    });
    let on_enable_app_two_factor = use_callback(
        app_two_factor_open_toggle.clone(),
        |_, app_two_factor_open_toggle| app_two_factor_open_toggle.set(true),
    );
    let on_save = use_callback(save_state.clone(), |_, save_state| save_state.run());
    let on_disable_totp = use_callback(disable_totp_state.clone(), |_, disable_totp_state| {
        disable_totp_state.run()
    });
    let on_close = on_close.clone();

    html!(
        <>
            <Helmet>
                <title>{"Profil bearbeiten"}</title>
            </Helmet>
            <CosmoModal title="Profil bearbeiten" is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Schließen" />
                    if profile_atom.profile.totp_validated.unwrap_or(false) {
                        <CosmoButton on_click={on_open_disable_totp} label="App Zwei Faktor deaktivieren" />
                    } else {
                        <CosmoButton on_click={on_enable_app_two_factor} label="App Zwei Faktor aktivieren" />
                    }
                    <CosmoButton is_submit={true} label="Profil speichern" />
                </>
            )}>
                if let Some(err) = &save_state.error {
                    if err.code == NOT_FOUND {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Bitte versuch es erneut um einen Fehler auszuschließen" header="Du wurdest scheinbar gelöscht" />
                    } else if err.code == CONFLICT {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Die Email oder der Name ist leider schon vergeben" header="Leider schon vergeben" />
                    } else if *unreported_error_toggle {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Dein Profil konnte leider nicht gespeichert werden" header="Fehler beim Speichern" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Dein Profil konnte leider nicht gespeichert werden" header="Fehler beim Speichern" />
                    }
                }
                if let Some(err) = &disable_totp_state.error {
                    if err.code == NOT_FOUND {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Bitte versuch es erneut um einen Fehler auszuschließen" header="Du wurdest scheinbar gelöscht" />
                    } else if *unreported_error_toggle {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Zwei Faktor per App konnte leider nicht deaktiviert werden" header="Fehler beim Deaktivieren" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Zwei Faktor per App konnte leider nicht deaktiviert werden" header="Fehler beim Deaktivieren" />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox label="Email" input_type={CosmoTextBoxType::Email} required={true} on_input={update_email} value={(*email_state).clone()} />
                    <CosmoTextBox label="Name" required={true} on_input={update_display_name} value={(*display_name_state).clone()} />
                    <CosmoTextBox label="Discord Name (optional)" on_input={update_discord_name} value={(*discord_name_state).clone()} />
                    <CosmoFilePicker label="Profilbild (optional)" on_select={select_profile_picture} />
                </CosmoInputGroup>
            </CosmoModal>
            if *disable_totp_open_toggle {
                <CosmoConfirm confirm_type={CosmoModalType::Warning} message="Möchtest du deine Zwei Faktor Authentifizierung per App deaktivieren? Du bekommst dann wieder eine Email." title="Zwei Faktor Authentifizierung deaktivieren" on_decline={on_close_disable_totp} on_confirm={on_disable_totp} confirm_label="Deaktivieren" decline_label="Nicht deaktivieren" />
            }
            if *app_two_factor_open_toggle {
                <EnableTotpDialog on_close={move |_| app_two_factor_open_toggle.set(false)} />
            }
        </>
    )
}

#[autoprops]
#[function_component(EnableTotpDialog)]
fn enable_totp_dialog(on_close: &Callback<()>) -> Html {
    log::debug!("Open dialog to enable totp");
    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(ApiError::default);

    let profile_atom = use_atom_setter::<storage::CurrentUser>();

    let code_state = use_state_eq(|| AttrValue::from(""));
    let current_password_state = use_state_eq(|| AttrValue::from(""));

    let enable_totp_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        use_async(async move {
            api::enable_totp()
                .await
                .map(|data| {
                    unreported_error_toggle.set(false);
                    data
                })
                .map_err(|err| {
                    unreported_error_toggle.set(true);
                    bamboo_error_state.set(err.clone());
                    err
                })
        })
    };
    let validate_totp_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();
        let code_state = code_state.clone();
        let current_password_state = current_password_state.clone();

        let on_close = on_close.clone();

        let profile_atom = profile_atom.clone();

        use_async(async move {
            if let Err(err) = api::validate_totp(
                (*code_state).to_string(),
                (*current_password_state).to_string(),
            )
            .await
            {
                log::error!("Failed to validate token: {err}");
                unreported_error_toggle.set(true);
                bamboo_error_state.set(err.clone());

                Err(err)
            } else {
                on_close.emit(());
                unreported_error_toggle.set(false);
                if let Ok(profile) = api::get_my_profile().await {
                    profile_atom(profile.into())
                }

                Ok(())
            }
        })
    };

    let update_code = use_callback(code_state.clone(), |value, state| state.set(value));
    let update_password = use_callback(current_password_state.clone(), |value, state| {
        state.set(value)
    });
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "layout",
                "enable_totp_dialog",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );
    let on_form_submit = use_callback(
        (enable_totp_state.clone(), validate_totp_state.clone()),
        |_, (enable_totp_state, validate_totp_state)| {
            if enable_totp_state.data.is_some() {
                validate_totp_state.run();
            } else {
                enable_totp_state.run();
            }
        },
    );

    {
        let enable_totp_state = enable_totp_state.clone();
        #[allow(clippy::identity_op)]
        use_timeout(
            move || {
                enable_totp_state.run();
            },
            1 * 1000,
        );
    }

    let img_style = use_style!(
        r#"
width: 24.5rem;
height: 24.5rem;
object-fit: scale-down;
grid-area: code;
"#
    );
    let logo_style = use_style!(
        r#"
width: 10rem;
height: 10rem;
place-self: center;
grid-area: code;
fill: var(--primary-color);
stroke: var(--white);
stroke-opacity: 1;
stroke-width: 57.6;
stroke-dasharray: none;
stroke-linejoin: miter;
paint-order: stroke markers fill;

    path {
    transform: scale(89%) translate(6%, 6%);
}
        "#
    );
    let container_style = use_style!(
        r#"
display: grid;
gap: 1rem;
grid-template-columns: [code] 24.5rem [details] auto;
grid-template-areas: "code details";
justify-content: center;
align-items: start;
        "#
    );
    let details_style = use_style!(
        r#"
grid-area: details;
display: flex;
flex-flow: column;
max-width: 30vw;
padding-top: 2rem;
    "#
    );

    html!(
        <>
            <Helmet>
                <title>{"Zwei Faktor per App aktivieren"}</title>
            </Helmet>
            <CosmoModal title="Zwei Faktor per App aktivieren" is_form={true} on_form_submit={on_form_submit} buttons={html!(
                <>
                    <CosmoButton on_click={on_close.clone()} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="App einrichten" />
                </>
            )}>
                <div class={container_style}>
                    if let Some(data) = &enable_totp_state.data {
                        <img class={img_style} src={data.qr_code.clone()} alt={data.secret.clone()} />
                        <svg class={logo_style} viewBox="0 0 512 512">
                            <path d="M511.094,264.722c-1.136-3.307-28.511-81.137-89.171-95.166c-30.729-7.107-63.124,3.303-96.526,30.938v-35.663
                                    c6.222-2.428,10.637-8.464,10.637-15.545s-4.415-13.117-10.637-15.545V21.124c0-9.22-7.475-16.696-16.696-16.696h-89.595
                                    c-9.22,0-16.696,7.475-16.696,16.696v46.166c-18.137-33.54-41.579-53.478-69.951-59.406C71.508-4.849,13.992,54.3,11.574,56.825
                                    C6.875,61.728,5.615,68.989,8.387,75.19c2.773,6.2,9.015,10.103,15.811,9.873c82.495-2.81,169.04,34.422,169.902,34.798
                                    c2.146,0.936,4.415,1.391,6.668,1.391c0.55,0,1.097-0.031,1.643-0.085v12.741c-5.986,2.538-10.185,8.467-10.185,15.378
                                    s4.2,12.84,10.185,15.378v99.481c-13.69-36.175-34.515-59.305-62.158-68.907C81.436,174.809,16.819,226.106,14.098,228.3
                                    c-5.288,4.262-7.467,11.302-5.513,17.805c1.956,6.503,7.654,11.176,14.416,11.815c6.876,0.651,13.745,1.588,20.559,2.751
                                    c-26.815,24.958-41.321,57.285-42.141,59.145c-2.739,6.214-1.443,13.469,3.281,18.349c3.208,3.314,7.561,5.083,11.999,5.083
                                    c2.096,0,4.212-0.395,6.233-1.209c76.563-30.832,170.624-25.43,171.564-25.372c2.816,0.178,5.51-0.359,7.913-1.449v27.787
                                    c-5.986,2.538-10.185,8.467-10.185,15.378s4.2,12.84,10.185,15.378v117.115c0,9.22,7.475,16.696,16.696,16.696H308.7
                                    c9.22,0,16.696-7.475,16.696-16.696V373.928c6.222-2.428,10.637-8.464,10.637-15.545s-4.415-13.117-10.637-15.545v-97.236
                                    c22.507,1.287,99.826,7.886,162.387,39.448c2.383,1.202,4.958,1.79,7.516,1.79c3.954,0,7.87-1.404,10.977-4.113
                                    C511.396,278.264,513.3,271.144,511.094,264.722z M70.033,53.522c16.303-9.503,36.4-16.998,55.681-12.936
                                    c16.129,3.398,30.358,14.887,42.528,34.277C142.992,66.766,107.92,57.514,70.033,53.522z M55.265,296.723
                                    c8.409-10.079,18.888-19.87,31.085-25.859c14.339,4.315,27.897,9.235,40.144,14.176
                                    C104.959,286.978,80.307,290.495,55.265,296.723z M72.688,232.553c17.389-7.306,38.216-12.161,56.607-5.773
                                    c15.598,5.418,28.267,18.643,37.87,39.466C143.202,255.001,109.679,241.362,72.688,232.553z M292.005,474.18h-56.204v-99.102
                                    h56.204V474.18z M292.005,341.687h-56.204V165.981h56.204V341.687z M292.005,132.589h-56.204v-94.77h56.204V132.589z
                                     M361.327,215.325c19.184-12.489,36.925-16.945,52.99-13.256c19.207,4.408,34.299,19.645,45.106,35.114
                                    C423.36,224.901,387.642,218.575,361.327,215.325z" />
                        </svg>
                    } else {
                        <CosmoProgressRing />
                    }
                    <div class={details_style}>
                        <CosmoMessage header="Schritte zum Aktivieren" message="Zu erst musst du den QR Code mit einer App wie Authy oder dem Google Authenticator scannen.\nAnschließend gibst du in den Feldern dein aktuelles Passwort ein und der Code der dir in der App angezeigt wird." message_type={CosmoMessageType::Information} />
                        if let Some(err) = &validate_totp_state.error {
                            if err.code == FORBIDDEN {
                                <CosmoMessage header="Code oder Passwort falsch" message="Der von dir eingegebene Code oder dein Passwort ist ungültig, versuch es nochmal" message_type={CosmoMessageType::Negative} />
                            } else if *unreported_error_toggle {
                                <CosmoMessage header="Fehler beim Aktivieren" message="Leider konnte Zwei Faktor per App nicht aktiviert werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                            } else {
                                <CosmoMessage header="Fehler beim Aktivieren" message="Leider konnte Zwei Faktor per App nicht aktiviert werden" message_type={CosmoMessageType::Negative} />
                            }
                        }
                        <CosmoInputGroup>
                            <CosmoTextBox input_type={CosmoTextBoxType::Password} label="Aktuelles Passwort" required={true} on_input={update_password} value={(*current_password_state).clone()} />
                            <CosmoTextBox label="Zwei Faktor Code" required={true} on_input={update_code} value={(*code_state).clone()} />
                        </CosmoInputGroup>
                    </div>
                </div>
            </CosmoModal>
        </>
    )
}

#[function_component(TopBar)]
fn top_bar() -> Html {
    log::debug!("Render top bar");
    let navigator = use_navigator().expect("Navigator should be available");

    let profile_atom = use_atom_value::<storage::CurrentUser>();

    let profile_open_toggle = use_bool_toggle(false);
    let password_open_toggle = use_bool_toggle(false);
    let leave_grove_open_toggle = use_bool_toggle(false);

    let profile_user_id = use_state(|| profile_atom.profile.id);

    let update = use_update();
    let leave_grove_state =
        use_async(async move { api::leave().await.map(|_| navigator.push(&AppRoute::Login)) });

    let navigator = use_navigator().expect("Navigator should be available");
    let logout = use_callback(navigator, |_: (), navigator| {
        api::logout();
        navigator.push(&AppRoute::Login);
    });
    let open_update_my_profile = use_callback(
        (profile_user_id.clone(), profile_open_toggle.clone()),
        |_, (state, toggle)| {
            toggle.set(true);
            state.set(-1);
        },
    );
    let open_change_password =
        use_callback(password_open_toggle.clone(), |_, password_open_state| {
            password_open_state.set(true);
        });
    let open_leave_grove = use_callback(leave_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(true)
    });
    let close_leave_grove = use_callback(leave_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(false)
    });
    let leave_grove = use_callback(leave_grove_state.clone(), |_, state| state.run());
    let profile_updated = use_callback(
        (
            profile_user_id.clone(),
            profile_open_toggle.clone(),
            profile_atom.clone(),
        ),
        move |_, (state, toggle, profile_atom)| {
            toggle.set(false);
            state.set(profile_atom.profile.id);
            update()
        },
    );

    let profile_picture = format!(
        "/api/user/{}/picture#time={}",
        *profile_user_id,
        chrono::offset::Local::now().timestamp_millis()
    );

    html!(
        <>
            <CosmoTopBar profile_picture={profile_picture} has_right_item={true} right_item_on_click={logout} right_item_label="Abmelden">
                <CosmoTopBarItemLink<AppRoute> label="Rechtliches" to={AppRoute::LegalRoot} />
                <CosmoTopBarItem label="Mein Profil" on_click={open_update_my_profile} />
                <CosmoTopBarItem label="Passwort ändern" on_click={open_change_password} />
                <CosmoTopBarItem label="Account löschen" on_click={open_leave_grove} />
            </CosmoTopBar>
            if *profile_open_toggle {
                <UpdateMyProfileDialog on_close={profile_updated} />
            }
            if *password_open_toggle {
                <ChangePasswordDialog on_close={move |_| password_open_toggle.set(false)} />
            }
            if *leave_grove_open_toggle {
                <CosmoConfirm confirm_type={CosmoModalType::Negative} on_confirm={leave_grove} on_decline={close_leave_grove} title="Account löschen" message="Bist du sicher, dass du deinen Account löschen möchtest?\nWenn du deinen Account löscht, werden alle deine Daten gelöscht und können nicht wiederhergestellt werden." confirm_label="Account löschen" decline_label="Account behalten" />
            }
        </>
    )
}

#[function_component(TopBarLegal)]
fn top_bar_legal() -> Html {
    log::debug!("Render top bar");
    let navigator = use_navigator().expect("Navigator should be available");

    let back = use_callback(navigator, |_: (), navigator| {
        navigator.push(&AppRoute::BambooGroveRoot);
    });

    html!(
        <CosmoTopBar profile_picture="/static/logo.webp" has_right_item={true} right_item_on_click={back} right_item_label="Zum Hain">
            <CosmoTopBarItemLink<AppRoute> label="" to={AppRoute::BambooGroveRoot} />
        </CosmoTopBar>
    )
}

#[function_component(Layout)]
fn layout() -> Html {
    log::info!("Run layout");
    html!(
        <Switch<AppRoute> render={switch_layout} />
    )
}

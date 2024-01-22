use std::ops::Deref;

use bounce::helmet::Helmet;
use bounce::{use_atom_setter, use_atom_value};
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount, UseAsyncHandle};
use yew_router::prelude::*;

use bamboo_common::core::entities::user::UpdateProfile;
use bamboo_pandas_frontend_base::api::{ApiError, CONFLICT, FORBIDDEN, NOT_FOUND};
use bamboo_pandas_frontend_base::{error, storage};
use bamboo_pandas_frontend_base::routing::{
    AppRoute, BambooGroveRoute, FinalFantasyRoute, LegalRoute, LicensesRoute, ModAreaRoute,
    SupportRoute,
};
use bamboo_pandas_frontend_section_authentication::LoginPage;
use bamboo_pandas_frontend_section_bamboo::CalendarPage;
use bamboo_pandas_frontend_section_bamboo::UsersPage;
use bamboo_pandas_frontend_section_final_fantasy::CharacterPage;
use bamboo_pandas_frontend_section_final_fantasy::SettingsPage;
use bamboo_pandas_frontend_section_legal::{DataProtectionPage, ImprintPage};
use bamboo_pandas_frontend_section_licenses::{
    BambooGrovePage, FontsPage, ImagesPage, SoftwareLicensesPage,
};
use bamboo_pandas_frontend_section_mod_area::{GroveManagementPage, UserManagementPage};
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
        AppRoute::ModAreaRoot | AppRoute::ModArea => html!(
            <CosmoSubMenuBar>
                <Switch<ModAreaRoute> render={render_sub_menu_entry("Benutzerverwaltung", ModAreaRoute::UserManagement)} />
                <Switch<ModAreaRoute> render={render_sub_menu_entry("Hainverwaltung", ModAreaRoute::GroveManagement)} />
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

fn switch_mod_area(route: ModAreaRoute) -> Html {
    match route {
        ModAreaRoute::UserManagement => html!(
            <>
                <Helmet>
                    <title>{"Benutzerverwaltung"}</title>
                </Helmet>
                <UserManagementPage />
            </>
        ),
        ModAreaRoute::GroveManagement => html!(
            <>
                <Helmet>
                    <title>{"Hainverwaltung"}</title>
                </Helmet>
                <GroveManagementPage />
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

fn switch_app(is_mod: bool, grove_is_enabled: bool) -> impl Fn(AppRoute) -> Html {
    move |route| {
        if grove_is_enabled {
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
                AppRoute::SupportRoot | AppRoute::Support => html!(
                    <>
                        <Helmet>
                            <title>{"Bambussupport"}</title>
                        </Helmet>
                        <Switch<SupportRoute> render={switch_support} />
                    </>
                ),
                AppRoute::ModAreaRoot | AppRoute::ModArea => {
                    if is_mod {
                        html!(
                            <>
                                <Helmet>
                                    <title>{"Mod Area"}</title>
                                </Helmet>
                                <Switch<ModAreaRoute> render={switch_mod_area} />
                            </>
                        )
                    } else {
                        html!(
                            <Redirect<AppRoute> to={AppRoute::BambooGroveRoot} />
                        )
                    }
                }
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
        } else {
            match route {
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
                _ => html!(
                    <>
                        <Helmet>
                            <title>{"Mod Area"}</title>
                        </Helmet>
                        <Switch<ModAreaRoute> render={switch_mod_area} />
                    </>
                ),
            }
        }
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

    let grove_is_enabled = use_bool_toggle(true);

    let navigator = use_navigator();

    let profile_state = use_async(async move {
        api::get_my_profile().await.map(|user| {
            profile_atom_setter(user.clone().into());
            user
        })
    });
    let grove_state = {
        let grove_is_enabled = grove_is_enabled.clone();

        use_async(async move {
            api::get_grove().await.map(|grove| {
                if !grove.is_enabled {
                    if let Some(navigator) = navigator {
                        navigator.push(&AppRoute::ModAreaRoot);
                    }
                    grove_is_enabled.set(false);
                }
            })
        })
    };

    {
        let profile_state = profile_state.clone();

        use_mount(move || {
            log::debug!(
                "First render, so lets send the request to check if the token is valid and see"
            );
            profile_state.run();
            grove_state.run();
        });
    }

    html!(
        if let Some(_) = &profile_state.error {
            <Redirect<AppRoute> to={AppRoute::Login} />
        } else if let Some(profile) = &profile_state.data {
            <>
                <Switch<AppRoute> render={switch_top_bar}/>
                <CosmoMenuBar>
                    <CosmoMainMenu>
                        if *grove_is_enabled {
                            <Switch<AppRoute> render={render_main_menu_entry("Bambushain", AppRoute::BambooGroveRoot, AppRoute::BambooGrove)} />
                            <Switch<AppRoute> render={render_main_menu_entry("Final Fantasy", AppRoute::FinalFantasyRoot, AppRoute::FinalFantasy)} />
                            <Switch<AppRoute> render={render_main_menu_entry("Bambussupport", AppRoute::SupportRoot, AppRoute::Support)} />
                        }
                        if profile.is_mod {
                            <Switch<AppRoute> render={render_main_menu_entry("Mod Area", AppRoute::ModAreaRoot, AppRoute::ModArea)} />
                        }
                    </CosmoMainMenu>
                    <Switch<AppRoute> render={switch_sub_menu} />
                </CosmoMenuBar>
                <CosmoPageBody>
                    <Switch<AppRoute> render={switch_app(profile.is_mod, *grove_is_enabled)} />
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
                <Switch<AppRoute> render={switch_app(false, true)} />
           </CosmoPageBody>
        </>
    )
}

#[autoprops]
#[function_component(ChangePasswordDialog)]
fn change_password_dialog(on_close: &Callback<()>, mods: &Vec<AttrValue>) -> Html {
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
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Wenn du möchtest dass es von einem Mod zurückgesetzt wird, einfach anschreiben" header="Das alte Passwort ist falsch" />
                    } else if err.code == NOT_FOUND {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Bitte versuch es erneut um einen Fehler auszuschließen" header="Du wurdest scheinbar gelöscht" />
                    } else if *unreported_error_toggle {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Leider konnte dein Passwort nicht geändert werden" header="Fehler beim ändern" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Leider konnte dein Passwort nicht geändert werden" header="Fehler beim ändern" />
                    }
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Information} message={format!("Falls du dich an dein altes Passwort nicht erinnern kannst,\nwende dich an einen Mod: {}", mods.join(", "))} header="Ändere dein Passwort" />
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

        let email_state = email_state.clone();
        let display_name_state = display_name_state.clone();
        let discord_name_state = discord_name_state.clone();

        let on_close = on_close.clone();

        use_async(async move {
            api::update_my_profile(UpdateProfile::new(
                (*email_state).to_string(),
                (*display_name_state).to_string(),
                (*discord_name_state).to_string(),
            ))
            .await
            .map(|_| {
                unreported_error_toggle.set(false);
                on_close.emit(())
            })
            .map_err(|err| {
                unreported_error_toggle.set(true);
                bamboo_error_state.set(err.clone());
                err
            })
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
                    if profile_atom.profile.app_totp_enabled {
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
            <CosmoModal title="Zwei Faktor per App aktivieren" is_form={true} on_form_submit={on_form_submit} buttons={html!(
                <>
                    <CosmoButton on_click={on_close.clone()} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="App einrichten" />
                </>
            )}>
                if let Some(data) = &enable_totp_state.data {
                    <img class={img_style} src={format!("data:image/png;base64,{}", data.qr_code.clone())} alt={data.secret.clone()} />
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
                } else {
                    if enable_totp_state.error.is_some() {
                        if *unreported_error_toggle {
                            <CosmoMessage header="Fehler beim Aktivieren" message="Leider konnte Zwei Faktor per App nicht aktiviert werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                        } else {
                            <CosmoMessage header="Fehler beim Aktivieren" message="Leider konnte Zwei Faktor per App nicht aktiviert werden" message_type={CosmoMessageType::Negative} />
                        }
                    }
                    <CosmoParagraph>{r#"Hier kannst du deinen Zwei Faktor Code anpassen, von Haus aus sendet Bambushain einen Code an deine Emailadresse,
du kannst allerdings auch eine App wie Google Authenticator oder Authy einrichten und dann damit einen Code generieren.
Um eine App einzurichten, musst du unten auf App einrichten klicken.
Anschließend kommt ein QR Code, den musst du scannen und danach einen Code aus deiner App eingeben."#}</CosmoParagraph>
                }
            </CosmoModal>
        </>
    )
}

#[function_component(TopBar)]
fn top_bar() -> Html {
    log::debug!("Render top bar");
    let navigator = use_navigator().expect("Navigator should be available");

    let profile_open_toggle = use_bool_toggle(false);
    let password_open_toggle = use_bool_toggle(false);
    let leave_grove_open_toggle = use_bool_toggle(false);

    let mods_state: UseAsyncHandle<_, ApiError> = use_async(async move {
        Ok(if let Ok(users) = api::get_users().await {
            users
                .into_iter()
                .filter_map(|user| {
                    if user.is_mod {
                        Some(AttrValue::from(user.display_name))
                    } else {
                        None
                    }
                })
                .collect::<Vec<AttrValue>>()
        } else {
            vec![]
        })
    });
    let leave_grove_state =
        use_async(async move { api::leave().await.map(|_| navigator.push(&AppRoute::Login)) });

    let navigator = use_navigator().expect("Navigator should be available");
    let logout = use_callback(navigator, |_: (), navigator| {
        api::logout();
        navigator.push(&AppRoute::Login);
    });
    let open_update_my_profile =
        use_callback(profile_open_toggle.clone(), |_, profile_open_state| {
            profile_open_state.set(true)
        });
    let open_change_password = use_callback(
        (mods_state.clone(), password_open_toggle.clone()),
        |_, (mods_state, password_open_state)| {
            password_open_state.set(true);
            mods_state.run();
        },
    );
    let open_leave_grove = use_callback(leave_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(true)
    });
    let close_leave_grove = use_callback(leave_grove_open_toggle.clone(), |_, toggle| {
        toggle.set(false)
    });
    let leave_grove = use_callback(leave_grove_state.clone(), |_, state| state.run());

    html!(
        <>
            <CosmoTopBar profile_picture="/static/logo.webp" has_right_item={true} right_item_on_click={logout} right_item_label="Abmelden">
                <CosmoTopBarItemLink<AppRoute> label="Rechtliches" to={AppRoute::LegalRoot} />
                <CosmoTopBarItem label="Mein Profil" on_click={open_update_my_profile} />
                <CosmoTopBarItem label="Passwort ändern" on_click={open_change_password} />
                <CosmoTopBarItem label="Hain verlassen" on_click={open_leave_grove} />
            </CosmoTopBar>
            if *profile_open_toggle {
                <UpdateMyProfileDialog on_close={move |_| profile_open_toggle.set(false)} />
            }
            if *password_open_toggle {
                if let Some(data) = &mods_state.data {
                    <ChangePasswordDialog on_close={move |_| password_open_toggle.set(false)} mods={data.clone()} />
                }
            }
            if *leave_grove_open_toggle {
                <CosmoConfirm confirm_type={CosmoModalType::Negative} on_confirm={leave_grove} on_decline={close_leave_grove} title="Hain verlassen" message="Bist du sicher, dass du den Hain verlassen möchtest?\nWenn du den Hain verlässt werden alle deine Daten gelöscht und können nicht wiederhergestellt werden." confirm_label="Hain verlassen" decline_label="Im Hain bleiben" />
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

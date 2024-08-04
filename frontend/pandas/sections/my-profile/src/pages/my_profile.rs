use crate::api;
use bamboo_common::core::entities::{UpdateProfile};
use bamboo_common::frontend::api::{ApiError, CONFLICT, FORBIDDEN, NOT_FOUND};
use bamboo_frontend_pandas_base::controls::{use_dialogs, BambooErrorMessage};
use bamboo_frontend_pandas_base::routing::{AppRoute, GroveRoute};
use bamboo_frontend_pandas_base::storage;
use bamboo_frontend_pandas_base::storage::CurrentUser;
use bamboo_frontend_pandas_section_groves::use_groves;
use bounce::helmet::Helmet;
use bounce::{use_atom, use_atom_setter};
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_timeout, UseAsyncHandle};
use yew_router::hooks::use_navigator;

#[autoprops]
#[function_component(UpdateMyProfileDialog)]
fn update_my_profile_dialog(on_close: &Callback<()>) -> Html {
    log::debug!("Open dialog to update profile");
    let profile_atom = use_atom::<storage::CurrentUser>();

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

    let save_state = {
        let profile_atom = profile_atom.clone();

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
            .await;
            if result.is_ok() {
                if let Some(profile_picture) = (*profile_picture_state).clone() {
                    api::upload_profile_picture(profile_picture).await?;
                }
                if let Ok(profile) = api::get_my_profile().await {
                    profile_atom.set(profile.clone().into());
                }
            }

            result.map(|_| on_close.emit(()))
        })
    };

    let on_save = use_callback(save_state.clone(), |_, save_state| save_state.run());
    let on_close = on_close.clone();

    html!(
        <>
            <Helmet>
                <title>{ "Profil bearbeiten" }</title>
            </Helmet>
            <CosmoModal
                title="Profil bearbeiten"
                is_form=true
                on_form_submit={on_save}
                buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Schließen" />
                    <CosmoButton is_submit={true} label="Profil speichern" />
                </>
            )}
            >
                if let Some(err) = save_state.error.clone() {
                    if err.code == NOT_FOUND {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            message="Bitte versuch es erneut um einen Fehler auszuschließen"
                            header="Du wurdest scheinbar gelöscht"
                        />
                    } else if err.code == CONFLICT {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            message="Die Email oder der Name ist leider schon vergeben"
                            header="Leider schon vergeben"
                        />
                    } else {
                        <BambooErrorMessage
                            message="Dein Profil konnte leider nicht gespeichert werden"
                            header="Fehler beim Speichern"
                            page="layout"
                            form="update_my_profile_dialog"
                            error={err}
                        />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox
                        label="Email"
                        input_type={CosmoTextBoxType::Email}
                        required=true
                        on_input={update_email}
                        value={(*email_state).clone()}
                    />
                    <CosmoTextBox
                        label="Name"
                        required=true
                        on_input={update_display_name}
                        value={(*display_name_state).clone()}
                    />
                    <CosmoTextBox
                        label="Discord Name (optional)"
                        on_input={update_discord_name}
                        value={(*discord_name_state).clone()}
                    />
                    <CosmoFilePicker
                        label="Profilbild (optional)"
                        on_select={select_profile_picture}
                    />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[autoprops]
#[function_component(ChangePasswordDialog)]
fn change_password_dialog(on_close: &Callback<()>) -> Html {
    log::debug!("Open dialog to change password");
    let navigator = use_navigator().expect("Navigator should be available");

    let old_password_state = use_state_eq(|| AttrValue::from(""));
    let new_password_state = use_state_eq(|| AttrValue::from(""));

    let update_old_password =
        use_callback(old_password_state.clone(), |value, state| state.set(value));
    let update_new_password =
        use_callback(new_password_state.clone(), |value, state| state.set(value));

    let save_state = {
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
                navigator.push(&AppRoute::Login);
            })
        })
    };

    let on_close = on_close.clone();
    let on_save = use_callback(save_state.clone(), |_, state| state.run());

    html!(
        <>
            <Helmet>
                <title>{ "Passwort ändern" }</title>
            </Helmet>
            <CosmoModal
                title="Passwort ändern"
                is_form=true
                on_form_submit={on_save}
                buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="Passwort ändern" />
                </>
            )}
            >
                if let Some(err) = save_state.error.clone() {
                    if err.code == FORBIDDEN {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            message="Falls du dein Passwort vergessen hast, melde dich bitte ab und klicke auf Passwort vergessen"
                            header="Das alte Passwort ist falsch"
                        />
                    } else if err.code == NOT_FOUND {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            message="Bitte versuch es erneut um einen Fehler auszuschließen"
                            header="Du wurdest scheinbar gelöscht"
                        />
                    } else {
                        <BambooErrorMessage
                            message="Leider konnte dein Passwort nicht geändert werden"
                            header="Fehler beim Ändern"
                            page="layout"
                            form="change_password_dialog"
                            error={err}
                        />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox
                        input_type={CosmoTextBoxType::Password}
                        label="Aktuelles Passwort"
                        on_input={update_old_password}
                        value={(*old_password_state).clone()}
                        required=true
                    />
                    <CosmoTextBox
                        input_type={CosmoTextBoxType::Password}
                        label="Neues Passwort"
                        on_input={update_new_password}
                        value={(*new_password_state).clone()}
                        required=true
                    />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[autoprops]
#[function_component(EnableTotpDialog)]
fn enable_totp_dialog(on_close: &Callback<()>) -> Html {
    log::debug!("Open dialog to enable totp");
    let profile_atom = use_atom_setter::<storage::CurrentUser>();

    let code_state = use_state_eq(|| AttrValue::from(""));
    let current_password_state = use_state_eq(|| AttrValue::from(""));

    let enable_totp_state = use_async(async move { api::enable_totp().await });
    let validate_totp_state: UseAsyncHandle<(), ApiError> = {
        let code_state = code_state.clone();
        let current_password_state = current_password_state.clone();

        let on_close = on_close.clone();

        let profile_atom = profile_atom.clone();

        use_async(async move {
            api::validate_totp(
                (*code_state).to_string(),
                (*current_password_state).to_string(),
            )
            .await
            .map_err(|err| {
                log::error!("Failed to validate token: {err}");

                err
            })?;

            on_close.emit(());
            if let Ok(profile) = api::get_my_profile().await {
                profile_atom(profile.into())
            }

            Ok(())
        })
    };

    let update_code = use_callback(code_state.clone(), |value, state| state.set(value));
    let update_password = use_callback(current_password_state.clone(), |value, state| {
        state.set(value)
    });
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
                <title>{ "Zwei Faktor per App aktivieren" }</title>
            </Helmet>
            <CosmoModal
                title="Zwei Faktor per App aktivieren"
                is_form=true
                on_form_submit={on_form_submit}
                buttons={html!(
                <>
                    <CosmoButton on_click={on_close.clone()} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="App einrichten" />
                </>
            )}
            >
                <div class={container_style}>
                    if let Some(data) = &enable_totp_state.data {
                        <img
                            class={img_style}
                            src={data.qr_code.clone()}
                            alt={data.secret.clone()}
                        />
                        <svg class={logo_style} viewBox="0 0 512 512">
                            <path
                                d="M511.094,264.722c-1.136-3.307-28.511-81.137-89.171-95.166c-30.729-7.107-63.124,3.303-96.526,30.938v-35.663
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
                                    C423.36,224.901,387.642,218.575,361.327,215.325z"
                            />
                        </svg>
                    } else {
                        <CosmoProgressRing />
                    }
                    <div class={details_style}>
                        <CosmoMessage
                            header="Schritte zum Aktivieren"
                            message="Zu erst musst du den QR Code mit einer App wie Authy oder dem Google Authenticator scannen.\nAnschließend gibst du in den Feldern dein aktuelles Passwort ein und der Code der dir in der App angezeigt wird."
                            message_type={CosmoMessageType::Information}
                        />
                        if let Some(err) = validate_totp_state.error.clone() {
                            if err.code == FORBIDDEN {
                                <CosmoMessage
                                    header="Code oder Passwort falsch"
                                    message="Der von dir eingegebene Code oder dein Passwort ist ungültig, versuch es nochmal"
                                    message_type={CosmoMessageType::Negative}
                                />
                            } else {
                                <BambooErrorMessage
                                    message="Zwei Faktor per App konnte leider nicht aktiviert werden"
                                    header="Fehler beim Aktivieren"
                                    page="layout"
                                    form="update_my_profile_dialog_totp_enable"
                                    error={err}
                                />
                            }
                        }
                        <CosmoInputGroup>
                            <CosmoTextBox
                                input_type={CosmoTextBoxType::Password}
                                label="Aktuelles Passwort"
                                required=true
                                on_input={update_password}
                                value={(*current_password_state).clone()}
                            />
                            <CosmoTextBox
                                label="Zwei Faktor Code"
                                required=true
                                on_input={update_code}
                                value={(*code_state).clone()}
                            />
                        </CosmoInputGroup>
                    </div>
                </div>
            </CosmoModal>
        </>
    )
}

#[function_component(MyProfilePage)]
pub fn my_profile_page() -> Html {
    let profile_atom = use_atom::<CurrentUser>();

    let email = profile_atom.profile.email.clone();
    let display_name = profile_atom.profile.display_name.clone();
    let discord_name = profile_atom.profile.discord_name.clone();

    let profile_open_toggle = use_bool_toggle(false);
    let password_open_toggle = use_bool_toggle(false);
    let app_two_factor_open_toggle = use_bool_toggle(false);

    let groves = use_groves();
    let dialogs = use_dialogs();
    let navigator = use_navigator().expect("Navigator should be available");

    let leave_grove_state =
        use_async(async move { api::leave().await.map(|_| navigator.push(&AppRoute::Login)) });

    let open_change_password =
        use_callback(password_open_toggle.clone(), |_, password_open_state| {
            password_open_state.set(true);
        });

    let leave_grove = use_callback(leave_grove_state.clone(), |_, state| state.run());
    let open_leave_grove = use_callback(
        (leave_grove.clone(), dialogs.clone()),
        |_, (leave_grove, dialogs)| {
            dialogs.confirm(
                "Account löschen",
                "Bist du sicher, dass du deinen Account löschen möchtest?\nWenn du deinen Account löscht, werden alle deine Daten gelöscht und können nicht wiederhergestellt werden.",
                "Account löschen",
                "Account behalten",
                CosmoModalType::Negative,
                leave_grove.clone(),
                Callback::noop(),
            )
        },
    );

    let disable_totp_state = {
        let profile_atom = profile_atom.clone();

        use_async(async move {
            if let Err(err) = api::disable_totp().await {
                Err(err)
            } else {
                if let Ok(profile) = api::get_my_profile().await {
                    profile_atom.set(profile.into());
                }

                Ok(())
            }
        })
    };

    let on_open_profile_edit =
        use_callback(profile_open_toggle.clone(), |_, profile_open_toggle| {
            profile_open_toggle.set(true)
        });

    let on_disable_totp = use_callback(disable_totp_state.clone(), |_, disable_totp_state| {
        disable_totp_state.run()
    });
    let on_open_disable_totp = use_callback(
        (dialogs.clone(), on_disable_totp.clone()),
        |_, (dialogs, on_disable_totp)| {
            dialogs.confirm(
                "Zwei Faktor Authentifizierung deaktivieren",
                "Möchtest du deine Zwei Faktor Authentifizierung per App deaktivieren?",
                "Deaktivieren",
                "Nicht deaktivieren",
                CosmoModalType::Warning,
                on_disable_totp.clone(),
                Callback::noop(),
            )
        },
    );

    let on_enable_app_two_factor = use_callback(
        app_two_factor_open_toggle.clone(),
        |_, app_two_factor_open_toggle| app_two_factor_open_toggle.set(true),
    );

    let grid_style = use_style!(
        r#"
display: grid;
grid-template-rows: auto auto auto 1fr;
grid-template-columns: min(20rem, 15vw) 1fr;
grid-template-areas: "picture toolbar" "picture email" "picture discord" "picture groves";
gap: 1rem;

p,
h2 {
    margin: 0;
}
    "#
    );
    let img_style = use_style!(
        r#"
width: 100%;
grid-area: picture;
    "#
    );

    html! {
        <>
            <Helmet>
                <title>{ format!("Profil von {display_name}") }</title>
            </Helmet>
            <CosmoTitle title={format!("Hallo {display_name}!")} />
            <div class={grid_style}>
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Profil bearbeiten" on_click={on_open_profile_edit} />
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Passwort ändern" on_click={open_change_password} />
                        if profile_atom.profile.totp_validated.unwrap_or(false) {
                            <CosmoButton label="App Zwei Faktor deaktivieren" on_click={on_open_disable_totp} />
                        } else {
                            <CosmoButton label="App Zwei Faktor aktivieren" on_click={on_enable_app_two_factor} />
                        }
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Account löschen" on_click={open_leave_grove} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
                <img
                    class={img_style}
                    src={format!("/api/user/{}/picture#time={}", profile_atom.profile.id, chrono::offset::Local::now().timestamp_millis())}
                />
                <div style="grid-area: email">
                    <CosmoHeader level={CosmoHeaderLevel::H2} header="Emailadresse" />
                    <CosmoAnchor href={format!("mailto:{email}")}>{ email }</CosmoAnchor>
                </div>
                <div style="grid-area: discord">
                    <CosmoHeader level={CosmoHeaderLevel::H2} header="Discordname" />
                    <CosmoParagraph>
                        if discord_name.is_empty() {
                            { "Kein Name hinterlegt" }
                        } else {
                            { discord_name }
                        }
                    </CosmoParagraph>
                </div>
                <div style="grid-area: groves">
                    <CosmoHeader level={CosmoHeaderLevel::H2} header="Meine Haine" />
                    <CosmoKeyValueList>
                        { for groves.groves.iter().map(|grove| {
                            html! {
                                <CosmoKeyValueListItem title={grove.name.clone()}>
                                    <CosmoAnchorLink<GroveRoute> to={GroveRoute::Grove { id: grove.id, name: grove.name.clone() } }>
                                        { format!("{} betreten", grove.name.clone()) }
                                    </CosmoAnchorLink<GroveRoute>>
                                </CosmoKeyValueListItem>
                            }
                        }) }
                    </CosmoKeyValueList>
                </div>
            </div>
            if *profile_open_toggle {
                <UpdateMyProfileDialog on_close={move |_| profile_open_toggle.set(false)} />
            }
            if *app_two_factor_open_toggle {
                <EnableTotpDialog on_close={move |_| app_two_factor_open_toggle.set(false)} />
            }
            if *password_open_toggle {
                <ChangePasswordDialog on_close={move |_| password_open_toggle.set(false)} />
            }
        </>
    }
}

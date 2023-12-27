use std::ops::Deref;

use bounce::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::use_mount;
use yew_hooks::{use_async, use_bool_toggle, use_unmount};

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::{CONFLICT, FORBIDDEN, NOT_FOUND};
use bamboo_frontend_base_error as error;
use bamboo_frontend_base_storage as storage;

use crate::api;
use crate::props::user::*;

#[derive(PartialEq, Clone)]
enum UserConfirmActions {
    MakeMod,
    RemoveMod,
    DisableTotp,
    Delete,
    ChangePassword(String),
    Closed,
}

#[function_component(CreateUserModal)]
fn create_user_modal(props: &CreateUserModalProps) -> Html {
    log::debug!("Create create user modal");
    log::debug!("Initialize state and callbacks");
    let email_state = use_state_eq(|| AttrValue::from(""));
    let display_name_state = use_state_eq(|| AttrValue::from(""));
    let discord_name_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| {
        AttrValue::from(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
        )
    });

    let is_mod_toggle = use_bool_toggle(false);
    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    {
        let email_state = email_state.clone();
        let password_state = password_state.clone();
        let display_name_state = display_name_state.clone();
        let discord_name_state = discord_name_state.clone();

        let is_mod_toggle = is_mod_toggle.clone();

        use_unmount(move || {
            is_mod_toggle.set(false);

            email_state.set("".into());
            password_state.set("".into());
            display_name_state.set("".into());
            discord_name_state.set("".into());
        })
    }

    let save_state = {
        let email_state = email_state.clone();
        let password_state = password_state.clone();
        let display_name_state = display_name_state.clone();
        let discord_name_state = discord_name_state.clone();

        let is_mod_toggle = is_mod_toggle.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        use_async(async move {
            api::create_user(User::new(
                (*email_state).to_string(),
                (*password_state).to_string(),
                (*display_name_state).to_string(),
                (*discord_name_state).to_string(),
                *is_mod_toggle,
            ))
            .await
            .map(|data| {
                log::debug!("User was created successfully");
                unreported_error_toggle.set(false);

                data
            })
            .map_err(|err| {
                log::warn!("Failed to create user {err}");
                unreported_error_toggle.set(true);
                bamboo_error_state.set(err.clone());

                password_state.set(
                    rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect::<String>()
                        .into(),
                );

                err
            })
        })
    };

    let update_email = use_callback(email_state.clone(), |value, state| state.set(value));
    let update_display_name =
        use_callback(display_name_state.clone(), |value, state| state.set(value));
    let update_discord_name =
        use_callback(discord_name_state.clone(), |value, state| state.set(value));

    let update_is_mod = use_callback(is_mod_toggle.clone(), |checked, state| state.set(checked));

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_user",
                "create_user_modal",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    let on_save = use_callback(save_state.clone(), |_, state| state.run());

    let on_saved = use_callback(
        (save_state.clone(), props.on_saved.clone()),
        |_, (save_state, on_saved)| {
            if let Some(user) = &save_state.data {
                on_saved.emit(user.clone())
            }
        },
    );

    html!(
        <CosmoModal title="Panda hinzufügen" is_form={true} on_form_submit={on_save} buttons={
            html!(
                if save_state.data.is_some() {
                    <CosmoButton on_click={on_saved} label="Alles klar" />
                } else {
                    <>
                        <CosmoButton on_click={props.on_close.clone()} label="Abbrechen" />
                        <CosmoButton is_submit={true} label="Panda hinzufügen" />
                    </>
                }
            )}>
            if save_state.data.is_some() {
                <CosmoParagraph>{format!("Das Passwort für {} ist ", (*email_state).clone())}<CosmoCode>{(*password_state).clone()}</CosmoCode></CosmoParagraph>
            } else {
                <>
                    if let Some(err) = &save_state.error {
                        if err.code == FORBIDDEN {
                            <CosmoMessage message="Du musst Mod sein um andere Pandas hinzuzufügen" message_type={CosmoMessageType::Negative} />
                        } else if err.code == CONFLICT {
                            <CosmoMessage message="Ein Panda mit dieser Emailadresse oder Namen ist bereits im Hain" message_type={CosmoMessageType::Negative} />
                        } else if *unreported_error_toggle {
                            <CosmoMessage message="Der Panda konnte leider nicht hinzugefügt werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                        } else {
                            <CosmoMessage message="Der Panda konnte leider nicht hinzugefügt werden" message_type={CosmoMessageType::Negative} />
                        }
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Information} header="Füge einen neuen Panda hinzu" message="Das Passwort wird angezeigt wenn der Panda erfolgreich hinzugefügt wurde" />
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Email" value={(*email_state).clone()} on_input={update_email} required={true} />
                        <CosmoTextBox label="Name" value={(*display_name_state).clone()} on_input={update_display_name} />
                        <CosmoTextBox label="Discord Name (optional)" value={(*discord_name_state).clone()} on_input={update_discord_name} />
                        <CosmoSwitch label="Moderator" on_check={update_is_mod} checked={*is_mod_toggle} />
                    </CosmoInputGroup>
                </>
            }
        </CosmoModal>
    )
}

#[function_component(UpdateProfileDialog)]
fn update_profile_dialog(props: &UpdateProfileDialogProps) -> Html {
    log::debug!("Open dialog to update profile");
    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let display_name_state = use_state_eq(|| props.display_name.clone());
    let email_state = use_state_eq(|| props.email.clone());
    let discord_name_state = use_state_eq(|| props.discord_name.clone());

    let save_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();

        let display_name_state = display_name_state.clone();
        let email_state = email_state.clone();
        let discord_name_state = discord_name_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let id = props.id;

        let on_update = props.on_update.clone();

        use_async(async move {
            api::update_profile(
                id,
                UpdateProfile::new(
                    (*email_state).to_string(),
                    (*display_name_state).to_string(),
                    (*discord_name_state).to_string(),
                ),
            )
            .await
            .map(|_| on_update.emit(()))
            .map_err(|err| {
                bamboo_error_state.set(err.clone());
                unreported_error_toggle.set(true);

                err
            })
        })
    };

    let update_display_name =
        use_callback(display_name_state.clone(), |value, state| state.set(value));
    let update_email = use_callback(email_state.clone(), |value, state| state.set(value));
    let update_discord_name =
        use_callback(discord_name_state.clone(), |value, state| state.set(value));
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_user",
                "update_profile_dialog",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );
    let on_save = use_callback(save_state.clone(), |_, state| state.run());

    html!(
        <>
            <CosmoModal title="Panda bearbeiten" is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={props.on_close.clone()} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="Panda speichern" />
                </>
            )}>
                if let Some(err) = &save_state.error {
                    if err.code == FORBIDDEN {
                        <CosmoMessage message="Du musst Mod sein um andere Pandas zu bearbeiten" message_type={CosmoMessageType::Negative} />
                    } else if err.code == NOT_FOUND {
                        <CosmoMessage message="Der Panda wurde nicht gefunden" message_type={CosmoMessageType::Negative} />
                    } else if err.code == CONFLICT {
                        <CosmoMessage message="Ein Panda mit dieser Emailadresse oder Namen ist bereits im Hain" message_type={CosmoMessageType::Negative} />
                    } else if *unreported_error_toggle {
                        <CosmoMessage message="Der Panda konnte leider nicht geändert werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message="Der Panda konnte leider nicht geändert werden" message_type={CosmoMessageType::Negative} />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox label="Email" required={true} input_type={CosmoTextBoxType::Email} on_input={update_email} value={(*email_state).clone()} />
                    <CosmoTextBox label="Name" required={true} on_input={update_display_name} value={(*display_name_state).clone()} />
                    <CosmoTextBox label="Discord Name (optional)" on_input={update_discord_name} value={(*discord_name_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(UserDetails)]
fn user_details(props: &UserDetailsProps) -> Html {
    log::debug!("Initialize table body state and callbacks");
    let confirm_state = use_state_eq(|| UserConfirmActions::Closed);

    let current_user = use_atom::<storage::CurrentUser>();

    let profile_edit_toggle = use_bool_toggle(false);
    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let delete_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let confirm_state = confirm_state.clone();

        let on_delete = props.on_delete.clone();

        let user_id = props.user.id;

        use_async(async move {
            api::delete_user(user_id)
                .await
                .map(|_| {
                    unreported_error_toggle.set(false);
                    confirm_state.set(UserConfirmActions::Closed);

                    on_delete.emit(())
                })
                .map_err(|err| {
                    log::error!("Failed to delete {err}");
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(true);

                    err
                })
        })
    };
    let make_mod_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let confirm_state = confirm_state.clone();

        let on_update = props.on_update.clone();

        let user_id = props.user.id;

        use_async(async move {
            api::make_user_mod(user_id)
                .await
                .map(|_| {
                    unreported_error_toggle.set(false);
                    confirm_state.set(UserConfirmActions::Closed);

                    on_update.emit(())
                })
                .map_err(|err| {
                    log::error!("Failed to make mod {err}");
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(true);

                    err
                })
        })
    };
    let remove_mod_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let confirm_state = confirm_state.clone();

        let on_update = props.on_update.clone();

        let user_id = props.user.id;

        use_async(async move {
            api::remove_user_mod(user_id)
                .await
                .map(|_| {
                    unreported_error_toggle.set(false);
                    confirm_state.set(UserConfirmActions::Closed);

                    on_update.emit(())
                })
                .map_err(|err| {
                    log::error!("Failed to remove mod {err}");
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(true);

                    err
                })
        })
    };
    let disable_totp_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let confirm_state = confirm_state.clone();

        let on_update = props.on_update.clone();

        let user_id = props.user.id;

        use_async(async move {
            api::disable_totp(user_id)
                .await
                .map(|_| {
                    unreported_error_toggle.set(false);
                    confirm_state.set(UserConfirmActions::Closed);
                    on_update.emit(())
                })
                .map_err(|err| {
                    log::error!("Failed to disable totp {err}");
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(true);

                    err
                })
        })
    };
    let change_password_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let confirm_state = confirm_state.clone();

        let on_update = props.on_update.clone();

        let user_id = props.user.id;

        use_async(async move {
            if let UserConfirmActions::ChangePassword(new_password) = (*confirm_state).clone() {
                api::change_user_password(user_id, new_password)
                    .await
                    .map(|_| {
                        unreported_error_toggle.set(false);
                        confirm_state.set(UserConfirmActions::Closed);

                        on_update.emit(())
                    })
                    .map_err(|err| {
                        log::error!("Failed to change password {err}");
                        bamboo_error_state.set(err.clone());
                        unreported_error_toggle.set(true);

                        err
                    })
            } else {
                Ok(())
            }
        })
    };

    let make_mod_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::MakeMod)
    });
    let remove_mod_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::RemoveMod)
    });
    let delete_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::Delete)
    });
    let disable_totp_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::DisableTotp)
    });
    let update_profile_click =
        use_callback(profile_edit_toggle.clone(), |_, profile_edit_toggle| {
            profile_edit_toggle.set(true)
        });
    let change_password_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::ChangePassword(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
        ))
    });
    let on_decline = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::Closed)
    });
    let on_update_close = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::Closed)
    });
    let on_confirm = use_callback(
        (
            confirm_state.clone(),
            delete_state.clone(),
            make_mod_state.clone(),
            remove_mod_state.clone(),
            disable_totp_state.clone(),
            change_password_state.clone(),
        ),
        |_,
         (
            confirm_state,
            delete_state,
            make_mod_state,
            remove_mod_state,
            disable_totp_state,
            change_password_state,
        )| match **confirm_state {
            UserConfirmActions::MakeMod => make_mod_state.run(),
            UserConfirmActions::RemoveMod => remove_mod_state.run(),
            UserConfirmActions::DisableTotp => disable_totp_state.run(),
            UserConfirmActions::Delete => delete_state.run(),
            UserConfirmActions::ChangePassword(_) => change_password_state.run(),
            UserConfirmActions::Closed => (),
        },
    );
    let on_update = use_callback(props.on_update.clone(), |_, on_update| on_update.emit(()));
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_user",
                "user_details",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    html!(
        <>
            <CosmoTitle title={props.user.display_name.clone()} subtitle={props.user.email.clone()} />
            if current_user.profile.is_mod {
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        if props.user.is_mod {
                            <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={remove_mod_click} label="Modrechte entziehen" />
                        } else {
                            <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={make_mod_click} label="Zum Mod machen" />
                        }
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={update_profile_click} label="Panda bearbeiten" />
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={change_password_click} label="Passwort ändern" />
                        <CosmoButton enabled={props.user.id != current_user.profile.id && props.user.app_totp_enabled} on_click={disable_totp_click} label="Zwei Faktor deaktivieren" />
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={delete_click} label="Aus dem Hain werfen" />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
            }
            if let Some(err) = &delete_state.error {
                if err.code == FORBIDDEN {
                    <CosmoMessage header="Fehler beim Rauswerfen" message="Du musst Mod sein um Pandas aus dem Hain zu werfen" message_type={CosmoMessageType::Negative} />
                } else if err.code == CONFLICT {
                    <CosmoMessage header="Fehler beim Rauswerfen" message="Du kannst dich nicht selbst aus dem Hain werfen, wenn du gehen möchtest, wende dich an einen Mod" message_type={CosmoMessageType::Negative} />
                } else if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Rauswerfen" message="Der Panda konnte nicht aus dem Hain geworfen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Rauswerfen" message="Der Panda konnte nicht aus dem Hain geworfen werden" message_type={CosmoMessageType::Negative} />
                }
            }
            if let Some(err) = &make_mod_state.error {
                if err.code == FORBIDDEN {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Du musst Mod sein um Mods zu ernennen" message_type={CosmoMessageType::Negative} />
                } else if err.code == CONFLICT {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Du kannst dich nicht selbst zum Mod machen" message_type={CosmoMessageType::Negative} />
                } else if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Der Panda konnte nicht zum Mod gemacht werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Der Panda konnte nicht zum Mod gemacht werden" message_type={CosmoMessageType::Negative} />
                }
            }
            if let Some(err) = &remove_mod_state.error {
                if err.code == FORBIDDEN {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Du musst Mod sein um Pandas die Modrechte zu entziehen" message_type={CosmoMessageType::Negative} />
                } else if err.code == CONFLICT {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Du kannst dir die Modrechte nicht entziehen" message_type={CosmoMessageType::Negative} />
                } else if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Dem Panda konnten die Modrechte nicht entzogen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Ändern des Modstatus" message="Dem Panda konnten die Modrechte nicht entzogen werden" message_type={CosmoMessageType::Negative} />
                }
            }
            if let Some(err) = &disable_totp_state.error {
                if err.code == FORBIDDEN {
                    <CosmoMessage header="Fehler beim Deaktivieren" message="Du musst Mod sein um die Zwei Faktor Authentifizierung von Pandas zu deaktivieren" message_type={CosmoMessageType::Negative} />
                } else if err.code == CONFLICT {
                    <CosmoMessage header="Fehler beim Deaktivieren" message="Du kannst deine eigene Zwei Faktor Authentifizierung über dein Profil deaktivieren" message_type={CosmoMessageType::Negative} />
                } else if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Deaktivieren" message="Die Zwei Faktor Authentifizierung konnte nicht deaktiviert werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Deaktivieren" message="Die Zwei Faktor Authentifizierung konnte nicht deaktiviert werden" message_type={CosmoMessageType::Negative} />
                }
            }
            if let Some(err) = &change_password_state.error {
                if err.code == FORBIDDEN {
                    <CosmoMessage header="Fehler beim Zurücksetzen" message="Du musst Mod sein um Passwörter zurückzusetzen" message_type={CosmoMessageType::Negative} />
                } else if err.code == CONFLICT {
                    <CosmoMessage header="Fehler beim Zurücksetzen" message="Wenn du dein Passwort ändern willst, kannst du das über Passwort ändern machen" message_type={CosmoMessageType::Negative} />
                } else if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Zurücksetzen" message="Das Passwort konnte nicht zurückgesetzt werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Zurücksetzen" message="Das Passwort konnte nicht zurückgesetzt werden" message_type={CosmoMessageType::Negative} />
                }
            }
            <CosmoKeyValueList>
                <CosmoKeyValueListItem title="Name">
                    {props.user.display_name.clone()}
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Email">
                    {props.user.email.clone()}
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Discord Name">
                    if props.user.discord_name.is_empty() {
                        {"Kein Discord Name gesetzt"}
                    } else {
                        {props.user.discord_name.clone()}
                    }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Ist Moderator">
                    if props.user.is_mod {
                        {"Ja"}
                    } else {
                        {"Nein"}
                    }
                </CosmoKeyValueListItem>
            </CosmoKeyValueList>
            {match (*confirm_state).clone() {
                UserConfirmActions::MakeMod => html!(
                    <CosmoConfirm message={format!("Soll der Panda {} zum Mod gemacht werden?", props.user.display_name.clone())} title="Zum Mod machen" on_decline={on_decline} on_confirm={on_confirm} decline_label="Abbrechen" confirm_label="Zum Mod machen" />
                ),
                UserConfirmActions::RemoveMod => html!(
                    <CosmoConfirm message={format!("Sollen dem Panda {} wirklich die Modrechte entzogen werden?", props.user.display_name.clone())} title="Modrechte entziehen" on_decline={on_decline} on_confirm={on_confirm} confirm_label="Modrechte entziehen" decline_label="Abbrechen" />
                ),
                UserConfirmActions::Delete => html!(
                    <CosmoConfirm confirm_type={CosmoModalType::Warning} message={format!("Soll der Panda {} wirklich aus dem Hain geworfen werden?", props.user.display_name.clone())} title="Panda rauswerfen" on_decline={on_decline} on_confirm={on_confirm} confirm_label="Panda rauswerfen" decline_label="Panda behalten" />
                ),
                UserConfirmActions::DisableTotp => html!(
                    <CosmoConfirm confirm_type={CosmoModalType::Warning} message={format!("Soll die Zwei Faktor Authentifizierung von {} wirklich deaktiviert werden?", props.user.display_name.clone())} title="Zwei Faktor Authentifizierung deaktivieren" on_decline={on_decline} on_confirm={on_confirm} confirm_label="Deaktivieren" decline_label="Nicht deaktivieren" />
                ),
                UserConfirmActions::ChangePassword(password) => {
                    html!(
                        <CosmoModal title="Passwort zurücksetzen" buttons={html!(
                            <>
                                <CosmoButton on_click={on_decline} label="Abbrechen" />
                                <CosmoButton on_click={on_confirm} label="Passwort zurücksetzen" />
                            </>
                        )}>
                            <CosmoParagraph>{format!("Das neue Passwort für {} wird auf ", props.user.display_name)}<CosmoCode>{password}</CosmoCode>{" gesetzt."}</CosmoParagraph>
                        </CosmoModal>
                    )
                },
                UserConfirmActions::Closed => html!(),
            }}
            if *profile_edit_toggle {
                <UpdateProfileDialog on_close={on_update_close} on_update={on_update} id={props.user.id} email={props.user.email.clone()} display_name={props.user.display_name.clone()} discord_name={props.user.discord_name.clone()} />
            }
        </>
    )
}

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    log::debug!("Render users page");
    log::debug!("Initialize state and callbacks");
    let current_user = use_atom::<storage::CurrentUser>();

    let open_create_user_modal_toggle = use_bool_toggle(false);
    let unreported_error_toggle = use_bool_toggle(false);

    let selected_user_state = use_state_eq(|| 0);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let users_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            unreported_error_toggle.set(false);

            api::get_users().await.map_err(|err| {
                bamboo_error_state.set(err.clone());
                unreported_error_toggle.set(true);

                err
            })
        })
    };

    let open_create_user_modal_click = use_callback(
        open_create_user_modal_toggle.clone(),
        |_, open_create_user_modal_toggle| open_create_user_modal_toggle.set(true),
    );
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_user",
                "users_page",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );
    let on_delete = use_callback(
        (selected_user_state.clone(), users_state.clone()),
        |_, (selected_state, users_state)| {
            users_state.run();
            selected_state.set(0);
        },
    );
    let on_update = use_callback(users_state.clone(), |_, users_state| {
        users_state.run();
    });
    let on_create_saved = use_callback(
        (selected_user_state.clone(), users_state.clone()),
        |user: WebUser, (selected_state, users_state)| {
            users_state.run();
            selected_state.set(user.id);
        },
    );
    let on_create_close = use_callback(open_create_user_modal_toggle.clone(), |_, state| {
        state.set(false)
    });

    {
        let users_state = users_state.clone();

        use_mount(move || {
            users_state.run();
        });
    }

    if users_state.loading {
        html!(
            <CosmoProgressRing />
        )
    } else if users_state.error.is_some() {
        if *unreported_error_toggle {
            html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Pandas konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
            )
        } else {
            html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Pandas konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
            )
        }
    } else if let Some(data) = &users_state.data {
        let select_user = {
            let data = data.clone();
            let selected_user_state = selected_user_state.clone();

            Callback::from(move |idx| {
                selected_user_state.set(data.get(idx).map(|u: &WebUser| u.id).unwrap_or(0))
            })
        };

        html!(
            <>
                <CosmoSideList on_select_item={select_user} selected_index={data.iter().position(|u| u.id == *selected_user_state).unwrap_or(0)} has_add_button={current_user.profile.is_mod} add_button_on_click={open_create_user_modal_click} add_button_label="Panda hinzufügen">
                    {for data.iter().map(|user| {
                        CosmoSideListItem::from_label_and_children(user.display_name.clone().into(), html!(
                            <UserDetails on_delete={on_delete.clone()} on_update={on_update.clone()} user={user.clone()} />
                        ))
                    })}
                </CosmoSideList>
                if *open_create_user_modal_toggle {
                    <CreateUserModal on_saved={on_create_saved} on_close={on_create_close} />
                }
            </>
        )
    } else {
        html!()
    }
}

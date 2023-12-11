use std::ops::Deref;

use bounce::helmet::Helmet;
use bounce::prelude::*;
use bounce::query::use_query_value;
use rand::distributions::Alphanumeric;
use rand::Rng;
use yew::prelude::*;
use yew_cosmo::prelude::*;

use bamboo_entities::prelude::*;

use crate::api::*;
use crate::storage::CurrentUser;

#[derive(Properties, PartialEq, Clone)]
struct UserDetailsProps {
    user: WebUser,
    on_delete: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
struct CreateUserModalProps {
    on_saved: Callback<WebUser>,
    on_close: Callback<()>,
}

#[derive(PartialEq, Clone)]
enum UserConfirmActions {
    MakeMod,
    RemoveMod,
    Delete,
    ChangePassword(String),
    Closed,
}

#[derive(Properties, Clone, PartialEq)]
struct UpdateProfileDialogProps {
    on_close: Callback<()>,
    display_name: AttrValue,
    email: AttrValue,
    discord_name: AttrValue,
    id: i32,
}

#[function_component(CreateUserModal)]
fn create_user_modal(props: &CreateUserModalProps) -> Html {
    log::debug!("Create create user modal");
    log::debug!("Initialize state and callbacks");
    let email_state = use_state_eq(|| AttrValue::from(""));
    let display_name_state = use_state_eq(|| AttrValue::from(""));
    let discord_name_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| {
        AttrValue::from(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
        )
    });

    let is_mod_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let created_state = use_state_eq(|| false);

    let created_user_state = use_state_eq(WebUser::default);

    let update_email = use_callback(email_state.clone(), |value, state| state.set(value));
    let update_display_name =
        use_callback(display_name_state.clone(), |value, state| state.set(value));
    let update_discord_name =
        use_callback(discord_name_state.clone(), |value, state| state.set(value));

    let update_is_mod = use_callback(is_mod_state.clone(), |checked, state| state.set(checked));

    let form_submit = {
        let email_state = email_state.clone();
        let password_state = password_state.clone();
        let display_name_state = display_name_state.clone();
        let discord_name_state = discord_name_state.clone();
        let error_message_state = error_message_state.clone();

        let is_mod_state = is_mod_state.clone();
        let error_state = error_state.clone();
        let created_state = created_state.clone();

        let created_user = created_user_state.clone();

        Callback::from(move |_| {
            log::debug!("Submit executed user is about to be created");
            let email_state = email_state.clone();
            let display_name_state = display_name_state.clone();
            let discord_name_state = discord_name_state.clone();
            let password_state = password_state.clone();
            let error_message_state = error_message_state.clone();

            let is_mod_state = is_mod_state.clone();
            let error_state = error_state.clone();
            let created_state = created_state.clone();

            let created_user = created_user.clone();

            let user = User::new(
                (*email_state).to_string(),
                (*password_state).to_string(),
                (*display_name_state).to_string(),
                (*discord_name_state).to_string(),
                *is_mod_state,
            );

            yew::platform::spawn_local(async move {
                log::debug!("Create new user with email {}", user.email.clone());
                match create_user(user).await {
                    Ok(user) => {
                        log::debug!("User was created successfully, lets reload the users");
                        error_message_state.set(AttrValue::from(""));
                        error_state.set(false);
                        created_state.set(true);
                        created_user.set(user);
                    }
                    Err(err) => {
                        log::warn!("Failed to create user {}", err);
                        error_state.set(true);
                        if err.code == CONFLICT {
                            error_message_state
                                .set("Ein Panda mit dieser Email ist bereits im Hain".into());
                        } else {
                            error_message_state.set("Der Panda konnte nicht hinzugefügt werden, bitte wende dich an Azami".into());
                        }
                        password_state.set(
                            rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(8)
                                .map(char::from)
                                .collect::<String>()
                                .into(),
                        );
                    }
                }
            });
        })
    };

    let on_saved = use_callback(
        (created_user_state, props.on_saved.clone()),
        |_, (created_user_state, on_saved)| on_saved.emit((**created_user_state).clone()),
    );

    html!(
        <CosmoModal title="Panda hinzufügen" is_form={true} on_form_submit={form_submit} buttons={
            html!(
                if *created_state {
                    <CosmoButton on_click={on_saved} label="Alles klar" />
                } else {
                    <>
                        <CosmoButton on_click={props.on_close.clone()} label="Abbrechen" />
                        <CosmoButton is_submit={true} label="Panda hinzufügen" />
                    </>
                }
            )}>
            if *created_state {
                <CosmoParagraph>{format!("Das Passwort für {} ist ", (*email_state).clone())}<CosmoCode>{(*password_state).clone()}</CosmoCode></CosmoParagraph>
            } else {
                <>
                    if *error_state {
                        <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim hinzufügen" message={(*error_message_state).clone()} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Information} header="Füge einen neuen Panda hinzu" message="Das Passwort wird angezeigt wenn der Panda erfolgreich hinzugefügt wurde" />
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Email" value={(*email_state).clone()} on_input={update_email} required={true} />
                        <CosmoTextBox label="Name" value={(*display_name_state).clone()} on_input={update_display_name} />
                        <CosmoTextBox label="Discord Name (optional)" value={(*discord_name_state).clone()} on_input={update_discord_name} />
                        <CosmoSwitch label="Moderator" on_check={update_is_mod} checked={*is_mod_state} />
                    </CosmoInputGroup>
                </>
            }
        </CosmoModal>
    )
}

#[function_component(UpdateProfileDialog)]
fn update_profile_dialog(props: &UpdateProfileDialogProps) -> Html {
    log::debug!("Open dialog to update profile");
    let error_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let display_name_state = use_state_eq(|| props.display_name.clone());
    let email_state = use_state_eq(|| props.email.clone());
    let discord_name_state = use_state_eq(|| props.discord_name.clone());

    let update_display_name =
        use_callback(display_name_state.clone(), |value, state| state.set(value));
    let update_email = use_callback(email_state.clone(), |value, state| state.set(value));
    let update_discord_name =
        use_callback(discord_name_state.clone(), |value, state| state.set(value));

    let on_save = {
        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let display_name_state = display_name_state.clone();
        let email_state = email_state.clone();
        let discord_name_state = discord_name_state.clone();

        let id = props.id;

        let on_close = props.on_close.clone();

        Callback::from(move |_| {
            log::debug!("Perform password change");
            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let display_name_state = display_name_state.clone();
            let discord_name_state = discord_name_state.clone();
            let email_state = email_state.clone();

            let on_close = on_close.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match update_profile(id, UpdateProfile::new((*email_state).to_string(), (*display_name_state).to_string(), (*discord_name_state).to_string())).await {
                    Ok(_) => {
                        log::debug!("Profile update successful");
                        on_close.emit(());

                        false
                    }
                    Err(err) => match err.code {
                        FORBIDDEN => {
                            error_message_state.set("Du musst Mod sein um andere Pandas zu bearbeiten".into());
                            true
                        }
                        NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set("Der Panda wurde nicht gefunden".into());

                            true
                        }
                        _ => {
                            log::warn!("Failed to change the profile {err}");
                            error_message_state.set("Der Panda konnte leider nicht geändert werden, bitte wende dich an Azami".into());

                            true
                        }
                    }
                });
            });
        })
    };
    let close_error = use_callback(error_state.clone(), |_, state| state.set(false));

    html!(
        <>
            <CosmoModal title="Panda bearbeiten" is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={props.on_close.clone()} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="Panda speichern" />
                </>
            )}>
                <CosmoInputGroup>
                    <CosmoTextBox label="Email" required={true} input_type={CosmoTextBoxType::Email} on_input={update_email} value={(*email_state).clone()} />
                    <CosmoTextBox label="Name" required={true} on_input={update_display_name} value={(*display_name_state).clone()} />
                    <CosmoTextBox label="Discord Name (optional)" on_input={update_discord_name} value={(*discord_name_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
            if *error_state {
                <CosmoAlert alert_type={CosmoModalType::Negative} title="Fehler" message={(*error_message_state).clone()} close_label="Schließen" on_close={close_error} />
            }
        </>
    )
}

#[function_component(UserDetails)]
fn user_details(props: &UserDetailsProps) -> Html {
    log::debug!("Initialize crew table body state and callbacks");
    let confirm_state = use_state_eq(|| UserConfirmActions::Closed);

    let current_user = use_atom::<CurrentUser>();

    let error_state = use_state_eq(|| false);
    let profile_edit_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let users_query_state = use_query_value::<Users>(().into());

    let make_mod_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::MakeMod)
    });
    let remove_mod_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::RemoveMod)
    });
    let delete_click = use_callback(confirm_state.clone(), |_, state| {
        state.set(UserConfirmActions::Delete)
    });
    let update_profile_click = use_callback(profile_edit_state.clone(), |_, profile_edit_state| {
        profile_edit_state.set(true)
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
    let on_confirm = {
        let confirm_state = confirm_state.clone();
        let error_state = error_state.clone();
        let error_message_state = error_message_state.clone();

        let users_query_state = users_query_state.clone();

        let on_delete = props.on_delete.clone();

        let id = props.user.id;

        Callback::from(move |_| {
            log::debug!("Modal was confirmed lets execute the request");
            let confirm_state = confirm_state.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();

            let users_query_state = users_query_state.clone();

            let on_delete = on_delete.clone();

            yew::platform::spawn_local(async move {
                let code = match confirm_state.deref() {
                    UserConfirmActions::MakeMod => match make_user_mod(id).await {
                        Ok(_) => {
                            confirm_state.set(UserConfirmActions::Closed);
                            NO_CONTENT
                        }
                        Err(err) => match err.code {
                            FORBIDDEN => {
                                error_message_state
                                    .set(AttrValue::from("Du musst Mod sein um Mods zu ernennen"));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from(
                                    "Du kannst dich nicht selbst zum Mod machen",
                                ));
                                CONFLICT
                            }
                            _ => {
                                error_message_state.set(AttrValue::from("Der Panda konnte nicht zum Mod gemacht werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                        },
                    },
                    UserConfirmActions::RemoveMod => match remove_user_mod(id).await {
                        Ok(_) => {
                            confirm_state.set(UserConfirmActions::Closed);
                            NO_CONTENT
                        }
                        Err(err) => match err.code {
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from(
                                    "Du musst Mod sein um Pandas die Modrechte zu entziehen",
                                ));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from(
                                    "Du kannst dir die Modrechte nicht entziehen",
                                ));
                                CONFLICT
                            }
                            _ => {
                                error_message_state.set(AttrValue::from("Dem Panda konnten die Modrechte nicht entzogen werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                        },
                    },
                    UserConfirmActions::Delete => match delete_user(id).await {
                        Ok(_) => {
                            confirm_state.set(UserConfirmActions::Closed);
                            on_delete.emit(());
                            NO_CONTENT
                        }
                        Err(err) => match err.code {
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from(
                                    "Du musst Mod sein um Pandas aus dem Hain zu werfen",
                                ));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from("Du kannst dich nicht selbst aus dem Hain werfen, wenn du gehen möchtest, wende dich an einen Mod"));
                                CONFLICT
                            }
                            _ => {
                                error_message_state.set(AttrValue::from("Der Panda konnte nicht aus dem Hain geworfen werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                        },
                    },
                    UserConfirmActions::ChangePassword(new_password) => {
                        match change_user_password(id, new_password.clone()).await {
                            Ok(_) => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            Err(err) => match err.code {
                                FORBIDDEN => {
                                    error_message_state.set(
                                        "Du musst Mod sein um Passwörter zurückzusetzen".into(),
                                    );
                                    FORBIDDEN
                                }
                                CONFLICT => {
                                    error_message_state.set("Wenn du dein Passwort ändern willst, mach das bitte über Passwort ändern".into());
                                    CONFLICT
                                }
                                _ => {
                                    error_message_state.set("Das Passwort konnte nicht zurückgesetzt werden, bitte wende dich an Azami".into());
                                    INTERNAL_SERVER_ERROR
                                }
                            },
                        }
                    }
                    UserConfirmActions::Closed => unreachable!(),
                };

                error_state.set(if code == NO_CONTENT {
                    log::debug!("Update was successful");
                    let _ = users_query_state.refresh().await;
                    false
                } else {
                    log::warn!("Change failed");
                    log::warn!("{}", *error_message_state);
                    true
                });
            });
        })
    };
    let on_alert_close = use_callback(
        (error_state.clone(), error_message_state.clone()),
        |_, (error_state, error_message_state)| {
            error_state.set(false);
            error_message_state.set("".into());
        },
    );
    let on_update_profile_close = {
        let profile_edit_state = profile_edit_state.clone();

        Callback::from(move |_: ()| {
            let users_query_state = users_query_state.clone();

            let profile_edit_state = profile_edit_state.clone();

            yew::platform::spawn_local(async move {
                let _ = users_query_state.refresh().await;
                profile_edit_state.set(false);
            });
        })
    };

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
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={change_password_click} label="Passwort ändern" />
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={delete_click} label="Aus dem Hain werfen" />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
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
                    <CosmoConfirm message={format!("Soll der Panda {} zum Mod gemacht werden?", props.user.email.clone())} title="Zum Mod machen" on_decline={on_decline} on_confirm={on_confirm} decline_label="Abbrechen" confirm_label="Zum Mod machen" />
                ),
                UserConfirmActions::RemoveMod => html!(
                    <CosmoConfirm message={format!("Sollen dem Panda {} wirklich die Modrechte entzogen werden?", props.user.email.clone())} title="Modrechte entziehen" on_decline={on_decline} on_confirm={on_confirm} confirm_label="Modrechte entziehen" decline_label="Abbrechen" />
                ),
                UserConfirmActions::Delete => html!(
                    <CosmoConfirm confirm_type={CosmoModalType::Warning} message={format!("Soll der Panda {} wirklich aus dem Hain geworfen werden?", props.user.email.clone())} title="Panda rauswerfen" on_decline={on_decline} on_confirm={on_confirm} confirm_label="Panda rauswerfen" decline_label="Panda behalten" />
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
            if *error_state {
                <CosmoAlert alert_type={CosmoModalType::Negative} title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={on_alert_close} close_label="Schließen" />
            }
            if *profile_edit_state {
                <UpdateProfileDialog on_close={on_update_profile_close} id={props.user.id} email={props.user.email.clone()} display_name={props.user.display_name.clone()} discord_name={props.user.discord_name.clone()} />
            }
        </>
    )
}

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    log::debug!("Render users page");
    log::debug!("Initialize state and callbacks");
    let current_user = use_atom::<CurrentUser>();

    let users_query_state = use_query_value::<Users>(().into());

    let users_state = use_state_eq(|| vec![] as Vec<WebUser>);

    let open_create_user_modal_state = use_state_eq(|| false);
    let initial_loaded_state = use_state_eq(|| false);

    let selected_user_state = use_state_eq(|| 0);

    let open_create_user_modal_click = use_callback(
        open_create_user_modal_state.clone(),
        |_, open_create_user_modal_state| open_create_user_modal_state.set(true),
    );
    let on_user_select = use_callback(selected_user_state.clone(), |idx, state| state.set(idx));

    let on_delete = {
        let users_query_state = users_query_state.clone();

        let selected_user_state = selected_user_state.clone();

        Callback::from(move |_| {
            let users_query_state = users_query_state.clone();

            let selected_user_state = selected_user_state.clone();

            yew::platform::spawn_local(async move {
                selected_user_state.set(0);
                let _ = users_query_state.refresh().await;
            })
        })
    };
    let on_create_saved = {
        let users_query_state = users_query_state.clone();

        let selected_user_state = selected_user_state.clone();

        let open_create_user_modal_state = open_create_user_modal_state.clone();

        Callback::from(move |user: WebUser| {
            let users_query_state = users_query_state.clone();

            let selected_user_state = selected_user_state.clone();

            let open_create_user_modal_state = open_create_user_modal_state.clone();

            let email = user.email;

            yew::platform::spawn_local(async move {
                open_create_user_modal_state.set(false);
                if let Ok(res) = users_query_state.refresh().await {
                    selected_user_state.set(
                        res.users
                            .iter()
                            .position(move |user| user.email.eq(&email))
                            .unwrap_or(0),
                    );
                }
            })
        })
    };
    let on_create_close = use_callback(open_create_user_modal_state.clone(), |_, state| {
        state.set(false)
    });

    match users_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <CosmoProgressRing />
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded users");
            users_state.set(res.users.clone());
            initial_loaded_state.set(true);
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Pandas konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Pandas"}</title>
            </Helmet>
            <CosmoSideList on_select_item={on_user_select} selected_index={*selected_user_state} has_add_button={current_user.profile.is_mod} add_button_on_click={open_create_user_modal_click} add_button_label="Panda hinzufügen">
                {for (*users_state).clone().into_iter().map(|user| {
                    CosmoSideListItem::from_label_and_children(user.display_name.clone().into(), html!(
                        <UserDetails on_delete={on_delete.clone()} user={user} />
                    ))
                })}
            </CosmoSideList>
            if *open_create_user_modal_state {
                <CreateUserModal on_saved={on_create_saved} on_close={on_create_close} />
            }
        </>
    )
}
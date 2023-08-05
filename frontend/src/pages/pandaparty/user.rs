use std::ops::Deref;

use bounce::helmet::Helmet;
use bounce::prelude::*;
use bounce::query::use_query_value;
use rand::distributions::Alphanumeric;
use rand::Rng;
use yew::prelude::*;
use yew_cosmo::prelude::*;

use pandaparty_entities::prelude::*;

use crate::api::*;
use crate::hooks::event_source::use_event_source;
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
    gear_level: AttrValue,
    job: AttrValue,
    discord_name: AttrValue,
    id: i32,
}

#[function_component(CreateUserModal)]
fn create_user_modal(props: &CreateUserModalProps) -> Html {
    log::debug!("Create create user modal");
    log::debug!("Initialize state and callbacks");
    let username_state = use_state_eq(|| AttrValue::from(""));
    let job_state = use_state_eq(|| AttrValue::from(""));
    let gear_level_state = use_state_eq(|| AttrValue::from(""));
    let discord_name_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| AttrValue::from(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>()));

    let is_mod_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let created_state = use_state_eq(|| false);

    let created_user_state = use_state_eq(WebUser::default);

    let update_username = use_callback(|value, state| state.set(value), username_state.clone());
    let update_job = use_callback(|value, state| state.set(value), job_state.clone());
    let update_gear_level = use_callback(|value, state| state.set(value), gear_level_state.clone());
    let update_discord_name = use_callback(|value, state| state.set(value), discord_name_state.clone());

    let update_is_mod = use_callback(|checked, state| state.set(checked), is_mod_state.clone());

    let form_submit = {
        let username_state = username_state.clone();
        let password_state = password_state.clone();
        let job_state = job_state.clone();
        let gear_level_state = gear_level_state.clone();
        let discord_name_state = discord_name_state.clone();
        let error_message_state = error_message_state.clone();

        let is_mod_state = is_mod_state.clone();
        let error_state = error_state.clone();
        let created_state = created_state.clone();

        let created_user = created_user_state.clone();

        Callback::from(move |_| {
            log::debug!("Submit executed user is about to be created");
            let username_state = username_state.clone();
            let job_state = job_state.clone();
            let gear_level_state = gear_level_state.clone();
            let password_state = password_state.clone();
            let error_message_state = error_message_state.clone();

            let is_mod_state = is_mod_state.clone();
            let error_state = error_state.clone();
            let created_state = created_state.clone();

            let created_user = created_user.clone();

            let user = User::new(
                (*username_state).to_string(),
                (*password_state).to_string(),
                (*job_state).to_string(),
                (*gear_level_state).to_string(),
                (*discord_name_state).to_string(),
                *is_mod_state,
            );

            yew::platform::spawn_local(async move {
                log::debug!("Create new user with username {}", user.username.clone());
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
                            error_message_state.set("Ein Benutzer mit diesem Namen existiert bereits".into());
                        } else {
                            error_message_state.set("Der Benutzer konnte nicht hinzugefügt werden, bitte wende dich an Azami".into());
                        }
                        password_state.set(rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(8)
                            .map(char::from)
                            .collect::<String>()
                            .into());
                    }
                }
            });
        })
    };

    let on_saved = use_callback(|_, (created_user_state, on_saved)| on_saved.emit((**created_user_state).clone()), (created_user_state, props.on_saved.clone()));

    html!(
        <CosmoModal title="Benutzer hinzufügen" is_form={true} on_form_submit={form_submit} buttons={
            html!(
                if *created_state {
                    <CosmoButton on_click={on_saved} label="Alles klar" />
                } else {
                    <>
                        <CosmoButton on_click={props.on_close.clone()} label="Abbrechen" />
                        <CosmoButton is_submit={true} label="Benutzer erstellen" />
                    </>
                }
            )}>
            if *created_state {
                <CosmoParagraph>{format!("Das Passwort für {} ist ", (*username_state).clone())}<CosmoCode>{(*password_state).clone()}</CosmoCode></CosmoParagraph>
            } else {
                <>
                    if *error_state {
                        <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim hinzufügen" message={(*error_message_state).clone()} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Information} header="Füge einen neuen Benutzer hinzu" message="Das Passwort wird angezeigt wenn der Benutzer erfolgreich hinzugefügt wurde" />
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Name" value={(*username_state).clone()} on_input={update_username} required={true} />
                        <CosmoTextBox label="Job (optional)" value={(*job_state).clone()} on_input={update_job} />
                        <CosmoTextBox label="Gearlevel (optional)" value={(*gear_level_state).clone()} on_input={update_gear_level} />
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
    let job_state = use_state_eq(|| props.job.clone());
    let gear_level_state = use_state_eq(|| props.gear_level.clone());
    let discord_name_state = use_state_eq(|| props.discord_name.clone());

    let update_job = use_callback(|value, state| state.set(value), job_state.clone());
    let update_gear_level = use_callback(|value, state| state.set(value), gear_level_state.clone());
    let update_discord_name = use_callback(|value, state| state.set(value), discord_name_state.clone());

    let on_save = {
        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let job_state = job_state.clone();
        let gear_level_state = gear_level_state.clone();
        let discord_name_state = discord_name_state.clone();

        let id = props.id;

        let on_close = props.on_close.clone();

        Callback::from(move |_| {
            log::debug!("Perform password change");
            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let job_state = job_state.clone();
            let gear_level_state = gear_level_state.clone();
            let discord_name_state = discord_name_state.clone();

            let on_close = on_close.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match update_profile(id, UpdateProfile::new((*gear_level_state).to_string(), (*job_state).to_string(), (*discord_name_state).to_string())).await {
                    Ok(_) => {
                        log::debug!("Profile update successful");
                        on_close.emit(());

                        false
                    }
                    Err(err) => match err.code {
                        FORBIDDEN => {
                            error_message_state.set("Du musst Mod sein um fremde Profile zu bearbeiten".into());
                            true
                        }
                        NOT_FOUND => {
                            log::warn!("The user was not found");
                            error_message_state.set("Der Benutzer wurde nicht gefunden".into());

                            true
                        }
                        _ => {
                            log::warn!("Failed to change the profile {err}");
                            error_message_state.set("Das Profil konnte leider nicht geändert werden, bitte wende dich an Azami".into());

                            true
                        }
                    }
                });
            });
        })
    };
    let close_error = use_callback(|_, state| state.set(false), error_state.clone());

    html!(
        <>
            <CosmoModal title="Profil bearbeiten" is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={props.on_close.clone()} label="Abbrechen" />
                    <CosmoButton is_submit={true} label="Profil speichern" />
                </>
            )}>
                <CosmoInputGroup>
                    <CosmoTextBox label="Rolle/Klasse (optional)" on_input={update_job} value={(*job_state).clone()} />
                    <CosmoTextBox label="Gear Level (optional)" on_input={update_gear_level} value={(*gear_level_state).clone()} />
                    <CosmoTextBox label="Discord Name (optional)" on_input={update_discord_name} value={(*discord_name_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
            if *error_state {
                <CosmoAlert title="Fehler" message={(*error_message_state).clone()} close_label="Schließen" on_close={close_error} />
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

    let make_mod_click = use_callback(|_, state| state.set(UserConfirmActions::MakeMod), confirm_state.clone());
    let remove_mod_click = use_callback(|_, state| state.set(UserConfirmActions::RemoveMod), confirm_state.clone());
    let delete_click = use_callback(|_, state| state.set(UserConfirmActions::Delete), confirm_state.clone());
    let update_profile_click = use_callback(|_, profile_edit_state| profile_edit_state.set(true), profile_edit_state.clone());
    let change_password_click = use_callback(|_, state| state.set(UserConfirmActions::ChangePassword(
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>())), confirm_state.clone());
    let on_decline = use_callback(|_, state| state.set(UserConfirmActions::Closed), confirm_state.clone());
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
                    UserConfirmActions::MakeMod => {
                        match make_user_mod(id).await {
                            Ok(_) => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            Err(err) => match err.code {
                                FORBIDDEN => {
                                    error_message_state.set(AttrValue::from("Du musst Mod sein um Mods zu ernennen"));
                                    FORBIDDEN
                                }
                                CONFLICT => {
                                    error_message_state.set(AttrValue::from("Du kannst dich nicht selbst zum Mod machen"));
                                    CONFLICT
                                }
                                _ => {
                                    error_message_state.set(AttrValue::from("Der Benutzer konnte nicht zum Mod gemacht werden, bitte wende dich an Azami"));
                                    INTERNAL_SERVER_ERROR
                                }
                            }
                        }
                    }
                    UserConfirmActions::RemoveMod => {
                        match remove_user_mod(id).await {
                            Ok(_) => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            Err(err) => match err.code {
                                FORBIDDEN => {
                                    error_message_state.set(AttrValue::from("Du musst Mod sein um Benutzern die Modrechte zu entziehen"));
                                    FORBIDDEN
                                }
                                CONFLICT => {
                                    error_message_state.set(AttrValue::from("Du kannst dir die Modrechte nicht entziehen"));
                                    CONFLICT
                                }
                                _ => {
                                    error_message_state.set(AttrValue::from("Dem Benutzer konnten die Modrechte nicht entzogen werden, bitte wende dich an Azami"));
                                    INTERNAL_SERVER_ERROR
                                }
                            }
                        }
                    }
                    UserConfirmActions::Delete => {
                        match delete_user(id).await {
                            Ok(_) => {
                                confirm_state.set(UserConfirmActions::Closed);
                                on_delete.emit(());
                                NO_CONTENT
                            }
                            Err(err) => match err.code {
                                FORBIDDEN => {
                                    error_message_state.set(AttrValue::from("Du musst Mod sein um Benutzern zu entfernen"));
                                    FORBIDDEN
                                }
                                CONFLICT => {
                                    error_message_state.set(AttrValue::from("Du kannst dich nicht selbst löschen, wenn du gehen möchtest, wende dich an einen Mod"));
                                    CONFLICT
                                }
                                _ => {
                                    error_message_state.set(AttrValue::from("Das Benutzer konnte nicht gelöscht werden, bitte wende dich an Azami"));
                                    INTERNAL_SERVER_ERROR
                                }
                            }
                        }
                    }
                    UserConfirmActions::ChangePassword(new_password) => {
                        match change_user_password(id, new_password.clone()).await {
                            Ok(_) => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            Err(err) => match err.code {
                                FORBIDDEN => {
                                    error_message_state.set("Du musst Mod sein um Passwörter zurückzusetzen".into());
                                    FORBIDDEN
                                }
                                CONFLICT => {
                                    error_message_state.set("Wenn du dein passwort ändern willst, mach das bitte über Mein Sheef".into());
                                    CONFLICT
                                }
                                _ => {
                                    error_message_state.set("Das Passwort konnte nicht zurückgesetzt werden, bitte wende dich an Azami".into());
                                    INTERNAL_SERVER_ERROR
                                }
                            }
                        }
                    }
                    UserConfirmActions::Closed => unreachable!()
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
    let on_alert_close = use_callback(|_, (error_state, error_message_state)| {
        error_state.set(false);
        error_message_state.set("".into());
    }, (error_state.clone(), error_message_state.clone()));
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
            <CosmoTitle title={props.user.username.clone()} />
            if current_user.profile.is_mod {
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        if props.user.is_mod {
                            <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={remove_mod_click} label="Modrechte entziehen" />
                        } else {
                            <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={make_mod_click} label="Zum Mod machen" />
                        }
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={update_profile_click} label="Profil bearbeiten" />
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={change_password_click} label="Passwort ändern" />
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoButton enabled={props.user.id != current_user.profile.id} on_click={delete_click} label="Löschen" />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
            }
            <CosmoKeyValueList>
                <CosmoKeyValueListItem title="Job">
                    if props.user.job.is_empty() {
                        {"Kein Job gesetzt"}
                    } else {
                        {props.user.job.clone()}
                    }
                </CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Gear Level">
                    if props.user.gear_level.is_empty() {
                        {"Kein Gear Level gesetzt"}
                    } else {
                        {props.user.gear_level.clone()}
                    }
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
                    <CosmoConfirm message={format!("Soll der Benutzer {} zum Mod gemacht werden?", props.user.username.clone())} title="Zum Mod machen" on_decline={on_decline} on_confirm={on_confirm} decline_label="Abbrechen" confirm_label="Zum Mod machen" />
                ),
                UserConfirmActions::RemoveMod => html!(
                    <CosmoConfirm message={format!("Sollen dem Benutzer {} wirklich die Modrechte entzogen werden?", props.user.username.clone())} title="Modrechte entziehen" on_decline={on_decline} on_confirm={on_confirm} confirm_label="Modrechte entziehen" decline_label="Abbrechen" />
                ),
                UserConfirmActions::Delete => html!(
                    <CosmoConfirm message={format!("Soll der Benutzer {} wirklich gelöscht werden?", props.user.username.clone())} title="Benutzer löschen" on_decline={on_decline} on_confirm={on_confirm} confirm_label="Benutzer löschen" decline_label="Benutzer behalten" />
                ),
                UserConfirmActions::ChangePassword(password) => {
                    html!(
                        <CosmoModal title="Passwort zurücksetzen" buttons={html!(
                            <>
                                <CosmoButton on_click={on_decline} label="Abbrechen" />
                                <CosmoButton on_click={on_confirm} label="Passwort zurücksetzen" />
                            </>
                        )}>
                            <CosmoParagraph>{format!("Das neue Passwort für {} wird auf ", props.user.username)}<CosmoCode>{password}</CosmoCode>{" gesetzt."}</CosmoParagraph>
                        </CosmoModal>
                    )
                },
                UserConfirmActions::Closed => html!(),
            }}
            if *error_state {
                <CosmoAlert title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={on_alert_close} close_label="Schließen" />
            }
            if *profile_edit_state {
                <UpdateProfileDialog on_close={on_update_profile_close} id={props.user.id} gear_level={props.user.gear_level.clone()} job={props.user.job.clone()} discord_name={props.user.discord_name.clone()} />
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

    let open_create_user_modal_click = use_callback(|_, open_create_user_modal_state| open_create_user_modal_state.set(true), open_create_user_modal_state.clone());
    let on_user_select = use_callback(|idx, state| state.set(idx), selected_user_state.clone());

    let event_source_trigger = {
        let users_query_state = users_query_state.clone();

        move |_| {
            log::debug!("Someone changed data on the server, trigger a refresh");
            let users_query_state = users_query_state.clone();

            yew::platform::spawn_local(async move {
                let _ = users_query_state.refresh().await;
            });
        }
    };

    use_event_source("/sse/user".to_string(), event_source_trigger);

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

            let username = user.username.clone();

            yew::platform::spawn_local(async move {
                open_create_user_modal_state.set(false);
                if let Ok(res) = users_query_state.refresh().await {
                    selected_user_state.set(res.users.iter().position(move |user| user.username.eq(&username)).unwrap_or(0));
                }
            })
        })
    };
    let on_create_close = use_callback(|_, state| state.set(false), open_create_user_modal_state.clone());

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
            log::warn!("Failed to load {}", err);
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Benutzer konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Party People"}</title>
            </Helmet>
            <CosmoSideList on_select_item={on_user_select} selected_index={*selected_user_state} has_add_button={current_user.profile.is_mod} add_button_on_click={open_create_user_modal_click} add_button_label="Benutzer hinzufügen">
                {for (*users_state).clone().into_iter().map(|user| {
                    CosmoSideListItem::from_label_and_children(user.username.clone().into(), html!(
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

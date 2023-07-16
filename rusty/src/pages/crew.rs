use std::ops::Deref;
use rand::Rng;
use rand::distributions::Alphanumeric;
use bounce::query::use_query_value;
use bounce::helmet::Helmet;
use bounce::use_atom_value;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use sheef_entities::user::WebUser;
use crate::api::{CONFLICT, NO_CONTENT, INTERNAL_SERVER_ERROR, FORBIDDEN};
use crate::api::user::{change_user_password, create_user, Crew, delete_user, make_user_main, make_user_mod, remove_user_main, remove_user_mod};
use crate::storage::CurrentUser;
use crate::ui::modal::{PicoModal, PicoConfirm, PicoAlert};

#[derive(Properties, PartialEq, Clone, Eq)]
struct TableBodyProps {
    users: Vec<sheef_entities::User>,
    is_mod: bool,
    username: AttrValue,
}

#[derive(Properties, PartialEq, Clone)]
struct CreateUserModalProps {
    on_close: Callback<()>,
}

#[function_component(CreateUserModal)]
fn create_user_modal(props: &CreateUserModalProps) -> Html {
    log::debug!("Create create user modal");
    log::debug!("Initialize state and callbacks");
    let username_state = use_state_eq(|| AttrValue::from(""));
    let job_state = use_state_eq(|| AttrValue::from(""));
    let gear_level_state = use_state_eq(|| AttrValue::from(""));
    let password_state = use_state_eq(|| AttrValue::from(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>()));
    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let is_mod_state = use_state_eq(|| false);
    let is_main_group_state = use_state_eq(|| false);
    let is_hidden_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);
    let created_state = use_state_eq(|| false);

    let update_username = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), username_state.clone());
    let update_job = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), job_state.clone());
    let update_gear_level = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), gear_level_state.clone());

    let update_is_mod = use_callback(|evt: MouseEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().checked()), is_mod_state.clone());
    let update_is_main_group = use_callback(|evt: MouseEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().checked()), is_main_group_state.clone());
    let update_is_hidden = use_callback(|evt: MouseEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().checked()), is_hidden_state.clone());

    let on_close = props.on_close.clone();

    let crew_query_state = use_query_value::<Crew>(().into());

    let form_submit = {
        let username_state = username_state.clone();
        let password_state = password_state.clone();
        let job_state = job_state.clone();
        let gear_level_state = gear_level_state.clone();
        let error_message_state = error_message_state.clone();

        let is_mod_state = is_mod_state.clone();
        let is_main_group_state = is_main_group_state.clone();
        let is_hidden_state = is_hidden_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        let created_state = created_state.clone();

        Callback::from(move |evt: SubmitEvent| {
            log::debug!("Submit executed user is about to be created");
            evt.prevent_default();
            loading_state.set(true);
            let username_state = username_state.clone();
            let job_state = job_state.clone();
            let gear_level_state = gear_level_state.clone();
            let password_state = password_state.clone();
            let error_message_state = error_message_state.clone();

            let is_mod_state = is_mod_state.clone();
            let is_main_group_state = is_main_group_state.clone();
            let is_hidden_state = is_hidden_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            let created_state = created_state.clone();

            let crew_query_state = crew_query_state.clone();

            let user = sheef_entities::user::User {
                username: (*username_state).to_string(),
                password: (*password_state).to_string(),
                job: (*job_state).to_string(),
                gear_level: (*gear_level_state).to_string(),
                is_mod: *is_mod_state,
                is_main_group: *is_main_group_state,
                is_hidden: *is_hidden_state,
            };

            yew::platform::spawn_local(async move {
                log::debug!("Create new user with username {}", user.username.clone());
                match create_user(user).await {
                    Ok(_) => {
                        log::debug!("User was created successfully, lets reload the crew");
                        error_message_state.set(AttrValue::from(""));
                        error_state.set(false);
                        let _ = crew_query_state.refresh().await;
                        error_state.set(false);
                        created_state.set(true);
                    }
                    Err(err) => {
                        log::warn!("Failed to create user {}", err);
                        error_state.set(true);
                        if err == CONFLICT {
                            error_message_state.set(AttrValue::from("Ein Mitglied mit diesem Namen existiert bereits"));
                        } else {
                            error_message_state.set(AttrValue::from("Das Mitglied konnte nicht hinzugefügt werden, bitte wende dich an Azami"));
                        }
                        password_state.set(AttrValue::from(rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(8)
                            .map(char::from)
                            .collect::<String>()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    html!(
        <PicoModal on_close={props.on_close.clone()} title="Mitglied hinzufügen" open={true} buttons={
            if *created_state {
                html!(<button onclick={move |_| on_close.clone().emit(())} type="button">{"Alles klar"}</button>)
            } else {
                html!(
                    <>
                        <button onclick={move |_| on_close.clone().emit(())} type="button" class="secondary">{"Abbrechen"}</button>
                        <button aria-busy={AttrValue::from((*loading_state).to_string())} form="create-user-modal-form" type="submit">{"Benutzer erstellen"}</button>
                    </>
                )
            }
        }>
            {if *created_state {
                html!(
                    <p>{format!("Das Passwort für {} ist ", (*username_state).clone())}<kbd>{(*password_state).clone()}</kbd></p>
                )
            } else {
                html!(
                    <>
                        {if *error_state {
                            html!(<p data-msg="negative">{(*error_message_state).clone()}</p>)
                        } else {
                            html!(
                                <p data-msg="info">{"Füge ein neues Mitglied hinzu"}<br />{"Das Passwort wird angezeigt wenn das Mitglied erfolgreich hinzugefügt wurde"}</p>
                            )
                        }}
                        <form id="create-user-modal-form" onsubmit={form_submit}>
                            <label for="username">{"Name"}</label>
                            <input readonly={*loading_state} value={(*username_state).clone()} oninput={update_username} type="text" required={true} id="username" name="username" />
                            <label for="job">{"Job (optional)"}</label>
                            <input readonly={*loading_state} value={(*job_state).clone()} oninput={update_job} type="text" id="job" name="job" />
                            <label for="gearlevel">{"Gearlevel (optional)"}</label>
                            <input readonly={*loading_state} value={(*gear_level_state).clone()} oninput={update_gear_level} type="text" id="gearlevel" name="gearlevel" />
                            <fieldset>
                                <label for="isMod">
                                    <input readonly={*loading_state} type="checkbox" id="isMod" name="isMod" role="switch" checked={*is_mod_state} onclick={update_is_mod} />
                                    {"Moderator"}
                                </label>
                                <label for="isMainGroup">
                                    <input readonly={*loading_state} type="checkbox" id="isMainGroup" name="isMainGroup" role="switch" checked={*is_main_group_state} onclick={update_is_main_group} />
                                    {"Mainkader"}
                                </label>
                                <label for="isHidden">
                                    <input readonly={*loading_state} type="checkbox" id="isHidden" name="isHidden" role="switch" checked={*is_hidden_state} onclick={update_is_hidden} />
                                    {"Versteckt"}
                                </label>
                            </fieldset>
                        </form>
                    </>
                )
            }}
        </PicoModal>
    )
}

#[derive(PartialEq, Clone)]
enum UserConfirmActions {
    MakeMod(sheef_entities::User),
    RemoveMod(sheef_entities::User),
    Delete(sheef_entities::User),
    MakeMain(sheef_entities::User),
    RemoveMain(sheef_entities::User),
    ChangePassword(sheef_entities::User, String),
    Closed,
}

#[function_component(TableBody)]
fn table_body(props: &TableBodyProps) -> Html {
    log::debug!("Initialize crew table body state and callbacks");
    let confirm_state = use_state_eq(|| UserConfirmActions::Closed);

    let error_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let users_query_state = use_query_value::<Crew>(().into());

    let make_mod_click = use_callback(|user: WebUser, state| state.set(UserConfirmActions::MakeMod(user)), confirm_state.clone());
    let remove_mod_click = use_callback(|user: WebUser, state| state.set(UserConfirmActions::RemoveMod(user)), confirm_state.clone());
    let delete_click = use_callback(|user: WebUser, state| state.set(UserConfirmActions::Delete(user)), confirm_state.clone());
    let make_main_click = use_callback(|user: WebUser, state| state.set(UserConfirmActions::MakeMain(user)), confirm_state.clone());
    let remove_main_click = use_callback(|user: WebUser, state| state.set(UserConfirmActions::RemoveMain(user)), confirm_state.clone());
    let change_password_click = use_callback(|user: WebUser, state| state.set(UserConfirmActions::ChangePassword(user, rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>())), confirm_state.clone());
    let on_decline = use_callback(|_, state| state.set(UserConfirmActions::Closed), confirm_state.clone());
    let on_confirm = {
        let confirm_state = confirm_state.clone();
        let error_state = error_state.clone();
        let error_message_state = error_message_state.clone();

        Callback::from(move |_| {
            log::debug!("Modal was confirmed lets execute the request");
            let confirm_state = confirm_state.clone();
            let error_state = error_state.clone();
            let error_message_state = error_message_state.clone();
            let users_query_state = users_query_state.clone();
            yew::platform::spawn_local(async move {
                let code = match confirm_state.deref() {
                    UserConfirmActions::MakeMod(user) => {
                        match make_user_mod(user.clone()).await {
                            NO_CONTENT => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from("Du musst Mod sein um Mods zu ernennen"));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from("Du kannst dich nicht selbst zum Mod machen"));
                                CONFLICT
                            }
                            INTERNAL_SERVER_ERROR => {
                                error_message_state.set(AttrValue::from("Das Mitglied konnte nicht zum Mod gemacht werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                            _ => unreachable!()
                        }
                    }
                    UserConfirmActions::RemoveMod(user) => {
                        match remove_user_mod(user.clone()).await {
                            NO_CONTENT => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from("Du musst Mod sein um Mitgliedern die Modrechte zu entziehen"));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from("Du kannst dir die Modrechte nicht entziehen"));
                                CONFLICT
                            }
                            INTERNAL_SERVER_ERROR => {
                                error_message_state.set(AttrValue::from("Dem Mitglied konnten die Modrechte nicht entzogen werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                            _ => unreachable!()
                        }
                    }
                    UserConfirmActions::Delete(user) => {
                        match delete_user(user.clone()).await {
                            NO_CONTENT => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from("Du musst Mod sein um Mitgliedern zu entfernen"));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from("Du kannst dich nicht selbst löschen, wenn du gehen möchtest, wende dich an einen Mod"));
                                CONFLICT
                            }
                            INTERNAL_SERVER_ERROR => {
                                error_message_state.set(AttrValue::from("Das Mitglied konnte nicht gelöscht werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                            _ => unreachable!()
                        }
                    }
                    UserConfirmActions::MakeMain(user) => {
                        match make_user_main(user.clone()).await {
                            NO_CONTENT => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from("Du musst Mod sein um Mitgliedern in den Mainkader hinzuzufügen"));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from("Du kannst dich nicht selbst in den Mainkader hinzufügen"));
                                CONFLICT
                            }
                            INTERNAL_SERVER_ERROR => {
                                error_message_state.set(AttrValue::from("Das Mitglied konnte nicht in den Mainkader hinzugefügt werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                            _ => unreachable!()
                        }
                    }
                    UserConfirmActions::RemoveMain(user) => {
                        match remove_user_main(user.clone()).await {
                            NO_CONTENT => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from("Du musst Mod sein um Mitgliedern aus dem Mainkader zu entfernen"));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from("Du kannst dich nicht selbst aus dem Mainkader entfernen"));
                                CONFLICT
                            }
                            INTERNAL_SERVER_ERROR => {
                                error_message_state.set(AttrValue::from("Das Mitglied konnte nicht aus dem Mainkader entfernt werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                            _ => unreachable!()
                        }
                    }
                    UserConfirmActions::ChangePassword(user, new_password) => {
                        match change_user_password(user.clone(), new_password.clone()).await {
                            NO_CONTENT => {
                                confirm_state.set(UserConfirmActions::Closed);
                                NO_CONTENT
                            }
                            FORBIDDEN => {
                                error_message_state.set(AttrValue::from("Du musst Mod sein um Passwörter zurückzusetzen"));
                                FORBIDDEN
                            }
                            CONFLICT => {
                                error_message_state.set(AttrValue::from("Wenn du dein passwort ändern willst, mach das bitte über Mein Sheef"));
                                CONFLICT
                            }
                            INTERNAL_SERVER_ERROR => {
                                error_message_state.set(AttrValue::from("Das Passwort konnte nicht zurückgesetzt werden, bitte wende dich an Azami"));
                                INTERNAL_SERVER_ERROR
                            }
                            _ => unreachable!()
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
        error_message_state.set(AttrValue::from(""));
    }, (error_state.clone(), error_message_state.clone()));

    html!(
        <>
            <tbody>
                {for props.users.iter().map(|user| html!(
                    <tr>
                        <td>{user.username.clone()}</td>
                        <td>{user.job.clone()}</td>
                        <td>{user.gear_level.clone()}</td>
                        <td>{if user.is_main_group { "Ja" } else { "Nein" }}</td>
                        <td>{if user.is_mod { "Ja" } else { "Nein" }}</td>
                        {if props.is_mod {
                            html!(
                                <td>
                                    {if props.username != user.username {
                                        let delete_click = delete_click.clone();
                                        let change_password_click = change_password_click.clone();

                                        let delete_user = user.clone();
                                        let password_user = user.clone();
                                        html!(
                                            <div class="gap-row">
                                                {if user.is_mod {
                                                    let remove_mod_click = remove_mod_click.clone();
                                                    let user = user.clone();
                                                    html!(
                                                        <button onclick={move |_| remove_mod_click.emit(user.clone())} type="button" class="outline">{"Modrechte entziehen"}</button>
                                                    )
                                                } else {
                                                    let make_mod_click = make_mod_click.clone();
                                                    let user = user.clone();
                                                    html!(
                                                        <button onclick={move |_| make_mod_click.emit(user.clone())} type="button" class="outline">{"Zum Mod machen"}</button>
                                                    )
                                                }}
                                                {if user.is_main_group {
                                                    let remove_main_click = remove_main_click.clone();
                                                    let user = user.clone();
                                                    html!(
                                                        <button onclick={move |_| remove_main_click.emit(user.clone())} type="button" class="outline">{"Aus Mainkader entfernen"}</button>
                                                    )
                                                } else {
                                                    let make_main_click = make_main_click.clone();
                                                    let user = user.clone();
                                                    html!(
                                                        <button onclick={move |_| make_main_click.emit(user.clone())} type="button" class="outline">{"Zum Mainkader hinzufügen"}</button>
                                                    )
                                                }}
                                                <button onclick={move |_| change_password_click.emit(password_user.clone())} type="button" class="outline">{"Passwort ändern"}</button>
                                                <button onclick={move |_| delete_click.emit(delete_user.clone())} type="button" class="outline">{"Entfernen"}</button>
                                            </div>
                                        )
                                    } else {
                                        html!()
                                    }}
                                </td>
                            )
                        } else {
                            html!()
                        }}
                    </tr>
                ))}
            </tbody>
            {match (*confirm_state).clone() {
                UserConfirmActions::MakeMod(user) => html!(
                    <PicoConfirm message={format!("Soll das Mitglied {} zum Mod gemacht werden?", user.username)} title="Zum Mod machen" open={true} on_decline={on_decline} on_confirm={on_confirm} confirm_label="Zum Mod machen" />
                ),
                UserConfirmActions::RemoveMod(user) => html!(
                    <PicoConfirm message={format!("Sollen dem Mitglied {} wirklich die Modrechte entzogen werden?", user.username)} title="Modrechte entziehen" open={true} on_decline={on_decline} on_confirm={on_confirm} confirm_label="Modrechte entziehen" />
                ),
                UserConfirmActions::Delete(user) => html!(
                    <PicoConfirm message={format!("Soll das Mitglied {} wirklich entfernt werden?", user.username)} title="Mitglied entfernen" open={true} on_decline={on_decline} on_confirm={on_confirm} confirm_label="Mitglied entfernen" />
                ),
                UserConfirmActions::MakeMain(user) => html!(
                    <PicoConfirm message={format!("Soll das Mitglied {} zum Mainkader hinzugefügt werden?", user.username)} title="Zum Mainkader hinzufügen" open={true} on_decline={on_decline} on_confirm={on_confirm} confirm_label="Zum Mainkader hinzufügen" />
                ),
                UserConfirmActions::RemoveMain(user) => html!(
                    <PicoConfirm message={format!("Soll das Mitglied {} aus dem Mainkader entfernt werden?", user.username)} title="Aus Mainkader entfernen" open={true} on_decline={on_decline} on_confirm={on_confirm} confirm_label="Aus Mainkader entfernen" />
                ),
                UserConfirmActions::ChangePassword(user, password) => html!(
                    <PicoModal open={true} on_close={on_decline.clone()} title="Passwort zurücksetzen" buttons={html!(
                        <>
                            <button type="button" class="secondary" onclick={move |_| on_decline.emit(())}>{"Abbrechen"}</button>
                            <button type="button" onclick={move |_| on_confirm.emit(())}>{"Passwort zurücksetzen"}</button>
                        </>
                    )}>
                        <p>{format!("Das neue Passwort für {} wird auf ", user.username)}<kbd>{password}</kbd>{" gesetzt."}</p>
                    </PicoModal>
                ),
                UserConfirmActions::Closed => html!(),
            }}
            {if *error_state {
                html!(<PicoAlert open={true} title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={on_alert_close} />)
            } else {
                html!()
            }}
        </>
    )
}

#[function_component(CrewPage)]
pub fn crew_page() -> Html {
    log::debug!("Render crew page");
    log::debug!("Initialize state and callbacks");
    let current_user = use_atom_value::<CurrentUser>();
    let users_query_state = use_query_value::<Crew>(().into());

    let initially_loaded_state = use_state_eq(|| false);
    let open_create_user_modal_state = use_state_eq(|| false);

    let state = use_state_eq(|| vec![] as Vec<sheef_entities::User>);

    let open_create_user_modal_click = use_callback(|_, open_create_user_modal_state| open_create_user_modal_state.set(true), open_create_user_modal_state.clone());

    match users_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initially_loaded_state {
                return html!(<p data-msg="info">{"Die Crew wird geladen"}</p>);
            }
        }
        Some(Ok(users)) => {
            log::debug!("Loaded users");
            initially_loaded_state.set(true);
            state.set(users.users.clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {}", err);
            return html!(<p data-msg="negative">{"Die Crew konnte nicht geladen werden, bitte wende dich an Azami"}</p>);
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Static „Sheef”"}</title>
            </Helmet>
            <h1>{"Static „Sheef”"}</h1>
            {if current_user.profile.is_mod {
                html!(
                    <nav>
                        <ul>
                            <li>
                                <button onclick={open_create_user_modal_click} type="button">{"Mitglied hinzufügen"}</button>
                                {if *open_create_user_modal_state {
                                    html!(
                                        <CreateUserModal on_close={move |_| open_create_user_modal_state.clone().set(false)} />
                                    )
                                } else {
                                    html!()
                                }}
                            </li>
                        </ul>
                    </nav>
                )
            } else {
                html!()
            }}
            <table role="grid">
                <thead>
                <tr>
                    <th>{"Name"}</th>
                    <th>{"Job"}</th>
                    <th>{"Gearlevel"}</th>
                    <th>{"Mainkader"}</th>
                    <th>{"Moderator"}</th>
                    {if current_user.profile.is_mod {
                        html!(
                            <th>{"Aktionen"}</th>
                        )
                    } else {
                        html!()
                    }}
                </tr>
                </thead>
                <TableBody username={current_user.profile.username.clone()} users={(*state).clone()} is_mod={current_user.profile.is_mod} />
            </table>
        </>
    )
}

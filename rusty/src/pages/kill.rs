use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use bounce::use_atom_value;
use yew::prelude::*;
use crate::api::kill::{activate_kill, create_kill, deactivate_kill, delete_kill, Kills, rename_kill};
use crate::api::{NOT_FOUND, NO_CONTENT, FORBIDDEN};
use crate::api::my::{activate_kill_for_me, deactivate_kill_for_me};
use crate::pages::boolean_table::{ActivationParams, BooleanTable, EntryModalState, ModifyEntryModalSaveData};
use crate::storage::CurrentUser;
use crate::ui::modal::PicoAlert;

#[function_component(KillPage)]
pub fn kill_page() -> Html {
    log::debug!("Render kills page");
    log::debug!("Initialize state and callbacks");
    let kill_query_state = use_query_value::<Kills>(().into());

    let current_user = use_atom_value::<CurrentUser>();
    let is_mod = current_user.profile.is_mod;

    let initially_loaded_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);
    let delete_entry_open = use_state_eq(|| false);

    let modify_modal_state = use_state_eq(|| EntryModalState::Closed);

    let state = use_state_eq(Kills::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_title_state = use_state_eq(|| AttrValue::from(""));
    let delete_entry_name_state = use_state_eq(|| AttrValue::from(""));
    let delete_entry_message_state = use_state_eq(|| AttrValue::from(""));

    let activate_kill = {
        let kill_query_state = kill_query_state.clone();

        let current_user = current_user.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        Callback::from(move |params: ActivationParams| {
            log::debug!("Activate kill {} for {}", params.key, params.user);
            let kill_query_state = kill_query_state.clone();

            let current_user = current_user.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                let params = params.clone();
                let result = if !current_user.profile.is_mod && current_user.profile.username == params.user.clone() {
                    activate_kill_for_me(params.key.clone()).await
                } else {
                    activate_kill(params.user.clone(), params.key.clone()).await
                };

                log::debug!("Execute request");
                error_state.set(match result {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        error_title_state.set(AttrValue::from(""));
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("User or kill not found");
                        error_message_state.set(AttrValue::from(if current_user.profile.is_mod { "Entweder das Kill oder das Crewmitglied konnte nicht gefunden werden" } else { "Der Kill konnte nicht gefunden werden" }));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Kills anderer Crewmitglieder zu aktivieren"));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let params = params.clone();

                        error_message_state.set(AttrValue::from(format!("Der Kill {} konnte für {} nicht aktiviert werden, bitte wende dich an Azami", params.key, params.user)));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                });
                let _ = kill_query_state.refresh().await;
            });
        })
    };
    let deactivate_kill = {
        let kill_query_state = kill_query_state.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        Callback::from(move |params: ActivationParams| {
            log::debug!("Deactivate kill {} for {}", params.key, params.user);
            let kill_query_state = kill_query_state.clone();

            let current_user = current_user.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                let params = params.clone();
                let result = if !current_user.profile.is_mod && current_user.profile.username == params.user.clone() {
                    deactivate_kill_for_me(params.key.clone()).await
                } else {
                    deactivate_kill(params.user.clone(), params.key.clone()).await
                };

                log::debug!("Execute request");
                error_state.set(match result {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("User or kill not found");
                        error_message_state.set(AttrValue::from(if current_user.profile.is_mod { "Entweder das Kill oder das Crewmitglied konnte nicht gefunden werden" } else { "Der Kill konnte nicht gefunden werden" }));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Kills anderer Crewmitglieder zu deaktivieren"));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let params = params.clone();

                        error_message_state.set(AttrValue::from(format!("Der Kill {} konnte für {} nicht deaktiviert werden, bitte wende dich an Azami", params.key, params.user)));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                });
                let _ = kill_query_state.refresh().await;
            });
        })
    };

    let on_add_save = {
        let kill_query_state = kill_query_state.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        let modify_modal_state = modify_modal_state.clone();

        Callback::from(move |data: ModifyEntryModalSaveData| {
            log::debug!("Add kill {}", data.new_name);
            loading_state.set(true);

            let kill_query_state = kill_query_state.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            let modify_modal_state = modify_modal_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match create_kill(data.new_name.to_string()).await {
                    Ok(_) => {
                        error_message_state.set(AttrValue::from(""));
                        modify_modal_state.set(EntryModalState::Closed);
                        false
                    }
                    Err(FORBIDDEN) => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Kills hinzuzufügen"));
                        error_title_state.set(AttrValue::from("Fehler beim Hinzufügen"));
                        true
                    }
                    Err(err) => {
                        log::warn!("Another error occurred {}", err);
                        error_message_state.set(AttrValue::from(format!("Der Kill {} nicht erstellt werden, bitte wende dich an Azami", data.new_name)));
                        error_title_state.set(AttrValue::from("Fehler beim Hinzufügen"));
                        true
                    }
                });
                loading_state.set(false);
                let _ = kill_query_state.refresh().await;
            })
        })
    };
    let on_edit_save = {
        let kill_query_state = kill_query_state.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        let modify_modal_state = modify_modal_state.clone();

        Callback::from(move |data: ModifyEntryModalSaveData| {
            loading_state.set(true);

            let kill_query_state = kill_query_state.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let modify_modal_state = modify_modal_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match rename_kill(data.old_name.to_string(), data.new_name.to_string()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        modify_modal_state.set(EntryModalState::Closed);
                        false
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Kills umzubenennen"));
                        error_title_state.set(AttrValue::from("Fehler beim Umbenennen"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        error_message_state.set(AttrValue::from(format!("Der Kill {} nicht umbenannt werden, bitte wende dich an Azami", data.old_name)));
                        error_title_state.set(AttrValue::from("Fehler beim Umbenennen"));
                        true
                    }
                });
                loading_state.set(false);
                let _ = kill_query_state.refresh().await;
            })
        })
    };

    let on_delete_decline = use_callback(|_, (name_state, message_state, open_state)| {
        name_state.set(AttrValue::from(""));
        message_state.set(AttrValue::from(""));
        open_state.set(false);
    }, (delete_entry_name_state.clone(), delete_entry_message_state.clone(), delete_entry_open.clone()));
    let on_delete_confirm = {
        let kill_query_state = kill_query_state.clone();

        let error_state = error_state.clone();
        let delete_entry_open = delete_entry_open.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();
        let delete_entry_name_state = delete_entry_name_state.clone();

        Callback::from(move |_| {
            log::debug!("Delete kill {}", (*delete_entry_name_state).clone());
            let kill_query_state = kill_query_state.clone();

            let error_state = error_state.clone();
            let delete_entry_open = delete_entry_open.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();
            let delete_entry_name_state = delete_entry_name_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match delete_kill((*delete_entry_name_state).to_string()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        delete_entry_open.set(false);
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("Kill not found");
                        error_message_state.set(AttrValue::from("Der Kill konnte nicht gefunden werden"));
                        error_title_state.set(AttrValue::from("Fehler beim Löschen"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Kills zu löschen"));
                        error_title_state.set(AttrValue::from("Fehler beim Löschen"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let delete_entry_name_state = delete_entry_name_state.clone();
                        error_message_state.set(AttrValue::from(format!("Der Kill {} nicht gelöscht werden, bitte wende dich an Azami", (*delete_entry_name_state).clone())));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                });
                let _ = kill_query_state.refresh().await;
            });
        })
    };

    let on_modify_modal_state_change = use_callback(|state: EntryModalState, modify_modal_state| modify_modal_state.set(state), modify_modal_state.clone());

    let on_delete_click = use_callback(|name: AttrValue, (name_state, message_state, open_state)| {
        name_state.set(name.clone());
        message_state.set(AttrValue::from(format!("Soll der Kill {} wirklich gelöscht werden?", name)));
        open_state.set(true);
    }, (delete_entry_name_state, delete_entry_message_state.clone(), delete_entry_open.clone()));

    match kill_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initially_loaded_state {
                return html!(<p data-msg="info">{"Die Kills werden geladen"}</p>);
            }
        }
        Some(Ok(kills)) => {
            log::debug!("Loaded kills");
            initially_loaded_state.set(true);
            let kills = kills.clone();
            state.set((*kills).clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {}", err);
            return html!(<p data-msg="negative">{"Die Kills konnten nicht geladen werden, bitte wende dich an Azami"}</p>);
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Kills"}</title>
            </Helmet>
            <h1>{"Kills"}</h1>
            <p data-msg="info">
                {if is_mod {
                    "Du bist Mod, daher hast du hier die Möglichkeit die Kills aller Crewmitglieder zu bearbeiten"
                } else {
                    "Da du kein Mod bist kannst du nur deine eigenen Kills bearbeiten"
                }}
            </p>
            <BooleanTable on_delete_confirm={on_delete_confirm} on_delete_decline={on_delete_decline} on_delete_click={on_delete_click} delete_message={(*delete_entry_message_state).clone()} delete_entry_open={*delete_entry_open} delete_title="Kill löschen" delete_confirm="Kill löschen" on_modify_modal_state_change={on_modify_modal_state_change} modify_modal_state={(*modify_modal_state).clone()} add_title="Kill hinzufügen" edit_title="Kill bearbeiten" add_label="Kill hinzufügen" add_save_label="Kill hinzufügen" edit_save_label="Kill speichern" has_error={*error_state} error_message={(*error_message_state).clone()} is_loading={*loading_state} on_add_save={on_add_save} on_edit_save={on_edit_save} table_data={state.data.clone()} on_activate_entry={activate_kill} on_deactivate_entry={deactivate_kill} />
            {if *error_state {
                html!(
                    <PicoAlert title={(*error_title_state).clone()} message={(*error_message_state).clone()} open={true} on_close={move |_| error_state.clone().set(false)} />
                )
            } else {
                html!()
            }}
        </>
    )
}

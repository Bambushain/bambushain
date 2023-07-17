use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use yew::prelude::*;
use crate::api::savage_mount::{activate_savage_mount, create_savage_mount, deactivate_savage_mount, delete_savage_mount, SavageMounts, rename_savage_mount};
use crate::api::{NOT_FOUND, NO_CONTENT, FORBIDDEN};
use crate::pages::boolean_table::{ActivationParams, BooleanTable, EntryModalState, ModifyEntryModalSaveData};
use crate::ui::modal::PicoAlert;

#[function_component(SavageMountPage)]
pub fn savage_mount_page() -> Html {
    log::debug!("Render savage_mounts page");
    log::debug!("Initialize state and callbacks");
    let savage_mount_query_state = use_query_value::<SavageMounts>(().into());

    let initially_loaded_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);
    let delete_entry_open = use_state_eq(|| false);

    let modify_modal_state = use_state_eq(|| EntryModalState::Closed);

    let state = use_state_eq(SavageMounts::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_title_state = use_state_eq(|| AttrValue::from(""));
    let delete_entry_name_state = use_state_eq(|| AttrValue::from(""));
    let delete_entry_message_state = use_state_eq(|| AttrValue::from(""));

    let activate_savage_mount = {
        let savage_mount_query_state = savage_mount_query_state.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        Callback::from(move |params: ActivationParams| {
            log::debug!("Activate savage_mount {} for {}", params.key, params.user);
            let savage_mount_query_state = savage_mount_query_state.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                let params = params.clone();

                log::debug!("Execute request");
                error_state.set(match activate_savage_mount(params.user.clone(), params.key.clone()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        error_title_state.set(AttrValue::from(""));
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("User or savage_mount not found");
                        error_message_state.set(AttrValue::from("Entweder das Savage Mount oder das Crewmitglied konnte nicht gefunden werden"));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Savage Mounts zu ändern, wenn du deine eigenen Savage Mounts anpassen möchtest, mach das über Mein Sheef"));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let params = params.clone();

                        error_message_state.set(AttrValue::from(format!("Das Savage Mount {} konnte für {} nicht aktiviert werden, bitte wende dich an Azami", params.key, params.user)));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                });
                let _ = savage_mount_query_state.refresh().await;
            });
        })
    };
    let deactivate_savage_mount = {
        let savage_mount_query_state = savage_mount_query_state.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        Callback::from(move |params: ActivationParams| {
            log::debug!("Deactivate savage_mount {} for {}", params.key, params.user);
            let savage_mount_query_state = savage_mount_query_state.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                let params = params.clone();

                log::debug!("Execute request");
                error_state.set(match deactivate_savage_mount(params.user.clone(), params.key.clone()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("User or savage_mount not found");
                        error_message_state.set(AttrValue::from("Entweder das Savage Mount oder das Crewmitglied konnte nicht gefunden werden"));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Savage Mounts zu ändern, wenn du deine eigenen Savage Mounts anpassen möchtest, mach das über Mein Sheef"));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let params = params.clone();

                        error_message_state.set(AttrValue::from(format!("Das Savage Mount {} konnte für {} nicht deaktiviert werden, bitte wende dich an Azami", params.key, params.user)));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                });
                let _ = savage_mount_query_state.refresh().await;
            });
        })
    };

    let on_add_save = {
        let savage_mount_query_state = savage_mount_query_state.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        let modify_modal_state = modify_modal_state.clone();

        Callback::from(move |data: ModifyEntryModalSaveData| {
            log::debug!("Add savage_mount {}", data.new_name);
            loading_state.set(true);

            let savage_mount_query_state = savage_mount_query_state.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            let modify_modal_state = modify_modal_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match create_savage_mount(data.new_name.to_string()).await {
                    Ok(_) => {
                        error_message_state.set(AttrValue::from(""));
                        modify_modal_state.set(EntryModalState::Closed);
                        false
                    }
                    Err(FORBIDDEN) => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Savage Mounts hinzuzufügen"));
                        error_title_state.set(AttrValue::from("Fehler beim Hinzufügen"));
                        true
                    }
                    Err(err) => {
                        log::warn!("Another error occurred {}", err);
                        error_message_state.set(AttrValue::from(format!("Das Savage Mount {} nicht erstellt werden, bitte wende dich an Azami", data.new_name)));
                        error_title_state.set(AttrValue::from("Fehler beim Hinzufügen"));
                        true
                    }
                });
                loading_state.set(false);
                let _ = savage_mount_query_state.refresh().await;
            })
        })
    };
    let on_edit_save = {
        let savage_mount_query_state = savage_mount_query_state.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        let modify_modal_state = modify_modal_state.clone();

        Callback::from(move |data: ModifyEntryModalSaveData| {
            loading_state.set(true);

            let savage_mount_query_state = savage_mount_query_state.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let modify_modal_state = modify_modal_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match rename_savage_mount(data.old_name.to_string(), data.new_name.to_string()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        modify_modal_state.set(EntryModalState::Closed);
                        false
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Savage Mounts umzubenennen"));
                        error_title_state.set(AttrValue::from("Fehler beim Umbenennen"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        error_message_state.set(AttrValue::from(format!("Das Savage Mount {} nicht umbenannt werden, bitte wende dich an Azami", data.old_name)));
                        error_title_state.set(AttrValue::from("Fehler beim Umbenennen"));
                        true
                    }
                });
                loading_state.set(false);
                let _ = savage_mount_query_state.refresh().await;
            })
        })
    };

    let on_delete_decline = use_callback(|_, (name_state, message_state, open_state)| {
        name_state.set(AttrValue::from(""));
        message_state.set(AttrValue::from(""));
        open_state.set(false);
    }, (delete_entry_name_state.clone(), delete_entry_message_state.clone(), delete_entry_open.clone()));
    let on_delete_confirm = {
        let savage_mount_query_state = savage_mount_query_state.clone();

        let error_state = error_state.clone();
        let delete_entry_open = delete_entry_open.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();
        let delete_entry_name_state = delete_entry_name_state.clone();

        Callback::from(move |_| {
            log::debug!("Delete savage_mount {}", (*delete_entry_name_state).clone());
            let savage_mount_query_state = savage_mount_query_state.clone();

            let error_state = error_state.clone();
            let delete_entry_open = delete_entry_open.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();
            let delete_entry_name_state = delete_entry_name_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match delete_savage_mount((*delete_entry_name_state).to_string()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        delete_entry_open.set(false);
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("SavageMount not found");
                        error_message_state.set(AttrValue::from("Das Savage Mount konnte nicht gefunden werden"));
                        error_title_state.set(AttrValue::from("Fehler beim Löschen"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Savage Mounts zu löschen"));
                        error_title_state.set(AttrValue::from("Fehler beim Löschen"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let delete_entry_name_state = delete_entry_name_state.clone();
                        error_message_state.set(AttrValue::from(format!("Das Savage Mount {} nicht gelöscht werden, bitte wende dich an Azami", (*delete_entry_name_state).clone())));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                });
                let _ = savage_mount_query_state.refresh().await;
            });
        })
    };

    let on_modify_modal_state_change = use_callback(|state: EntryModalState, modify_modal_state| modify_modal_state.set(state), modify_modal_state.clone());

    let on_delete_click = use_callback(|name: AttrValue, (name_state, message_state, open_state)| {
        name_state.set(name.clone());
        message_state.set(AttrValue::from(format!("Soll das Savage Mount {} wirklich gelöscht werden?", name)));
        open_state.set(true);
    }, (delete_entry_name_state, delete_entry_message_state.clone(), delete_entry_open.clone()));

    match savage_mount_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initially_loaded_state {
                return html!(<p data-msg="info">{"Die Savage Mounts werden geladen"}</p>);
            }
        }
        Some(Ok(savage_mounts)) => {
            log::debug!("Loaded savage_mounts");
            initially_loaded_state.set(true);
            let savage_mounts = savage_mounts.clone();
            state.set((*savage_mounts).clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {}", err);
            return html!(<p data-msg="negative">{"Die Savage Mounts konnten nicht geladen werden, bitte wende dich an Azami"}</p>);
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Savage Mounts"}</title>
            </Helmet>
            <h1>{"Savage Mounts"}</h1>
            <BooleanTable on_delete_confirm={on_delete_confirm} on_delete_decline={on_delete_decline} on_delete_click={on_delete_click} delete_message={(*delete_entry_message_state).clone()} delete_entry_open={*delete_entry_open} delete_title="Savage Mount löschen" delete_confirm="Savage Mount löschen" on_modify_modal_state_change={on_modify_modal_state_change} modify_modal_state={(*modify_modal_state).clone()} add_title="Savage Mount hinzufügen" edit_title="Savage Mount bearbeiten" add_label="Savage Mount hinzufügen" add_save_label="Savage Mount hinzufügen" edit_save_label="Savage Mount speichern" has_error={*error_state} error_message={(*error_message_state).clone()} is_loading={*loading_state} on_add_save={on_add_save} on_edit_save={on_edit_save} table_data={state.data.clone()} on_activate_entry={activate_savage_mount} on_deactivate_entry={deactivate_savage_mount} />
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

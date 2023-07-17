use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use bounce::use_atom_value;
use yew::prelude::*;
use crate::api::mount::{activate_mount, create_mount, deactivate_mount, delete_mount, Mounts, rename_mount};
use crate::api::{NOT_FOUND, NO_CONTENT, FORBIDDEN};
use crate::api::my::{activate_mount_for_me, deactivate_mount_for_me};
use crate::pages::boolean_table::{ActivationParams, BooleanTable, EntryModalState, ModifyEntryModalSaveData};
use crate::storage::CurrentUser;
use crate::ui::modal::PicoAlert;

#[function_component(MountPage)]
pub fn mount_page() -> Html {
    log::debug!("Render mounts page");
    log::debug!("Initialize state and callbacks");
    let mount_query_state = use_query_value::<Mounts>(().into());

    let current_user = use_atom_value::<CurrentUser>();
    let is_mod = current_user.profile.is_mod;

    let initially_loaded_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);
    let delete_entry_open = use_state_eq(|| false);

    let modify_modal_state = use_state_eq(|| EntryModalState::Closed);

    let state = use_state_eq(Mounts::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_title_state = use_state_eq(|| AttrValue::from(""));
    let delete_entry_name_state = use_state_eq(|| AttrValue::from(""));
    let delete_entry_message_state = use_state_eq(|| AttrValue::from(""));

    let activate_mount = {
        let mount_query_state = mount_query_state.clone();

        let current_user = current_user.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        Callback::from(move |params: ActivationParams| {
            log::debug!("Activate mount {} for {}", params.key, params.user);
            let mount_query_state = mount_query_state.clone();

            let current_user = current_user.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                let params = params.clone();

                let result = if !current_user.profile.is_mod && current_user.profile.username == params.user.clone() {
                    activate_mount_for_me(params.key.clone()).await
                } else {
                    activate_mount(params.user.clone(), params.key.clone()).await
                };

                log::debug!("Execute request");
                error_state.set(match result {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        error_title_state.set(AttrValue::from(""));
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("User or mount not found");
                        error_message_state.set(AttrValue::from(if current_user.profile.is_mod { "Entweder das Mount oder das Crewmitglied konnte nicht gefunden werden" } else { "Das Mount konnte nicht gefunden werden" }));
                        error_message_state.set(AttrValue::from(""));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Mounts anderer Crewmitglieder zu aktivieren"));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let params = params.clone();

                        error_message_state.set(AttrValue::from(format!("Das Mount {} konnte für {} nicht aktiviert werden, bitte wende dich an Azami", params.key, params.user)));
                        error_title_state.set(AttrValue::from("Fehler beim Aktivieren"));
                        true
                    }
                });
                let _ = mount_query_state.refresh().await;
            });
        })
    };
    let deactivate_mount = {
        let mount_query_state = mount_query_state.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        Callback::from(move |params: ActivationParams| {
            log::debug!("Deactivate mount {} for {}", params.key, params.user);
            let mount_query_state = mount_query_state.clone();

            let current_user = current_user.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                let params = params.clone();

                let result = if !current_user.profile.is_mod && current_user.profile.username == params.user.clone() {
                    deactivate_mount_for_me(params.key.clone()).await
                } else {
                    deactivate_mount(params.user.clone(), params.key.clone()).await
                };

                log::debug!("Execute request");
                error_state.set(match result {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("User or mount not found");
                        error_message_state.set(AttrValue::from(if current_user.profile.is_mod { "Entweder das Mount oder das Crewmitglied konnte nicht gefunden werden" } else { "Das Mount konnte nicht gefunden werden" }));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Mounts anderer Crewmitglieder zu deaktivieren"));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let params = params.clone();

                        error_message_state.set(AttrValue::from(format!("Das Mount {} konnte für {} nicht deaktiviert werden, bitte wende dich an Azami", params.key, params.user)));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                });
                let _ = mount_query_state.refresh().await;
            });
        })
    };

    let on_add_save = {
        let mount_query_state = mount_query_state.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        let modify_modal_state = modify_modal_state.clone();

        Callback::from(move |data: ModifyEntryModalSaveData| {
            log::debug!("Add mount {}", data.new_name);
            loading_state.set(true);

            let mount_query_state = mount_query_state.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            let modify_modal_state = modify_modal_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match create_mount(data.new_name.to_string()).await {
                    Ok(_) => {
                        error_message_state.set(AttrValue::from(""));
                        modify_modal_state.set(EntryModalState::Closed);
                        false
                    }
                    Err(FORBIDDEN) => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Mounts hinzuzufügen"));
                        error_title_state.set(AttrValue::from("Fehler beim Hinzufügen"));
                        true
                    }
                    Err(err) => {
                        log::warn!("Another error occurred {}", err);
                        error_message_state.set(AttrValue::from(format!("Das Mount {} nicht erstellt werden, bitte wende dich an Azami", data.new_name)));
                        error_title_state.set(AttrValue::from("Fehler beim Hinzufügen"));
                        true
                    }
                });
                loading_state.set(false);
                let _ = mount_query_state.refresh().await;
            })
        })
    };
    let on_edit_save = {
        let mount_query_state = mount_query_state.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();

        let modify_modal_state = modify_modal_state.clone();

        Callback::from(move |data: ModifyEntryModalSaveData| {
            loading_state.set(true);

            let mount_query_state = mount_query_state.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let modify_modal_state = modify_modal_state.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match rename_mount(data.old_name.to_string(), data.new_name.to_string()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        modify_modal_state.set(EntryModalState::Closed);
                        false
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Mounts umzubenennen"));
                        error_title_state.set(AttrValue::from("Fehler beim Umbenennen"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        error_message_state.set(AttrValue::from(format!("Das Mount {} nicht umbenannt werden, bitte wende dich an Azami", data.old_name)));
                        error_title_state.set(AttrValue::from("Fehler beim Umbenennen"));
                        true
                    }
                });
                loading_state.set(false);
                let _ = mount_query_state.refresh().await;
            })
        })
    };

    let on_delete_decline = use_callback(|_, (name_state, message_state, open_state)| {
        name_state.set(AttrValue::from(""));
        message_state.set(AttrValue::from(""));
        open_state.set(false);
    }, (delete_entry_name_state.clone(), delete_entry_message_state.clone(), delete_entry_open.clone()));
    let on_delete_confirm = {
        let mount_query_state = mount_query_state.clone();

        let error_state = error_state.clone();
        let delete_entry_open = delete_entry_open.clone();

        let error_message_state = error_message_state.clone();
        let error_title_state = error_title_state.clone();
        let delete_entry_name_state = delete_entry_name_state.clone();

        Callback::from(move |_| {
            log::debug!("Delete mount {}", (*delete_entry_name_state).clone());
            let mount_query_state = mount_query_state.clone();

            let error_state = error_state.clone();
            let delete_entry_open = delete_entry_open.clone();

            let error_message_state = error_message_state.clone();
            let error_title_state = error_title_state.clone();
            let delete_entry_name_state = delete_entry_name_state.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Execute request");
                error_state.set(match delete_mount((*delete_entry_name_state).to_string()).await {
                    NO_CONTENT => {
                        error_message_state.set(AttrValue::from(""));
                        delete_entry_open.set(false);
                        false
                    }
                    NOT_FOUND => {
                        log::warn!("Mount not found");
                        error_message_state.set(AttrValue::from("Das Mount konnte nicht gefunden werden"));
                        error_title_state.set(AttrValue::from("Fehler beim Löschen"));
                        true
                    }
                    FORBIDDEN => {
                        log::warn!("User is not mod");
                        error_message_state.set(AttrValue::from("Du musst Mod sein um Mounts zu löschen"));
                        error_title_state.set(AttrValue::from("Fehler beim Löschen"));
                        true
                    }
                    err => {
                        log::warn!("Another error occurred {}", err);
                        let delete_entry_name_state = delete_entry_name_state.clone();
                        error_message_state.set(AttrValue::from(format!("Das Mount {} nicht gelöscht werden, bitte wende dich an Azami", (*delete_entry_name_state).clone())));
                        error_title_state.set(AttrValue::from("Fehler beim Deaktivieren"));
                        true
                    }
                });
                let _ = mount_query_state.refresh().await;
            });
        })
    };

    let on_modify_modal_state_change = use_callback(|state: EntryModalState, modify_modal_state| modify_modal_state.set(state), modify_modal_state.clone());

    let on_delete_click = use_callback(|name: AttrValue, (name_state, message_state, open_state)| {
        name_state.set(name.clone());
        message_state.set(AttrValue::from(format!("Soll das Mount {} wirklich gelöscht werden?", name)));
        open_state.set(true);
    }, (delete_entry_name_state, delete_entry_message_state.clone(), delete_entry_open.clone()));

    match mount_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initially_loaded_state {
                return html!(<p data-msg="info">{"Die Mounts werden geladen"}</p>);
            }
        }
        Some(Ok(mounts)) => {
            log::debug!("Loaded mounts");
            initially_loaded_state.set(true);
            let mounts = mounts.clone();
            state.set((*mounts).clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {}", err);
            return html!(<p data-msg="negative">{"Die Mounts konnten nicht geladen werden, bitte wende dich an Azami"}</p>);
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Mounts"}</title>
            </Helmet>
            <h1>{"Mounts"}</h1>
            <p data-msg="info">
                {if is_mod {
                    "Du bist Mod, daher hast du hier die Möglichkeit die Mounts aller Crewmitglieder zu bearbeiten"
                } else {
                    "Da du kein Mod bist kannst du nur deine eigenen Mounts bearbeiten"
                }}
            </p>
            <BooleanTable on_delete_confirm={on_delete_confirm} on_delete_decline={on_delete_decline} on_delete_click={on_delete_click} delete_message={(*delete_entry_message_state).clone()} delete_entry_open={*delete_entry_open} delete_title="Mount löschen" delete_confirm="Mount löschen" on_modify_modal_state_change={on_modify_modal_state_change} modify_modal_state={(*modify_modal_state).clone()} add_title="Mount hinzufügen" edit_title="Mount bearbeiten" add_label="Mount hinzufügen" add_save_label="Mount hinzufügen" edit_save_label="Mount speichern" has_error={*error_state} error_message={(*error_message_state).clone()} is_loading={*loading_state} on_add_save={on_add_save} on_edit_save={on_edit_save} table_data={state.data.clone()} on_activate_entry={activate_mount} on_deactivate_entry={deactivate_mount} />
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

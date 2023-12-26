use std::ops::Deref;

use bounce::query::use_query_value;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount, use_unmount};
use yew_icons::Icon;

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::CONFLICT;
use bamboo_frontend_base_error as error;

use crate::api;
use crate::models::CustomCharacterFields;
use crate::props::settings::*;

#[function_component(FieldsTabItem)]
fn fields_tab_item(props: &FieldsTabItemProps) -> Html {
    let delete_open_state = use_state_eq(|| false);
    let rename_open_state = use_state_eq(|| false);
    let add_option_open_state = use_state_eq(|| false);
    let edit_option_open_state = use_state_eq(|| false);
    let delete_option_open_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let delete_error_state = use_state_eq(|| false);
    let move_error_state = use_state_eq(|| false);
    let unreported_error_toggle = use_state_eq(|| false);

    let selected_option_id_state = use_state_eq(|| -1);

    let rename_name_state = use_state_eq(|| AttrValue::from(props.field.label.clone()));
    let option_label_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let selected_option_label_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    {
        let error_state = error_state.clone();
        let delete_error_state = delete_error_state.clone();
        let rename_name_state = rename_name_state.clone();
        let option_label_state = option_label_state.clone();

        use_unmount(move || {
            error_state.set(false);
            delete_error_state.set(false);
            rename_name_state.set("".into());
            option_label_state.set("".into());
        })
    }

    let report_unknown_error = use_callback(
        (
            bamboo_error_state.clone(),
            error_message_form_state.clone(),
            unreported_error_toggle.clone(),
        ),
        |_, (bamboo_error_state, error_message_form_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "final_fantasy_settings",
                error_message_form_state.deref().to_string(),
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    let on_rename_open = use_callback(
        (rename_open_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(true);
            error_state.set(false);
        },
    );
    let on_rename_close = use_callback(
        (rename_open_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(false);
            error_state.set(false);
        },
    );
    let on_rename_save = {
        let id = props.field.id;

        let rename_name_state = rename_name_state.clone();
        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let rename_open_state = rename_open_state.clone();
        let error_state = error_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let on_change = props.on_change.clone();

        Callback::from(move |_| {
            let rename_name_state = rename_name_state.clone();
            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let rename_open_state = rename_open_state.clone();
            let error_state = error_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let on_change = on_change.clone();

            yew::platform::spawn_local(async move {
                error_state.set(
                    match api::update_custom_field(id, (*rename_name_state).clone().to_string(), 0)
                        .await
                    {
                        Ok(_) => {
                            on_change.emit(());
                            rename_open_state.set(false);
                            false
                        }
                        Err(err) => {
                            log::error!("Failed to rename custom field {err}");
                            if err.code == CONFLICT {
                                error_message_state
                                    .set("Ein Feld mit dem Namen existiert bereits".into());
                                unreported_error_toggle.set(false);
                            } else {
                                error_message_state.set(
                                    "Beim Umbennen des Felds ist ein Fehler aufgetreten".into(),
                                );
                                error_message_form_state.set("update_custom_field".into());
                                bamboo_error_state.set(err.clone());
                                unreported_error_toggle.set(true);
                            }

                            true
                        }
                    },
                );
            })
        })
    };

    let on_add_option_open = use_callback(
        (
            add_option_open_state.clone(),
            error_state.clone(),
            option_label_state.clone(),
        ),
        |_, (open_state, error_state, option_label_state)| {
            open_state.set(true);
            error_state.set(false);
            option_label_state.set("".into());
        },
    );
    let on_add_option_close = use_callback(
        (add_option_open_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(false);
            error_state.set(false);
        },
    );
    let on_add_option_save = {
        let id = props.field.id;

        let option_label_state = option_label_state.clone();
        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let add_option_open_state = add_option_open_state.clone();
        let error_state = error_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let on_change = props.on_change.clone();

        Callback::from(move |_| {
            let option_label_state = option_label_state.clone();
            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let add_option_open_state = add_option_open_state.clone();
            let error_state = error_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let on_change = on_change.clone();

            yew::platform::spawn_local(async move {
                error_state.set(
                    match api::add_custom_field_option(
                        id,
                        (*option_label_state).clone().to_string(),
                    )
                    .await
                    {
                        Ok(_) => {
                            on_change.emit(());
                            add_option_open_state.set(false);
                            unreported_error_toggle.set(false);
                            option_label_state.set("".into());

                            false
                        }
                        Err(err) => {
                            log::error!("Failed to add custom field option {err}");
                            if err.code == CONFLICT {
                                error_message_state
                                    .set("Eine Option mit dem Namen existiert bereits".into());
                                unreported_error_toggle.set(false);
                            } else {
                                error_message_state
                                    .set("Die Option konnte nicht hinzugefügt werden".into());
                                error_message_form_state.set("add_custom_field_option".into());
                                bamboo_error_state.set(err.clone());
                                unreported_error_toggle.set(true);
                            }

                            true
                        }
                    },
                );
            })
        })
    };

    let on_edit_option_open =
        use_callback(
            (
                selected_option_id_state.clone(),
                selected_option_label_state.clone(),
                edit_option_open_state.clone(),
                option_label_state.clone(),
                error_state.clone(),
            ),
            |(id, label): (i32, AttrValue),
             (
                selected_id_state,
                selected_label_state,
                open_state,
                option_label_state,
                error_state,
            )| {
                selected_id_state.set(id);
                selected_label_state.set(label.clone());
                option_label_state.set(label);
                open_state.set(true);
                error_state.set(false);
            },
        );
    let on_edit_option_close = use_callback(
        (edit_option_open_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(false);
            error_state.set(false);
        },
    );
    let on_edit_option_save = {
        let id = props.field.id;

        let option_label_state = option_label_state.clone();
        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let selected_option_id_state = selected_option_id_state.clone();

        let edit_option_open_state = edit_option_open_state.clone();
        let error_state = error_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let on_change = props.on_change.clone();

        Callback::from(move |_| {
            let option_label_state = option_label_state.clone();
            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let selected_option_id_state = selected_option_id_state.clone();

            let edit_option_open_state = edit_option_open_state.clone();
            let error_state = error_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let on_change = on_change.clone();

            yew::platform::spawn_local(async move {
                error_state.set(
                    match api::update_custom_field_option(
                        id,
                        *selected_option_id_state,
                        (*option_label_state).clone().to_string(),
                    )
                    .await
                    {
                        Ok(_) => {
                            on_change.emit(());
                            edit_option_open_state.set(false);
                            option_label_state.set("".into());
                            unreported_error_toggle.set(false);

                            false
                        }
                        Err(err) => {
                            log::error!("Failed to edit custom field option {err}");
                            if err.code == CONFLICT {
                                error_message_state
                                    .set("Eine Option mit dem Namen existiert bereits".into());
                                unreported_error_toggle.set(false);
                            } else {
                                error_message_state
                                    .set("Die Option konnte nicht umbenannt werden".into());
                                error_message_form_state.set("update_custom_field_option".into());
                                bamboo_error_state.set(err.clone());
                                unreported_error_toggle.set(true);
                            }

                            true
                        }
                    },
                );
            })
        })
    };

    let on_delete_open = use_callback(
        (delete_open_state.clone(), delete_error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(true);
            error_state.set(false);
        },
    );
    let on_delete_close = use_callback(delete_open_state.clone(), |_, state| state.set(false));
    let on_delete = {
        let id = props.field.id;

        let on_change = props.on_change.clone();

        let delete_error_state = delete_error_state.clone();
        let delete_open_state = delete_open_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        Callback::from(move |_| {
            let on_change = on_change.clone();

            let delete_error_state = delete_error_state.clone();
            let delete_open_state = delete_open_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            yew::platform::spawn_local(async move {
                delete_error_state.set(match api::delete_custom_field(id).await {
                    Ok(_) => {
                        delete_open_state.set(false);
                        on_change.emit(());
                        unreported_error_toggle.set(false);

                        false
                    }
                    Err(err) => {
                        log::error!("Delete failed: {err}");
                        error_message_state.set("Das Feld konnte nicht gelöscht werden".into());
                        error_message_form_state.set("delete_custom_field".into());
                        bamboo_error_state.set(err.clone());
                        unreported_error_toggle.set(true);

                        true
                    }
                });
            })
        })
    };

    let on_delete_option_open = use_callback(
        (
            selected_option_id_state.clone(),
            selected_option_label_state.clone(),
            delete_option_open_state.clone(),
            delete_error_state.clone(),
        ),
        |(id, label), (selected_id_state, selected_label_state, open_state, error_state)| {
            selected_id_state.set(id);
            selected_label_state.set(label);
            open_state.set(true);
            error_state.set(false);
        },
    );
    let on_delete_option_close = use_callback(delete_option_open_state.clone(), |_, state| {
        state.set(false)
    });
    let on_delete_option = {
        let field_id = props.field.id;

        let on_change = props.on_change.clone();

        let error_state = error_state.clone();
        let delete_option_open_state = delete_option_open_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let selected_option_id_state = selected_option_id_state;

        Callback::from(move |_| {
            let on_change = on_change.clone();

            let error_state = error_state.clone();
            let delete_option_open_state = delete_option_open_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let selected_option_id_state = selected_option_id_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(
                    match api::delete_custom_field_option(field_id, *selected_option_id_state).await
                    {
                        Ok(_) => {
                            delete_option_open_state.set(false);
                            on_change.emit(());
                            unreported_error_toggle.set(false);

                            false
                        }
                        Err(err) => {
                            log::error!("Delete failed: {err}");
                            error_message_state
                                .set("Die Option konnte nicht gelöscht werden".into());
                            error_message_form_state.set("delete_custom_field_option".into());
                            bamboo_error_state.set(err.clone());
                            unreported_error_toggle.set(true);

                            true
                        }
                    },
                );
            })
        })
    };

    let update_rename_name = use_callback(rename_name_state.clone(), |val, state| state.set(val));
    let update_option_label = use_callback(option_label_state.clone(), |val, state| state.set(val));

    let on_move_right = {
        let id = props.field.id;

        let on_move = props.on_move.clone();

        let position = props.position;

        let move_error_state = move_error_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        Callback::from(move |_| {
            let on_move = on_move.clone();

            let move_error_state = move_error_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            yew::platform::spawn_local(async move {
                match api::move_custom_field(id, position as usize + 1).await {
                    Ok(_) => {
                        on_move.emit(position as usize + 1);
                        unreported_error_toggle.set(false);
                        move_error_state.set(false);
                    }
                    Err(err) => {
                        log::error!("Move failed: {err}");
                        bamboo_error_state.set(err.clone());
                        unreported_error_toggle.set(true);
                        move_error_state.set(true);
                        error_message_state.set("Das Feld konnte nicht verschoben werden".into());
                        error_message_form_state.set("move_custom_field".into());
                    }
                }
            })
        })
    };
    let on_move_left = {
        let id = props.field.id;

        let on_move = props.on_move.clone();

        let position = props.position;

        let move_error_state = move_error_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        Callback::from(move |_| {
            let on_move = on_move.clone();

            let move_error_state = move_error_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            yew::platform::spawn_local(async move {
                match api::move_custom_field(id, position as usize - 1).await {
                    Ok(_) => {
                        on_move.emit(position as usize + 1);
                        unreported_error_toggle.set(false);
                        move_error_state.set(false);
                    }
                    Err(err) => {
                        log::error!("Move failed: {err}");
                        bamboo_error_state.set(err.clone());
                        unreported_error_toggle.set(true);
                        move_error_state.set(true);
                        error_message_state.set("Das Feld konnte nicht verschoben werden".into());
                        error_message_form_state.set("move_custom_field".into());
                    }
                }
            })
        })
    };

    let list_style = use_style!(
        r#"
display: flex;
flex-flow: row wrap;
gap: 0.125rem;
    "#
    );
    let item_style = use_style!(
        r#"
display: flex;
gap: 0.25rem;
flex: 0 0 100%;
min-width: 100%;
align-items: center;
    "#
    );

    let mut options = props.field.options.clone();
    options.sort();

    html!(
        <>
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Feld bearbeiten" on_click={on_rename_open} />
                    <CosmoButton label="Feld löschen" on_click={on_delete_open} />
                </CosmoToolbarGroup>
                <CosmoToolbarGroup>
                    <CosmoButton label="Option hinzufügen" on_click={on_add_option_open} />
                </CosmoToolbarGroup>
                <CosmoToolbarGroup>
                    <CosmoButton label="Nach links verschieben" enabled={!props.is_first} on_click={on_move_left} />
                    <CosmoButton label="Nach rechts verschieben" enabled={!props.is_last} on_click={on_move_right} />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            <CosmoHeader level={CosmoHeaderLevel::H3} header="Optionen" />
            if *delete_error_state {
                if *unreported_error_toggle {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} />
                }
            }
            if *move_error_state {
                if *unreported_error_toggle {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Verschieben" message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Verschieben" message={(*error_message_state).clone()} />
                }
            }
            <div class={list_style}>
                {for options.iter().map(|option| {
                    let delete_option = option.clone();
                    let edit_option = option.clone();

                    let on_delete_option_open = on_delete_option_open.clone();
                    let on_edit_option_open = on_edit_option_open.clone();

                    html!(
                        <div class={item_style.clone()}>
                            {option.label.clone()}
                            <Icon style="cursor: pointer;" width="1rem" height="1rem" icon_id={IconId::LucideEdit} onclick={move |_| on_edit_option_open.emit((edit_option.id, edit_option.label.clone().into()))} />
                            <Icon style="cursor: pointer;" width="1rem" height="1rem" icon_id={IconId::LucideTrash} onclick={move |_| on_delete_option_open.emit((delete_option.id, delete_option.label.clone().into()))} />
                        </div>
                    )
                })}
            </div>
            if *rename_open_state {
                <CosmoModal title="Feld umbenennen" is_form={true} on_form_submit={on_rename_save} buttons={html!(
                    <>
                        <CosmoButton on_click={on_rename_close} label="Abbrechen" />
                        <CosmoButton label="Feld speichern" is_submit={true} />
                    </>
                )}>
                    if *error_state {
                        if *unreported_error_toggle {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                        } else {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Name" on_input={update_rename_name} value={(*rename_name_state).clone()} required={true} />
                    </CosmoInputGroup>
                </CosmoModal>
            }
            if *add_option_open_state {
                <CosmoModal title="Option hinzufügen" is_form={true} on_form_submit={on_add_option_save} buttons={html!(
                    <>
                        <CosmoButton on_click={on_add_option_close} label="Abbrechen" />
                        <CosmoButton label="Option hinzufügen" is_submit={true} />
                    </>
                )}>
                    if *error_state {
                        if *unreported_error_toggle {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                        } else {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Name" on_input={update_option_label.clone()} value={(*option_label_state).clone()} required={true} />
                    </CosmoInputGroup>
                </CosmoModal>
            }
            if *edit_option_open_state {
                <CosmoModal title="Option umbenennen" is_form={true} on_form_submit={on_edit_option_save} buttons={html!(
                    <>
                        <CosmoButton on_click={on_edit_option_close} label="Abbrechen" />
                        <CosmoButton label="Option umbenennen" is_submit={true} />
                    </>
                )}>
                    if *error_state {
                        if *unreported_error_toggle {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                        } else {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Name" on_input={update_option_label} value={(*option_label_state).clone()} required={true} />
                    </CosmoInputGroup>
                </CosmoModal>
            }
            if *delete_open_state {
                <CosmoConfirm confirm_type={CosmoModalType::Warning} title="Feld löschen" message={format!("Soll das Feld {} wirklich gelöscht werden?", props.field.label.clone())} confirm_label="Feld Löschen" decline_label="Nicht löschen"  on_decline={on_delete_close} on_confirm={on_delete} />
            }
            if *delete_option_open_state {
                <CosmoConfirm confirm_type={CosmoModalType::Warning} title="Option löschen" message={format!("Soll die Option {} wirklich gelöscht werden?", (*selected_option_label_state).clone())} confirm_label="Option Löschen" decline_label="Nicht löschen"  on_decline={on_delete_option_close} on_confirm={on_delete_option} />
            }
        </>
    )
}

#[function_component(CustomFieldPage)]
fn custom_field_page() -> Html {
    log::debug!("Render custom fields page");
    log::debug!("Initialize state and callbacks");
    let fields_query_state = use_query_value::<CustomCharacterFields>(().into());

    let initial_loaded_state = use_state_eq(|| false);
    let add_open_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let unreported_error_toggle = use_state_eq(|| false);

    let add_name_state = use_state_eq(|| AttrValue::from(""));
    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let fields_state = use_state_eq(|| vec![] as Vec<CustomCharacterField>);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let selected_item_state = use_state(|| Some(0usize));

    {
        let error_state = error_state.clone();
        let add_name_state = add_name_state.clone();

        use_unmount(move || {
            error_state.set(false);
            add_name_state.set("".into());
        })
    }

    let report_unknown_error = use_callback(
        (
            bamboo_error_state.clone(),
            error_message_form_state.clone(),
            unreported_error_toggle.clone(),
        ),
        |_, (bamboo_error_state, error_message_form_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "final_fantasy_settings",
                error_message_form_state.deref().to_string(),
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    let on_add_open = use_callback(
        (
            add_open_state.clone(),
            error_state.clone(),
            add_name_state.clone(),
        ),
        |_, (open_state, error_state, add_name_state)| {
            open_state.set(true);
            error_state.set(false);
            add_name_state.set("".into());
        },
    );
    let on_change = {
        let fields_query_state = fields_query_state.clone();

        Callback::from(move |_| {
            let fields_query_state = fields_query_state.clone();

            yew::platform::spawn_local(async move {
                let _ = fields_query_state.refresh().await;
            })
        })
    };
    let on_move = {
        let fields_query_state = fields_query_state.clone();

        let selected_item_state = selected_item_state.clone();

        Callback::from(move |idx| {
            let fields_query_state = fields_query_state.clone();

            let selected_item_state = selected_item_state.clone();

            yew::platform::spawn_local(async move {
                let _ = fields_query_state.refresh().await;
                selected_item_state.set(Some(idx));
            })
        })
    };
    let on_add_save = {
        let add_name_state = add_name_state.clone();
        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let add_open_state = add_open_state.clone();
        let error_state = error_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let fields_query_state = fields_query_state.clone();

        Callback::from(move |_| {
            let add_name_state = add_name_state.clone();
            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let add_open_state = add_open_state.clone();
            let error_state = error_state.clone();
            let unreported_error_toggle = unreported_error_toggle.clone();

            let fields_query_state = fields_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(
                    match api::create_custom_field((*add_name_state).clone().to_string(), 0).await {
                        Ok(_) => {
                            let _ = fields_query_state.refresh().await;
                            add_open_state.set(false);
                            add_name_state.set("".into());
                            false
                        }
                        Err(err) => {
                            log::error!("Failed to create new custom field {err}");
                            if err.code == CONFLICT {
                                error_message_state
                                    .set("Ein Feld mit dem Namen existiert bereits".into());
                                unreported_error_toggle.set(false);
                            } else {
                                error_message_state.set(
                                    "Beim Erstellen des Felds ist ein Fehler aufgetreten".into(),
                                );
                                error_message_form_state.set("create_custom_field".into());
                                unreported_error_toggle.set(true);
                            }

                            true
                        }
                    },
                );
            })
        })
    };
    let on_add_close = use_callback(
        (add_open_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(false);
            error_state.set(false);
        },
    );

    let update_add_name = use_callback(add_name_state.clone(), |val, state| state.set(val));

    let on_select_item = use_callback(selected_item_state.clone(), |idx, state| {
        state.set(Some(idx))
    });

    match fields_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <CosmoProgressRing />
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded custom fields");
            initial_loaded_state.set(true);
            let mut fields = res.fields.clone();
            fields.sort();
            fields_state.set(fields);
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            if !*initial_loaded_state {
                unreported_error_toggle.set(true);
            }
            initial_loaded_state.set(true);

            return html!(
                if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Laden" message="Deine eigenen Felder konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Deine eigenen Felder konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
                }
            );
        }
    }

    let last_item = if (*fields_state).clone().is_empty() {
        0
    } else {
        (*fields_state).clone().len() - 1
    };

    html!(
        <>
            <CosmoTitle title="Eigene Felder für Charaktere" />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Neues Feld" on_click={on_add_open} />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            <CosmoTabControl selected_index={*selected_item_state} on_select_item={on_select_item}>
                {for (*fields_state).clone().iter().enumerate().map(|(idx, field)| CosmoTabItem::from_label_and_children(field.label.clone().into(), html!(
                    <FieldsTabItem on_move={on_move.clone()} position={field.position} field={field.clone()} on_change={on_change.clone()} is_first={idx == 0} is_last={idx == last_item} />
                )))}
            </CosmoTabControl>
            if *add_open_state {
                <CosmoModal title="Neues Feld hinzufügen" is_form={true} on_form_submit={on_add_save} buttons={html!(
                    <>
                        <CosmoButton on_click={on_add_close} label="Abbrechen" />
                        <CosmoButton label="Feld speichern" is_submit={true} />
                    </>
                )}>
                    if *error_state {
                        if *unreported_error_toggle {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                        } else {
                            <CosmoMessage message_type={CosmoMessageType::Negative} message={(*error_message_state).clone()} />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Name" on_input={update_add_name} value={(*add_name_state).clone()} required={true} />
                    </CosmoInputGroup>
                </CosmoModal>
            }
        </>
    )
}

#[function_component(FreeCompaniesPage)]
fn free_companies() -> Html {
    log::debug!("Render free companies page");
    log::debug!("Initialize state and callbacks");
    let add_open_state = use_bool_toggle(false);
    let edit_open_state = use_bool_toggle(false);
    let delete_open_state = use_bool_toggle(false);
    let unreported_error_toggle = use_bool_toggle(false);

    let selected_id_state = use_state_eq(|| -1);

    let name_state = use_state_eq(|| AttrValue::from(""));
    let selected_name_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let free_companies_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_form_state = error_message_form_state.clone();

        use_async(async move {
            unreported_error_toggle.set(false);

            api::get_free_companies().await.map_err(|err| {
                bamboo_error_state.set(err.clone());
                unreported_error_toggle.set(true);
                error_message_form_state.set("get_free_companies".into());

                err
            })
        })
    };
    let create_state = {
        let name_state = name_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let add_open_state = add_open_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            api::create_free_company(FreeCompany::new((*name_state).to_string()))
                .await
                .map(|_| {
                    add_open_state.set(false);
                    name_state.set("".into());
                    unreported_error_toggle.set(false)
                })
                .map_err(|err| {
                    unreported_error_toggle.set(true);
                    error_message_form_state.set("create_free_company".into());
                    bamboo_error_state.set(err.clone());

                    err
                })
        })
    };
    let edit_state = {
        let name_state = name_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let selected_id_state = selected_id_state.clone();

        let edit_open_state = edit_open_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            api::update_free_company(
                *selected_id_state,
                FreeCompany::new((*name_state).to_string()),
            )
            .await
            .map(|_| {
                edit_open_state.set(false);
                name_state.set("".into());
                unreported_error_toggle.set(false)
            })
            .map_err(|err| {
                unreported_error_toggle.set(true);
                error_message_form_state.set("update_free_company".into());
                bamboo_error_state.set(err.clone());

                err
            })
        })
    };
    let delete_state = {
        let error_message_form_state = error_message_form_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let selected_id_state = selected_id_state.clone();

        let delete_open_state = delete_open_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            api::delete_free_company(*selected_id_state)
                .await
                .map(|_| {
                    delete_open_state.set(false);
                    unreported_error_toggle.set(false)
                })
                .map_err(|err| {
                    delete_open_state.set(false);
                    unreported_error_toggle.set(true);
                    error_message_form_state.set("delete_free_company".into());
                    bamboo_error_state.set(err.clone());

                    err
                })
        })
    };

    {
        let name_state = name_state.clone();

        use_unmount(move || {
            name_state.set("".into());
        })
    }
    {
        let free_companies_state = free_companies_state.clone();

        use_mount(move || {
            free_companies_state.run();
        })
    }

    let report_unknown_error = use_callback(
        (
            bamboo_error_state.clone(),
            error_message_form_state.clone(),
            unreported_error_toggle.clone(),
        ),
        |_, (bamboo_error_state, error_message_form_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "final_fantasy_settings",
                error_message_form_state.deref().to_string(),
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );
    let on_add_open = use_callback(
        (add_open_state.clone(), name_state.clone()),
        |_, (open_state, name_state)| {
            open_state.set(true);
            name_state.set("".into());
        },
    );
    let on_add_close = use_callback(
        (add_open_state.clone(), unreported_error_toggle.clone()),
        |_, (open_state, unreported_error_toggle)| {
            open_state.set(false);
            unreported_error_toggle.set(false);
        },
    );
    let on_add_save = use_callback(create_state.clone(), |_, state| state.run());
    let on_edit_open = use_callback(
        (
            selected_id_state.clone(),
            name_state.clone(),
            edit_open_state.clone(),
        ),
        |(id, name): (i32, AttrValue), (selected_id_state, name_state, open_state)| {
            selected_id_state.set(id);
            name_state.set(name);
            open_state.set(true);
        },
    );
    let on_edit_close = use_callback(
        (edit_open_state.clone(), unreported_error_toggle.clone()),
        |_, (open_state, unreported_error_toggle)| {
            open_state.set(false);
            unreported_error_toggle.set(false);
        },
    );
    let on_edit_save = use_callback(edit_state.clone(), |_, state| state.run());
    let on_delete_open = use_callback(
        (
            selected_id_state.clone(),
            selected_name_state.clone(),
            delete_open_state.clone(),
        ),
        |(id, name), (selected_id_state, selected_name_state, open_state)| {
            selected_id_state.set(id);
            selected_name_state.set(name);
            open_state.set(true);
        },
    );
    let on_delete_close = use_callback(
        (delete_open_state.clone(), unreported_error_toggle.clone()),
        |_, (state, unreported_error_toggle)| {
            state.set(false);
            unreported_error_toggle.set(false);
        },
    );
    let on_delete = use_callback(delete_state.clone(), |_, state| state.run());
    let update_name = use_callback(name_state.clone(), |val, state| state.set(val));

    let list_style = use_style!(
        r#"
display: flex;
flex-flow: row wrap;
gap: 0.125rem;
    "#
    );
    let item_style = use_style!(
        r#"
display: flex;
gap: 0.25rem;
flex: 0 0 100%;
min-width: 100%;
align-items: center;
    "#
    );

    html!(
        <>
            <CosmoTitle title="Freie Gesellschaften" />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Freie Gesellschaft hinzufügen" on_click={on_add_open} />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            if free_companies_state.loading {
                <CosmoProgressRing />
            } else if let Some(data) = &free_companies_state.data {
                <div class={list_style}>
                    {for data.iter().map(|free_company| {
                        let delete_free_company = free_company.clone();
                        let edit_free_company = free_company.clone();

                        let on_delete_open = on_delete_open.clone();
                        let on_edit_open = on_edit_open.clone();

                        html!(
                            <div class={item_style.clone()}>
                                {free_company.name.clone()}
                                <Icon style="cursor: pointer;" width="1rem" height="1rem" icon_id={IconId::LucideEdit} onclick={move |_| on_edit_open.emit((edit_free_company.id, edit_free_company.name.clone().into()))} />
                                <Icon style="cursor: pointer;" width="1rem" height="1rem" icon_id={IconId::LucideTrash} onclick={move |_| on_delete_open.emit((delete_free_company.id, delete_free_company.name.clone().into()))} />
                            </div>
                        )
                    })}
                </div>
            } else if free_companies_state.error.is_some() {
                if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Laden" message="Deine Freien Gesellschaften konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Deine Freien Gesellschaften konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
                }
            }
            if delete_state.error.is_some() {
                if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Laden" message="Die Freie Gesellschaft konnte nicht gelöscht werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Die Freie Gesellschaft konnte nicht gelöscht werden" message_type={CosmoMessageType::Negative} />
                }
            }
            if *edit_open_state {
                <CosmoModal title="Freie Gesellschaft bearbeiten" is_form={true} on_form_submit={on_edit_save} buttons={html!(
                    <>
                        <CosmoButton on_click={on_edit_close} label="Abbrechen" />
                        <CosmoButton label="Freie Gesellschaft speichern" is_submit={true} />
                    </>
                )}>
                    if let Some(err) = &edit_state.error {
                        if err.code == CONFLICT {
                            <CosmoMessage message="Die Freie Gesellschaft existiert bereits" message_type={CosmoMessageType::Negative} />
                        } else if *unreported_error_toggle {
                            <CosmoMessage message="Die Freie Gesellschaft konnte nicht umbenannt werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                        } else {
                            <CosmoMessage message="Die Freie Gesellschaft konnte nicht umbenannt werden" message_type={CosmoMessageType::Negative} />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Name" on_input={update_name.clone()} value={(*name_state).clone()} required={true} />
                    </CosmoInputGroup>
                </CosmoModal>
            }
            if *add_open_state {
                <CosmoModal title="Freie Gesellschaft hinzufügen" is_form={true} on_form_submit={on_add_save} buttons={html!(
                    <>
                        <CosmoButton on_click={on_add_close} label="Abbrechen" />
                        <CosmoButton label="Freie Gesellschaft hinzufügen" is_submit={true} />
                    </>
                )}>
                    if let Some(err) = &create_state.error {
                        if err.code == CONFLICT {
                            <CosmoMessage message="Die Freie Gesellschaft existiert bereits" message_type={CosmoMessageType::Negative} />
                        } else if *unreported_error_toggle {
                            <CosmoMessage message="Die Freie Gesellschaft konnte nicht hinzugefügt werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                        } else {
                            <CosmoMessage message="Die Freie Gesellschaft konnte nicht hinzugefügt werden" message_type={CosmoMessageType::Negative} />
                        }
                    }
                    <CosmoInputGroup>
                        <CosmoTextBox label="Name" on_input={update_name.clone()} value={(*name_state).clone()} required={true} />
                    </CosmoInputGroup>
                </CosmoModal>
            }
            if *delete_open_state {
                <CosmoConfirm confirm_type={CosmoModalType::Warning} title="Freie Gesellschaft löschen" message={format!("Soll die Freie Gesellschaft {} wirklich gelöscht werden?", (*selected_name_state).clone())} confirm_label="Freie Gesellschaft Löschen" decline_label="Nicht löschen"  on_decline={on_delete_close} on_confirm={on_delete} />
            }
        </>
    )
}

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    html!(
        <CosmoSideList>
            <CosmoSideListItem label="Eigene Felder">
                <CustomFieldPage />
            </CosmoSideListItem>
            <CosmoSideListItem label="Freie Gesellschaften">
                <FreeCompaniesPage />
            </CosmoSideListItem>
        </CosmoSideList>
    )
}

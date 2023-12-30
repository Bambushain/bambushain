use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

use gloo_dialogs::alert;
use stylist::yew::use_style;
use yew::prelude::*;
use yew::props;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{
    use_async, use_bool_toggle, use_drag_with_options, use_drop_with_options, use_event, use_list,
    use_mount, UseDragOptions, UseDropOptions,
};
use yew_icons::{get_svg, Icon, IconProps};

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api::CONFLICT;
use bamboo_frontend_base_error as error;

use crate::api;

#[derive(Clone, PartialEq, Eq, Hash)]
enum CustomFieldOptionActionType {
    Add,
    Update(String),
    Delete,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct CustomFieldOptionAction {
    label: String,
    action_type: CustomFieldOptionActionType,
}

#[autoprops]
#[function_component(CustomFieldOptionDialog)]
fn custom_field_options_dialog(
    id: i32,
    options: &Vec<CustomCharacterFieldOption>,
    close: &Callback<()>,
    save: &Callback<()>,
) -> Html {
    let label_state = use_state_eq(|| AttrValue::from(""));
    let edit_label_state = use_state_eq(|| AttrValue::from(""));

    let old_label_ref = use_mut_ref(|| "".to_string());
    let passed_options_ref = use_mut_ref(|| options.clone());

    let options_list = use_list(
        options
            .iter()
            .map(|option| AttrValue::from(option.label.clone()))
            .collect::<Vec<_>>(),
    );

    let option_in_edit_mode = use_state_eq(|| AttrValue::from(""));

    let actions_stack = use_list(Vec::new() as Vec<CustomFieldOptionAction>);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let unreported_error_toggle = use_bool_toggle(false);

    let save_state = {
        let options_list = options_list.clone();

        let passed_options_ref = passed_options_ref.clone();

        let actions_stack = actions_stack.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let save = save.clone();

        use_async(async move {
            let mut failed = false;
            let mut label_id_map = HashMap::new();
            for option in &*passed_options_ref.borrow() {
                label_id_map.insert(option.label.clone(), option.id);
            }

            while let Some(action) = actions_stack.pop() {
                let result = match action.action_type {
                    CustomFieldOptionActionType::Add => {
                        api::add_custom_field_option(id, action.label.clone())
                            .await
                            .map(|res| {
                                label_id_map.insert(action.label.clone(), res.id);
                            })
                    }
                    CustomFieldOptionActionType::Update(old_label) => {
                        if let Some(option_id) = label_id_map.get(&old_label) {
                            api::update_custom_field_option(id, *option_id, action.label.clone())
                                .await
                        } else {
                            Ok(())
                        }
                    }
                    CustomFieldOptionActionType::Delete => {
                        if let Some(option_id) = label_id_map.get(&action.label.clone()) {
                            api::delete_custom_field_option(id, *option_id).await
                        } else {
                            Ok(())
                        }
                    }
                };

                if let Err(err) = result {
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(err.code != CONFLICT);
                    failed = true;
                    break;
                }
            }

            if failed {
                label_id_map.clear();
                if let Ok(options) = api::get_custom_field_options(id).await {
                    *passed_options_ref.borrow_mut() = options.clone();
                    options_list.set(
                        options
                            .iter()
                            .map(|option| AttrValue::from(option.label.clone()))
                            .collect::<Vec<_>>(),
                    );
                    actions_stack.clear();
                } else {
                    alert("Leider ist ein unerwarteter und nicht lösbarer Fehler aufgetreten, bitte lade die Seite neu");
                };

                Err((*bamboo_error_state).clone())
            } else {
                save.emit(());

                Ok(())
            }
        })
    };

    let on_save = use_callback(
        (save_state.clone(), unreported_error_toggle.clone()),
        |_, (state, unreported_error_toggle)| {
            unreported_error_toggle.set(false);
            state.run();
        },
    );
    let update_label = use_callback(label_state.clone(), |value, state| state.set(value));
    let update_edit_label = use_callback(edit_label_state.clone(), |value, state| state.set(value));
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "final_fantasy_settings",
                "update_custom_field_options",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );
    let add_option = use_callback(
        (
            label_state.clone(),
            options_list.clone(),
            actions_stack.clone(),
        ),
        |_, (label_state, options_list, actions_stack)| {
            if !options_list
                .current()
                .iter()
                .any(|f| f == &(**label_state).clone())
            {
                actions_stack.push(CustomFieldOptionAction {
                    label: (**label_state).to_string(),
                    action_type: CustomFieldOptionActionType::Add,
                });
                options_list.push((**label_state).clone());
            }

            label_state.set("".into());
        },
    );
    let update_option = use_callback(
        (
            edit_label_state.clone(),
            old_label_ref.clone(),
            options_list.clone(),
            actions_stack.clone(),
            option_in_edit_mode.clone(),
        ),
        |_, (label_state, old_label_ref, options_list, actions_stack, option_in_edit_mode)| {
            let old_label = (*old_label_ref.borrow()).clone();
            let idx = options_list
                .current()
                .iter()
                .position(|item| item == &AttrValue::from(old_label.clone()));

            if let Some(idx) = idx {
                actions_stack.push(CustomFieldOptionAction {
                    label: (**label_state).to_string(),
                    action_type: CustomFieldOptionActionType::Update(old_label.clone()),
                });
                options_list.update(idx, (**label_state).clone());

                label_state.set("".into());
                option_in_edit_mode.set("".into());
            }
        },
    );
    let delete_option = use_callback(
        (options_list.clone(), actions_stack.clone()),
        |label: AttrValue, (options_list, actions_stack)| {
            actions_stack.push(CustomFieldOptionAction {
                label: label.to_string(),
                action_type: CustomFieldOptionActionType::Delete,
            });
            options_list.retain(|r| r != &label.clone());
        },
    );
    let cancel_edit = use_callback(
        (label_state.clone(), option_in_edit_mode.clone()),
        |_, (label_state, option_in_edit_mode)| {
            label_state.set("".into());
            option_in_edit_mode.set("".into());
        },
    );
    let edit_option = use_callback(
        (
            edit_label_state.clone(),
            old_label_ref.clone(),
            option_in_edit_mode.clone(),
        ),
        |old_label: AttrValue, (edit_label_state, old_label_ref, option_in_edit_mode)| {
            edit_label_state.set(old_label.clone());
            *old_label_ref.borrow_mut() = old_label.to_string();
            option_in_edit_mode.set(old_label.clone());
        },
    );

    let edit_buttons = use_style!(
        r#"
display: flex;
gap: 0.5rem;
"#
    );
    let button_style = use_style!(
        r#"
color: var(--primary-color);
cursor: pointer;
"#
    );
    let option_list_style = use_style!(
        r#"
display: flex;
flex-flow: column;
gap: 0.5rem;
"#
    );
    let option_style = use_style!(
        r#"
display: flex;
justify-content: space-between;
"#
    );

    html!(
        <CosmoModal title="Optionen ändern" is_form={false} buttons={html!(
            <>
                <CosmoButton label="Abbrechen" on_click={close} />
                <CosmoButton label="Optionen speichern" on_click={on_save} />
            </>
        )}>
            if save_state.error.is_some() {
                if *unreported_error_toggle {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Die Optionen konnten nur teilweise gespeichert werden" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Die Optionen konnten nur teilweise gespeichert werden" />
                }
            }
            <div class={option_list_style}>
                {for options_list.current().iter().map(|item| {
                    if *option_in_edit_mode == item.clone() {
                        html!(
                            <CosmoForm on_submit={update_option.clone()} buttons={html!(
                                <>
                                    <CosmoButton label="Abbrechen" on_click={cancel_edit.clone()} />
                                    <CosmoButton label="Option speichern" is_submit={true} />
                                </>
                            )}>
                                <CosmoFieldset title="Option bearbeiten">
                                    <CosmoTextBox width={CosmoInputWidth::Small} label="Name" on_input={update_edit_label.clone()} value={(*edit_label_state).clone()} required={true} />
                                </CosmoFieldset>
                            </CosmoForm>
                        )
                    } else {
                        let delete_option = delete_option.clone();
                        let delete_label = item.clone();
                        let edit_label = item.clone();
                        let edit_option = edit_option.clone();

                        html!(
                            <div class={option_style.clone()}>
                                <span>{item.clone()}</span>
                                <span class={edit_buttons.clone()}>
                                    <Icon class={button_style.clone()} width="1rem" height="1rem" icon_id={IconId::LucidePencil} onclick={move |_| edit_option.emit(edit_label.clone())} />
                                    <Icon class={button_style.clone()} width="1rem" height="1rem" icon_id={IconId::LucideTrash} onclick={move |_| delete_option.emit(delete_label.clone())} />
                                </span>
                            </div>
                        )
                    }
                })}
            </div>
            if (*option_in_edit_mode).eq(&AttrValue::from("")) {
                <CosmoForm on_submit={add_option} buttons={html!(<CosmoButton label="Option hinzufügen" is_submit={true} />)}>
                    <CosmoFieldset title="Neue Option">
                        <CosmoTextBox width={CosmoInputWidth::Small} label="Name" on_input={update_label.clone()} value={(*label_state).clone()} required={true} />
                    </CosmoFieldset>
                </CosmoForm>
            }
        </CosmoModal>
    )
}

#[autoprops]
#[function_component(AddCustomFieldDialog)]
fn add_custom_field_dialog(position: usize, close: &Callback<()>, save: &Callback<()>) -> Html {
    let label_state = use_state_eq(|| AttrValue::from(""));

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let unreported_error_toggle = use_bool_toggle(false);

    let save_state = {
        let save = save.clone();

        let label_state = label_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            api::create_custom_field((*label_state).to_string(), position)
                .await
                .map_err(|err| {
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(err.code != CONFLICT);
                    err
                })
                .map(|_| {
                    unreported_error_toggle.set(false);
                    save.emit(())
                })
        })
    };

    let on_save = use_callback(
        (save_state.clone(), unreported_error_toggle.clone()),
        |_, (state, unreported_error_toggle)| {
            unreported_error_toggle.set(false);
            state.run();
        },
    );
    let update_label = use_callback(label_state.clone(), |value, state| state.set(value));
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "final_fantasy_settings",
                "create_custom_field",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    html!(
        <CosmoModal title="Feld hinzufügen" is_form={true} on_form_submit={on_save} buttons={html!(
            <>
                <CosmoButton on_click={close} label="Abbrechen" />
                <CosmoButton label="Feld hinzufügen" is_submit={true} />
            </>
        )}>
            if let Some(err) = &save_state.error {
                if err.code == CONFLICT {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Ein Feld mit dem Namen existiert bereits" />
                } else if *unreported_error_toggle {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Feld konnte leider nicht hinzugefügt werden" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Feld konnte leider nicht hinzugefügt werden" />
                }
            }
            <CosmoInputGroup>
                <CosmoTextBox label="Name" on_input={update_label.clone()} value={(*label_state).clone()} required={true} />
            </CosmoInputGroup>
        </CosmoModal>
    )
}

#[autoprops]
#[function_component(EditCustomFieldDialog)]
fn edit_custom_field_dialog(
    field: &CustomCharacterField,
    close: &Callback<()>,
    save: &Callback<()>,
) -> Html {
    let label_state = use_state_eq(|| AttrValue::from(field.label.clone()));

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let unreported_error_toggle = use_bool_toggle(false);

    let save_state = {
        let id = field.id;

        let save = save.clone();

        let label_state = label_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            api::update_custom_field(id, (*label_state).to_string())
                .await
                .map_err(|err| {
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(err.code != CONFLICT);
                    err
                })
                .map(|_| {
                    unreported_error_toggle.set(false);
                    save.emit(())
                })
        })
    };

    let on_save = use_callback(
        (save_state.clone(), unreported_error_toggle.clone()),
        |_, (state, unreported_error_toggle)| {
            unreported_error_toggle.set(false);
            state.run();
        },
    );
    let update_label = use_callback(label_state.clone(), |value, state| state.set(value));
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "final_fantasy_settings",
                "update_custom_field",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    html!(
        <CosmoModal title="Feld bearbeiten" is_form={true} on_form_submit={on_save} buttons={html!(
            <>
                <CosmoButton on_click={close} label="Abbrechen" />
                <CosmoButton label="Feld speichern" is_submit={true} />
            </>
        )}>
            if let Some(err) = &save_state.error {
                if err.code == CONFLICT {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Ein Feld mit dem Namen existiert bereits" />
                } else if *unreported_error_toggle {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Feld konnte leider nicht gespeichert werden" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Feld konnte leider nicht gespeichert werden" />
                }
            }
            <CosmoInputGroup>
                <CosmoTextBox label="Name" on_input={update_label.clone()} value={(*label_state).clone()} required={true} />
            </CosmoInputGroup>
        </CosmoModal>
    )
}

#[autoprops]
#[function_component(DraggableItem)]
fn draggable_item(
    custom_field: &CustomCharacterField,
    drag_start: &Callback<i32>,
    edit: &Callback<()>,
    delete: &Callback<(i32, AttrValue)>,
) -> Html {
    let fieldset_style = use_style!(
        r#"
min-width: 0;
padding: 0;
margin: 0;
border: 0;
    "#
    );
    let legend_style = use_style!(
        r#"
font-size: 1.25rem;
height: 1.25rem;
font-weight: var(--font-weight-light);
font-family: var(--font-family-heading);
display: flex;
align-items: center;
justify-content: space-between;
width: 100%;
    "#
    );
    let list_style = use_style!(
        r#"
display: flex;
flex-flow: row wrap;
gap: 0.125rem;
margin: 0;
padding: 0;
    "#
    );
    let item_style = use_style!(
        r#"
display: flex;
gap: 0.25rem;
flex: 0 0 100%;
min-width: 100%;
align-items: center;
margin: 0;
padding: 0;
    "#
    );
    let draggable_item = use_style!(
        r#"
display: grid;
grid-template-columns: 1.5rem 1fr;
align-items: center;
background: var(--modal-backdrop);
backdrop-filter: var(--modal-container-backdrop-filter);
padding: 0.5rem;
border-radius: var(--border-radius);
border: var(--input-border-width) solid var(--control-border-color);
gap: 1rem;
"#
    );
    let edit_buttons = use_style!(
        r#"
display: flex;
gap: 0.5rem;
"#
    );
    let drag_handle_style = use_style!(
        r#"
cursor: move;
"#
    );
    let button_style = use_style!(
        r#"
color: var(--primary-color);
cursor: pointer;
"#
    );

    let edit_open_toggle = use_bool_toggle(false);
    let options_open_toggle = use_bool_toggle(false);
    let allow_drag_toggle = use_bool_toggle(false);

    let node = use_node_ref();
    let handle_node = use_node_ref();

    {
        let allow_drag_toggle = allow_drag_toggle.clone();

        use_event(handle_node.clone(), "mousedown", move |e: MouseEvent| {
            allow_drag_toggle.set(e.button() == 0i16);
        })
    };
    {
        let allow_drag_toggle = allow_drag_toggle.clone();

        use_event(
            handle_node.clone(),
            "pointerdown",
            move |e: PointerEvent| {
                allow_drag_toggle.set(e.button() == 0i16);
            },
        )
    };
    let _ = {
        let allow_drag_toggle = allow_drag_toggle.clone();

        let drag_start = drag_start.clone();

        let id = custom_field.id;

        use_drag_with_options(
            node.clone(),
            UseDragOptions {
                ondragstart: Some(Box::new(move |e| {
                    if !*allow_drag_toggle {
                        e.prevent_default();
                    } else {
                        drag_start.emit(id);
                    }
                })),
                ondragend: Some(Box::new(move |_| {})),
            },
        )
    };

    let open_delete_dialog = use_callback(
        (custom_field.id, custom_field.label.clone(), delete.clone()),
        |_, (id, label, delete)| delete.emit((*id, AttrValue::from(label.clone()))),
    );
    let open_edit_dialog = use_callback(edit_open_toggle.clone(), |_, toggle| toggle.set(true));
    let close_edit_dialog = use_callback(edit_open_toggle.clone(), |_, toggle| toggle.set(false));
    let save_edit_dialog = use_callback(
        (edit_open_toggle.clone(), edit.clone()),
        |_, (toggle, edit)| {
            edit.emit(());
            toggle.set(false)
        },
    );
    let open_options_dialog =
        use_callback(options_open_toggle.clone(), |_, toggle| toggle.set(true));
    let close_options_dialog =
        use_callback(options_open_toggle.clone(), |_, toggle| toggle.set(false));
    let save_options_dialog = use_callback(
        (options_open_toggle.clone(), edit.clone()),
        |_, (toggle, edit)| {
            edit.emit(());
            toggle.set(false)
        },
    );

    let drag_handle = get_svg(&props! {
        IconProps {
            width: "1.5rem",
            height: "1.5rem",
            icon_id: IconId::LucideGripVertical,
        }
    });

    html!(
        <>
            <div ref={node} class={draggable_item.clone()}>
                <div ref={handle_node} class={drag_handle_style.clone()}>{drag_handle.clone()}</div>
                <fieldset class={fieldset_style.clone()}>
                    <legend class={legend_style.clone()}>
                        {custom_field.label.clone()}
                        <div class={edit_buttons.clone()}>
                            <Icon class={button_style.clone()} width="1rem" height="1rem" icon_id={IconId::LucidePencil} onclick={open_edit_dialog} />
                            <Icon class={button_style.clone()} width="1rem" height="1rem" icon_id={IconId::LucideList} onclick={open_options_dialog} />
                            <Icon class={button_style} width="1rem" height="1rem" icon_id={IconId::LucideTrash} onclick={open_delete_dialog} />
                        </div>
                    </legend>
                    <ul class={list_style.clone()}>
                        {for custom_field.options.iter().map(|option| {
                            html!(
                                <li class={item_style.clone()}>{option.label.clone()}</li>
                            )
                        })}
                    </ul>
                </fieldset>
            </div>
            if *edit_open_toggle {
                <EditCustomFieldDialog close={close_edit_dialog} save={save_edit_dialog} field={custom_field.clone()} />
            }
            if *options_open_toggle {
                <CustomFieldOptionDialog id={custom_field.id} options={custom_field.options.clone()} close={close_options_dialog} save={save_options_dialog} />
            }
        </>
    )
}

#[autoprops]
#[function_component(DropZone)]
fn drop_zone(new_position: i32, drop: &Callback<i32>) -> Html {
    let is_over_toggle = use_bool_toggle(false);

    let node = use_node_ref();

    let _ = {
        let enter_is_over_toggle = is_over_toggle.clone();
        let leave_is_over_toggle = is_over_toggle.clone();
        let drop_is_over_toggle = is_over_toggle.clone();

        let drop = drop.clone();

        use_drop_with_options(
            node.clone(),
            UseDropOptions {
                ondragenter: Some(Box::new(move |_| {
                    enter_is_over_toggle.set(true);
                })),
                ondragleave: Some(Box::new(move |_| {
                    leave_is_over_toggle.set(false);
                })),
                ondrop: Some(Box::new(move |_| {
                    drop_is_over_toggle.set(false);
                    drop.emit(new_position);
                })),
                ..Default::default()
            },
        )
    };

    let drop_zone_style = use_style!(
        r#"
height: 1.25rem;
position: relative;
transition: border-left-color 0.1s;
border-left: 0.125rem solid transparent;
border-radius: 0.125rem;

&::before {
    content: '';
    position: absolute;
    border: 0.625rem solid transparent;
    border-right-color: transparent;
    border-bottom-color: transparent;
    border-top-color: transparent;
    transition: border-left-color 0.1s;
}

&::after {
    content: '';
    position: absolute;
    top: 50%;
    width: calc(var(--input-width-medium) - 0.125rem);
    height: 0.125rem;
    transform: translateY(-50%);
    transition: background 0.1s;
}
"#
    );
    let drop_zone_over_style = use_style!(
        r#"
border-left-color: var(--primary-color);

&::before {
    border-left-color: var(--primary-color);
}

&::after {
    background: var(--primary-color);
}
    "#
    );

    let classes = if *is_over_toggle {
        classes!(drop_zone_style, drop_zone_over_style)
    } else {
        classes!(drop_zone_style)
    };

    html!(
        <div ref={node} class={classes}></div>
    )
}

#[allow(clippy::await_holding_refcell_ref)]
#[function_component(CustomFieldsPage)]
pub fn custom_fields_page() -> Html {
    log::debug!("Render custom fields page");
    log::debug!("Initialize state and callbacks");
    let add_open_toggle = use_bool_toggle(false);
    let delete_open_toggle = use_bool_toggle(false);
    let unreported_error_toggle = use_bool_toggle(false);

    let dragged_item_id_ref = use_mut_ref(|| -1);
    let drop_new_position_ref = use_mut_ref(|| -1);
    let delete_id_ref = use_mut_ref(|| -1);

    let error_message_form_state = use_state_eq(|| AttrValue::from(""));
    let selected_label_state = use_state_eq(|| AttrValue::from(""));

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let fields_state = {
        let error_message_form_state = error_message_form_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            api::get_custom_fields()
                .await
                .map_err(|err| {
                    unreported_error_toggle.set(true);
                    error_message_form_state.set("get_custom_fields".into());
                    bamboo_error_state.set(err.clone());

                    err
                })
                .map(|mut data| {
                    data.sort();

                    data
                })
        })
    };
    let delete_state = {
        let error_message_form_state = error_message_form_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let delete_id_ref = delete_id_ref.clone();

        let fields_state = fields_state.clone();

        use_async(async move {
            let id = *delete_id_ref.borrow();

            api::delete_custom_field(id)
                .await
                .map_err(|err| {
                    unreported_error_toggle.set(true);
                    error_message_form_state.set("delete_custom_field".into());
                    bamboo_error_state.set(err.clone());

                    err
                })
                .map(|_| fields_state.run())
        })
    };
    let drop_state = {
        let dragged_item_id_ref = dragged_item_id_ref.clone();
        let drop_new_position_ref = drop_new_position_ref.clone();

        let fields_state = fields_state.clone();

        use_async(async move {
            if let Err(err) = api::move_custom_field(
                *dragged_item_id_ref.borrow(),
                *drop_new_position_ref.borrow(),
            )
            .await
            {
                Err(err)
            } else {
                fields_state.run();
                Ok(())
            }
        })
    };

    {
        let fields_state = fields_state.clone();

        use_mount(move || {
            fields_state.run();
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
    let drag_start = use_callback(dragged_item_id_ref.clone(), |id, mut_ref| {
        *mut_ref.borrow_mut() = id;
    });
    let drop = use_callback(
        (drop_new_position_ref.clone(), drop_state.clone()),
        |new_position, (mut_ref, drop_state)| {
            *mut_ref.borrow_mut() = new_position;
            drop_state.run();
        },
    );
    let open_add_dialog = use_callback(add_open_toggle.clone(), |_, toggle| toggle.set(true));
    let close_add_dialog = use_callback(add_open_toggle.clone(), |_, toggle| toggle.set(false));
    let save_add_dialog = use_callback(
        (add_open_toggle.clone(), fields_state.clone()),
        |_, (toggle, fields_state)| {
            fields_state.run();
            toggle.set(false)
        },
    );
    let open_delete_dialog = use_callback(
        (
            delete_id_ref.clone(),
            selected_label_state.clone(),
            delete_open_toggle.clone(),
        ),
        |(id, label), (id_ref, selected_label_state, toggle)| {
            *id_ref.borrow_mut() = id;
            selected_label_state.set(label);
            toggle.set(true)
        },
    );
    let close_delete_dialog =
        use_callback(delete_open_toggle.clone(), |_, toggle| toggle.set(false));
    let confirm_delete_dialog = use_callback(
        (delete_open_toggle.clone(), delete_state.clone()),
        |_, (toggle, delete_state)| {
            delete_state.run();
            toggle.set(false)
        },
    );
    let edit = use_callback(fields_state.clone(), |_, fields_state| {
        fields_state.run();
    });

    let container_style = use_style!(
        r#"
max-width: var(--input-width-medium);
gap: 0.5rem;
display: grid;
"#
    );

    html!(
        <>
            <CosmoTitle title="Eigene Felder für Charaktere" />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Neues Feld" on_click={open_add_dialog} />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            if fields_state.loading {
                <CosmoProgressRing />
            } else if fields_state.error.is_some() {
                if *unreported_error_toggle {
                    <CosmoMessage header="Fehler beim Laden" message="Deine eigenen Felder konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Deine eigenen Felder konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
                }
            } else if let Some(data) = &fields_state.data {
                if delete_state.error.is_some() {
                    if *unreported_error_toggle {
                        <CosmoMessage header="Fehler beim Laden" message="Das Feld konnte leider nicht gelöscht werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage header="Fehler beim Laden" message="Das Feld konnte leider nicht gelöscht werden" message_type={CosmoMessageType::Negative} />
                    }
                }
                <div class={container_style}>
                    <DropZone drop={drop.clone()} new_position={0} />
                    {for data.iter().map(|field| {
                        html!(
                            <>
                                <DraggableItem drag_start={drag_start.clone()} custom_field={field.clone()} edit={edit.clone()} delete={open_delete_dialog.clone()} />
                                <DropZone drop={drop.clone()} new_position={field.position + 1} />
                            </>
                        )
                    })}
                </div>
                if *add_open_toggle {
                    <AddCustomFieldDialog close={close_add_dialog} save={save_add_dialog} position={data.len()} />
                }
                if *delete_open_toggle {
                    <CosmoConfirm confirm_type={CosmoModalType::Warning} title="Feld löschen" message={format!("Soll das Feld {} wirklich gelöscht werden?", (*selected_label_state).clone())} confirm_label="Feld Löschen" decline_label="Nicht löschen" on_decline={close_delete_dialog} on_confirm={confirm_delete_dialog} />
                }
            }
        </>
    )
}

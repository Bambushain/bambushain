use std::ops::Deref;

use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount, use_unmount};
use yew_icons::Icon;

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{ApiError, CONFLICT};
use bamboo_pandas_frontend_base::error;

use crate::api;

#[function_component(FreeCompaniesPage)]
pub fn free_companies() -> Html {
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

    let bamboo_error_state = use_state_eq(ApiError::default);

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

        let free_companies_state = free_companies_state.clone();

        use_async(async move {
            api::create_free_company(FreeCompany::new((*name_state).to_string()))
                .await
                .map(|_| {
                    free_companies_state.run();
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

        let free_companies_state = free_companies_state.clone();

        use_async(async move {
            api::update_free_company(
                *selected_id_state,
                FreeCompany::new((*name_state).to_string()),
            )
            .await
            .map(|_| {
                free_companies_state.run();
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

        let free_companies_state = free_companies_state.clone();

        use_async(async move {
            api::delete_free_company(*selected_id_state)
                .await
                .map(|_| {
                    free_companies_state.run();
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
                <CosmoConfirm confirm_type={CosmoModalType::Warning} title="Freie Gesellschaft löschen" message={format!("Soll die Freie Gesellschaft {} wirklich gelöscht werden?", (*selected_name_state).clone())} confirm_label="Freie Gesellschaft Löschen" decline_label="Nicht löschen" on_decline={on_delete_close} on_confirm={on_delete} />
            }
        </>
    )
}

use std::ops::Deref;
use std::rc::Rc;

use bounce::query::use_query_value;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::use_unmount;

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api as api;
use bamboo_frontend_base_api::{CONFLICT, NOT_FOUND};
use bamboo_frontend_base_error as error;

use crate::api::*;
use crate::models::*;
use crate::props::crafter::*;

#[derive(PartialEq, Clone)]
enum CrafterActions {
    Edit(Crafter),
    Delete(Crafter),
    Closed,
}

#[function_component(ModifyCrafterModal)]
fn modify_crafter_modal(props: &ModifyCrafterModalProps) -> Html {
    let job_state = use_state_eq(|| {
        AttrValue::from(if props.is_edit {
            props.crafter.job.get_job_name()
        } else {
            props.jobs.first().unwrap().get_job_name()
        })
    });
    let level_state =
        use_state_eq(|| AttrValue::from(props.crafter.level.clone().unwrap_or_default()));

    let on_close = props.on_close.clone();
    let on_save = use_callback(
        (
            job_state.clone(),
            level_state.clone(),
            props.on_save.clone(),
            props.character_id,
        ),
        |_, (job_state, level_state, on_save, character_id)| {
            on_save.emit(Crafter::new(
                *character_id,
                CrafterJob::from((**job_state).clone().to_string()),
                (*level_state).to_string(),
            ))
        },
    );
    let update_job = use_callback(job_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let update_level = use_callback(level_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });

    let jobs = if props.is_edit {
        vec![CosmoModernSelectItem::new(
            props.crafter.job.to_string(),
            props.crafter.job.get_job_name(),
            true,
        )]
    } else {
        props
            .jobs
            .iter()
            .map(|job| {
                CosmoModernSelectItem::new(
                    job.to_string(),
                    job.get_job_name(),
                    (*job_state).clone().eq(&job.get_job_name()),
                )
            })
            .collect::<Vec<CosmoModernSelectItem>>()
    };

    html!(
        <>
            <CosmoModal title={props.title.clone()} is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Abbrechen" />
                    <CosmoButton label={props.save_label.clone()} is_submit={true} />
                </>
            )}>
                if props.has_error {
                    if props.has_unknown_error {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={props.on_error_close.clone()} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} />
                    }
                }
                <CosmoInputGroup>
                    <CosmoModernSelect readonly={props.is_edit} label="Job" on_select={update_job} required={true} items={jobs} />
                    <CosmoTextBox label="Level (optional)" on_input={update_level} value={(*level_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(CrafterDetails)]
pub fn crafter_details(props: &CrafterDetailsProps) -> Html {
    log::debug!("Render crafter details");
    let crafter_query_state = use_query_value::<CrafterForCharacter>(Rc::new(props.character.id));

    let action_state = use_state_eq(|| CrafterActions::Closed);

    let initial_loaded_state = use_state_eq(|| false);
    let open_create_crafter_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let delete_error_state = use_state_eq(|| false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let crafter_state = use_state_eq(|| vec![] as Vec<Crafter>);

    let jobs_state = use_state_eq(|| CrafterJob::iter().collect::<Vec<CrafterJob>>());

    {
        let error_state = error_state.clone();

        use_unmount(move || {
            error_state.set(false);
        })
    }

    let on_modal_create_close = use_callback(
        (open_create_crafter_modal_state.clone(), error_state.clone()),
        |_, (state, error_state)| {
            state.set(false);
            error_state.set(false);
        },
    );

    let report_unknown_error = use_callback(
        (
            bamboo_error_state.clone(),
            error_message_form_state.clone(),
            unknown_error_state.clone(),
        ),
        |_, (bamboo_error_state, error_message_form_state, unknown_error_state)| {
            error::report_unknown_error(
                "final_fantasy_character",
                error_message_form_state.deref().to_string(),
                bamboo_error_state.deref().clone(),
            );
            unknown_error_state.set(false);
        },
    );

    let on_modal_create_save = {
        let error_state = error_state.clone();
        let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let crafter_query_state = crafter_query_state.clone();

        let character_id = props.character.id;

        Callback::from(move |crafter: Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_crafter(character_id, crafter).await {
                    Ok(_) => {
                        open_create_crafter_modal_state.clone().set(false);
                        let _ = crafter_query_state.refresh().await;
                        unknown_error_state.set(false);
                        false
                    }
                    Err(err) => {
                        error_message_state.set(
                            if err.code == CONFLICT {
                                unknown_error_state.set(false);
                                "Ein Crafter mit diesem Job existiert bereits"
                            } else {
                                bamboo_error_state.set(err.clone());
                                error_message_form_state.set("create_crafter".into());
                                unknown_error_state.set(true);
                                "Der Crafter konnte nicht hinzugefügt werden"
                            }
                            .into(),
                        );
                        true
                    }
                });
            });
        })
    };
    let on_modal_update_save = {
        let crafter_query_state = crafter_query_state.clone();

        let on_modal_close = on_modal_create_close.clone();

        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |crafter: Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let action_state = action_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            let id = if let CrafterActions::Edit(crafter) = (*action_state).clone() {
                crafter.id
            } else {
                -1
            };

            yew::platform::spawn_local(async move {
                error_state.set(match update_crafter(character_id, id, crafter).await {
                    Ok(_) => {
                        let _ = crafter_query_state.refresh().await;
                        on_modal_close.emit(());
                        action_state.set(CrafterActions::Closed);
                        unknown_error_state.set(false);
                        false
                    }
                    Err(err) => {
                        match err.code {
                            CONFLICT => {
                                error_message_state
                                    .set("Ein Crafter mit diesem Job existiert bereits".into());
                                unknown_error_state.set(false);
                            }
                            NOT_FOUND => {
                                error_message_state
                                    .set("Der Crafter konnte nicht gefunden werden".into());
                                unknown_error_state.set(false);
                            }
                            _ => {
                                bamboo_error_state.set(err.clone());
                                error_message_form_state.set("update_crafter".into());
                                unknown_error_state.set(true);
                                error_message_state
                                    .set("Der Crafter konnte nicht gespeichert werden".into());
                            }
                        };
                        true
                    }
                });
            })
        })
    };
    let on_modal_delete = {
        let crafter_query_state = crafter_query_state.clone();

        let delete_error_state = delete_error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |id: i32| {
            let crafter_query_state = crafter_query_state.clone();

            let delete_error_state = delete_error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let action_state = action_state.clone();

            yew::platform::spawn_local(async move {
                delete_error_state.set(match delete_crafter(character_id, id).await {
                    Ok(_) => {
                        let _ = crafter_query_state.refresh().await;
                        unknown_error_state.set(false);
                        false
                    }
                    Err(err) => match err.code {
                        NOT_FOUND => {
                            error_message_state
                                .set("Der Crafter konnte nicht gefunden werden".into());
                            unknown_error_state.set(false);
                            true
                        }
                        _ => {
                            error_message_state
                                .set("Der Crafter konnte nicht gelöscht werden".into());
                            bamboo_error_state.set(err.clone());
                            error_message_form_state.set("delete_crafter".into());
                            unknown_error_state.set(true);
                            true
                        }
                    },
                });
                action_state.set(CrafterActions::Closed);
            })
        })
    };
    let on_modal_action_close = use_callback(
        (action_state.clone(), error_state.clone()),
        |_, (state, error_state)| {
            state.set(CrafterActions::Closed);
            error_state.set(false);
        },
    );
    let on_create_open = use_callback(
        (open_create_crafter_modal_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(true);
            error_state.set(false);
        },
    );
    let on_edit_open = use_callback(
        (action_state.clone(), error_state.clone()),
        |crafter, (action_state, error_state)| {
            action_state.set(CrafterActions::Edit(crafter));
            error_state.set(false);
        },
    );
    let on_delete_open = use_callback(
        (action_state.clone(), error_state.clone()),
        |crafter, (action_state, error_state)| {
            action_state.set(CrafterActions::Delete(crafter));
            error_state.set(false);
        },
    );

    match crafter_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <CosmoProgressRing />
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded crafter");
            initial_loaded_state.set(true);
            let mut all_jobs = CrafterJob::iter().collect::<Vec<CrafterJob>>();
            for crafter in res.crafter.clone() {
                let _ = all_jobs
                    .iter()
                    .position(|job| job.eq(&crafter.job))
                    .map(|idx| all_jobs.swap_remove(idx));
            }
            all_jobs.sort();
            jobs_state.set(all_jobs);
            let mut crafter = res.crafter.clone();
            crafter.sort();
            crafter_state.set(crafter);
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            bamboo_error_state.set(err.clone());
            if !*initial_loaded_state {
                unknown_error_state.set(true);
            }
            initial_loaded_state.set(true);

            return html!(
                if *unknown_error_state {
                    <CosmoMessage header="Fehler beim Laden" message="Die Crafter konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Die Crafter konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
                }
            );
        }
    }

    let new_crafter = (*jobs_state).clone().first().map(|job| Crafter {
        job: *job,
        ..Crafter::default()
    });

    html!(
        <>
            if new_crafter.is_some() {
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Crafter hinzufügen" on_click={on_create_open} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
            }
            if *delete_error_state {
                if *unknown_error_state {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} />
                }
            }
            <CosmoTable headers={vec![AttrValue::from(""), AttrValue::from("Job"), AttrValue::from("Level"), AttrValue::from("Aktionen")]}>
                {for (*crafter_state).clone().into_iter().map(|crafter| {
                    let edit_crafter = crafter.clone();
                    let delete_crafter = crafter.clone();

                    let on_edit_open = on_edit_open.clone();
                    let on_delete_open = on_delete_open.clone();

                    CosmoTableRow::from_table_cells(vec![
                        CosmoTableCell::from_html(html!(<img src={format!("/static/crafter_jobs/{}", crafter.job.get_file_name())} />), None),
                        CosmoTableCell::from_html(html!({crafter.job.to_string()}), None),
                        CosmoTableCell::from_html(html!({crafter.level.clone().unwrap_or("".into())}), None),
                        CosmoTableCell::from_html(html!(
                            <>
                                <CosmoToolbarGroup>
                                    <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_crafter.clone())} />
                                    <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_crafter.clone())} />
                                </CosmoToolbarGroup>
                            </>
                        ), None),
                    ], Some(crafter.id.into()))
                })}
            </CosmoTable>
            if *open_create_crafter_modal_state {
                <ModifyCrafterModal on_error_close={report_unknown_error.clone()} has_unknown_error={*unknown_error_state} crafter={new_crafter.unwrap_or(Crafter::default())} character_id={props.character.id} jobs={(*jobs_state).clone()} is_edit={false} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_create_close} title="Crafter hinzufügen" save_label="Crafter hinzufügen" on_save={on_modal_create_save} />
            }
            {match (*action_state).clone() {
                CrafterActions::Edit(crafter) => html!(
                    <ModifyCrafterModal on_error_close={report_unknown_error.clone()} has_unknown_error={*unknown_error_state} character_id={props.character.id} is_edit={true} jobs={(*jobs_state).clone()} title={format!("Crafter {} bearbeiten", crafter.job.to_string())} save_label="Crafter speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} crafter={crafter} error_message={(*error_message_state).clone()} has_error={*error_state} />
                ),
                CrafterActions::Delete(crafter) => html!(
                    <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={move |_| on_modal_delete.emit(crafter.id)} on_decline={on_modal_action_close} confirm_label="Crafter löschen" decline_label="Crafter behalten" title="Crafter löschen" message={format!("Soll der Crafter {} auf Level {} wirklich gelöscht werden?", crafter.job.to_string(), crafter.level.unwrap_or_default())} />
                ),
                CrafterActions::Closed => html!(),
            }}
        </>
    )
}

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
use crate::props::fighter::*;

#[derive(PartialEq, Clone)]
enum FighterActions {
    Edit(Fighter),
    Delete(Fighter),
    Closed,
}

#[function_component(ModifyFighterModal)]
fn modify_fighter_modal(props: &ModifyFighterModalProps) -> Html {
    let job_state = use_state_eq(|| {
        AttrValue::from(if props.is_edit {
            props.fighter.job.get_job_name()
        } else {
            props.jobs.first().unwrap().get_job_name()
        })
    });
    let level_state =
        use_state_eq(|| AttrValue::from(props.fighter.level.clone().unwrap_or_default()));
    let gear_score_state =
        use_state_eq(|| AttrValue::from(props.fighter.gear_score.clone().unwrap_or_default()));

    let on_close = props.on_close.clone();
    let on_save = use_callback(
        (
            job_state.clone(),
            level_state.clone(),
            gear_score_state.clone(),
            props.on_save.clone(),
            props.character_id,
        ),
        |_, (job_state, level_state, gear_score_state, on_save, character_id)| {
            on_save.emit(Fighter::new(
                *character_id,
                FighterJob::from((**job_state).clone().to_string()),
                (*level_state).to_string(),
                (*gear_score_state).to_string(),
            ))
        },
    );
    let update_job = use_callback(job_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let update_level = use_callback(level_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let update_gear_score = use_callback(gear_score_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });

    let jobs = if props.is_edit {
        vec![CosmoModernSelectItem::new(
            props.fighter.job.to_string(),
            props.fighter.job.get_job_name(),
            true,
        )]
    } else {
        props
            .jobs
            .iter()
            .map(|job| {
                log::debug!("Current job state: {}", (*job_state).clone());
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
                    <CosmoTextBox label="Gear Score (optional)" on_input={update_gear_score} value={(*gear_score_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(FighterDetails)]
pub fn fighter_details(props: &FighterDetailsProps) -> Html {
    log::debug!("Render fighter details");
    let fighter_query_state = use_query_value::<FighterForCharacter>(Rc::new(props.character.id));

    let action_state = use_state_eq(|| FighterActions::Closed);

    let initial_loaded_state = use_state_eq(|| false);
    let open_create_fighter_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let delete_error_state = use_state_eq(|| false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let fighter_state = use_state_eq(|| vec![] as Vec<Fighter>);

    let jobs_state = use_state_eq(|| FighterJob::iter().collect::<Vec<FighterJob>>());

    {
        let error_state = error_state.clone();

        use_unmount(move || {
            error_state.set(false);
        })
    }

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

    let on_modal_create_close = use_callback(
        (open_create_fighter_modal_state.clone(), error_state.clone()),
        |_, (state, error_state)| {
            state.set(false);
            error_state.set(false);
        },
    );
    let on_modal_create_save = {
        let error_state = error_state.clone();
        let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let fighter_query_state = fighter_query_state.clone();

        let character_id = props.character.id;

        Callback::from(move |fighter: Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_fighter(character_id, fighter).await {
                    Ok(_) => {
                        open_create_fighter_modal_state.clone().set(false);
                        let _ = fighter_query_state.refresh().await;
                        unknown_error_state.set(false);

                        false
                    }
                    Err(err) => {
                        error_message_state.set(
                            if err.code == CONFLICT {
                                unknown_error_state.set(false);
                                "Ein Kämpfer mit diesem Job existiert bereits"
                            } else {
                                bamboo_error_state.set(err.clone());
                                error_message_form_state.set("create_fighter".into());
                                unknown_error_state.set(true);
                                "Der Kämpfer konnte nicht hinzugefügt werden"
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
        let fighter_query_state = fighter_query_state.clone();

        let on_modal_close = on_modal_create_close.clone();

        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |fighter: Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let action_state = action_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            let id = if let FighterActions::Edit(fighter) = (*action_state).clone() {
                fighter.id
            } else {
                -1
            };

            yew::platform::spawn_local(async move {
                error_state.set(match update_fighter(character_id, id, fighter).await {
                    Ok(_) => {
                        let _ = fighter_query_state.refresh().await;
                        on_modal_close.emit(());
                        action_state.set(FighterActions::Closed);
                        unknown_error_state.set(false);

                        false
                    }
                    Err(err) => {
                        match err.code {
                            CONFLICT => {
                                error_message_state
                                    .set("Ein Kämpfer mit diesem Job existiert bereits".into());
                                unknown_error_state.set(false);
                            }
                            NOT_FOUND => {
                                error_message_state
                                    .set("Der Kämpfer konnte nicht gefunden werden".into());
                                unknown_error_state.set(false);
                            }
                            _ => {
                                bamboo_error_state.set(err.clone());
                                error_message_form_state.set("update_fighter".into());
                                unknown_error_state.set(true);
                                error_message_state
                                    .set("Der Kämpfer konnte nicht gespeichert werden".into());
                            }
                        };
                        true
                    }
                });
            })
        })
    };
    let on_modal_delete = {
        let fighter_query_state = fighter_query_state.clone();

        let delete_error_state = delete_error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |id: i32| {
            let fighter_query_state = fighter_query_state.clone();

            let delete_error_state = delete_error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let action_state = action_state.clone();

            yew::platform::spawn_local(async move {
                delete_error_state.set(match delete_fighter(character_id, id).await {
                    Ok(_) => {
                        let _ = fighter_query_state.refresh().await;
                        unknown_error_state.set(false);

                        false
                    }
                    Err(err) => match err.code {
                        NOT_FOUND => {
                            error_message_state
                                .set("Der Kämpfer konnte nicht gefunden werden".into());
                            unknown_error_state.set(false);

                            true
                        }
                        _ => {
                            bamboo_error_state.set(err.clone());
                            error_message_form_state.set("delete_fighter".into());
                            unknown_error_state.set(true);
                            error_message_state
                                .set("Der Kämpfer konnte nicht gelöscht werden".into());
                            true
                        }
                    },
                });
                action_state.set(FighterActions::Closed);
            })
        })
    };
    let on_modal_action_close = use_callback(
        (action_state.clone(), error_state.clone()),
        |_, (state, error_state)| {
            state.set(FighterActions::Closed);
            error_state.set(false);
        },
    );
    let on_create_open = use_callback(
        (open_create_fighter_modal_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(true);
            error_state.set(false);
        },
    );
    let on_edit_open = use_callback(
        (action_state.clone(), error_state.clone()),
        |fighter, (action_state, error_state)| {
            action_state.set(FighterActions::Edit(fighter));
            error_state.set(false);
        },
    );
    let on_delete_open = use_callback(
        (action_state.clone(), error_state.clone()),
        |fighter, (action_state, error_state)| {
            action_state.set(FighterActions::Delete(fighter));
            error_state.set(false);
        },
    );

    match fighter_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <CosmoProgressRing />
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded fighter");
            initial_loaded_state.set(true);
            let mut all_jobs = FighterJob::iter().collect::<Vec<FighterJob>>();
            for fighter in res.fighter.clone() {
                let _ = all_jobs
                    .iter()
                    .position(|job| job.eq(&fighter.job))
                    .map(|idx| all_jobs.swap_remove(idx));
            }
            all_jobs.sort();
            jobs_state.set(all_jobs);
            let mut fighter = res.fighter.clone();
            fighter.sort();
            fighter_state.set(fighter);
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
                    <CosmoMessage header="Fehler beim Laden" message="Die Kämpfer konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Die Kämpfer konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
                }
            );
        }
    }

    let new_fighter = (*jobs_state).clone().first().map(|job| Fighter {
        job: *job,
        ..Fighter::default()
    });

    html!(
        <>
            if new_fighter.is_some() {
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Kämpfer hinzufügen" on_click={on_create_open} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
            }
            if *delete_error_state {
                if *unknown_error_state {
                    <CosmoMessage header="Fehler beim Löschen" message={(*error_message_state).clone()} message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Löschen" message={(*error_message_state).clone()} message_type={CosmoMessageType::Negative} />
                }
            }
            <CosmoTable headers={vec![AttrValue::from(""), AttrValue::from("Job"), AttrValue::from("Level"), AttrValue::from("Gear Score"), AttrValue::from("Aktionen")]}>
                {for (*fighter_state).clone().into_iter().map(|fighter| {
                    let edit_fighter = fighter.clone();
                    let delete_fighter = fighter.clone();

                    let on_edit_open = on_edit_open.clone();
                    let on_delete_open = on_delete_open.clone();

                    CosmoTableRow::from_table_cells(vec![
                        CosmoTableCell::from_html(html!(<img src={format!("/static/fighter_jobs/{}", fighter.job.get_file_name())} />), None),
                        CosmoTableCell::from_html(html!({fighter.job.to_string()}), None),
                        CosmoTableCell::from_html(html!({fighter.level.clone().unwrap_or("".into())}), None),
                        CosmoTableCell::from_html(html!({fighter.gear_score.clone().unwrap_or("".into())}), None),
                        CosmoTableCell::from_html(html!(
                            <>
                                <CosmoToolbarGroup>
                                    <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_fighter.clone())} />
                                    <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_fighter.clone())} />
                                </CosmoToolbarGroup>
                            </>
                        ), None),
                    ], Some(fighter.id.into()))
                })}
            </CosmoTable>
            if *open_create_fighter_modal_state {
                <ModifyFighterModal has_unknown_error={*unknown_error_state} on_error_close={report_unknown_error.clone()} fighter={new_fighter.unwrap_or(Fighter::default())} character_id={props.character.id} jobs={(*jobs_state).clone()} is_edit={false} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_create_close} title="Kämpfer hinzufügen" save_label="Kämpfer hinzufügen" on_save={on_modal_create_save} />
            }
            {match (*action_state).clone() {
                FighterActions::Edit(fighter) => html!(
                    <ModifyFighterModal has_unknown_error={*unknown_error_state} on_error_close={report_unknown_error.clone()} character_id={props.character.id} is_edit={true} jobs={(*jobs_state).clone()} title={format!("Kämpfer {} bearbeiten", fighter.job.to_string())} save_label="Kämpfer speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} fighter={fighter} error_message={(*error_message_state).clone()} has_error={*error_state} />
                ),
                FighterActions::Delete(fighter) => html!(
                    <>
                        <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={move |_| on_modal_delete.emit(fighter.id)} on_decline={on_modal_action_close} confirm_label="Kämpfer löschen" decline_label="Kämpfer behalten" title="Kämpfer löschen" message={format!("Soll der Kämpfer {} auf Level {} wirklich gelöscht werden?", fighter.job.to_string(), fighter.level.unwrap_or_default())} />
                    </>
                ),
                FighterActions::Closed => html!(),
            }}
        </>
    )
}

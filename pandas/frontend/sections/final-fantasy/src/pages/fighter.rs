use std::ops::Deref;

use strum::IntoEnumIterator;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_effect_update, use_mount};

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{ApiError, CONFLICT, NOT_FOUND};
use bamboo_common::frontend::ui::{BambooCard, BambooCardList};
use bamboo_pandas_frontend_base::error;

use crate::api;

#[derive(PartialEq, Clone)]
enum FighterActions {
    Create,
    Edit(Fighter),
    Delete(Fighter),
    Closed,
}

#[autoprops]
#[function_component(ModifyFighterModal)]
fn modify_fighter_modal(
    on_close: &Callback<()>,
    on_error_close: &Callback<()>,
    title: &AttrValue,
    save_label: &AttrValue,
    error_message: &AttrValue,
    has_error: bool,
    has_unknown_error: bool,
    #[prop_or_default] fighter: &Fighter,
    character_id: i32,
    on_save: &Callback<Fighter>,
    is_edit: bool,
    jobs: &Vec<FighterJob>,
) -> Html {
    let job_state = use_state_eq(|| {
        AttrValue::from(if is_edit {
            fighter.job.get_job_name()
        } else {
            jobs.first().unwrap().get_job_name()
        })
    });
    let level_state = use_state_eq(|| AttrValue::from(fighter.level.clone().unwrap_or_default()));
    let gear_score_state =
        use_state_eq(|| AttrValue::from(fighter.gear_score.clone().unwrap_or_default()));

    let on_close = on_close.clone();
    let on_save = use_callback(
        (
            job_state.clone(),
            level_state.clone(),
            gear_score_state.clone(),
            on_save.clone(),
            character_id,
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

    let jobs = if is_edit {
        vec![CosmoModernSelectItem::new(
            fighter.job.to_string(),
            fighter.job.get_job_name(),
            true,
        )]
    } else {
        jobs.iter()
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
            <CosmoModal
                title={title.clone()}
                is_form=true
                on_form_submit={on_save}
                buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Abbrechen" />
                    <CosmoButton label={save_label.clone()} is_submit={true} />
                </>
            )}
            >
                if has_error {
                    if has_unknown_error {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            message={error_message.clone()}
                            actions={html!(<CosmoButton label="Fehler melden" on_click={on_error_close.clone()} />)}
                        />
                    } else {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            message={error_message.clone()}
                        />
                    }
                }
                <CosmoInputGroup>
                    <CosmoModernSelect
                        readonly={is_edit}
                        label="Job"
                        on_select={update_job}
                        required=true
                        items={jobs}
                    />
                    <CosmoTextBox
                        label="Level (optional)"
                        on_input={update_level}
                        value={(*level_state).clone()}
                    />
                    <CosmoTextBox
                        label="Gear Score (optional)"
                        on_input={update_gear_score}
                        value={(*gear_score_state).clone()}
                    />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[allow(clippy::await_holding_refcell_ref)]
#[autoprops]
#[function_component(FighterDetails)]
pub fn fighter_details(character: &Character) -> Html {
    log::debug!("Render fighter details");
    let action_state = use_state_eq(|| FighterActions::Closed);

    let props_character_id_state = use_state_eq(|| character.id);

    let create_fighter_ref = use_mut_ref(|| None as Option<Fighter>);
    let edit_fighter_ref = use_mut_ref(|| None as Option<Fighter>);
    let edit_id_fighter_ref = use_mut_ref(|| -1);
    let delete_fighter_ref = use_mut_ref(|| None as Option<i32>);

    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(ApiError::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let fighter_state = {
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_form_state = error_message_form_state.clone();

        let character_id = character.id;

        use_async(async move {
            api::get_fighters(character_id).await.map_err(|err| {
                bamboo_error_state.set(err.clone());
                unreported_error_toggle.set(true);
                error_message_form_state.set("get_fighters".into());

                err
            })
        })
    };
    let create_state = {
        let action_state = action_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let fighter_state = fighter_state.clone();

        let character_id = character.id;

        let create_fighter_ref = create_fighter_ref.clone();

        use_async(async move {
            if let Some(fighter) = create_fighter_ref.borrow().clone() {
                api::create_fighter(character_id, fighter)
                    .await
                    .map(|_| {
                        action_state.set(FighterActions::Closed);
                        unreported_error_toggle.set(false);
                        fighter_state.run()
                    })
                    .map_err(|err| {
                        unreported_error_toggle.set(true);
                        error_message_form_state.set("create_fighter".into());
                        bamboo_error_state.set(err.clone());
                        if err.code == CONFLICT {
                            unreported_error_toggle.set(false);
                            error_message_state
                                .set("Ein Kämpfer mit diesem Job existiert bereits".into());
                        } else {
                            unreported_error_toggle.set(true);
                            error_message_state
                                .set("Der Kämpfer konnte nicht hinzugefügt werden".into());
                        }

                        err
                    })
            } else {
                Ok(())
            }
        })
    };
    let update_state = {
        let action_state = action_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let fighter_state = fighter_state.clone();

        let character_id = character.id;

        let edit_fighter_ref = edit_fighter_ref.clone();
        let edit_id_fighter_ref = edit_id_fighter_ref.clone();

        use_async(async move {
            let id = *edit_id_fighter_ref.borrow();

            if let Some(fighter) = edit_fighter_ref.borrow().clone() {
                api::update_fighter(character_id, id, fighter)
                    .await
                    .map(|_| {
                        action_state.set(FighterActions::Closed);
                        unreported_error_toggle.set(false);
                        fighter_state.run()
                    })
                    .map_err(|err| {
                        unreported_error_toggle.set(true);
                        error_message_form_state.set("update_fighter".into());
                        bamboo_error_state.set(err.clone());
                        match err.code {
                            CONFLICT => {
                                unreported_error_toggle.set(false);
                                error_message_state
                                    .set("Ein Kämpfer mit diesem Job existiert bereits".into());
                            }
                            NOT_FOUND => {
                                unreported_error_toggle.set(false);
                                error_message_state
                                    .set("Der Kämpfer konnte nicht gefunden werden".into());
                            }
                            _ => {
                                unreported_error_toggle.set(true);
                                error_message_state
                                    .set("Der Kämpfer konnte nicht gespeichert werden".into());
                            }
                        };

                        err
                    })
            } else {
                Ok(())
            }
        })
    };
    let delete_state = {
        let action_state = action_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_form_state = error_message_form_state.clone();

        let fighter_state = fighter_state.clone();

        let character_id = character.id;

        let delete_fighter_ref = delete_fighter_ref.clone();

        use_async(async move {
            if let Some(fighter) = *delete_fighter_ref.borrow() {
                api::delete_fighter(character_id, fighter)
                    .await
                    .map(|_| {
                        action_state.set(FighterActions::Closed);
                        unreported_error_toggle.set(false);
                        fighter_state.run()
                    })
                    .map_err(|err| {
                        unreported_error_toggle.set(true);
                        error_message_form_state.set("delete_fighter".into());
                        bamboo_error_state.set(err.clone());

                        err
                    })
            } else {
                Ok(())
            }
        })
    };

    let on_modal_create_save = use_callback(
        (create_fighter_ref.clone(), create_state.clone()),
        |fighter, (create_fighter_ref, create_state)| {
            *create_fighter_ref.borrow_mut() = Some(fighter);
            create_state.run();
        },
    );
    let on_modal_update_save = use_callback(
        (edit_fighter_ref.clone(), update_state.clone()),
        |fighter, (edit_fighter_ref, update_state)| {
            *edit_fighter_ref.borrow_mut() = Some(fighter);
            update_state.run();
        },
    );
    let on_modal_delete = use_callback(
        (delete_fighter_ref.clone(), delete_state.clone()),
        |fighter_id, (delete_fighter_ref, delete_state)| {
            *delete_fighter_ref.borrow_mut() = Some(fighter_id);
            delete_state.run();
        },
    );
    let on_modal_action_close = use_callback(
        (action_state.clone(), unreported_error_toggle.clone()),
        |_, (state, unreported_error_toggle)| {
            state.set(FighterActions::Closed);
            unreported_error_toggle.set(false);
        },
    );
    let on_create_open = use_callback(action_state.clone(), |_, action_state| {
        action_state.set(FighterActions::Create);
    });
    let on_edit_open = use_callback(
        (action_state.clone(), edit_id_fighter_ref.clone()),
        |fighter: Fighter, (action_state, edit_id_fighter_ref)| {
            *edit_id_fighter_ref.borrow_mut() = fighter.id;
            action_state.set(FighterActions::Edit(fighter));
        },
    );
    let on_delete_open = use_callback(action_state.clone(), |fighter, action_state| {
        action_state.set(FighterActions::Delete(fighter));
    });
    let report_unknown_error = use_callback(
        (
            bamboo_error_state.clone(),
            error_message_form_state.clone(),
            unreported_error_toggle.clone(),
        ),
        |_, (bamboo_error_state, error_message_form_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "final_fantasy_character",
                error_message_form_state.deref().to_string(),
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    {
        let fighter_state = fighter_state.clone();

        use_mount(move || fighter_state.run());
    }
    {
        let fighter_state = fighter_state.clone();

        let props_character_id_state = props_character_id_state.clone();

        let character = character.clone();

        use_effect_update(move || {
            if *props_character_id_state != character.id {
                fighter_state.run();
                props_character_id_state.set(character.id);
            }

            || ()
        })
    }

    let logo_style = use_style!(
        r#"
position: absolute;
top: 0.75rem;
right: 0.75rem;    
"#
    );

    if fighter_state.loading {
        html!(<CosmoProgressRing />)
    } else if let Some(data) = &fighter_state.data {
        let mut all_jobs = FighterJob::iter().collect::<Vec<FighterJob>>();
        for fighter in data.clone() {
            let _ = all_jobs
                .iter()
                .position(|job| job.eq(&fighter.job))
                .map(|idx| all_jobs.swap_remove(idx));
        }
        let new_fighter = all_jobs.first().map(|job| Fighter {
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
                if let Some(err) = &delete_state.error {
                    if err.code == NOT_FOUND {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            header="Fehler beim Löschen"
                            message="Der Kämpfer konnte nicht gefunden werden"
                        />
                    } else if *unreported_error_toggle {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            header="Fehler beim Löschen"
                            message="Der Kämpfer konnte nicht gelöscht werden"
                            actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)}
                        />
                    } else {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            header="Fehler beim Löschen"
                            message="Der Kämpfer konnte nicht gelöscht werden"
                        />
                    }
                }
                <BambooCardList>
                    { for data.iter().map(|fighter| {
                        let edit_fighter = fighter.clone();
                        let delete_fighter = fighter.clone();

                        let on_edit_open = on_edit_open.clone();
                        let on_delete_open = on_delete_open.clone();

                        html!(
                            <BambooCard title={fighter.job.to_string()} buttons={html!(
                                <>
                                    <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_fighter.clone())} />
                                    <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_fighter.clone())} />
                                </>
                            )}>
                                <img class={logo_style.clone()} src={format!("/pandas/static/fighter_jobs/{}", fighter.job.get_file_name())} />
                                if let Some(level) = fighter.level.clone() {
                                    if level.is_empty() {
                                        <span>{"Kein Level angegeben"}</span><br/>
                                    } else {
                                        <span>{format!("Level {level}")}</span><br/>
                                    }
                                } else {
                                    <span>{"Kein Level angegeben"}</span><br/>
                                }
                                if let Some(gear_score) = fighter.gear_score.clone() {
                                    if gear_score.is_empty() {
                                        <span>{"Kein Gear Score angegeben"}</span><br/>
                                    } else {
                                        <span>{format!("Gear Score {gear_score}")}</span><br/>
                                    }
                                } else {
                                    <span>{"Kein Gear Score angegeben"}</span><br/>
                                }
                            </BambooCard>
                        )
                    }) }
                </BambooCardList>
                { match (*action_state).clone() {
                    FighterActions::Create => html!(
                        <ModifyFighterModal on_error_close={report_unknown_error.clone()} has_unknown_error={*unreported_error_toggle} fighter={new_fighter.unwrap_or(Fighter::default())} character_id={character.id} jobs={all_jobs} is_edit={false} error_message={(*error_message_state).clone()} has_error={create_state.error.is_some()} on_close={on_modal_action_close} title="Kämpfer hinzufügen" save_label="Kämpfer hinzufügen" on_save={on_modal_create_save} />
                    ),
                    FighterActions::Edit(fighter) => html!(
                        <ModifyFighterModal on_error_close={report_unknown_error.clone()} has_unknown_error={*unreported_error_toggle} character_id={character.id} is_edit={true} jobs={FighterJob::iter().collect::<Vec<FighterJob>>()} title={format!("Kämpfer {} bearbeiten", fighter.job.to_string())} save_label="Kämpfer speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} fighter={fighter} error_message={(*error_message_state).clone()} has_error={update_state.error.is_some()} />
                    ),
                    FighterActions::Delete(fighter) => html!(
                        <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={move |_| on_modal_delete.emit(fighter.id)} on_decline={on_modal_action_close} confirm_label="Kämfper löschen" decline_label="Kämpfer behalten" title="Kämpfer löschen" message={format!("Soll der Kämpfer {} auf Level {} wirklich gelöscht werden?", fighter.job.to_string(), fighter.level.unwrap_or_default())} />
                    ),
                    FighterActions::Closed => html!(),
                } }
            </>
        )
    } else if fighter_state.error.is_some() {
        html!(
            if *unreported_error_toggle {
                <CosmoMessage
                    header="Fehler beim Laden"
                    message="Die Kämpfer konnten nicht geladen werden"
                    message_type={CosmoMessageType::Negative}
                    actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)}
                />
            } else {
                <CosmoMessage
                    header="Fehler beim Laden"
                    message="Die Kämpfer konnten nicht geladen werden"
                    message_type={CosmoMessageType::Negative}
                />
            }
        )
    } else {
        html!()
    }
}

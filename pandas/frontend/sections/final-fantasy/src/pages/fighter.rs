use strum::IntoEnumIterator;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_effect_update, use_mount};

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{ApiError, CONFLICT, NOT_FOUND};
use bamboo_common::frontend::ui::{BambooCard, BambooCardList};
use bamboo_pandas_frontend_base::controls::{use_dialogs, BambooErrorMessage};

use crate::api;

#[derive(PartialEq, Clone)]
enum FighterActions {
    Create,
    Edit(Fighter),
    Closed,
}

#[autoprops]
#[function_component(ModifyFighterModal)]
fn modify_fighter_modal(
    on_close: &Callback<()>,
    title: &AttrValue,
    save_label: &AttrValue,
    error_message: &AttrValue,
    has_error: bool,
    api_error: &Option<ApiError>,
    #[prop_or_default] fighter: &Fighter,
    character_id: i32,
    on_save: &Callback<Fighter>,
    is_edit: bool,
    jobs: &Vec<FighterJob>,
) -> Html {
    let job_state = use_state_eq(|| {
        AttrValue::from(
            if is_edit {
                fighter.job
            } else {
                jobs.first().unwrap().clone()
            }
            .get_job_name(),
        )
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
            .collect::<Vec<_>>()
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
                    if let Some(err) = api_error.clone() {
                        <BambooErrorMessage
                            message={error_message.clone()}
                            header="Fehler beim Speichern"
                            page="fighter"
                            form="modify_fighter"
                            error={err}
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
    let delete_fighter_ref = use_mut_ref(|| -1);

    let bamboo_error_state = use_state_eq(|| None as Option<ApiError>);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let dialogs = use_dialogs();

    let fighter_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let character_id = character.id;

        use_async(async move {
            bamboo_error_state.set(None);

            api::get_fighters(character_id).await.map_err(|err| {
                bamboo_error_state.set(Some(err.clone()));

                err
            })
        })
    };
    let create_state = {
        let action_state = action_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();

        let fighter_state = fighter_state.clone();

        let character_id = character.id;

        let create_fighter_ref = create_fighter_ref.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            if let Some(fighter) = create_fighter_ref.borrow().clone() {
                api::create_fighter(character_id, fighter)
                    .await
                    .map(|_| {
                        action_state.set(FighterActions::Closed);
                        fighter_state.run()
                    })
                    .map_err(|err| {
                        if err.code == CONFLICT {
                            error_message_state
                                .set("Ein Kämpfer mit diesem Job existiert bereits".into());
                        } else {
                            bamboo_error_state.set(Some(err.clone()));
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

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();

        let fighter_state = fighter_state.clone();

        let character_id = character.id;

        let edit_fighter_ref = edit_fighter_ref.clone();
        let edit_id_fighter_ref = edit_id_fighter_ref.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            let id = *edit_id_fighter_ref.borrow();

            if let Some(fighter) = edit_fighter_ref.borrow().clone() {
                api::update_fighter(character_id, id, fighter)
                    .await
                    .map(|_| {
                        action_state.set(FighterActions::Closed);
                        fighter_state.run()
                    })
                    .map_err(|err| {
                        match err.code {
                            CONFLICT => {
                                error_message_state
                                    .set("Ein Kämpfer mit diesem Job existiert bereits".into());
                            }
                            NOT_FOUND => {
                                error_message_state
                                    .set("Der Kämpfer konnte nicht gefunden werden".into());
                            }
                            _ => {
                                bamboo_error_state.set(Some(err.clone()));
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
        let bamboo_error_state = bamboo_error_state.clone();

        let fighter_state = fighter_state.clone();

        let character_id = character.id;

        let delete_fighter_ref = delete_fighter_ref.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            let fighter_id = *delete_fighter_ref.borrow();
            api::delete_fighter(character_id, fighter_id)
                .await
                .map(|_| fighter_state.run())
                .map_err(|err| {
                    bamboo_error_state.set(Some(err.clone()));

                    err
                })
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
    let on_modal_action_close = use_callback(action_state.clone(), |_, state| {
        state.set(FighterActions::Closed);
    });
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

    let on_delete = use_callback(delete_state.clone(), |_, delete_state| {
        delete_state.run();
    });
    let on_delete_open = use_callback(
        (
            delete_fighter_ref.clone(),
            on_delete.clone(),
            dialogs.clone(),
        ),
        |fighter: Fighter, (delete_fighter_ref, on_delete, dialogs)| {
            *delete_fighter_ref.borrow_mut() = fighter.id;
            dialogs.confirm(
                "Kämpfer löschen",
                format!(
                    "Soll der Kämpfer {} wirklich gelöscht werden?",
                    fighter.job.to_string()
                ),
                "Kämpfer löschen",
                "Nicht löschen",
                CosmoModalType::Warning,
                on_delete.clone(),
                Callback::noop(),
            )
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
width: 2rem;
height: 2rem;
"#
    );

    if fighter_state.loading {
        html!(<CosmoProgressRing />)
    } else if let Some(data) = &fighter_state.data {
        let mut all_jobs = FighterJob::iter().collect::<Vec<_>>();
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
                if let Some(err) = delete_state.error.clone() {
                    if err.code == NOT_FOUND {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            header="Fehler beim Löschen"
                            message="Der Kämpfer konnte nicht gefunden werden"
                        />
                    } else {
                        <BambooErrorMessage
                            message="Der Kämpfer konnten leider nicht gelöscht werden"
                            header="Fehler beim Löschen"
                            page="fighter_details"
                            form="delete_fighter"
                            error={err}
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
                        <ModifyFighterModal api_error={(*bamboo_error_state).clone()} fighter={new_fighter.unwrap_or(Fighter::default())} character_id={character.id} jobs={all_jobs} is_edit={false} error_message={(*error_message_state).clone()} has_error={create_state.error.is_some()} on_close={on_modal_action_close} title="Kämpfer hinzufügen" save_label="Kämpfer hinzufügen" on_save={on_modal_create_save} />
                    ),
                    FighterActions::Edit(fighter) => html!(
                        <ModifyFighterModal api_error={(*bamboo_error_state).clone()} character_id={character.id} is_edit={true} jobs={FighterJob::iter().collect::<Vec<_>>()} title={format!("Kämpfer {} bearbeiten", fighter.job.to_string())} save_label="Kämpfer speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} fighter={fighter} error_message={(*error_message_state).clone()} has_error={update_state.error.is_some()} />
                    ),
                    FighterActions::Closed => html!(),
                } }
            </>
        )
    } else if let Some(err) = fighter_state.error.clone() {
        html!(
            <BambooErrorMessage
                message="Die Kämpfer konnten leider nicht geladen werden"
                header="Fehler beim Laden"
                page="fighter"
                form="fighter_details"
                error={err}
            />
        )
    } else {
        html!()
    }
}

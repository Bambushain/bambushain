use strum::IntoEnumIterator;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_effect_update, use_mount};

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{ApiError, CONFLICT, NOT_FOUND};
use bamboo_common::frontend::ui::{BambooCard, BambooCardList};
use bamboo_pandas_frontend_base::controls::BambooErrorMessage;

use crate::api;

#[derive(PartialEq, Clone)]
enum CrafterActions {
    Create,
    Edit(Crafter),
    Delete(Crafter),
    Closed,
}

#[autoprops]
#[function_component(ModifyCrafterModal)]
fn modify_crafter_modal(
    on_close: &Callback<()>,
    title: &AttrValue,
    save_label: &AttrValue,
    error_message: &AttrValue,
    has_error: bool,
    api_error: &Option<ApiError>,
    #[prop_or_default] crafter: &Crafter,
    character_id: i32,
    on_save: &Callback<Crafter>,
    is_edit: bool,
    jobs: &Vec<CrafterJob>,
) -> Html {
    let job_state = use_state_eq(|| {
        AttrValue::from(
            if is_edit {
                crafter.job
            } else {
                jobs.first().unwrap().clone()
            }
            .get_job_name(),
        )
    });
    let level_state = use_state_eq(|| AttrValue::from(crafter.level.clone().unwrap_or_default()));

    let on_close = on_close.clone();
    let on_save = use_callback(
        (
            job_state.clone(),
            level_state.clone(),
            on_save.clone(),
            character_id,
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

    let jobs = if is_edit {
        vec![CosmoModernSelectItem::new(
            crafter.job.to_string(),
            crafter.job.get_job_name(),
            true,
        )]
    } else {
        jobs.iter()
            .map(|job| {
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
                            page="crafter"
                            form="modify_crafter"
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
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[allow(clippy::await_holding_refcell_ref)]
#[autoprops]
#[function_component(CrafterDetails)]
pub fn crafter_details(character: &Character) -> Html {
    log::debug!("Render crafter details");
    let action_state = use_state_eq(|| CrafterActions::Closed);

    let props_character_id_state = use_state_eq(|| character.id);

    let create_crafter_ref = use_mut_ref(|| None as Option<Crafter>);
    let edit_crafter_ref = use_mut_ref(|| None as Option<Crafter>);
    let edit_id_crafter_ref = use_mut_ref(|| -1);
    let delete_crafter_ref = use_mut_ref(|| None as Option<i32>);

    let bamboo_error_state = use_state_eq(|| None as Option<ApiError>);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let crafter_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let character_id = character.id;

        use_async(async move {
            bamboo_error_state.set(None);

            api::get_crafters(character_id).await.map_err(|err| {
                bamboo_error_state.set(Some(err.clone()));

                err
            })
        })
    };
    let create_state = {
        let action_state = action_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();

        let crafter_state = crafter_state.clone();

        let character_id = character.id;

        let create_crafter_ref = create_crafter_ref.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            if let Some(crafter) = create_crafter_ref.borrow().clone() {
                api::create_crafter(character_id, crafter)
                    .await
                    .map(|_| {
                        action_state.set(CrafterActions::Closed);
                        crafter_state.run()
                    })
                    .map_err(|err| {
                        if err.code == CONFLICT {
                            error_message_state
                                .set("Ein Handwerker mit diesem Job existiert bereits".into());
                        } else {
                            bamboo_error_state.set(Some(err.clone()));
                            error_message_state
                                .set("Der Handwerker konnte nicht hinzugefügt werden".into());
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

        let crafter_state = crafter_state.clone();

        let character_id = character.id;

        let edit_crafter_ref = edit_crafter_ref.clone();
        let edit_id_crafter_ref = edit_id_crafter_ref.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            let id = *edit_id_crafter_ref.borrow();
            if let Some(crafter) = edit_crafter_ref.borrow().clone() {
                api::update_crafter(character_id, id, crafter)
                    .await
                    .map(|_| {
                        action_state.set(CrafterActions::Closed);
                        crafter_state.run()
                    })
                    .map_err(|err| {
                        match err.code {
                            CONFLICT => {
                                error_message_state
                                    .set("Ein Handwerker mit diesem Job existiert bereits".into());
                            }
                            NOT_FOUND => {
                                error_message_state
                                    .set("Der Handwerker konnte nicht gefunden werden".into());
                            }
                            _ => {
                                bamboo_error_state.set(Some(err.clone()));
                                error_message_state
                                    .set("Der Handwerker konnte nicht gespeichert werden".into());
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

        let bamboo_error_state = bamboo_error_state.clone();

        let crafter_state = crafter_state.clone();

        let character_id = character.id;

        let delete_crafter_ref = delete_crafter_ref.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            if let Some(crafter) = *delete_crafter_ref.borrow() {
                api::delete_crafter(character_id, crafter)
                    .await
                    .map(|_| {
                        action_state.set(CrafterActions::Closed);
                        crafter_state.run()
                    })
                    .map_err(|err| {
                        bamboo_error_state.set(Some(err.clone()));

                        err
                    })
            } else {
                Ok(())
            }
        })
    };

    let on_modal_create_save = use_callback(
        (create_crafter_ref.clone(), create_state.clone()),
        |crafter, (create_crafter_ref, create_state)| {
            *create_crafter_ref.borrow_mut() = Some(crafter);
            create_state.run();
        },
    );
    let on_modal_update_save = use_callback(
        (edit_crafter_ref.clone(), update_state.clone()),
        |crafter, (edit_crafter_ref, update_state)| {
            *edit_crafter_ref.borrow_mut() = Some(crafter);
            update_state.run();
        },
    );
    let on_modal_delete = use_callback(
        (delete_crafter_ref.clone(), delete_state.clone()),
        |crafter_id, (delete_crafter_ref, delete_state)| {
            *delete_crafter_ref.borrow_mut() = Some(crafter_id);
            delete_state.run();
        },
    );
    let on_modal_action_close = use_callback(action_state.clone(), |_, state| {
        state.set(CrafterActions::Closed);
    });
    let on_create_open = use_callback(action_state.clone(), |_, action_state| {
        action_state.set(CrafterActions::Create);
    });
    let on_edit_open = use_callback(
        (action_state.clone(), edit_id_crafter_ref.clone()),
        |crafter: Crafter, (action_state, edit_id_crafter_ref)| {
            *edit_id_crafter_ref.borrow_mut() = crafter.id;
            action_state.set(CrafterActions::Edit(crafter));
        },
    );
    let on_delete_open = use_callback(action_state.clone(), |crafter, action_state| {
        action_state.set(CrafterActions::Delete(crafter));
    });

    {
        let crafter_state = crafter_state.clone();

        use_mount(move || crafter_state.run());
    }
    {
        let crafter_state = crafter_state.clone();

        let props_character_id_state = props_character_id_state.clone();

        let character = character.clone();

        use_effect_update(move || {
            if *props_character_id_state != character.id {
                crafter_state.run();
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

    if crafter_state.loading {
        html!(<CosmoProgressRing />)
    } else if let Some(data) = &crafter_state.data {
        let mut all_jobs = CrafterJob::iter().collect::<Vec<_>>();
        for crafter in data.clone() {
            let _ = all_jobs
                .iter()
                .position(|job| job.eq(&crafter.job))
                .map(|idx| all_jobs.swap_remove(idx));
        }
        let new_crafter = all_jobs.first().map(|job| Crafter {
            job: *job,
            ..Crafter::default()
        });

        html!(
            <>
                if new_crafter.is_some() {
                    <CosmoToolbar>
                        <CosmoToolbarGroup>
                            <CosmoButton label="Handwerker hinzufügen" on_click={on_create_open} />
                        </CosmoToolbarGroup>
                    </CosmoToolbar>
                }
                if let Some(err) = delete_state.error.clone() {
                    if err.code == NOT_FOUND {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            header="Fehler beim Löschen"
                            message="Der Handwerker konnte nicht gefunden werden"
                        />
                    } else {
                        <BambooErrorMessage
                            message="Der Handwerker konnten leider nicht gelöscht werden"
                            header="Fehler beim Löschen"
                            page="fighter_details"
                            form="delete_fighter"
                            error={err}
                        />
                    }
                }
                <BambooCardList>
                    { for data.iter().map(|crafter| {
                        let edit_crafter = crafter.clone();
                        let delete_crafter = crafter.clone();

                        let on_edit_open = on_edit_open.clone();
                        let on_delete_open = on_delete_open.clone();

                        html!(
                            <BambooCard title={crafter.job.to_string()} buttons={html!(
                                <>
                                    <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_crafter.clone())} />
                                    <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_crafter.clone())} />
                                </>
                            )}>
                                <img class={logo_style.clone()} src={format!("/pandas/static/crafter_jobs/{}", crafter.job.get_file_name())} />
                                if let Some(level) = crafter.level.clone() {
                                    if level.is_empty() {
                                        <span>{"Kein Level angegeben"}</span><br/>
                                    } else {
                                        <span>{format!("Level {level}")}</span><br/>
                                    }
                                } else {
                                    <span>{"Kein Level angegeben"}</span><br/>
                                }
                            </BambooCard>
                        )
                    }) }
                </BambooCardList>
                { match (*action_state).clone() {
                    CrafterActions::Create => html!(
                        <ModifyCrafterModal api_error={(*bamboo_error_state).clone()} crafter={new_crafter.unwrap_or(Crafter::default())} character_id={character.id} jobs={all_jobs} is_edit={false} error_message={(*error_message_state).clone()} has_error={create_state.error.is_some()} on_close={on_modal_action_close} title="Handwerker hinzufügen" save_label="Handwerker hinzufügen" on_save={on_modal_create_save} />
                    ),
                    CrafterActions::Edit(crafter) => html!(
                        <ModifyCrafterModal api_error={(*bamboo_error_state).clone()} character_id={character.id} is_edit={true} jobs={CrafterJob::iter().collect::<Vec<_>>()} title={format!("Handwerker {} bearbeiten", crafter.job.to_string())} save_label="Handwerker speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} crafter={crafter} error_message={(*error_message_state).clone()} has_error={update_state.error.is_some()} />
                    ),
                    CrafterActions::Delete(crafter) => html!(
                        <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={move |_| on_modal_delete.emit(crafter.id)} on_decline={on_modal_action_close} confirm_label="Handwerker löschen" decline_label="Handwerker behalten" title="Handwerker löschen" message={format!("Soll der Handwerker {} auf Level {} wirklich gelöscht werden?", crafter.job.to_string(), crafter.level.unwrap_or_default())} />
                    ),
                    CrafterActions::Closed => html!(),
                } }
            </>
        )
    } else if let Some(err) = crafter_state.error.clone() {
        html!(
            <BambooErrorMessage
                message="Die Handwerker konnten leider nicht geladen werden"
                header="Fehler beim Laden"
                page="crafter"
                form="crafter_details"
                error={err}
            />
        )
    } else {
        html!()
    }
}

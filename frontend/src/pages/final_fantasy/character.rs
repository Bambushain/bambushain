use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew::virtual_dom::{Key, VChild};
use yew_cosmo::prelude::*;
use yew_hooks::use_mount;

use pandaparty_entities::prelude::*;

use crate::api::character::MyCharacters;
use crate::api::free_company::get_free_companies;
use crate::api::*;

#[derive(Properties, PartialEq, Clone)]
struct ModifyCharacterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    #[prop_or_default]
    character: Character,
    on_save: Callback<Character>,
    on_error_close: Callback<()>,
    custom_fields: Vec<CustomCharacterField>,
    free_companies: Vec<FreeCompany>,
}

#[derive(Properties, PartialEq, Clone)]
struct CharacterDetailsProps {
    character: Character,
    on_delete: Callback<()>,
    custom_fields: Vec<CustomCharacterField>,
    free_companies: Vec<FreeCompany>,
}

#[derive(Properties, PartialEq, Clone)]
struct CrafterDetailsProps {
    character: Character,
}

#[derive(Properties, PartialEq, Clone)]
struct FighterDetailsProps {
    character: Character,
}

#[derive(Properties, PartialEq, Clone)]
struct ModifyCrafterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    #[prop_or_default]
    crafter: Crafter,
    character_id: i32,
    on_save: Callback<Crafter>,
    is_edit: bool,
    jobs: Vec<CrafterJob>,
}

#[derive(Properties, PartialEq, Clone)]
struct ModifyFighterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    #[prop_or_default]
    fighter: Fighter,
    character_id: i32,
    on_save: Callback<Fighter>,
    is_edit: bool,
    jobs: Vec<FighterJob>,
}

#[derive(PartialEq, Clone)]
enum CrafterActions {
    Edit(Crafter),
    Delete(Crafter),
    Closed,
}

#[derive(PartialEq, Clone)]
enum FighterActions {
    Edit(Fighter),
    Delete(Fighter),
    Closed,
}

#[derive(PartialEq, Clone)]
enum CharacterActions {
    Edit,
    Delete,
    Closed,
}

#[derive(PartialEq, Clone)]
enum ErrorState {
    Edit,
    Delete,
    None,
}

#[function_component(ModifyCharacterModal)]
fn modify_character_modal(props: &ModifyCharacterModalProps) -> Html {
    let race_state = use_state_eq(|| Some(AttrValue::from(props.character.race.get_race_name())));
    let world_state = use_state_eq(|| AttrValue::from(props.character.world.clone()));
    let name_state = use_state_eq(|| AttrValue::from(props.character.name.clone()));
    let custom_fields_state = use_state_eq(|| {
        let character_fields = props.character.custom_fields.clone();
        let mut custom_fields = HashMap::new();
        for character_field in character_fields {
            custom_fields.insert(
                AttrValue::from(character_field.label),
                character_field
                    .values
                    .iter()
                    .map(|val| AttrValue::from(val.clone()))
                    .collect::<HashSet<AttrValue>>(),
            );
        }

        custom_fields
    });
    let free_company_state = use_state_eq(|| {
        if let Some(free_company) = props.character.free_company.clone() {
            Some(AttrValue::from(free_company.id.to_string()))
        } else {
            None
        }
    });

    let on_close = props.on_close.clone();
    let on_save = use_callback(
        |_,
         (
            race_state,
            world_state,
            name_state,
            custom_fields_state,
            free_company_state,
            free_companies,
            on_save,
        )| {
            let mut custom_fields = vec![];
            for (label, values) in (**custom_fields_state).clone() {
                custom_fields.push(CustomField {
                    label: label.to_string(),
                    values: values
                        .iter()
                        .map(|val| val.to_string())
                        .collect::<BTreeSet<String>>(),
                    position: 0,
                })
            }

            let free_company = if let Some(id) = (**free_company_state).clone() {
                free_companies.iter().find_map(|company| {
                    if id == company.id.to_string() {
                        Some(company.clone())
                    } else {
                        None
                    }
                })
            } else {
                None
            };

            let character = Character::new(
                CharacterRace::from((**race_state).clone().unwrap().to_string()),
                (**name_state).to_string(),
                (**world_state).to_string(),
                custom_fields,
                free_company,
            );
            on_save.emit(character);
        },
        (
            race_state.clone(),
            world_state.clone(),
            name_state.clone(),
            custom_fields_state.clone(),
            free_company_state.clone(),
            props.free_companies.clone(),
            props.on_save.clone(),
        ),
    );

    let update_race = use_callback(
        |value: Option<AttrValue>, state| state.set(value),
        race_state.clone(),
    );
    let update_world = use_callback(
        |value: AttrValue, state| state.set(value),
        world_state.clone(),
    );
    let update_name = use_callback(
        |value: AttrValue, state| state.set(value),
        name_state.clone(),
    );
    let custom_field_select = use_callback(
        |(label, value): (AttrValue, AttrValue), state| {
            let mut map = (**state).clone();
            let mut set = if let Some(set) = map.get(&label) {
                set.clone()
            } else {
                HashSet::new()
            };
            set.insert(value);
            map.insert(label, set);

            state.set(map);
        },
        custom_fields_state.clone(),
    );
    let custom_field_deselect = use_callback(
        |(label, value): (AttrValue, AttrValue), state| {
            let mut map = (**state).clone();
            let mut set = if let Some(set) = map.get(&label) {
                set.clone()
            } else {
                HashSet::new()
            };
            set.remove(&value);
            map.insert(label, set);

            state.set(map);
        },
        custom_fields_state.clone(),
    );
    let update_free_company = use_callback(
        |value: Option<AttrValue>, state| state.set(value),
        free_company_state.clone(),
    );

    let mut all_races = CharacterRace::iter().collect::<Vec<CharacterRace>>();
    all_races.sort();

    let races = all_races
        .iter()
        .map(|race| {
            (
                Some(AttrValue::from(race.get_race_name())),
                AttrValue::from(race.to_string()),
            )
        })
        .collect::<Vec<(Option<AttrValue>, AttrValue)>>();

    let mut all_free_companies = props.free_companies.clone();
    all_free_companies.sort();

    let mut free_companies = vec![(None, AttrValue::from("Keine Freie Gesellschaft"))];
    free_companies.append(
        all_free_companies
            .iter()
            .map(|free_company| {
                (
                    Some(AttrValue::from(free_company.id.to_string())),
                    AttrValue::from(free_company.name.clone()),
                )
            })
            .collect::<Vec<(Option<AttrValue>, AttrValue)>>()
            .as_mut(),
    );

    let mut custom_field_inputs = vec![];
    for field in props.custom_fields.clone() {
        let state = (*custom_fields_state).clone();
        let values = if let Some(values) = state.get(&AttrValue::from(field.label.clone())) {
            values.clone()
        } else {
            HashSet::new()
        };

        let on_select = custom_field_select.clone();
        let on_deselect = custom_field_deselect.clone();

        let on_select_label = field.label.clone();
        let on_deselect_label = field.label.clone();
        let items = field
            .options
            .clone()
            .iter()
            .map(|option| {
                let item = AttrValue::from(option.label.clone());
                CosmoModernSelectItem {
                    label: item.clone(),
                    value: item.clone(),
                    selected: values.contains(&item),
                }
            })
            .collect::<Vec<CosmoModernSelectItem>>();
        let custom_field = VChild::<CosmoModernSelect>::new(
            CosmoModernSelectProps {
                label: field.label.clone().into(),
                id: None,
                on_select: Callback::from(move |val| {
                    on_select.emit((on_select_label.clone().into(), val));
                }),
                on_deselect: Some(Callback::from(move |val| {
                    on_deselect.emit((on_deselect_label.clone().into(), val));
                })),
                on_filter: None,
                required: false,
                readonly: false,
                width: CosmoInputWidth::Full,
                items,
            },
            Some(Key::from(field.label.clone())),
        );
        custom_field_inputs.push(custom_field);
    }

    html!(
        <CosmoModal title={props.title.clone()} is_form={true} on_form_submit={on_save} buttons={html!(
            <>
                <CosmoButton on_click={on_close} label="Abbrechen" />
                <CosmoButton label={props.save_label.clone()} is_submit={true} />
            </>
        )}>
            if props.has_error {
                <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} />
            }
            <CosmoInputGroup>
                <CosmoTextBox label="Name" on_input={update_name} value={(*name_state).clone()} required={true} />
                <CosmoDropdown label="Rasse" on_select={update_race} value={(*race_state).clone()} required={true} items={races} />
                <CosmoTextBox label="Welt" on_input={update_world} value={(*world_state).clone()} required={true} />
                <CosmoDropdown label="Freie Gesellschaft" on_select={update_free_company} value={(*free_company_state).clone()} required={true} items={free_companies} />
                {for custom_field_inputs}
            </CosmoInputGroup>
        </CosmoModal>
    )
}

#[function_component(ModifyCrafterModal)]
fn modify_crafter_modal(props: &ModifyCrafterModalProps) -> Html {
    let job_state = use_state_eq(|| Some(AttrValue::from(props.crafter.job.get_job_name())));
    let level_state =
        use_state_eq(|| AttrValue::from(props.crafter.level.clone().unwrap_or_default()));

    let on_close = props.on_close.clone();
    let on_save = use_callback(
        |_, (job_state, level_state, on_save, character_id)| {
            on_save.emit(Crafter::new(
                *character_id,
                CrafterJob::from((**job_state).clone().unwrap().to_string()),
                (*level_state).to_string(),
            ))
        },
        (
            job_state.clone(),
            level_state.clone(),
            props.on_save.clone(),
            props.character_id,
        ),
    );
    let update_job = use_callback(
        |value: Option<AttrValue>, state| state.set(value),
        job_state.clone(),
    );
    let update_level = use_callback(
        |value: AttrValue, state| state.set(value),
        level_state.clone(),
    );

    let jobs = if props.is_edit {
        vec![(
            Some(AttrValue::from(props.crafter.job.get_job_name())),
            AttrValue::from(props.crafter.job.to_string()),
        )]
    } else {
        props
            .jobs
            .iter()
            .map(|job| {
                (
                    Some(AttrValue::from(job.get_job_name())),
                    AttrValue::from(job.to_string()),
                )
            })
            .collect::<Vec<(Option<AttrValue>, AttrValue)>>()
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
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} />
                }
                <CosmoInputGroup>
                    <CosmoDropdown readonly={props.is_edit} label="Job" on_select={update_job} value={(*job_state).clone()} required={true} items={jobs} />
                    <CosmoTextBox label="Level (optional)" on_input={update_level} value={(*level_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(ModifyFighterModal)]
fn modify_fighter_modal(props: &ModifyFighterModalProps) -> Html {
    let job_state = use_state_eq(|| Some(AttrValue::from(props.fighter.job.get_job_name())));
    let level_state =
        use_state_eq(|| AttrValue::from(props.fighter.level.clone().unwrap_or_default()));
    let gear_score_state =
        use_state_eq(|| AttrValue::from(props.fighter.gear_score.clone().unwrap_or_default()));

    let on_close = props.on_close.clone();
    let on_save = use_callback(
        |_, (job_state, level_state, gear_score_state, on_save, character_id)| {
            on_save.emit(Fighter::new(
                *character_id,
                FighterJob::from((**job_state).clone().unwrap().to_string()),
                (*level_state).to_string(),
                (*gear_score_state).to_string(),
            ))
        },
        (
            job_state.clone(),
            level_state.clone(),
            gear_score_state.clone(),
            props.on_save.clone(),
            props.character_id,
        ),
    );
    let update_job = use_callback(
        |value: Option<AttrValue>, state| state.set(value),
        job_state.clone(),
    );
    let update_level = use_callback(
        |value: AttrValue, state| state.set(value),
        level_state.clone(),
    );
    let update_gear_score = use_callback(
        |value: AttrValue, state| state.set(value),
        gear_score_state.clone(),
    );

    let jobs = if props.is_edit {
        vec![(
            Some(AttrValue::from(props.fighter.job.get_job_name())),
            AttrValue::from(props.fighter.job.to_string()),
        )]
    } else {
        props
            .jobs
            .iter()
            .map(|job| {
                (
                    Some(AttrValue::from(job.get_job_name())),
                    AttrValue::from(job.to_string()),
                )
            })
            .collect::<Vec<(Option<AttrValue>, AttrValue)>>()
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
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} />
                }
                <CosmoInputGroup>
                    <CosmoDropdown readonly={props.is_edit} label="Job" on_select={update_job} value={(*job_state).clone()} required={true} items={jobs} />
                    <CosmoTextBox label="Level (optional)" on_input={update_level} value={(*level_state).clone()} />
                    <CosmoTextBox label="Gear Score (optional)" on_input={update_gear_score} value={(*gear_score_state).clone()} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(CharacterDetails)]
fn character_details(props: &CharacterDetailsProps) -> Html {
    log::debug!("Initialize character details state and callbacks");
    let action_state = use_state_eq(|| CharacterActions::Closed);

    let error_state = use_state_eq(|| ErrorState::None);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let character_query_state = use_query_value::<MyCharacters>(().into());

    let edit_character_click = use_callback(
        |_, state| state.set(CharacterActions::Edit),
        action_state.clone(),
    );
    let delete_character_click = use_callback(
        |_, state| state.set(CharacterActions::Delete),
        action_state.clone(),
    );

    let on_modal_close = {
        let action_state = action_state.clone();

        let error_state = error_state.clone();

        let character_query_state = character_query_state.clone();

        Callback::from(move |_| {
            let action_state = action_state.clone();

            let error_state = error_state.clone();

            let character_query_state = character_query_state.clone();

            yew::platform::spawn_local(async move {
                action_state.set(CharacterActions::Closed);
                error_state.set(ErrorState::None);

                let _ = character_query_state.refresh().await;
            });
        })
    };
    let on_modal_delete = {
        let action_state = action_state.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();

        let character_query_state = character_query_state.clone();

        let on_delete = props.on_delete.clone();

        let id = props.character.id;

        Callback::from(move |_| {
            log::debug!("Modal was confirmed lets execute the request");

            let error_state = error_state.clone();

            let action_state = action_state.clone();

            let error_message_state = error_message_state.clone();

            let character_query_state = character_query_state.clone();

            let on_delete = on_delete.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_character(id).await {
                    Ok(_) => {
                        on_delete.emit(());
                        ErrorState::None
                    }
                    Err(err) => {
                        match err.code {
                            NOT_FOUND => {
                                error_message_state.set("Der Charakter konnte nicht gefunden werden".into());
                                ErrorState::Delete
                            }
                            _ => {
                                error_message_state.set("Der Charakter konnte nicht gelöscht werden, bitte wende dich an Azami".into());
                                ErrorState::Delete
                            }
                        }
                    }
                });
                action_state.set(CharacterActions::Closed);

                let _ = character_query_state.refresh().await;
            });
        })
    };
    let on_modal_save = {
        let on_modal_close = on_modal_close.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();

        let action_state = action_state.clone();

        let id = props.character.id;

        Callback::from(move |character: Character| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();

            let action_state = action_state.clone();

            let character_query_state = character_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match update_character(id, character).await {
                    Ok(_) => {
                        let _ = character_query_state.refresh().await;
                        on_modal_close.emit(());
                        action_state.set(CharacterActions::Closed);
                        ErrorState::None
                    }
                    Err(err) => {
                        match err.code {
                            CONFLICT => {
                                error_message_state.set("Ein Charakter mit diesem Namen existiert bereits".into());
                                ErrorState::Edit
                            }
                            NOT_FOUND => {
                                error_message_state.set("Der Charakter konnte nicht gefunden werden".into());
                                ErrorState::Edit
                            }
                            _ => {
                                error_message_state.set("Der Charakter konnte nicht gespeichert werden, bitte wende dich an Azami".into());
                                ErrorState::None
                            }
                        }
                    }
                });
            })
        })
    };
    let on_error_close = use_callback(|_, state| state.set(ErrorState::None), error_state.clone());

    html!(
        <>
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton on_click={edit_character_click} label="Bearbeiten" />
                    <CosmoButton on_click={delete_character_click} label="Löschen" />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            <CosmoKeyValueList>
                <CosmoKeyValueListItem title="Name">{props.character.name.clone()}</CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Rasse">{props.character.race.to_string()}</CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Welt">{props.character.world.clone()}</CosmoKeyValueListItem>
                if let Some(free_company) = props.character.free_company.clone() {
                    <CosmoKeyValueListItem title="Freie Gesellschaft">{free_company.name.clone()}</CosmoKeyValueListItem>
                }
                {for props.character.custom_fields.clone().iter().map(|field| {
                    html!(
                        <CosmoKeyValueListItem title={field.label.clone()}>{field.values.clone().into_iter().collect::<Vec<String>>().join(", ")}</CosmoKeyValueListItem>
                    )
                })}
            </CosmoKeyValueList>
            {match (*action_state).clone() {
                CharacterActions::Edit => html!(
                    <ModifyCharacterModal free_companies={props.free_companies.clone()} on_error_close={on_error_close.clone()} title={format!("Charakter {} bearbeiten", props.character.name.clone())} save_label="Character speichern" on_save={on_modal_save} on_close={on_modal_close} character={props.character.clone()} custom_fields={props.custom_fields.clone()} error_message={(*error_message_state).clone()} has_error={*error_state == ErrorState::Edit} />
                ),
                CharacterActions::Delete => {
                    let character = props.character.clone();
                    html!(
                        <CosmoConfirm on_confirm={on_modal_delete} on_decline={on_modal_close} confirm_label="Character löschen" decline_label="Character behalten" title="Character löschen" message={format!("Soll der Character {} wirklich gelöscht werden?", character.name)} />
                    )
                }
                CharacterActions::Closed => html!(),
            }}
            if *error_state == ErrorState::Delete {
                <CosmoAlert alert_type={CosmoAlertType::Negative} close_label="Schließen" title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={on_error_close} />
            }
        </>
    )
}

#[function_component(CrafterDetails)]
fn crafter_details(props: &CrafterDetailsProps) -> Html {
    log::debug!("Render crafter details");
    let crafter_query_state = use_query_value::<CrafterForCharacter>(Rc::new(props.character.id));

    let action_state = use_state_eq(|| CrafterActions::Closed);

    let initial_loaded_state = use_state_eq(|| false);
    let open_create_crafter_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let crafter_state = use_state_eq(|| vec![] as Vec<Crafter>);

    let jobs_state = use_state_eq(|| CrafterJob::iter().collect::<Vec<CrafterJob>>());

    let on_modal_create_close = use_callback(
        |_, (state, error_state)| {
            state.set(false);
            error_state.set(false);
        },
        (open_create_crafter_modal_state.clone(), error_state.clone()),
    );
    let on_modal_create_save = {
        let error_state = error_state.clone();
        let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();

        let error_message_state = error_message_state.clone();

        let crafter_query_state = crafter_query_state.clone();

        let character_id = props.character.id;

        Callback::from(move |crafter: Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();

            let error_message_state = error_message_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_crafter(character_id, crafter).await {
                    Ok(_) => {
                        open_create_crafter_modal_state.clone().set(false);
                        let _ = crafter_query_state.refresh().await;
                        false
                    }
                    Err(err) => {
                        error_message_state.set(if err.code == CONFLICT {
                            "Ein Crafter mit diesem Job existiert bereits"
                        } else {
                            "Der Crafter konnte nicht hinzugefügt werden, bitte wende dich an Azami"
                        }.into());
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

        let error_message_state = error_message_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |crafter: Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();

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
                        false
                    }
                    Err(err) => {
                        match err.code {
                            CONFLICT => {
                                error_message_state.set("Ein Crafter mit diesem Job existiert bereits".into());
                            }
                            NOT_FOUND => {
                                error_message_state.set("Der Crafter konnte nicht gefunden werden".into());
                            }
                            _ => {
                                error_message_state.set("Der Crafter konnte nicht gespeichert werden, bitte wende dich an Azami".into());
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

        let error_message_state = error_message_state.clone();

        let error_state = error_state.clone();

        let character_id = props.character.id;

        Callback::from(move |id: i32| {
            let crafter_query_state = crafter_query_state.clone();

            let error_message_state = error_message_state.clone();

            let error_state = error_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_crafter(character_id, id).await {
                    Ok(_) => {
                        let _ = crafter_query_state.refresh().await;
                        false
                    }
                    Err(err) => {
                        match err.code {
                            NOT_FOUND => {
                                error_message_state.set("Der Crafter konnte nicht gefunden werden".into());
                                true
                            }
                            _ => {
                                error_message_state.set("Der Crafter konnte nicht gelöscht werden, bitte wende dich an Azami".into());
                                true
                            }
                        }
                    }
                })
            })
        })
    };
    let on_modal_action_close = use_callback(
        |_, (state, error_state)| {
            state.set(CrafterActions::Closed);
            error_state.set(false);
        },
        (action_state.clone(), error_state.clone()),
    );
    let on_error_close = use_callback(|_, state| state.set(false), error_state.clone());
    let on_create_open = use_callback(
        |_, state| state.set(true),
        open_create_crafter_modal_state.clone(),
    );
    let on_edit_open = use_callback(
        |crafter, action_state| action_state.set(CrafterActions::Edit(crafter)),
        action_state.clone(),
    );
    let on_delete_open = use_callback(
        |crafter, action_state| action_state.set(CrafterActions::Delete(crafter)),
        action_state.clone(),
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
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Crafter konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    let new_crafter = (*jobs_state).clone().first().map(|job| Crafter {
        job: *job,
        ..Crafter::default()
    });

    html!(
        <>
            <CosmoHeader level={CosmoHeaderLevel::H3} header={format!("{}s Crafter", props.character.name.clone())} />
            if new_crafter.is_some() {
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Crafter hinzufügen" on_click={on_create_open} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
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
                                <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_crafter.clone())} />
                                <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_crafter.clone())} />
                            </>
                        ), None),
                    ], Some(crafter.id.into()))
                })}
            </CosmoTable>
            {match (*action_state).clone() {
                CrafterActions::Edit(crafter) => html!(
                    <ModifyCrafterModal character_id={props.character.id} is_edit={true} jobs={(*jobs_state).clone()} title={format!("Crafter {} bearbeiten", crafter.job.to_string())} save_label="Crafter speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} crafter={crafter} error_message={(*error_message_state).clone()} has_error={*error_state} />
                ),
                CrafterActions::Delete(crafter) => html!(
                    <>
                        <CosmoConfirm on_confirm={move |_| on_modal_delete.emit(crafter.id)} on_decline={on_modal_action_close} confirm_label="Crafter löschen" decline_label="Crafter behalten" title="Crafter löschen" message={format!("Soll der Crafter {} auf Level {} wirklich gelöscht werden?", crafter.job.to_string(), crafter.level.unwrap_or_default())} />
                        if *error_state {
                            <CosmoAlert alert_type={CosmoAlertType::Negative} close_label="Schließen" title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={on_error_close} />
                        }
                    </>
                ),
                CrafterActions::Closed => html!(),
            }}
            if *open_create_crafter_modal_state {
                <ModifyCrafterModal crafter={new_crafter.unwrap_or(Crafter::default())} character_id={props.character.id} jobs={(*jobs_state).clone()} is_edit={false} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_create_close} title="Crafter hinzufügen" save_label="Crafter hinzufügen" on_save={on_modal_create_save} />
            }
        </>
    )
}

#[function_component(FighterDetails)]
fn fighter_details(props: &FighterDetailsProps) -> Html {
    log::debug!("Render fighter details");
    let fighter_query_state = use_query_value::<FighterForCharacter>(Rc::new(props.character.id));

    let action_state = use_state_eq(|| FighterActions::Closed);

    let initial_loaded_state = use_state_eq(|| false);
    let open_create_fighter_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let fighter_state = use_state_eq(|| vec![] as Vec<Fighter>);

    let jobs_state = use_state_eq(|| FighterJob::iter().collect::<Vec<FighterJob>>());

    let on_modal_create_close = use_callback(
        |_, (state, error_state)| {
            state.set(false);
            error_state.set(false);
        },
        (open_create_fighter_modal_state.clone(), error_state.clone()),
    );
    let on_modal_create_save = {
        let error_state = error_state.clone();
        let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();

        let error_message_state = error_message_state.clone();

        let fighter_query_state = fighter_query_state.clone();

        let character_id = props.character.id;

        Callback::from(move |fighter: Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();

            let error_message_state = error_message_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_fighter(character_id, fighter).await {
                    Ok(_) => {
                        open_create_fighter_modal_state.clone().set(false);
                        let _ = fighter_query_state.refresh().await;
                        false
                    }
                    Err(err) => {
                        error_message_state.set(if err.code == CONFLICT {
                            "Ein Kämpfer mit diesem Job existiert bereits"
                        } else {
                            "Der Kämpfer konnte nicht hinzugefügt werden, bitte wende dich an Azami"
                        }.into());
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

        let error_message_state = error_message_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |fighter: Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();

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
                        false
                    }
                    Err(err) => {
                        match err.code {
                            CONFLICT => {
                                error_message_state.set("Ein Kämpfer mit diesem Job existiert bereits".into());
                            }
                            NOT_FOUND => {
                                error_message_state.set("Der Kämpfer konnte nicht gefunden werden".into());
                            }
                            _ => {
                                error_message_state.set("Der Kämpfer konnte nicht gespeichert werden, bitte wende dich an Azami".into());
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

        let error_message_state = error_message_state.clone();

        let error_state = error_state.clone();

        let character_id = props.character.id;

        Callback::from(move |id: i32| {
            let fighter_query_state = fighter_query_state.clone();

            let error_message_state = error_message_state.clone();

            let error_state = error_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_fighter(character_id, id).await {
                    Ok(_) => {
                        let _ = fighter_query_state.refresh().await;
                        false
                    }
                    Err(err) => {
                        match err.code {
                            NOT_FOUND => {
                                error_message_state.set("Der Kämpfer konnte nicht gefunden werden".into());
                                true
                            }
                            _ => {
                                error_message_state.set("Der Kämpfer konnte nicht gelöscht werden, bitte wende dich an Azami".into());
                                true
                            }
                        }
                    }
                })
            })
        })
    };
    let on_modal_action_close = use_callback(
        |_, (state, error_state)| {
            state.set(FighterActions::Closed);
            error_state.set(false);
        },
        (action_state.clone(), error_state.clone()),
    );
    let on_error_close = use_callback(|_, state| state.set(false), error_state.clone());
    let on_create_open = use_callback(
        |_, state| state.set(true),
        open_create_fighter_modal_state.clone(),
    );
    let on_edit_open = use_callback(
        |fighter, action_state| action_state.set(FighterActions::Edit(fighter)),
        action_state.clone(),
    );
    let on_delete_open = use_callback(
        |fighter, action_state| action_state.set(FighterActions::Delete(fighter)),
        action_state.clone(),
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
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Kämpfer konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    let new_fighter = (*jobs_state).clone().first().map(|job| Fighter {
        job: *job,
        ..Fighter::default()
    });

    html!(
        <>
            <CosmoHeader level={CosmoHeaderLevel::H3} header={format!("{}s Kämpfer", props.character.name.clone())} />
            if new_fighter.is_some() {
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Kämpfer hinzufügen" on_click={on_create_open} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
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
                                <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_fighter.clone())} />
                                <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_fighter.clone())} />
                            </>
                        ), None),
                    ], Some(fighter.id.into()))
                })}
            </CosmoTable>
            {match (*action_state).clone() {
                FighterActions::Edit(fighter) => html!(
                    <ModifyFighterModal character_id={props.character.id} is_edit={true} jobs={(*jobs_state).clone()} title={format!("Kämpfer {} bearbeiten", fighter.job.to_string())} save_label="Kämpfer speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} fighter={fighter} error_message={(*error_message_state).clone()} has_error={*error_state} />
                ),
                FighterActions::Delete(fighter) => html!(
                    <>
                        <CosmoConfirm on_confirm={move |_| on_modal_delete.emit(fighter.id)} on_decline={on_modal_action_close} confirm_label="Kämpfer löschen" decline_label="Kämpfer behalten" title="Kämpfer löschen" message={format!("Soll der Kämpfer {} auf Level {} wirklich gelöscht werden?", fighter.job.to_string(), fighter.level.unwrap_or_default())} />
                        if *error_state {
                            <CosmoAlert alert_type={CosmoAlertType::Negative} close_label="Schließen" title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={on_error_close} />
                        }
                    </>
                ),
                FighterActions::Closed => html!(),
            }}
            if *open_create_fighter_modal_state {
                <ModifyFighterModal fighter={new_fighter.unwrap_or(Fighter::default())} character_id={props.character.id} jobs={(*jobs_state).clone()} is_edit={false} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_create_close} title="Kämpfer hinzufügen" save_label="Kämpfer hinzufügen" on_save={on_modal_create_save} />
            }
        </>
    )
}

#[function_component(CharacterPage)]
pub fn character_page() -> Html {
    log::debug!("Render character page");
    log::debug!("Initialize state and callbacks");
    let character_query_state = use_query_value::<MyCharacters>(().into());

    let open_create_character_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let initial_loaded_state = use_state_eq(|| false);

    let character_state = use_state_eq(|| vec![] as Vec<Character>);

    let free_companies_state = use_state_eq(|| vec![] as Vec<FreeCompany>);

    let custom_fields_state = use_state_eq(|| vec![] as Vec<CustomCharacterField>);

    let selected_character_state = use_state_eq(|| 0);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let open_create_character_modal_click = use_callback(
        |_, open_create_character_modal_state| open_create_character_modal_state.set(true),
        open_create_character_modal_state.clone(),
    );
    let on_character_select = use_callback(
        |idx, state| state.set(idx),
        selected_character_state.clone(),
    );
    let on_modal_close = use_callback(
        |_, state| state.set(false),
        open_create_character_modal_state.clone(),
    );
    let on_modal_save = {
        let error_state = error_state.clone();
        let open_create_character_modal_state = open_create_character_modal_state.clone();

        let error_message_state = error_message_state.clone();

        let character_query_state = character_query_state.clone();

        let selected_character_state = selected_character_state.clone();

        Callback::from(move |character: Character| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_character_modal_state = open_create_character_modal_state.clone();

            let selected_character_state = selected_character_state.clone();
            let error_message_state = error_message_state.clone();

            let character_query_state = character_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_character(character).await {
                    Ok(character) => {
                        let id = character.id;
                        open_create_character_modal_state.clone().set(false);
                        if let Ok(res) = character_query_state.refresh().await {
                            selected_character_state.set(res.character.iter().position(move |character| character.id.eq(&id)).unwrap_or(0));
                        }
                        false
                    }
                    Err(err) => {
                        error_message_state.set(if err.code == CONFLICT {
                            "Ein Charakter mit diesem Namen existiert bereits"
                        } else {
                            "Der Charakter konnte nicht hinzugefügt werden, bitte wende dich an Azami"
                        }.into());
                        true
                    }
                });
            });
        })
    };
    let on_delete = {
        let character_query_state = character_query_state.clone();

        let selected_character_state = selected_character_state.clone();

        Callback::from(move |_| {
            let character_query_state = character_query_state.clone();

            let selected_character_state = selected_character_state.clone();

            yew::platform::spawn_local(async move {
                let _ = character_query_state.refresh().await;
                selected_character_state.set(0);
            })
        })
    };
    let on_error_close = use_callback(|_, state| state.set(false), error_state.clone());

    {
        let custom_fields_state = custom_fields_state.clone();

        let free_companies_state = free_companies_state.clone();

        use_mount(move || {
            let custom_fields_state = custom_fields_state;

            let free_companies_state = free_companies_state;

            yew::platform::spawn_local(async move {
                if let Ok(custom_fields) = get_custom_fields().await {
                    custom_fields_state.set(custom_fields);
                }
                if let Ok(free_companies) = get_free_companies().await {
                    free_companies_state.set(free_companies);
                }
            });
        });
    }

    match character_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <CosmoProgressRing />
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded character");
            initial_loaded_state.set(true);
            character_state.set(res.character.clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Deine Charaktere konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Meine Charaktere"}</title>
            </Helmet>
            <CosmoSideList on_select_item={on_character_select} selected_index={*selected_character_state} has_add_button={true} add_button_on_click={open_create_character_modal_click} add_button_label="Charakter hinzufügen">
                {for (*character_state).clone().into_iter().map(|character| {
                    CosmoSideListItem::from_label_and_children(character.name.clone().into(), html!(
                        <>
                            <CosmoTitle title={character.name.clone()} />
                            <CosmoTabControl>
                                <CosmoTabItem label="Details">
                                    <CharacterDetails free_companies={(*free_companies_state).clone()} custom_fields={(*custom_fields_state).clone()} on_delete={on_delete.clone()} character={character.clone()} />
                                </CosmoTabItem>
                                <CosmoTabItem label="Kämpfer">
                                    <FighterDetails character={character.clone()} />
                                </CosmoTabItem>
                                <CosmoTabItem label="Crafter">
                                    <CrafterDetails character={character} />
                                </CosmoTabItem>
                            </CosmoTabControl>
                        </>
                    ))
                })}
            </CosmoSideList>
            if *open_create_character_modal_state {
                <ModifyCharacterModal free_companies={(*free_companies_state).clone()} on_error_close={on_error_close} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_close} title="Charakter hinzufügen" save_label="Charakter hinzufügen" on_save={on_modal_save} custom_fields={(*custom_fields_state).clone()} />
            }
        </>
    )
}

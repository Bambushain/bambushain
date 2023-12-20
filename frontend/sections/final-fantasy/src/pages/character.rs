use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use strum::IntoEnumIterator;
use stylist::yew::use_style;
use yew::prelude::*;
use yew::virtual_dom::{Key, VChild};
use yew_cosmo::prelude::*;
use yew_hooks::{use_mount, use_unmount};

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api as api;
use bamboo_frontend_base_api::{CONFLICT, NOT_FOUND};
use bamboo_frontend_base_error as error;

use crate::api::*;
use crate::models::*;

#[derive(Properties, PartialEq, Clone)]
struct ModifyCharacterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    has_unknown_error: bool,
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
struct HousingDetailsProps {
    character: Character,
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
struct ModifyHousingModalProps {
    on_close: Callback<()>,
    on_error_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    has_unknown_error: bool,
    #[prop_or_default]
    housing: CharacterHousing,
    character_id: i32,
    on_save: Callback<CharacterHousing>,
    is_edit: bool,
}

#[derive(Properties, PartialEq, Clone)]
struct ModifyCrafterModalProps {
    on_close: Callback<()>,
    on_error_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    has_unknown_error: bool,
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
    on_error_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    has_unknown_error: bool,
    #[prop_or_default]
    fighter: Fighter,
    character_id: i32,
    on_save: Callback<Fighter>,
    is_edit: bool,
    jobs: Vec<FighterJob>,
}

#[derive(PartialEq, Clone)]
enum HousingActions {
    Edit(CharacterHousing),
    Delete(CharacterHousing),
    Closed,
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
    let race_state = use_state_eq(|| AttrValue::from(props.character.race.get_race_name()));
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
        (
            race_state.clone(),
            world_state.clone(),
            name_state.clone(),
            custom_fields_state.clone(),
            free_company_state.clone(),
            props.free_companies.clone(),
            props.on_save.clone(),
        ),
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
                CharacterRace::from((**race_state).clone().to_string()),
                (**name_state).to_string(),
                (**world_state).to_string(),
                custom_fields,
                free_company,
            );
            on_save.emit(character);
        },
    );

    let update_race = use_callback(race_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let update_world = use_callback(world_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let update_name = use_callback(name_state.clone(), |value: AttrValue, state| {
        state.set(value)
    });
    let custom_field_select = use_callback(
        custom_fields_state.clone(),
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
    );
    let custom_field_deselect = use_callback(
        custom_fields_state.clone(),
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
    );
    let update_free_company =
        use_callback(free_company_state.clone(), |value: AttrValue, state| {
            state.set(if !value.is_empty() { Some(value) } else { None })
        });

    let mut all_races = CharacterRace::iter().collect::<Vec<CharacterRace>>();
    all_races.sort();

    let races = all_races
        .iter()
        .map(|race| {
            CosmoModernSelectItem::new(
                AttrValue::from(race.to_string()),
                AttrValue::from(race.get_race_name()),
                (*race_state).clone().eq(&race.get_race_name()),
            )
        })
        .collect::<Vec<CosmoModernSelectItem>>();

    let mut all_free_companies = props.free_companies.clone();
    all_free_companies.sort();

    let mut free_companies = vec![CosmoModernSelectItem::new(
        "Keine Freie Gesellschaft",
        "",
        (*free_company_state).clone().is_none(),
    )];
    free_companies.append(
        all_free_companies
            .iter()
            .map(|free_company| {
                let selected = if let Some(value) = (*free_company_state).clone() {
                    value.clone().eq(&free_company.id.to_string())
                } else {
                    false
                };

                log::debug!("Name: {}", free_company.name.clone());
                log::debug!("Id: {}", free_company.id.clone());
                log::debug!("Selected: {}", selected);

                CosmoModernSelectItem::new(
                    free_company.name.clone(),
                    free_company.id.to_string(),
                    selected,
                )
            })
            .collect::<Vec<CosmoModernSelectItem>>()
            .as_mut(),
    );

    log::debug!("Found {} free companies", free_companies.len());

    let mut custom_field_inputs = vec![];
    let mut fields = props.custom_fields.clone();
    fields.sort();
    for field in fields {
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
                if props.has_unknown_error {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={props.on_error_close.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} />
                }
            }
            <CosmoInputGroup>
                <CosmoTextBox label="Name" on_input={update_name} value={(*name_state).clone()} required={true} />
                <CosmoModernSelect label="Rasse" on_select={update_race} required={true} items={races} />
                <CosmoTextBox label="Welt" on_input={update_world} value={(*world_state).clone()} required={true} />
                <CosmoModernSelect label="Freie Gesellschaft" on_select={update_free_company} required={true} items={free_companies} />
                {for custom_field_inputs}
            </CosmoInputGroup>
        </CosmoModal>
    )
}

#[function_component(ModifyHousingModal)]
fn modify_housing_modal(props: &ModifyHousingModalProps) -> Html {
    let district_state = use_state_eq(|| props.housing.district);
    let ward_state = use_state_eq(|| props.housing.ward);
    let plot_state = use_state_eq(|| props.housing.plot);

    let districts = HousingDistrict::iter()
        .map(|district| {
            CosmoModernSelectItem::new(
                district.to_string(),
                district.get_name(),
                (*district_state).clone().eq(&district),
            )
        })
        .collect::<Vec<CosmoModernSelectItem>>();
    let wards = (1..31i16)
        .map(|ward| {
            CosmoModernSelectItem::new(
                ward.to_string(),
                ward.to_string(),
                (*ward_state).clone().eq(&ward),
            )
        })
        .collect::<Vec<CosmoModernSelectItem>>();
    let plots = (1..61i16)
        .map(|plot| {
            CosmoModernSelectItem::new(
                plot.to_string(),
                plot.to_string(),
                (*plot_state).clone().eq(&plot),
            )
        })
        .collect::<Vec<CosmoModernSelectItem>>();

    let on_close = props.on_close.clone();
    let on_save = use_callback(
        (
            district_state.clone(),
            ward_state.clone(),
            plot_state.clone(),
            props.on_save.clone(),
            props.character_id,
        ),
        |_, (district_state, ward_state, plot_state, on_save, character_id)| {
            on_save.emit(CharacterHousing::new(
                *character_id,
                *(*district_state).clone(),
                *(*ward_state).clone(),
                *(*plot_state).clone(),
            ))
        },
    );

    let update_district = use_callback(district_state.clone(), |value: AttrValue, state| {
        state.set(HousingDistrict::from(value.to_string()))
    });
    let update_ward = use_callback(ward_state.clone(), |value: AttrValue, state| {
        state.set(value.to_string().as_str().parse::<i16>().unwrap())
    });
    let update_plot = use_callback(plot_state.clone(), |value: AttrValue, state| {
        state.set(value.to_string().as_str().parse::<i16>().unwrap())
    });

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
                    <CosmoModernSelect label="Gebiet" on_select={update_district} required={true} items={districts} />
                    <CosmoModernSelect label="Bezirk" on_select={update_ward} required={true} items={wards} />
                    <CosmoModernSelect label="Nummer" on_select={update_plot} required={true} items={plots} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
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

#[function_component(CharacterDetails)]
fn character_details(props: &CharacterDetailsProps) -> Html {
    log::debug!("Initialize character details state and callbacks");
    let action_state = use_state_eq(|| CharacterActions::Closed);

    let error_state = use_state_eq(|| ErrorState::None);

    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let character_query_state = use_query_value::<MyCharacters>(().into());

    {
        let error_state = error_state.clone();

        use_unmount(move || {
            error_state.set(ErrorState::None);
        })
    }

    let edit_character_click = use_callback(
        (action_state.clone(), error_state.clone()),
        |_, (state, error_state)| {
            state.set(CharacterActions::Edit);
            error_state.set(ErrorState::None);
        },
    );
    let delete_character_click = use_callback(
        (action_state.clone(), error_state.clone()),
        |_, (state, error_state)| {
            state.set(CharacterActions::Delete);
            error_state.set(ErrorState::None);
        },
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
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let character_query_state = character_query_state.clone();

        let on_delete = props.on_delete.clone();

        let id = props.character.id;

        Callback::from(move |_| {
            log::debug!("Modal was confirmed lets execute the request");

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let action_state = action_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let character_query_state = character_query_state.clone();

            let on_delete = on_delete.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_character(id).await {
                    Ok(_) => {
                        on_delete.emit(());
                        ErrorState::None
                    }
                    Err(err) => match err.code {
                        NOT_FOUND => {
                            error_message_state
                                .set("Der Charakter konnte nicht gefunden werden".into());
                            unknown_error_state.set(false);

                            ErrorState::Delete
                        }
                        _ => {
                            error_message_state
                                .set("Der Charakter konnte nicht gelöscht werden".into());
                            unknown_error_state.set(true);
                            error_message_form_state.set("delete_character".into());
                            bamboo_error_state.set(err.clone());

                            ErrorState::Delete
                        }
                    },
                });
                action_state.set(CharacterActions::Closed);

                let _ = character_query_state.refresh().await;
            });
        })
    };
    let on_modal_save = {
        let on_modal_close = on_modal_close.clone();

        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let id = props.character.id;

        Callback::from(move |character: Character| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let action_state = action_state.clone();

            let character_query_state = character_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match update_character(id, character).await {
                    Ok(_) => {
                        let _ = character_query_state.refresh().await;
                        on_modal_close.emit(());
                        action_state.set(CharacterActions::Closed);
                        unknown_error_state.set(false);

                        ErrorState::None
                    }
                    Err(err) => match err.code {
                        CONFLICT => {
                            error_message_state
                                .set("Ein Charakter mit diesem Namen existiert bereits".into());
                            unknown_error_state.set(false);

                            ErrorState::Edit
                        }
                        NOT_FOUND => {
                            error_message_state
                                .set("Der Charakter konnte nicht gefunden werden".into());
                            unknown_error_state.set(false);

                            ErrorState::Edit
                        }
                        _ => {
                            error_message_state
                                .set("Der Charakter konnte nicht gespeichert werden".into());
                            unknown_error_state.set(true);
                            bamboo_error_state.set(err.clone());
                            error_message_form_state.set("update_character".into());

                            ErrorState::Edit
                        }
                    },
                });
            })
        })
    };

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

    html!(
        <>
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton on_click={edit_character_click} label="Bearbeiten" />
                    <CosmoButton on_click={delete_character_click} label="Löschen" />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            if *error_state == ErrorState::Delete {
                if *unknown_error_state {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} />
                }
            }
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
                    <ModifyCharacterModal has_unknown_error={*unknown_error_state} free_companies={props.free_companies.clone()} on_error_close={report_unknown_error.clone()} title={format!("Charakter {} bearbeiten", props.character.name.clone())} save_label="Character speichern" on_save={on_modal_save} on_close={on_modal_close} character={props.character.clone()} custom_fields={props.custom_fields.clone()} error_message={(*error_message_state).clone()} has_error={*error_state == ErrorState::Edit} />
                ),
                CharacterActions::Delete => {
                    let character = props.character.clone();
                    html!(
                        <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={on_modal_delete} on_decline={on_modal_close} confirm_label="Character löschen" decline_label="Character behalten" title="Character löschen" message={format!("Soll der Character {} wirklich gelöscht werden?", character.name)} />
                    )
                }
                CharacterActions::Closed => html!(),
            }}
        </>
    )
}

#[function_component(HousingDetails)]
fn housing_details(props: &HousingDetailsProps) -> Html {
    log::debug!("Render housing details");
    let housing_query_state =
        use_query_value::<CharacterHousingForCharacter>(Rc::new(props.character.id));

    let action_state = use_state_eq(|| HousingActions::Closed);

    let initial_loaded_state = use_state_eq(|| false);
    let open_create_housing_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let delete_error_state = use_state_eq(|| false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let housing_state = use_state_eq(|| vec![] as Vec<CharacterHousing>);

    {
        let error_state = error_state.clone();

        use_unmount(move || {
            error_state.set(false);
        })
    }

    let on_modal_create_close =
        use_callback(open_create_housing_modal_state.clone(), |_, state| {
            state.set(false)
        });
    let on_modal_create_save = {
        let error_state = error_state.clone();
        let open_create_housing_modal_state = open_create_housing_modal_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let housing_query_state = housing_query_state.clone();

        let character_id = props.character.id;

        Callback::from(move |housing: CharacterHousing| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_housing_modal_state = open_create_housing_modal_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let housing_query_state = housing_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(
                    match create_character_housing(character_id, housing).await {
                        Ok(_) => {
                            open_create_housing_modal_state.clone().set(false);
                            let _ = housing_query_state.refresh().await;
                            unknown_error_state.set(false);

                            false
                        }
                        Err(err) => {
                            error_message_state.set(
                                if err.code == CONFLICT {
                                    unknown_error_state.set(false);
                                    "Eine Unterkunft an dieser Adresse existiert bereits"
                                } else {
                                    unknown_error_state.set(true);
                                    error_message_form_state.set("create_character_housing".into());
                                    bamboo_error_state.set(err.clone());
                                    "Die Unterkunft konnte nicht hinzugefügt werden"
                                }
                                .into(),
                            );
                            true
                        }
                    },
                );
            });
        })
    };
    let on_modal_update_save = {
        let housing_query_state = housing_query_state.clone();

        let on_modal_close = on_modal_create_close.clone();

        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |housing: CharacterHousing| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let action_state = action_state.clone();

            let housing_query_state = housing_query_state.clone();

            let id = if let HousingActions::Edit(housing) = (*action_state).clone() {
                housing.id
            } else {
                -1
            };

            yew::platform::spawn_local(async move {
                error_state.set(
                    match update_character_housing(character_id, id, housing).await {
                        Ok(_) => {
                            let _ = housing_query_state.refresh().await;
                            on_modal_close.emit(());
                            unknown_error_state.set(false);

                            false
                        }
                        Err(err) => {
                            match err.code {
                                CONFLICT => {
                                    error_message_state.set(
                                        "Eine Unterkunft an dieser Adresse existiert bereits"
                                            .into(),
                                    );
                                    unknown_error_state.set(false);
                                }
                                NOT_FOUND => {
                                    error_message_state
                                        .set("Die Unterkunft konnte nicht gefunden werden".into());
                                    unknown_error_state.set(false);
                                }
                                _ => {
                                    error_message_state.set(
                                        "Die Unterkunft konnte nicht gespeichert werden".into(),
                                    );
                                    unknown_error_state.set(true);
                                    error_message_form_state.set("update_character_housing".into());
                                    bamboo_error_state.set(err.clone());
                                }
                            };
                            true
                        }
                    },
                );
                action_state.set(HousingActions::Closed);
            })
        })
    };
    let on_modal_delete = {
        let housing_query_state = housing_query_state.clone();

        let delete_error_state = delete_error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let character_id = props.character.id;

        Callback::from(move |id: i32| {
            let housing_query_state = housing_query_state.clone();

            let delete_error_state = delete_error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let action_state = action_state.clone();

            yew::platform::spawn_local(async move {
                delete_error_state.set(match delete_character_housing(character_id, id).await {
                    Ok(_) => {
                        let _ = housing_query_state.refresh().await;
                        unknown_error_state.set(false);

                        false
                    }
                    Err(err) => match err.code {
                        NOT_FOUND => {
                            error_message_state
                                .set("Die Unterkunft konnte nicht gefunden werden".into());
                            unknown_error_state.set(false);

                            true
                        }
                        _ => {
                            error_message_state
                                .set("Die Unterkunft konnte nicht gelöscht werden".into());
                            unknown_error_state.set(true);
                            error_message_form_state.set("delete_character_housing".into());
                            bamboo_error_state.set(err.clone());

                            true
                        }
                    },
                });
                action_state.set(HousingActions::Closed);
            })
        })
    };
    let on_modal_action_close = use_callback(
        (action_state.clone(), error_state.clone()),
        |_, (state, error_state)| {
            state.set(HousingActions::Closed);
            error_state.set(false);
        },
    );
    let on_create_open = use_callback(
        (open_create_housing_modal_state.clone(), error_state.clone()),
        |_, (open_state, error_state)| {
            open_state.set(true);
            error_state.set(false);
        },
    );
    let on_edit_open = use_callback(
        (action_state.clone(), error_state.clone()),
        |housing, (action_state, error_state)| {
            action_state.set(HousingActions::Edit(housing));
            error_state.set(false);
        },
    );
    let on_delete_open = use_callback(
        (action_state.clone(), error_state.clone()),
        |housing, (action_state, error_state)| {
            action_state.set(HousingActions::Delete(housing));
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

    let housing_list_style = use_style!(
        r#"
display: flex;
margin-top: 2rem;
flex-flow: row wrap;
gap: 1rem;
"#
    );
    let housing_container_style = use_style!(
        r#"
display: flex;
flex-flow: column;
background: var(--modal-backdrop);
backdrop-filter: var(--modal-container-backdrop-filter);

h5 {
    margin-top: 0;
}

button {
    border-top-left-radius: 0 !important;
    border-top-right-radius: 0 !important;
}
"#
    );
    let housing_address_style = use_style!(
        r#"
border: var(--input-border-width) solid var(--control-border-color);
border-radius: var(--border-radius);
border-bottom: 0;
padding: 0.5rem 1rem;
margin-bottom: calc(var(--input-border-width) * -1 * 2);
font-style: normal;
    "#
    );

    match housing_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <CosmoProgressRing />
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded housing");
            initial_loaded_state.set(true);
            let mut housing = res.character_housing.clone();
            housing.sort();
            housing_state.set(housing);
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            bamboo_error_state.set(err.clone());
            unknown_error_state.set(true);

            return html!(
                if *unknown_error_state {
                    <CosmoMessage header="Fehler beim Laden" message="Die Unterkünfte konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Die Unterkünfte konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
                }
            );
        }
    }

    html!(
        <>
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Unterkunft hinzufügen" on_click={on_create_open} />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            if *delete_error_state {
                if *unknown_error_state {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message={(*error_message_state).clone()} />
                }
            }
            <div class={housing_list_style}>
                {for (*housing_state).clone().into_iter().map(|housing| {
                    let edit_housing = housing.clone();
                    let delete_housing = housing.clone();

                    let on_edit_open = on_edit_open.clone();
                    let on_delete_open = on_delete_open.clone();

                    html!(
                        <div class={housing_container_style.clone()}>
                            <address class={housing_address_style.clone()}>
                                <CosmoHeader level={CosmoHeaderLevel::H5} header={housing.district.to_string()} />
                                <span>{format!("Bezirk {}", housing.ward)}</span><br />
                                <span>{format!("Nr. {}", housing.plot)}</span>
                            </address>
                            <CosmoToolbarGroup>
                                <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_housing.clone())} />
                                <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_housing.clone())} />
                            </CosmoToolbarGroup>
                        </div>
                    )
                })}
            </div>
            if *open_create_housing_modal_state {
                <ModifyHousingModal has_unknown_error={*unknown_error_state} on_error_close={report_unknown_error.clone()} housing={CharacterHousing::new(props.character.id, HousingDistrict::TheLavenderBeds, 1, 1)} character_id={props.character.id} is_edit={false} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_create_close} title="Unterkunft hinzufügen" save_label="Unterkunft hinzufügen" on_save={on_modal_create_save} />
            }
            {match (*action_state).clone() {
                HousingActions::Edit(housing) => html!(
                    <ModifyHousingModal has_unknown_error={*unknown_error_state} on_error_close={report_unknown_error.clone()} character_id={props.character.id} is_edit={true} title="Unterkunft bearbeiten" save_label="Unterkunft speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} housing={housing} error_message={(*error_message_state).clone()} has_error={*error_state} />
                ),
                HousingActions::Delete(housing) => html!(
                    <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={move |_| on_modal_delete.emit(housing.id)} on_decline={on_modal_action_close} confirm_label="Unterkunft löschen" decline_label="Unterkunft behalten" title="Unterkunft löschen" message={format!("Soll die Unterkunft in {} im Bezirk {} mit der Nummer {} wirklich gelöscht werden?", housing.district.to_string(), housing.ward, housing.plot)} />
                ),
                HousingActions::Closed => html!(),
            }}
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

#[function_component(FighterDetails)]
fn fighter_details(props: &FighterDetailsProps) -> Html {
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

#[function_component(CharacterPage)]
pub fn character_page() -> Html {
    log::debug!("Render character page");
    log::debug!("Initialize state and callbacks");
    let character_query_state = use_query_value::<MyCharacters>(().into());

    let open_create_character_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let initial_loaded_state = use_state_eq(|| false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let character_state = use_state_eq(|| vec![] as Vec<Character>);

    let free_companies_state = use_state_eq(|| vec![] as Vec<FreeCompany>);

    let custom_fields_state = use_state_eq(|| vec![] as Vec<CustomCharacterField>);

    let selected_character_state = use_state_eq(|| 0);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

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

    let open_create_character_modal_click = use_callback(
        (
            open_create_character_modal_state.clone(),
            error_state.clone(),
        ),
        |_, (open_create_character_modal_state, error_state)| {
            open_create_character_modal_state.set(true);
            error_state.set(false);
        },
    );
    let on_character_select = use_callback(
        (selected_character_state.clone(), error_state.clone()),
        |idx, (state, error_state)| {
            state.set(idx);
            error_state.set(false);
        },
    );
    let on_modal_close = use_callback(open_create_character_modal_state.clone(), |_, state| {
        state.set(false)
    });
    let on_modal_save = {
        let error_state = error_state.clone();
        let open_create_character_modal_state = open_create_character_modal_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let character_query_state = character_query_state.clone();

        let selected_character_state = selected_character_state.clone();

        Callback::from(move |character: Character| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_character_modal_state = open_create_character_modal_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let error_message_state = error_message_state.clone();
            let error_message_form_state = error_message_form_state.clone();

            let selected_character_state = selected_character_state.clone();

            let character_query_state = character_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_character(character).await {
                    Ok(character) => {
                        let id = character.id;
                        open_create_character_modal_state.clone().set(false);
                        if let Ok(res) = character_query_state.refresh().await {
                            selected_character_state.set(
                                res.character
                                    .iter()
                                    .position(move |character| character.id.eq(&id))
                                    .unwrap_or(0),
                            );
                        }
                        false
                    }
                    Err(err) => {
                        error_message_state.set(
                            if err.code == CONFLICT {
                                "Ein Charakter mit diesem Namen existiert bereits"
                            } else {
                                bamboo_error_state.set(err.clone());
                                error_message_form_state.set("character_page".into());
                                unknown_error_state.set(true);
                                "Der Charakter konnte nicht hinzugefügt werden"
                            }
                            .into(),
                        );
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
            bamboo_error_state.set(err.clone());
            if !*initial_loaded_state {
                unknown_error_state.set(true);
            }
            initial_loaded_state.set(true);

            return html!(
                if *unknown_error_state {
                    <CosmoMessage header="Fehler beim Laden" message="Deine Charaktere konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                } else {
                    <CosmoMessage header="Fehler beim Laden" message="Deine Charaktere konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
                }
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
                                    <CrafterDetails character={character.clone()} />
                                </CosmoTabItem>
                                <CosmoTabItem label="Unterkünfte">
                                    <HousingDetails character={character} />
                                </CosmoTabItem>
                            </CosmoTabControl>
                        </>
                    ))
                })}
            </CosmoSideList>
            if *open_create_character_modal_state {
                <ModifyCharacterModal has_unknown_error={*unknown_error_state} on_error_close={report_unknown_error.clone()} free_companies={(*free_companies_state).clone()} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_close} title="Charakter hinzufügen" save_label="Charakter hinzufügen" on_save={on_modal_save} custom_fields={(*custom_fields_state).clone()} />
            }
        </>
    )
}

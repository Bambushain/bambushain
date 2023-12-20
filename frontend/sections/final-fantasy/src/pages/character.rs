use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::Deref;

use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use strum::IntoEnumIterator;
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
use crate::pages::crafter::CrafterDetails;
use crate::pages::fighter::FighterDetails;
use crate::pages::housing::HousingDetails;
use crate::props::character::*;

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

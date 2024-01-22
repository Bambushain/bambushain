use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::Deref;

use strum::IntoEnumIterator;
use yew::prelude::*;
use yew::virtual_dom::{Key, VChild};
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_map, use_mount};

use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::{ApiError, CONFLICT, NOT_FOUND};
use bamboo_pandas_frontend_base::error;

use crate::api;
use crate::pages::crafter::CrafterDetails;
use crate::pages::fighter::FighterDetails;
use crate::pages::housing::HousingDetails;

#[derive(PartialEq, Clone)]
enum CharacterActions {
    Edit,
    Delete,
    Closed,
}

#[autoprops]
#[function_component(ModifyCharacterModal)]
fn modify_character_modal(
    on_close: &Callback<()>,
    title: &AttrValue,
    save_label: &AttrValue,
    error_message: &AttrValue,
    has_error: bool,
    has_unknown_error: bool,
    #[prop_or_default] character: &Character,
    on_save: &Callback<Character>,
    on_error_close: &Callback<()>,
    custom_fields: &Vec<CustomCharacterField>,
    free_companies: &Vec<FreeCompany>,
) -> Html {
    let race_state = use_state_eq(|| AttrValue::from(character.race.get_race_name()));
    let world_state = use_state_eq(|| AttrValue::from(character.world.clone()));
    let name_state = use_state_eq(|| AttrValue::from(character.name.clone()));

    let free_company_state = use_state_eq(|| {
        if let Some(free_company) = character.free_company.clone() {
            Some(AttrValue::from(free_company.id.to_string()))
        } else {
            None
        }
    });

    let mut custom_fields_map = HashMap::new();
    character
        .custom_fields
        .clone()
        .iter()
        .for_each(|character_field| {
            custom_fields_map.insert(
                AttrValue::from(character_field.label.clone()),
                character_field
                    .values
                    .iter()
                    .map(|val| AttrValue::from(val.clone()))
                    .collect::<HashSet<AttrValue>>(),
            );
        });
    let custom_fields_map = use_map(custom_fields_map);

    let on_close = on_close.clone();
    let on_save = use_callback(
        (
            race_state.clone(),
            world_state.clone(),
            name_state.clone(),
            custom_fields_map.clone(),
            free_company_state.clone(),
            free_companies.clone(),
            on_save.clone(),
        ),
        |_,
         (
             race_state,
             world_state,
             name_state,
             custom_fields_map,
             free_company_state,
             free_companies,
             on_save,
         )| {
            let custom_fields = custom_fields_map
                .current()
                .iter()
                .map(|(label, values)| CustomField {
                    label: label.to_string(),
                    values: values
                        .iter()
                        .map(|val| val.to_string())
                        .collect::<BTreeSet<String>>(),
                    position: 0,
                })
                .collect::<Vec<_>>();

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
        custom_fields_map.clone(),
        |(label, value): (AttrValue, AttrValue), map| {
            let mut data = map.current().clone();
            if let Entry::Occupied(mut entry) = data.entry(label.clone()) {
                let set = entry.get_mut();
                set.insert(value);
                map.update(&label, set.clone());
            } else {
                let mut set = HashSet::new();
                set.insert(value);
                map.insert(label, set.clone());
            }
        },
    );
    let custom_field_deselect = use_callback(
        custom_fields_map.clone(),
        |(label, value): (AttrValue, AttrValue), map| {
            let mut data = map.current().clone();
            if let Entry::Occupied(mut entry) = data.entry(label.clone()) {
                let set = entry.get_mut();
                set.remove(&value);
                map.update(&label, set.clone());
            } else {
                let set = HashSet::new();
                map.insert(label, set.clone());
            }
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

    let mut all_free_companies = free_companies.clone();
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
    let mut fields = custom_fields.clone();
    fields.sort();
    for field in fields {
        let map = custom_fields_map.clone();
        let data = map.current().clone();
        let values = data.get(&AttrValue::from(field.label.clone()));

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
                    selected: values.map(|set| set.contains(&item)).unwrap_or(false),
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
        <CosmoModal title={title.clone()} is_form={true} on_form_submit={on_save} buttons={html!(
            <>
                <CosmoButton on_click={on_close} label="Abbrechen" />
                <CosmoButton label={save_label.clone()} is_submit={true} />
            </>
        )}>
            if has_error {
                if has_unknown_error {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={error_message.clone()} actions={html!(<CosmoButton label="Fehler melden" on_click={on_error_close.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={error_message.clone()} />
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

#[autoprops]
#[function_component(CharacterDetails)]
fn character_details(
    character: &Character,
    on_delete: &Callback<()>,
    on_save: &Callback<()>,
    custom_fields: &Vec<CustomCharacterField>,
    free_companies: &Vec<FreeCompany>,
) -> Html {
    log::debug!("Initialize character details state and callbacks");
    let action_state = use_state_eq(|| CharacterActions::Closed);

    let edit_character_ref = use_mut_ref(|| None as Option<Character>);

    let edit_error_toggle = use_bool_toggle(false);
    let unreported_error_toggle = use_bool_toggle(false);

    let bamboo_error_state = use_state_eq(ApiError::default);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let save_state = {
        let edit_error_toggle = edit_error_toggle.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let edit_character_ref = edit_character_ref.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let action_state = action_state.clone();

        let id = character.id;

        let on_save = on_save.clone();

        #[allow(clippy::await_holding_refcell_ref)]
        use_async(async move {
            if let Some(character) = edit_character_ref.borrow().clone() {
                match api::update_character(id, character.clone()).await {
                    Ok(_) => {
                        action_state.set(CharacterActions::Closed);
                        unreported_error_toggle.set(false);
                        edit_error_toggle.set(false);
                        on_save.emit(());
                        Ok(())
                    }
                    Err(err) => {
                        edit_error_toggle.set(true);
                        match err.code {
                            CONFLICT => {
                                error_message_state
                                    .set("Ein Charakter mit diesem Namen existiert bereits für diese Welt".into());
                                unreported_error_toggle.set(false);
                            }
                            NOT_FOUND => {
                                error_message_state
                                    .set("Der Charakter konnte nicht gefunden werden".into());
                                unreported_error_toggle.set(false);
                            }
                            _ => {
                                error_message_state
                                    .set("Der Charakter konnte nicht gespeichert werden".into());
                                unreported_error_toggle.set(true);
                                bamboo_error_state.set(err.clone());
                                error_message_form_state.set("update_character".into());
                            }
                        }

                        Err(())
                    }
                }
            } else {
                Err(())
            }
        })
    };
    let delete_state = {
        let action_state = action_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_form_state = error_message_form_state.clone();

        let on_delete = on_delete.clone();

        let id = character.id;

        use_async(async move {
            api::delete_character(id)
                .await
                .map(|_| {
                    action_state.set(CharacterActions::Closed);
                    on_delete.emit(())
                })
                .map_err(|err| {
                    unreported_error_toggle.set(true);
                    error_message_form_state.set("delete_character".into());
                    bamboo_error_state.set(err.clone());

                    err
                })
        })
    };

    let edit_character_click = use_callback(
        (action_state.clone(), edit_error_toggle.clone()),
        |_, (state, edit_error_toggle)| {
            state.set(CharacterActions::Edit);
            edit_error_toggle.set(false);
        },
    );
    let delete_character_click = use_callback(action_state.clone(), |_, state| {
        state.set(CharacterActions::Delete);
    });
    let on_modal_save = use_callback(
        (edit_character_ref.clone(), save_state.clone()),
        |character, (edit_character_ref, save_state)| {
            *edit_character_ref.borrow_mut() = Some(character);
            save_state.run();
        },
    );
    let on_modal_delete = use_callback(delete_state.clone(), |_, delete_state| {
        delete_state.run();
    });
    let on_modal_close = use_callback(
        (action_state.clone(), unreported_error_toggle.clone()),
        |_, (state, unreported_error_toggle)| {
            state.set(CharacterActions::Closed);
            unreported_error_toggle.set(false);
        },
    );

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

    html!(
        <>
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton on_click={edit_character_click} label="Bearbeiten" />
                    <CosmoButton on_click={delete_character_click} label="Löschen" />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            if let Some(err) = &delete_state.error {
                if err.code == NOT_FOUND {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message="Der Charakter konnte nicht gefunden werden" />
                } else if *unreported_error_toggle {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message="Der Charakter konnte nicht gelöscht werden" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                } else {
                    <CosmoMessage message_type={CosmoMessageType::Negative} header="Fehler beim Löschen" message="Der Charakter konnte nicht gelöscht werden" />
                }
            }
            <CosmoKeyValueList>
                <CosmoKeyValueListItem title="Name">{character.name.clone()}</CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Rasse">{character.race.to_string()}</CosmoKeyValueListItem>
                <CosmoKeyValueListItem title="Welt">{character.world.clone()}</CosmoKeyValueListItem>
                if let Some(free_company) = character.free_company.clone() {
                    <CosmoKeyValueListItem title="Freie Gesellschaft">{free_company.name.clone()}</CosmoKeyValueListItem>
                }
                {for character.custom_fields.clone().iter().map(|field| {
                    html!(
                        <CosmoKeyValueListItem title={field.label.clone()}>{field.values.clone().into_iter().collect::<Vec<String>>().join(", ")}</CosmoKeyValueListItem>
                    )
                })}
            </CosmoKeyValueList>
            {match (*action_state).clone() {
                CharacterActions::Edit => html!(
                    <ModifyCharacterModal has_unknown_error={*unreported_error_toggle} free_companies={free_companies.clone()} on_error_close={report_unknown_error.clone()} title={format!("Charakter {} bearbeiten", character.name.clone())} save_label="Character speichern" on_save={on_modal_save} on_close={on_modal_close} character={character.clone()} custom_fields={custom_fields.clone()} error_message={(*error_message_state).clone()} has_error={*edit_error_toggle} />
                ),
                CharacterActions::Delete => {
                    let character = character.clone();
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
    let open_create_character_modal_toggle = use_bool_toggle(false);
    let unreported_error_toggle = use_state_eq(|| false);

    let create_character_ref = use_mut_ref(|| None as Option<Character>);

    let bamboo_error_state = use_state_eq(ApiError::default);

    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let selected_character_state = use_state_eq(|| 0);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let characters_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            api::get_characters()
                .await
                .map_err(|err| {
                    unreported_error_toggle.set(true);
                    bamboo_error_state.set(err.clone());

                    err
                })
                .map(|data| {
                    unreported_error_toggle.set(false);

                    data
                })
        })
    };
    let free_companies_state = use_async(async move { api::get_free_companies().await });
    let custom_fields_state = use_async(async move { api::get_custom_fields().await });
    let create_state = {
        let open_create_character_modal_state = open_create_character_modal_toggle.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let characters_state = characters_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let selected_character_state = selected_character_state.clone();

        let create_character_ref = create_character_ref.clone();

        #[allow(clippy::await_holding_refcell_ref)]
        use_async(async move {
            if let Some(character) = create_character_ref.borrow().clone() {
                api::create_character(character)
                    .await
                    .map(|character| {
                        open_create_character_modal_state.set(false);
                        selected_character_state.set(character.id);

                        characters_state.run()
                    })
                    .map_err(|err| {
                        error_message_state.set(
                            if err.code == CONFLICT {
                                "Ein Charakter mit diesem Namen existiert bereits für diese Welt"
                            } else {
                                bamboo_error_state.set(err.clone());
                                error_message_form_state.set("character_page".into());
                                unreported_error_toggle.set(true);
                                "Der Charakter konnte nicht hinzugefügt werden"
                            }
                                .into(),
                        );

                        err
                    })
            } else {
                Ok(())
            }
        })
    };

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
    let open_create_character_modal_click = use_callback(
        open_create_character_modal_toggle.clone(),
        |_, open_create_character_modal_state| {
            open_create_character_modal_state.set(true);
        },
    );
    let on_modal_close = use_callback(open_create_character_modal_toggle.clone(), |_, state| {
        state.set(false)
    });
    let on_modal_save = use_callback(
        (create_character_ref.clone(), create_state.clone()),
        |character, (create_character_ref, create_state)| {
            *create_character_ref.borrow_mut() = Some(character);
            create_state.run();
        },
    );
    let on_delete = use_callback(
        (characters_state.clone(), selected_character_state.clone()),
        |_, (characters_state, selected_character_state)| {
            selected_character_state.set(0);
            characters_state.run();
        },
    );
    let on_save = use_callback(characters_state.clone(), |_, characters_state| {
        characters_state.run();
    });

    {
        let custom_fields_state = custom_fields_state.clone();
        let free_companies_state = free_companies_state.clone();
        let characters_state = characters_state.clone();

        use_mount(move || {
            free_companies_state.run();
            custom_fields_state.run();
            characters_state.run();
        });
    }

    if characters_state.loading {
        html!(
            <CosmoProgressRing />
        )
    } else if characters_state.error.is_some() {
        if *unreported_error_toggle {
            html!(
                <CosmoMessage header="Fehler beim Laden" message="Deine Charaktere konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
            )
        } else {
            html!(
                <CosmoMessage header="Fehler beim Laden" message="Deine Charaktere konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
            )
        }
    } else if let Some(data) = &characters_state.data {
        let select_character = {
            let data = data.clone();
            let selected_character_state = selected_character_state.clone();

            Callback::from(move |idx| {
                selected_character_state.set(data.get(idx).map(|u: &Character| u.id).unwrap_or(0))
            })
        };

        html!(
            <>
                <CosmoSideList on_select_item={select_character} selected_index={data.iter().position(|u| u.id == *selected_character_state).unwrap_or(0)} has_add_button={true} add_button_on_click={open_create_character_modal_click} add_button_label="Charakter hinzufügen">
                    {for data.iter().map(|character| {
                        CosmoSideListItem::from_label_and_children(character.name.clone().into(), html!(
                            <>
                                <CosmoTitle title={character.name.clone()} />
                                <CosmoTabControl>
                                    <CosmoTabItem label="Details">
                                        <CharacterDetails free_companies={free_companies_state.data.clone().unwrap_or(Vec::new()).clone()} custom_fields={custom_fields_state.data.clone().unwrap_or(Vec::new()).clone()} on_save={on_save.clone()} on_delete={on_delete.clone()} character={character.clone()} />
                                    </CosmoTabItem>
                                    <CosmoTabItem label="Kämpfer">
                                        <FighterDetails character={character.clone()} />
                                    </CosmoTabItem>
                                    <CosmoTabItem label="Crafter">
                                        <CrafterDetails character={character.clone()} />
                                    </CosmoTabItem>
                                    <CosmoTabItem label="Unterkünfte">
                                        <HousingDetails character={character.clone()} />
                                    </CosmoTabItem>
                                </CosmoTabControl>
                            </>
                        ))
                    })}
                </CosmoSideList>
                if *open_create_character_modal_toggle {
                    <ModifyCharacterModal has_unknown_error={*unreported_error_toggle} on_error_close={report_unknown_error.clone()} free_companies={free_companies_state.data.clone().unwrap_or(Vec::new()).clone()} error_message={(*error_message_state).clone()} has_error={create_state.error.is_some()} on_close={on_modal_close} title="Charakter hinzufügen" save_label="Charakter hinzufügen" on_save={on_modal_save} custom_fields={custom_fields_state.data.clone().unwrap_or(Vec::new()).clone()} />
                }
            </>
        )
    } else {
        html!()
    }
}

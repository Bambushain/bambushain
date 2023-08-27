use std::rc::Rc;

use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use strum::IntoEnumIterator;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;

use pandaparty_entities::character::CharacterRace;
use pandaparty_entities::prelude::*;

use crate::api;
use crate::api::*;
use crate::api::character::MyCharacters;

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
}

#[function_component(ModifyCharacterModal)]
fn modify_character_modal(props: &ModifyCharacterModalProps) -> Html {
    let race_state = use_state_eq(|| Some(AttrValue::from(props.character.race.get_race_name())));
    let world_state = use_state_eq(|| AttrValue::from(props.character.world.clone()));
    let name_state = use_state_eq(|| AttrValue::from(props.character.name.clone()));

    let on_close = props.on_close.clone();
    let on_save = use_callback(|_, (race_state, world_state, name_state, on_save)| on_save.emit(Character::new(CharacterRace::from((**race_state).clone().unwrap().to_string()), (**name_state).to_string(), (**world_state).to_string())), (race_state.clone(), world_state.clone(), name_state.clone(), props.on_save.clone()));

    let update_race = use_callback(|value: Option<AttrValue>, state| state.set(value), race_state.clone());
    let update_world = use_callback(|value: AttrValue, state| state.set(value), world_state.clone());
    let update_name = use_callback(|value: AttrValue, state| state.set(value), name_state.clone());

    let mut all_races = CharacterRace::iter().collect::<Vec<CharacterRace>>();
    all_races.sort();

    let races = all_races.iter().map(|race| (Some(AttrValue::from(race.get_race_name())), AttrValue::from(race.to_string()))).collect::<Vec<(Option<AttrValue>, AttrValue)>>();

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
                    <CosmoTextBox label="Name" on_input={update_name} value={(*name_state).clone()} required={true} />
                    <CosmoDropdown label="Rasse" on_select={update_race} value={(*race_state).clone()} required={true} items={races} />
                    <CosmoTextBox label="Welt" on_input={update_world} value={(*world_state).clone()} required={true} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct CharacterDetailsProps {
    character: Character,
    on_delete: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
struct CrafterDetailsProps {
    character: Character,
}

#[derive(Properties, PartialEq, Clone)]
struct FighterDetailsProps {
    character: Character,
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

#[function_component(CharacterDetails)]
fn character_details(props: &CharacterDetailsProps) -> Html {
    log::debug!("Initialize character details state and callbacks");
    let action_state = use_state_eq(|| CharacterActions::Closed);

    let error_state = use_state_eq(|| ErrorState::None);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let character_query_state = use_query_value::<MyCharacters>(().into());

    let edit_character_click = use_callback(|_, state| state.set(CharacterActions::Edit), action_state.clone());
    let delete_character_click = use_callback(|_, state| state.set(CharacterActions::Delete), action_state.clone());

    let on_modal_close = {
        let action_state = action_state.clone();

        let character_query_state = character_query_state.clone();

        Callback::from(move |_| {
            let action_state = action_state.clone();

            let character_query_state = character_query_state.clone();

            yew::platform::spawn_local(async move {
                action_state.set(CharacterActions::Closed);

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

    let header_style = use_style!(r#"
display: flex;
gap: 16px;
align-items: center;

img {
    height: 36px;
    width: 36px;
    object-fit: scale-down;
}
    "#);

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
            </CosmoKeyValueList>
            {match (*action_state).clone() {
                CharacterActions::Edit => html!(
                    <ModifyCharacterModal on_error_close={on_error_close.clone()} title={format!("Charakter {} bearbeiten", props.character.name.clone())} save_label="Character speichern" on_save={on_modal_save} on_close={on_modal_close} character={props.character.clone()} error_message={(*error_message_state).clone()} has_error={*error_state == ErrorState::Edit} />
                ),
                CharacterActions::Delete => {
                    let character = props.character.clone();
                    html!(
                        <CosmoConfirm on_confirm={on_modal_delete} on_decline={on_modal_close} confirm_label="Character löschen" decline_label="Character behalten" title="Character löschen" message={format!("Soll der Character {} wirklich gelöscht werden?", character.name.to_string())} />
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

    let initial_loaded_state = use_state_eq(|| false);

    let crafter_state = use_state_eq(|| vec![] as Vec<Crafter>);

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
            crafter_state.set(res.crafter.clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Crafter konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    html!(
        <>
            <CosmoHeader level={CosmoHeaderLevel::H3} header={format!("{}s Crafter", props.character.name.clone())} />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Crafter hinzufügen" />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            <CosmoTable headers={vec![AttrValue::from("Job"), AttrValue::from("Level"), AttrValue::from("Aktionen")]}>
                {for (*crafter_state).clone().into_iter().map(|crafter|
                    CosmoTableRow::from_table_cells(vec![
                        CosmoTableCell::from_html(html!({crafter.job.to_string()}), None),
                        CosmoTableCell::from_html(html!({crafter.level.clone().unwrap_or("".into())}), None),
                        CosmoTableCell::from_html(html!(
                            <>
                                <CosmoButton label="Bearbeiten" />
                                <CosmoButton label="Löschen" />
                            </>
                        ), None),
                    ], Some(crafter.id.into()))
                )}
            </CosmoTable>
        </>
    )
}

#[function_component(FighterDetails)]
fn fighter_details(props: &FighterDetailsProps) -> Html {
    log::debug!("Render fighter details");
    let fighter_query_state = use_query_value::<FighterForCharacter>(Rc::new(props.character.id));

    let initial_loaded_state = use_state_eq(|| false);

    let fighter_state = use_state_eq(|| vec![] as Vec<Fighter>);

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
            fighter_state.set(res.fighter.clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Fighter konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    html!(
        <>
            <CosmoHeader level={CosmoHeaderLevel::H3} header={format!("{}s Kämpfer", props.character.name.clone())} />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton label="Crafter hinzufügen" />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            <CosmoTable headers={vec![AttrValue::from("Job"), AttrValue::from("Level"), AttrValue::from("Gear Score"), AttrValue::from("Aktionen")]}>
                {for (*fighter_state).clone().into_iter().map(|fighter|
                    CosmoTableRow::from_table_cells(vec![
                        CosmoTableCell::from_html(html!({fighter.job.to_string()}), None),
                        CosmoTableCell::from_html(html!({fighter.level.clone().unwrap_or("".into())}), None),
                        CosmoTableCell::from_html(html!({fighter.gear_score.clone().unwrap_or("".into())}), None),
                        CosmoTableCell::from_html(html!(
                            <>
                                <CosmoButton label="Bearbeiten" />
                                <CosmoButton label="Löschen" />
                            </>
                        ), None),
                    ], Some(fighter.id.into()))
                )}
            </CosmoTable>
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

    let selected_character_state = use_state_eq(|| 0);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let open_create_character_modal_click = use_callback(|_, open_create_character_modal_state| open_create_character_modal_state.set(true), open_create_character_modal_state.clone());
    let on_character_select = use_callback(|idx, state| state.set(idx), selected_character_state.clone());
    let on_modal_close = use_callback(|_, state| state.set(false), open_create_character_modal_state.clone());
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
                error_state.set(match api::create_character(character).await {
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
                                    <CharacterDetails on_delete={on_delete.clone()} character={character.clone()} />
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
                <ModifyCharacterModal on_error_close={on_error_close} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_close} title="Charakter hinzufügen" save_label="Charakter hinzufügen" on_save={on_modal_save} />
            }
        </>
    )
}

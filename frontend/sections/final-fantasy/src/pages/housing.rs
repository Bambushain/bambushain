use std::ops::Deref;
use std::rc::Rc;

use bounce::query::use_query_value;
use strum::IntoEnumIterator;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::use_unmount;

use bamboo_entities::prelude::*;
use bamboo_frontend_base_api as api;
use bamboo_frontend_base_api::{CONFLICT, NOT_FOUND};
use bamboo_frontend_base_error as error;

use crate::api::*;
use crate::models::*;
use crate::props::housing::*;

#[derive(PartialEq, Clone)]
enum HousingActions {
    Edit(CharacterHousing),
    Delete(CharacterHousing),
    Closed,
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

#[function_component(HousingDetails)]
pub fn housing_details(props: &HousingDetailsProps) -> Html {
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

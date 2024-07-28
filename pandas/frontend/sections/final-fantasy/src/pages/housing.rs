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
enum HousingActions {
    Create,
    Edit(CharacterHousing),
    Delete(CharacterHousing),
    Closed,
}

#[autoprops]
#[function_component(ModifyHousingModal)]
fn modify_housing_modal(
    on_close: &Callback<()>,
    title: &AttrValue,
    save_label: &AttrValue,
    error_message: &AttrValue,
    has_error: bool,
    api_error: &Option<ApiError>,
    #[prop_or_default] housing: &CharacterHousing,
    character_id: i32,
    on_save: &Callback<CharacterHousing>,
) -> Html {
    let district_state = use_state_eq(|| housing.district);
    let housing_type_state = use_state_eq(|| housing.housing_type);
    let ward_state = use_state_eq(|| housing.ward);
    let plot_state = use_state_eq(|| housing.plot);

    let districts = HousingDistrict::iter()
        .map(|district| {
            CosmoModernSelectItem::new(
                district.to_string(),
                district.get_name(),
                (*district_state).clone().eq(&district),
            )
        })
        .collect::<Vec<_>>();
    let housing_types = HousingType::iter()
        .map(|housing_type| {
            CosmoModernSelectItem::new(
                housing_type.to_string(),
                housing_type.get_name(),
                (*housing_type_state).clone().eq(&housing_type),
            )
        })
        .collect::<Vec<_>>();
    let wards = (1..31i16)
        .map(|ward| {
            CosmoModernSelectItem::new(
                ward.to_string(),
                ward.to_string(),
                (*ward_state).clone().eq(&ward),
            )
        })
        .collect::<Vec<_>>();
    let plots = (1..61i16)
        .map(|plot| {
            CosmoModernSelectItem::new(
                plot.to_string(),
                plot.to_string(),
                (*plot_state).clone().eq(&plot),
            )
        })
        .collect::<Vec<_>>();

    let on_close = on_close.clone();
    let on_save = use_callback(
        (
            district_state.clone(),
            housing_type_state.clone(),
            ward_state.clone(),
            plot_state.clone(),
            on_save.clone(),
            character_id,
        ),
        |_, (district_state, housing_type_state, ward_state, plot_state, on_save, character_id)| {
            on_save.emit(CharacterHousing::new(
                *character_id,
                *(*district_state).clone(),
                *(*housing_type_state).clone(),
                *(*ward_state).clone(),
                *(*plot_state).clone(),
            ))
        },
    );

    let update_district = use_callback(district_state.clone(), |value: AttrValue, state| {
        state.set(HousingDistrict::from(value.to_string()))
    });
    let update_housing_type =
        use_callback(housing_type_state.clone(), |value: AttrValue, state| {
            state.set(HousingType::from(value.to_string()))
        });
    let update_ward = use_callback(ward_state.clone(), |value: AttrValue, state| {
        state.set(value.to_string().as_str().parse::<i16>().unwrap())
    });
    let update_plot = use_callback(plot_state.clone(), |value: AttrValue, state| {
        state.set(value.to_string().as_str().parse::<i16>().unwrap())
    });

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
                            page="housing"
                            form="modify_housing"
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
                        width={CosmoInputWidth::Medium}
                        label="Gebiet"
                        on_select={update_district}
                        required=true
                        items={districts}
                    />
                    <CosmoModernSelect
                        width={CosmoInputWidth::Medium}
                        label="Bezirk"
                        on_select={update_ward}
                        required=true
                        items={wards}
                    />
                    <CosmoModernSelect
                        width={CosmoInputWidth::Medium}
                        label="Nummer"
                        on_select={update_plot}
                        required=true
                        items={plots}
                    />
                    <CosmoModernSelect
                        width={CosmoInputWidth::Medium}
                        label="Kategorie"
                        on_select={update_housing_type}
                        required=true
                        items={housing_types}
                    />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[allow(clippy::await_holding_refcell_ref)]
#[autoprops]
#[function_component(HousingDetails)]
pub fn housing_details(character: &Character) -> Html {
    log::debug!("Render housing details");
    let action_state = use_state_eq(|| HousingActions::Closed);

    let props_character_id_state = use_state_eq(|| character.id);

    let create_housing_ref = use_mut_ref(|| None as Option<CharacterHousing>);
    let edit_housing_ref = use_mut_ref(|| None as Option<CharacterHousing>);
    let edit_id_housing_ref = use_mut_ref(|| -1);
    let delete_housing_ref = use_mut_ref(|| None as Option<i32>);

    let bamboo_error_state = use_state_eq(|| None as Option<ApiError>);

    let error_message_state = use_state_eq(|| AttrValue::from(""));
    let error_message_form_state = use_state_eq(|| AttrValue::from(""));

    let housing_state = {
        let character_id = character.id;

        let error_message_form_state = error_message_form_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        use_async(async move {
            api::get_character_housing(character_id)
                .await
                .map_err(|err| {
                    bamboo_error_state.set(Some(err.clone()));
                    error_message_form_state.set("get_character_housing".into());

                    err
                })
        })
    };
    let create_state = {
        let action_state = action_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let error_message_state = error_message_state.clone();
        let error_message_form_state = error_message_form_state.clone();

        let character_id = character.id;

        let create_housing_ref = create_housing_ref.clone();

        let housing_state = housing_state.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            if let Some(housing) = create_housing_ref.borrow().clone() {
                api::create_character_housing(character_id, housing)
                    .await
                    .map(|_| {
                        action_state.set(HousingActions::Closed);
                        housing_state.run();
                    })
                    .map_err(|err| {
                        error_message_form_state.set("create_character_housing".into());
                        if err.code == CONFLICT {
                            error_message_state
                                .set("Eine Unterkunft an dieser Adresse existiert bereits".into());
                        } else {
                            bamboo_error_state.set(Some(err.clone()));
                            error_message_state
                                .set("Die Unterkunft konnte nicht hinzugefügt werden".into());
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
        let error_message_form_state = error_message_form_state.clone();

        let character_id = character.id;

        let edit_housing_ref = edit_housing_ref.clone();
        let edit_id_housing_ref = edit_id_housing_ref.clone();

        let housing_state = housing_state.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            let id = *edit_id_housing_ref.borrow();

            if let Some(housing) = edit_housing_ref.borrow().clone() {
                api::update_character_housing(character_id, id, housing)
                    .await
                    .map(|_| {
                        action_state.set(HousingActions::Closed);
                        housing_state.run();
                    })
                    .map_err(|err| {
                        error_message_form_state.set("update_character_housing".into());
                        match err.code {
                            CONFLICT => {
                                error_message_state.set(
                                    "Eine Unterkunft an dieser Adresse existiert bereits".into(),
                                );
                            }
                            NOT_FOUND => {
                                error_message_state
                                    .set("Die Unterkunft konnte nicht gefunden werden".into());
                            }
                            _ => {
                                bamboo_error_state.set(Some(err.clone()));
                                error_message_state
                                    .set("Die Unterkunft konnte nicht gespeichert werden".into());
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

        let error_message_form_state = error_message_form_state.clone();

        let character_id = character.id;

        let delete_housing_ref = delete_housing_ref.clone();

        let housing_state = housing_state.clone();

        use_async(async move {
            bamboo_error_state.set(None);
            if let Some(housing) = *delete_housing_ref.borrow() {
                api::delete_character_housing(character_id, housing)
                    .await
                    .map(|_| {
                        action_state.set(HousingActions::Closed);
                        housing_state.run();
                    })
                    .map_err(|err| {
                        error_message_form_state.set("delete_character_housing".into());
                        bamboo_error_state.set(Some(err.clone()));

                        err
                    })
            } else {
                Ok(())
            }
        })
    };

    let on_modal_create_save = use_callback(
        (create_housing_ref.clone(), create_state.clone()),
        |housing, (create_housing_ref, create_state)| {
            *create_housing_ref.borrow_mut() = Some(housing);
            create_state.run();
        },
    );
    let on_modal_update_save = use_callback(
        (edit_housing_ref.clone(), update_state.clone()),
        |housing, (edit_housing_ref, update_state)| {
            *edit_housing_ref.borrow_mut() = Some(housing);
            update_state.run();
        },
    );
    let on_modal_delete = use_callback(
        (delete_housing_ref.clone(), delete_state.clone()),
        |housing_id, (delete_housing_ref, delete_state)| {
            *delete_housing_ref.borrow_mut() = Some(housing_id);
            delete_state.run();
        },
    );
    let on_modal_action_close = use_callback(action_state.clone(), |_, state| {
        state.set(HousingActions::Closed);
    });
    let on_create_open = use_callback(action_state.clone(), |_, action_state| {
        action_state.set(HousingActions::Create);
    });
    let on_edit_open = use_callback(
        (action_state.clone(), edit_id_housing_ref.clone()),
        |housing: CharacterHousing, (action_state, edit_id_housing_ref)| {
            *edit_id_housing_ref.borrow_mut() = housing.id;
            action_state.set(HousingActions::Edit(housing));
        },
    );
    let on_delete_open = use_callback(action_state.clone(), |housing, action_state| {
        action_state.set(HousingActions::Delete(housing));
    });

    {
        let housing_state = housing_state.clone();

        use_mount(move || housing_state.run());
    }
    {
        let housing_state = housing_state.clone();

        let props_character_id_state = props_character_id_state.clone();

        let character = character.clone();

        use_effect_update(move || {
            if *props_character_id_state != character.id {
                housing_state.run();
                props_character_id_state.set(character.id);
            }

            || ()
        })
    }

    let housing_address_style = use_style!(
        r#"
border-bottom: 0;
margin-bottom: calc(var(--input-border-width) * -1 * 2);
font-style: normal;
    "#
    );
    html!(
        if housing_state.loading {
            <CosmoProgressRing />
        } else if let Some(data) = &housing_state.data {
            <>
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Unterkunft hinzufügen" on_click={on_create_open} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
                if let Some(err) = delete_state.error.clone() {
                    if err.code == NOT_FOUND {
                        <CosmoMessage
                            message_type={CosmoMessageType::Negative}
                            header="Fehler beim Löschen"
                            message="Die Unterkunft konnte nicht gefunden werden"
                        />
                    } else {
                        <BambooErrorMessage
                            message="Die Unterkunft konnte leider nicht gelöscht werden"
                            header="Fehler beim Löschen"
                            page="housing"
                            form="delete_housing"
                            error={err}
                        />
                    }
                }
                <BambooCardList>
                    { for data.iter().map(|housing| {
                        let edit_housing = housing.clone();
                        let delete_housing = housing.clone();

                        let on_edit_open = on_edit_open.clone();
                        let on_delete_open = on_delete_open.clone();

                        html!(
                            <BambooCard title={housing.district.to_string()} buttons={html!(
                                <>
                                    <CosmoButton label="Bearbeiten" on_click={move |_| on_edit_open.emit(edit_housing.clone())} />
                                    <CosmoButton label="Löschen" on_click={move |_| on_delete_open.emit(delete_housing.clone())} />
                                </>
                            )}>
                                <address class={housing_address_style.clone()}>
                                    <span>{housing.housing_type.to_string()}</span><br />
                                    <span>{format!("Bezirk {}", housing.ward)}</span><br />
                                    <span>{format!("Nr. {}", housing.plot)}</span>
                                </address>
                            </BambooCard>
                        )
                    }) }
                </BambooCardList>
                { match (*action_state).clone() {
                    HousingActions::Create => html!(
                        <ModifyHousingModal api_error={(*bamboo_error_state).clone()} housing={CharacterHousing::new(character.id, HousingDistrict::TheLavenderBeds, HousingType::Private, 1, 1)} character_id={character.id} error_message={(*error_message_state).clone()} has_error={create_state.error.is_some()} on_close={on_modal_action_close} title="Unterkunft hinzufügen" save_label="Unterkunft hinzufügen" on_save={on_modal_create_save} />
                    ),
                    HousingActions::Edit(housing) => html!(
                        <ModifyHousingModal api_error={(*bamboo_error_state).clone()} character_id={character.id} title="Unterkunft bearbeiten" save_label="Unterkunft speichern" on_save={on_modal_update_save} on_close={on_modal_action_close} housing={housing} error_message={(*error_message_state).clone()} has_error={update_state.error.is_some()} />
                    ),
                    HousingActions::Delete(housing) => html!(
                        <CosmoConfirm confirm_type={CosmoModalType::Warning} on_confirm={move |_| on_modal_delete.emit(housing.id)} on_decline={on_modal_action_close} confirm_label="Unterkunft löschen" decline_label="Unterkunft behalten" title="Unterkunft löschen" message={format!("Soll die Unterkunft in {} im Bezirk {} mit der Nummer {} wirklich gelöscht werden?", housing.district.to_string(), housing.ward, housing.plot)} />
                    ),
                    HousingActions::Closed => html!(),
                } }
            </>
        } else if let Some(err) = housing_state.error.clone() {
            <BambooErrorMessage
                message="Die Unterkünfte konnten leider nicht geladen werden"
                header="Fehler beim Laden"
                page="housing"
                form="housing_details"
                error={err}
            />
        }
    )
}

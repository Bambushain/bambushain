use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use yew::prelude::*;
use yew_cosmo::prelude::*;

use pandaparty_entities::prelude::*;

use crate::api::*;

#[derive(Properties, PartialEq, Clone)]
struct ModifyFighterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    #[prop_or_default]
    fighter: Fighter,
    on_save: Callback<Fighter>,
    on_error_close: Callback<()>,
}

#[function_component(ModifyFighterModal)]
fn modify_fighter_modal(props: &ModifyFighterModalProps) -> Html {
    let job_state = use_state_eq(|| AttrValue::from(props.fighter.job.clone()));
    let level_state = use_state_eq(|| AttrValue::from(props.fighter.level.clone().unwrap_or_default()));
    let gear_score_state = use_state_eq(|| AttrValue::from(props.fighter.gear_score.clone().unwrap_or_default()));

    let on_close = props.on_close.clone();
    let on_save = use_callback(|_, (job_state, level_state, gear_score, on_save)| on_save.emit(Fighter::new((*job_state).to_string(), (*level_state).to_string(), (*gear_score).to_string())), (job_state.clone(), level_state.clone(), gear_score_state.clone(), props.on_save.clone()));
    let update_job = use_callback(|value, state| state.set(value), job_state.clone());
    let update_level = use_callback(|value, state| state.set(value), level_state.clone());
    let update_gear_score = use_callback(|value, state| state.set(value), gear_score_state.clone());

    html!(
        <>
            <CosmoModal title={props.title.clone()} is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label={"Abbrechen"} />
                    <CosmoButton is_submit={true} label={props.save_label.clone()} />
                </>
            )}>
                if props.has_error {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} />
                }
                <CosmoInputGroup>
                    <CosmoTextBox label="Job" on_input={update_job} value={(*job_state).clone()} required={true} />
                    <CosmoTextBox label="Level" on_input={update_level} value={(*level_state).clone()} required={true} />
                    <CosmoTextBox label="Gear Score" on_input={update_gear_score} value={(*gear_score_state).clone()} required={true} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct FighterDetailsProps {
    fighter: Fighter,
    on_delete: Callback<()>,
}

#[derive(PartialEq, Clone)]
enum FighterActions {
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

#[function_component(FighterDetails)]
fn table_body(props: &FighterDetailsProps) -> Html {
    log::debug!("Initialize fighter table body state and callbacks");
    let action_state = use_state_eq(|| FighterActions::Closed);

    let error_state = use_state_eq(|| ErrorState::None);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let fighter_query_state = use_query_value::<MyFighter>(().into());

    let edit_fighter_click = use_callback(|_, state| state.set(FighterActions::Edit), action_state.clone());
    let delete_fighter_click = use_callback(|_, state| state.set(FighterActions::Delete), action_state.clone());

    let on_modal_close = {
        let action_state = action_state.clone();

        let fighter_query_state = fighter_query_state.clone();

        Callback::from(move |_| {
            let action_state = action_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            yew::platform::spawn_local(async move {
                action_state.set(FighterActions::Closed);

                let _ = fighter_query_state.refresh().await;
            });
        })
    };
    let on_modal_delete = {
        let action_state = action_state.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();

        let fighter_query_state = fighter_query_state.clone();

        let on_delete = props.on_delete.clone();

        let id = props.fighter.id;

        Callback::from(move |_| {
            log::debug!("Modal was confirmed lets execute the request");

            let error_state = error_state.clone();

            let action_state = action_state.clone();

            let error_message_state = error_message_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            let on_delete = on_delete.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_fighter(id).await {
                    Ok(_) => {
                        on_delete.emit(());
                        ErrorState::None
                    }
                    Err(err) => match err.code {
                        NOT_FOUND => {
                            error_message_state.set("Der Kämpfer konnte nicht gefunden werden".into());
                            ErrorState::Delete
                        }
                        _ => {
                            error_message_state.set("Der Kämpfer konnte nicht gelöscht werden, bitte wende dich an Azami".into());
                            ErrorState::Delete
                        }
                    }
                });
                action_state.set(FighterActions::Closed);

                let _ = fighter_query_state.refresh().await;
            });
        })
    };
    let on_modal_save = {
        let on_modal_close = on_modal_close.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();

        let action_state = action_state.clone();

        let id = props.fighter.id;

        Callback::from(move |fighter: Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            let action_state = action_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match update_fighter(id, fighter).await {
                    Ok(_) => {
                        let _ = fighter_query_state.refresh().await;
                        on_modal_close.emit(());
                        action_state.set(FighterActions::Closed);
                        ErrorState::None
                    }
                    Err(err) => match err.code {
                        CONFLICT => {
                            error_message_state.set("Ein Kämpfer mit diesem Job existiert bereits".into());
                            ErrorState::Edit
                        }
                        NOT_FOUND => {
                            error_message_state.set("Der Kämpfer konnte nicht gefunden werden".into());
                            ErrorState::Edit
                        }
                        _ => {
                            error_message_state.set("Der Kämpfer konnte nicht gespeichert werden, bitte wende dich an Azami".into());
                            ErrorState::None
                        }
                    }
                });
            })
        })
    };
    let on_error_close = use_callback(|_, state| state.set(ErrorState::None), error_state.clone());

    html!(
        <>
            <CosmoTitle title={props.fighter.job.clone()} />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton on_click={edit_fighter_click} label="Bearbeiten" />
                    <CosmoButton on_click={delete_fighter_click} label="Löschen" />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            <CosmoKeyValueList>
                if let Some(level) = props.fighter.level.clone() {
                    <CosmoKeyValueListItem title="Level">{level}</CosmoKeyValueListItem>
                } else {
                    <CosmoKeyValueListItem title="Level">{"Kein Level festgelegt"}</CosmoKeyValueListItem>
                }
                if let Some(gear_score) = props.fighter.gear_score.clone() {
                    <CosmoKeyValueListItem title="Gear Score">{gear_score}</CosmoKeyValueListItem>
                } else {
                    <CosmoKeyValueListItem title="Gear Score">{"Kein Gear Score festgelegt"}</CosmoKeyValueListItem>
                }
            </CosmoKeyValueList>
            {match (*action_state).clone() {
                FighterActions::Edit => html!(
                    <ModifyFighterModal on_error_close={on_error_close} title={format!("Kämpfer {} bearbeiten", props.fighter.job)} save_label="Kämpfer speichern" on_save={on_modal_save} on_close={on_modal_close} fighter={props.fighter.clone()} error_message={(*error_message_state).clone()} has_error={*error_state == ErrorState::Edit} />
                ),
                FighterActions::Delete => {
                    html!(
                        <CosmoConfirm on_confirm={on_modal_delete} on_decline={on_modal_close} confirm_label="Kämpfer löschen" decline_label="Kämpfer behalten" title="Kämpfer löschen" message={format!("Soll der Kämpfer {} auf Level {} wirklich gelöscht werden?", props.fighter.job.clone(), props.fighter.level.clone().unwrap_or_default())} />
                    )
                }
                FighterActions::Closed => html!(),
            }}
            if *error_state == ErrorState::Delete {
                <CosmoAlert alert_type={CosmoAlertType::Negative} close_label="Schließen" title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={move |_| error_state.set(ErrorState::None)} />
            }
        </>
    )
}

#[function_component(FighterPage)]
pub fn fighter_page() -> Html {
    log::debug!("Render fighter page");
    log::debug!("Initialize state and callbacks");
    let fighter_query_state = use_query_value::<MyFighter>(().into());

    let open_create_fighter_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let initial_loaded_state = use_state_eq(|| false);

    let fighter_state = use_state_eq(|| vec![] as Vec<Fighter>);

    let selected_fighter_state = use_state_eq(|| 0);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let open_create_fighter_modal_click = use_callback(|_, open_create_fighter_modal_state| open_create_fighter_modal_state.set(true), open_create_fighter_modal_state.clone());
    let on_fighter_select = use_callback(|idx, state| state.set(idx), selected_fighter_state.clone());
    let on_modal_close = use_callback(|_, state| state.set(false), open_create_fighter_modal_state.clone());
    let on_modal_save = {
        let error_state = error_state.clone();
        let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();

        let error_message_state = error_message_state.clone();

        let selected_fighter_state = selected_fighter_state.clone();

        let fighter_query_state = fighter_query_state.clone();

        Callback::from(move |fighter: Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();

            let job = fighter.job.clone();

            let error_message_state = error_message_state.clone();

            let selected_fighter_state = selected_fighter_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_fighter(fighter).await {
                    Ok(_) => {
                        open_create_fighter_modal_state.clone().set(false);
                        if let Ok(res) = fighter_query_state.refresh().await {
                            selected_fighter_state.set(res.fighter.iter().position(move |fighter| fighter.job.eq(&job)).unwrap_or(0));
                        }
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
    let on_delete = {
        let fighter_query_state = fighter_query_state.clone();

        let selected_fighter_state = selected_fighter_state.clone();

        Callback::from(move |_| {
            let fighter_query_state = fighter_query_state.clone();

            let selected_fighter_state = selected_fighter_state.clone();

            yew::platform::spawn_local(async move {
                let _ = fighter_query_state.refresh().await;
                selected_fighter_state.set(0);
            })
        })
    };
    let on_error_close = use_callback(|_, state| state.set(false), error_state.clone());

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
            log::warn!("Failed to load {}", err);
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Deine Kämpfer konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Meine Kämpfer"}</title>
            </Helmet>
            <CosmoSideList on_select_item={on_fighter_select} selected_index={*selected_fighter_state} has_add_button={true} add_button_on_click={open_create_fighter_modal_click} add_button_label="Kämpfer hinzufügen">
                {for (*fighter_state).clone().into_iter().map(|fighter| {
                    CosmoSideListItem::from_label_and_children(fighter.job.clone().into(), html!(
                        <FighterDetails on_delete={on_delete.clone()} fighter={fighter} />
                    ))
                })}
            </CosmoSideList>
            if *open_create_fighter_modal_state {
                <ModifyFighterModal on_error_close={on_error_close} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_close} title="Kämpfer hinzufügen" save_label="Kämpfer hinzufügen" on_save={on_modal_save} />
            }
        </>
    )
}

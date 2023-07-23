use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api::{CONFLICT, NOT_FOUND};
use crate::api::fighter::{create_fighter, delete_fighter, MyFighter, update_fighter};
use crate::ui::modal::{PicoAlert, PicoConfirm, PicoModal};

#[derive(Properties, PartialEq, Clone)]
struct ModifyFighterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    is_loading: bool,
    #[prop_or_default]
    fighter: sheef_entities::Fighter,
    on_save: Callback<sheef_entities::Fighter>,
}

#[function_component(ModifyFighterModal)]
fn modify_fighter_modal(props: &ModifyFighterModalProps) -> Html {
    let job_state = use_state_eq(|| AttrValue::from(props.fighter.job.clone()));
    let level_state = use_state_eq(|| AttrValue::from(props.fighter.level.clone()));
    let gear_score_state = use_state_eq(|| AttrValue::from(props.fighter.gear_score.clone()));

    let on_close = props.on_close.clone();
    let on_save = {
        let job_state = job_state.clone();
        let level_state = level_state.clone();
        let gear_score_state = gear_score_state.clone();

        let on_save = props.on_save.clone();

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();
            let fighter = sheef_entities::Fighter {
                job: (*job_state).to_string(),
                level: (*level_state).to_string(),
                gear_score: (*gear_score_state).to_string(),
            };

            on_save.emit(fighter);
        })
    };
    let update_job = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), job_state.clone());
    let update_level = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), level_state.clone());
    let update_gear_score = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), gear_score_state.clone());

    html!(
        <PicoModal title="Kämpfer hinzufügen" on_close={on_close.clone()} open={true} buttons={html!(
            <>
                <button onclick={move |_| on_close.emit(())} type="button" class="secondary">{"Abbrechen"}</button>
                <button form="create-fighter-modal" aria-busy={props.is_loading.to_string()} type="submit">{props.save_label.clone()}</button>
            </>
        )}>
            {if props.has_error {
                html!(<p data-msg="negative">{props.error_message.clone()}</p>)
            } else {
                html!()
            }}
            <form onsubmit={on_save} id="create-fighter-modal">
                <label for="job">{"Job"}</label>
                <input oninput={update_job} readonly={props.is_loading} type="text" value={(*job_state).clone()} required={true} id="job" name="job" />
                <label for="level">{"Level"}</label>
                <input oninput={update_level} readonly={props.is_loading} type="text" value={(*level_state).clone()} required={true} id="level" name="level" />
                <label for="gearScore">{"Gear Score"}</label>
                <input oninput={update_gear_score} readonly={props.is_loading} type="text" value={(*gear_score_state).clone()} required={true} id="gearScore" name="gearScore" />
            </form>
        </PicoModal>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct TableBodyProps {
    fighter: Vec<sheef_entities::Fighter>,
}

#[derive(PartialEq, Clone)]
enum FighterActions {
    Edit(sheef_entities::Fighter),
    Delete(sheef_entities::Fighter),
    Closed,
}

#[derive(PartialEq, Clone)]
enum ErrorState {
    Edit,
    Delete,
    None,
}

#[function_component(TableBody)]
fn table_body(props: &TableBodyProps) -> Html {
    log::debug!("Initialize fighter table body state and callbacks");
    let action_state = use_state_eq(|| FighterActions::Closed);

    let error_state = use_state_eq(|| ErrorState::None);
    let loading_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let fighter_query_state = use_query_value::<MyFighter>(().into());

    let edit_fighter_click = use_callback(|fighter: sheef_entities::Fighter, state| state.set(FighterActions::Edit(fighter)), action_state.clone());
    let delete_fighter_click = use_callback(|fighter: sheef_entities::Fighter, state| state.set(FighterActions::Delete(fighter)), action_state.clone());

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
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();

        let fighter_query_state = fighter_query_state.clone();

        Callback::from(move |fighter: sheef_entities::Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            loading_state.set(true);

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let action_state = action_state.clone();

            let error_message_state = error_message_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_fighter(fighter.clone()).await {
                    Ok(_) => ErrorState::None,
                    Err(err) => match err.code {
                        NOT_FOUND => {
                            error_message_state.set(AttrValue::from("Der Kämpfer konnte nicht gefunden werden"));
                            ErrorState::Delete
                        }
                        _ => {
                            error_message_state.set(AttrValue::from("Der Kämpfer konnte nicht gelöscht werden, bitte wende dich an Azami"));
                            ErrorState::Delete
                        }
                    }
                });
                loading_state.set(false);
                action_state.set(FighterActions::Closed);

                let _ = fighter_query_state.refresh().await;
            });
        })
    };
    let on_modal_save = {
        let on_modal_close = on_modal_close.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();

        let action_state = action_state.clone();

        Callback::from(move |fighter: sheef_entities::Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            loading_state.set(true);
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let error_message_state = error_message_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            let action_state = action_state.clone();

            yew::platform::spawn_local(async move {
                let event_fighter = match (*action_state).clone() {
                    FighterActions::Edit(fighter) => fighter,
                    _ => unreachable!(),
                };

                error_state.set(match update_fighter(event_fighter.job, fighter).await {
                    Ok(_) => {
                        let _ = fighter_query_state.refresh().await;
                        on_modal_close.emit(());
                        ErrorState::Edit
                    }
                    Err(err) => match err.code {
                        CONFLICT => {
                            error_message_state.set(AttrValue::from("Ein Kämpfer mit diesem Job existiert bereits"));
                            ErrorState::Edit
                        }
                        NOT_FOUND => {
                            error_message_state.set(AttrValue::from("Der Kämpfer konnte nicht gefunden werden"));
                            ErrorState::Edit
                        }
                        _ => {
                            error_message_state.set(AttrValue::from("Der Kämpfer konnte nicht gespeichert werden, bitte wende dich an Azami"));
                            ErrorState::None
                        }
                    }
                });
                loading_state.set(false)
            })
        })
    };

    html!(
        <>
            <tbody>
                {for props.fighter.iter().map(|fighter|{
                    let edit_fighter = fighter.clone();
                    let delete_fighter = fighter.clone();

                    let edit_fighter_click = edit_fighter_click.clone();
                    let delete_fighter_click = delete_fighter_click.clone();

                    html!(
                        <tr>
                            <td>{fighter.job.clone()}</td>
                            <td>{fighter.level.clone()}</td>
                            <td>{fighter.gear_score.clone()}</td>
                            <td>
                                <div class="gap-row">
                                    <button onclick={move |_| edit_fighter_click.emit(edit_fighter.clone())} type="button" class="outline">{"Bearbeiten"}</button>
                                    <button onclick={move |_| delete_fighter_click.emit(delete_fighter.clone())} type="button" class="outline">{"Löschen"}</button>
                                </div>
                            </td>
                        </tr>
                    )}
                )}
            </tbody>
            {match (*action_state).clone() {
                FighterActions::Edit(fighter) => html!(<ModifyFighterModal title={format!("Kämpfer {} bearbeiten", fighter.job)} save_label="Kämpfer speichern" on_save={on_modal_save} on_close={on_modal_close} fighter={fighter} error_message={(*error_message_state).clone()} has_error={*error_state == ErrorState::Edit} is_loading={*loading_state} />),
                FighterActions::Delete(fighter) => {
                    let cloned_fighter = fighter.clone();
                    html!(<PicoConfirm open={true} on_confirm={move |_| on_modal_delete.emit(cloned_fighter.clone())} on_decline={on_modal_close} confirm_label="Kämpfer löschen" title="Kämpfer löschen" message={format!("Soll der Kämpfer {} auf Level {} wirklich gelöscht werden?", fighter.job, fighter.level)} />)
                }
                FighterActions::Closed => html!(),
            }}
            {match (*error_state).clone() {
                ErrorState::Delete => html!(<PicoAlert open={true} title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={move |_| error_state.set(ErrorState::None)} />),
                _ => html!()
            }}
        </>
    )
}

#[function_component(FighterPage)]
pub fn fighter_page() -> Html {
    log::debug!("Render fighter page");
    log::debug!("Initialize state and callbacks");
    let fighter_query_state = use_query_value::<MyFighter>(().into());

    let initially_loaded_state = use_state_eq(|| false);
    let open_create_fighter_modal_state = use_state_eq(|| false);

    let state = use_state_eq(|| vec![] as Vec<sheef_entities::Fighter>);

    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let open_create_fighter_modal_click = use_callback(|_, open_create_fighter_modal_state| open_create_fighter_modal_state.set(true), open_create_fighter_modal_state.clone());
    let on_modal_close = use_callback(|_, state| state.set(false), open_create_fighter_modal_state.clone());
    let on_modal_save = {
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();

        let error_message_state = error_message_state.clone();

        let fighter_query_state = fighter_query_state.clone();

        Callback::from(move |fighter: sheef_entities::Fighter| {
            log::debug!("Modal was confirmed lets execute the request");
            loading_state.set(true);

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            let open_create_fighter_modal_state = open_create_fighter_modal_state.clone();

            let error_message_state = error_message_state.clone();

            let fighter_query_state = fighter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_fighter(fighter).await {
                    Ok(_) => {
                        open_create_fighter_modal_state.clone().set(false);
                        let _ = fighter_query_state.refresh().await;
                        false
                    }
                    Err(err) => {
                        error_message_state.set(AttrValue::from(if err.code == CONFLICT {
                            "Ein Kämpfer mit diesem Job existiert bereits"
                        } else {
                            "Der Kämpfer konnte nicht hinzugefügt werden, bitte wende dich an Azami"
                        }));
                        true
                    }
                });
                loading_state.set(false);
            });
        })
    };

    match fighter_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initially_loaded_state {
                return html!(<p data-msg="info">{"Deine Kämpfer werden geladen"}</p>);
            }
        }
        Some(Ok(fighter)) => {
            log::debug!("Loaded fighter");
            initially_loaded_state.set(true);
            state.set(fighter.fighter.clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {}", err);
            return html!(<p data-msg="negative">{"Deine Kämpfer konnten nicht geladen werden, bitte wende dich an Azami"}</p>);
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Meine Kämpfer"}</title>
            </Helmet>
            <h1>{"Meine Kämpfer"}</h1>
            <nav>
                <ul>
                    <li>
                        <button onclick={open_create_fighter_modal_click} type="button">{"Kämpfer hinzufügen"}</button>
                        {if *open_create_fighter_modal_state {
                            html!(
                                <ModifyFighterModal error_message={(*error_message_state).clone()} has_error={*error_state} is_loading={*loading_state} on_close={on_modal_close} title="Kämpfer hinzufügen" save_label="Kämpfer hinzufügen" on_save={on_modal_save} />
                            )
                        } else {
                            html!()
                        }}
                    </li>
                </ul>
            </nav>
            <table role="grid">
                <thead>
                <tr>
                    <th>{"Job"}</th>
                    <th>{"Level"}</th>
                    <th>{"Gear Score"}</th>
                    <th>{"Aktionen"}</th>
                </tr>
                </thead>
                <TableBody fighter={(*state).clone()} />
            </table>
        </>
    )
}

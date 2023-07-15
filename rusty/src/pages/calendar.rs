use std::rc::Rc;
use chrono::{Datelike, Local, Month, Months, NaiveDate};
use yew::prelude::*;
use bounce::helmet::Helmet;
use bounce::query::{use_mutation, use_query_value};
use bounce::use_atom_value;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew_router::prelude::*;
use crate::api::calendar::{Calendar, UpdateEvent, UpdateEventMutation};
use crate::routing::SheefRoute;
use crate::storage::CurrentUser;
use crate::ui::modal::PicoModal;

#[derive(Properties, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
struct CalendarQuery {
    year: i32,
    month: u32,
}

impl From<CalendarQuery> for Option<NaiveDate> {
    fn from(value: CalendarQuery) -> Self {
        NaiveDate::from_ymd_opt(value.year, value.month, 1)
    }
}

impl From<&CalendarQuery> for NaiveDate {
    fn from(value: &CalendarQuery) -> Self {
        NaiveDate::from_ymd_opt(value.year, value.month, 1).expect("Date should be valid")
    }
}

impl From<NaiveDate> for CalendarQuery {
    fn from(value: NaiveDate) -> Self {
        Self {
            year: value.year(),
            month: value.month(),
        }
    }
}

impl Default for CalendarQuery {
    fn default() -> Self {
        let date = Local::now().date_naive();
        Self {
            month: date.month(),
            year: date.year(),
        }
    }
}

#[derive(Properties, PartialEq, Clone, Default)]
struct DayProps {
    available: AttrValue,
    unavailable: AttrValue,
    date: NaiveDate,
    time: AttrValue,
    me_available: bool,
    full_group: bool,
}

fn month_to_german(month: u32) -> AttrValue {
    match Month::from_u32(month).expect("Month should be in range") {
        Month::January => AttrValue::from("Januar"),
        Month::February => AttrValue::from("Februar"),
        Month::March => AttrValue::from("März"),
        Month::April => AttrValue::from("April"),
        Month::May => AttrValue::from("Mai"),
        Month::June => AttrValue::from("Juni"),
        Month::July => AttrValue::from("Juli"),
        Month::August => AttrValue::from("August"),
        Month::September => AttrValue::from("September"),
        Month::October => AttrValue::from("Oktober"),
        Month::November => AttrValue::from("November"),
        Month::December => AttrValue::from("Dezember"),
    }
}

#[derive(Properties, PartialEq, Clone)]
struct DayEditState {
    available: bool,
    time: AttrValue,
}

#[derive(Properties, PartialEq, Clone)]
struct UpdateDayModalProps {
    date: NaiveDate,
    available: bool,
    time: AttrValue,
    on_close: Callback<()>,
}

#[function_component(UpdateDayModal)]
fn update_day_modal(props: &UpdateDayModalProps) -> Html {
    log::debug!("Render the update modal");
    let error_state = use_state(|| false);
    let day_edit_state = use_state(|| DayEditState {
        time: props.time.clone(),
        available: props.available,
    });
    let update_event = use_mutation::<UpdateEventMutation>();
    let calendar_query_state = use_query_value::<Calendar>(Rc::new((props.date.year(), props.date.month())));
    let on_date_save = {
        let error_state = error_state.clone();
        let day_edit_state = day_edit_state.clone();
        let date = props.date;
        let on_close = props.on_close.clone();
        Callback::from(move |evt: SubmitEvent| {
            log::debug!("The form for updating event was submitted");
            evt.prevent_default();
            let on_close = on_close.clone();
            let update_event = update_event.clone();
            let day_edit_state = day_edit_state.clone();
            let error_state = error_state.clone();
            let calendar_query_state = calendar_query_state.clone();
            yew::platform::spawn_local(async move {
                let data = UpdateEvent {
                    time: day_edit_state.time.to_string(),
                    available: day_edit_state.available,
                    date,
                };
                log::debug!("Save the data in the system");
                match update_event.run(data).await {
                    Ok(_) => {
                        log::debug!("Saving was successful, refresh the calendar and close the modal");
                        yew::platform::spawn_local(async move {
                            let _ = calendar_query_state.refresh().await;
                            on_close.emit(());
                            error_state.set(false);
                        });
                    }
                    Err(err) => {
                        log::warn!("Failed to save event data {}", err);
                        error_state.set(true);
                    }
                };
            });
        })
    };
    let update_time = use_callback(move |evt: InputEvent, state| {
        state.set(DayEditState {
            time: evt.target_unchecked_into::<HtmlInputElement>().value().into(),
            available: state.available,
        })
    }, day_edit_state.clone());
    let update_available = use_callback(move |evt: MouseEvent, state| {
        state.set(DayEditState {
            available: evt.target_unchecked_into::<HtmlInputElement>().checked(),
            time: state.time.clone(),
        })
    }, day_edit_state.clone());
    let on_close = props.on_close.clone();

    html!(
        <PicoModal open={true} title={format!("Planung für {}. {} {}", props.date.day(), month_to_german(props.date.month()), props.date.year())} on_close={props.on_close.clone()}>
            <form id={format!("form-{}", props.date.format("%Y-%m-%d"))} onsubmit={on_date_save}>
                {if *error_state {
                    html!(<p data-msg="negative">{"Leider konnte deine Planung nicht gespeichert werden, bitte wende dich an Azami"}</p>)
                } else {
                    html!()
                }}
                <fieldset>
                    <label for="available">
                        <input onclick={update_available} type="checkbox" id="available" name="available" role="switch" checked={day_edit_state.available} />
                        {format!("Ich kann am {}. {} {}", props.date.day(), month_to_german(props.date.month()), props.date.year())}
                    </label>
                </fieldset>
                <label for="timeAvailable">{"Uhrzeit (optional)"}</label>
                <input oninput={update_time} type="text" id="timeAvailable" name="timeAvailable" value={day_edit_state.time.clone()} />
            </form>
            <footer>
                <button data-inline="true" type="button" onclick={move |_| on_close.clone().emit(())} class="secondary" role="button">{"Schließen"}</button>
                <button data-inline="true" form={format!("form-{}", props.date.format("%Y-%m-%d"))} type="submit" role="button">
                    {"Meine Verfügbarkeit speichern"}
                </button>
            </footer>
        </PicoModal>
    )
}

#[function_component(Day)]
fn day(props: &DayProps) -> Html {
    log::debug!("Render day for the date {}", props.date);
    let modal_open_state = use_state(|| false);
    let today = Local::now().date_naive();
    let mut class = vec!["day"];
    if today > props.date {
        log::debug!("Date is in the past");
        class.push("day-in-past")
    }
    if props.full_group {
        log::debug!("On this date all main group players are available");
        class.push("fullgroup-day")
    }

    let on_click = use_callback(|evt: MouseEvent, modal_open_state| {
        evt.prevent_default();
        modal_open_state.set(true);
    }, modal_open_state.clone());
    let on_close = use_callback(|_, modal_open_state| {
        modal_open_state.set(false);
    }, modal_open_state.clone());

    html!(
        <>
            <details class={classes!(class)} open={today <= props.date || props.date.month() != today.month()}>
                <summary><a onclick={on_click}>{props.date.day()}</a></summary>
                <br />
                {if props.available.is_empty() {
                    html!()
                } else {
                    html!(
                        <>
                            <strong>{"Kann"}</strong>
                            <p>{props.available.clone()}</p>
                        </>
                    )
                }}
                {if props.unavailable.is_empty() {
                    html!()
                } else {
                    html!(
                        <>
                            <strong>{"Kann nicht"}</strong>
                            <p>{props.unavailable.clone()}</p>
                        </>
                    )
                }}
            </details>
            {if *modal_open_state {
                html!(
                    <UpdateDayModal date={props.date} time={props.time.clone()} available={props.me_available} on_close={on_close} />
                )
            } else {
                html!()
            }}
        </>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct CalendarProps {
    days: Vec<DayProps>,
    date: NaiveDate,
}

#[function_component(CalendarData)]
fn calendar_data(props: &CalendarProps) -> Html {
    log::debug!("Render CalendarData");
    let first_day_of_month: NaiveDate = props.date;
    let first_day_offset = vec![0; first_day_of_month.weekday() as usize];

    log::debug!("The first day of month is {}", first_day_of_month);
    log::debug!("The first day offset is {}", first_day_offset.len());

    html!(
        <>
            {for first_day_offset.iter().map(|_| html!(<div></div>))}

            {for props.days.iter().map(|day| {
                html!(
                    <Day key={day.date.format("day-%Y-%m-%d").to_string()} date={day.date} available={day.available.clone()} unavailable={day.unavailable.clone()} me_available={day.me_available} time={day.time.clone()} full_group={day.full_group} />
                )
            })}
        </>
    )
}

#[function_component(CalendarPage)]
pub fn calendar_page() -> Html {
    log::debug!("Render calendar page");
    let query = if let Ok(params) = use_location().expect("Location should be available").query::<CalendarQuery>() {
        params
    } else {
        CalendarQuery::default()
    };
    let date: NaiveDate = if let Some(date) = query.into() {
        date
    } else {
        return html!(<Redirect<SheefRoute> to={SheefRoute::Calendar} />);
    };
    let prev_month = date - Months::new(1);
    let next_month = date + Months::new(1);

    let calendar_query_state = use_query_value::<Calendar>(Rc::new((query.year, query.month)));
    let state = use_state_eq(|| vec![] as Vec<DayProps>);
    let initially_loaded_state = use_state_eq(|| false);
    let current_user = use_atom_value::<CurrentUser>();

    match calendar_query_state.result() {
        Some(Ok(result)) => {
            log::debug!("Loaded calendar data");
            let data = result.calendar.days.iter().map(|cal_day| {
                log::debug!("Find event for the current user {}", current_user.profile.username);
                let my_event = if let Some(evt) = cal_day.events.iter().find(|event| event.username == current_user.profile.username.clone()) {
                    log::debug!("Found event for current user");
                    evt.clone()
                } else {
                    log::debug!("Couldn't find event for current user, create an empty one");
                    sheef_entities::Event {
                        username: current_user.profile.username.clone(),
                        user: current_user.profile.clone(),
                        time: "".to_string(),
                        date: cal_day.date,
                        available: false,
                    }
                };
                let me_available = my_event.available;
                let time = AttrValue::from(my_event.time);
                let date = cal_day.date;

                let available = AttrValue::from(cal_day.events
                    .iter()
                    .filter_map(|evt|
                        if evt.available {
                            Some(format!("{}{}", evt.username, if evt.time.is_empty() {
                                "".to_string()
                            } else {
                                format!(" ({})", evt.time)
                            }))
                        } else {
                            None
                        })
                    .collect::<Vec<String>>()
                    .join(", "));
                log::debug!("Available are: {}", available);

                let unavailable = AttrValue::from(cal_day.events
                    .iter()
                    .filter_map(|evt|
                        if !evt.available {
                            Some(evt.username.clone())
                        } else {
                            None
                        })
                    .collect::<Vec<String>>()
                    .join(", "));

                let main_group_members = cal_day.events
                    .iter()
                    .filter(|evt| {
                        log::debug!("Evt: {:?}", evt);
                        evt.user.is_main_group
                    })
                    .collect::<Vec<&sheef_entities::Event>>();
                log::debug!("We have {} main group members for {}", main_group_members.len(), cal_day.date);

                let full_group = main_group_members.iter().all(|evt| evt.available);
                log::debug!("Is there a full group? {}", full_group);

                DayProps {
                    full_group,
                    available,
                    unavailable,
                    me_available,
                    time,
                    date,
                }
            }).collect::<Vec<DayProps>>();

            initially_loaded_state.set(true);
            state.set(data);
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {}", err);
            return html!(<p data-msg="negative">{"Der Kalender konnte nicht geladen werden, bitte wende dich an Azami"}</p>);
        }
        None => {
            log::debug!("Still loading");
            if !*initially_loaded_state {
                return html!(<p data-msg="info">{"Lädt den Kalender"}</p>);
            }
        }
    };

    html!(
        <>
            <Helmet>
                <title>{"Static Kalender"}</title>
            </Helmet>
            <h1>{"Static Kalender"}</h1>
            <div class="calendar-header">
                <Link<SheefRoute, CalendarQuery> to={SheefRoute::Calendar} query={Some(prev_month.into())}>{month_to_german(prev_month.month())}</Link<SheefRoute, CalendarQuery>>
                <h4>{format!("{} {}", month_to_german(date.month()), date.year())}</h4>
                <Link<SheefRoute, CalendarQuery> to={SheefRoute::Calendar} query={Some(next_month.into())}>{month_to_german(next_month.month())}</Link<SheefRoute, CalendarQuery>>
            </div>
            <div class="calendar">
                <div class="day-title">{"Montag"}</div>
                <div class="day-title">{"Dienstag"}</div>
                <div class="day-title">{"Mittwoch"}</div>
                <div class="day-title">{"Donnerstag"}</div>
                <div class="day-title">{"Freitag"}</div>
                <div class="day-title">{"Samstag"}</div>
                <div class="day-title">{"Sonntag"}</div>

                <CalendarData days={(*state).clone()} date={date} />
            </div>
        </>
    )
}

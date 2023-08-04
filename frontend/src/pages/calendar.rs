use bounce::use_atom_value;
use chrono::{Datelike, Local, Month, Months, NaiveDate};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routing::SheefRoute;
use crate::hooks::event_source::use_event_source;
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
    let loading_state = use_state(|| false);
    let available_state = use_state(|| props.available);

    let time_state = use_state(|| props.time.clone());

    let on_close = props.on_close.clone();

    let on_date_save = {
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        let available_state = available_state.clone();

        let time_state = time_state.clone();

        let date = props.date;

        let on_close = on_close.clone();

        Callback::from(move |evt: SubmitEvent| {
            log::debug!("The form for updating event was submitted");
            evt.prevent_default();
            loading_state.set(true);

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            let available_state = available_state.clone();

            let time_state = time_state.clone();

            let on_close = on_close.clone();

            yew::platform::spawn_local(async move {
                log::debug!("Save the data in the system");
                loading_state.set(false);
            });
        })
    };

    let update_time = use_callback(move |evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), time_state.clone());
    let update_available = use_callback(move |evt: MouseEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().checked()), available_state.clone());

    html!(
        <PicoModal open={true} title={format!("Planung für {}. {} {}", props.date.day(), month_to_german(props.date.month()), props.date.year())} on_close={props.on_close.clone()} buttons={
            html!(
                <>
                    <button type="button" onclick={move |_| on_close.clone().emit(())} class="secondary" role="button">{"Schließen"}</button>
                    <button aria-busy={AttrValue::from((*loading_state).to_string())} form={format!("form-{}", props.date.format("%Y-%m-%d"))} type="submit" role="button">{"Meine Verfügbarkeit speichern"}</button>
                </>
            )
        }>
            <form id={format!("form-{}", props.date.format("%Y-%m-%d"))} onsubmit={on_date_save}>
                if *error_state {
                    <p data-msg="negative">{"Leider konnte deine Planung nicht gespeichert werden, bitte wende dich an Azami"}</p>
                }
                <fieldset>
                    <label for="available">
                        <input readonly={*loading_state} onclick={update_available} type="checkbox" id="available" name="available" role="switch" checked={*available_state} />
                        {format!("Ich kann am {}. {} {}", props.date.day(), month_to_german(props.date.month()), props.date.year())}
                    </label>
                </fieldset>
                <label for="timeAvailable">{"Uhrzeit (optional)"}</label>
                <input readonly={*loading_state} oninput={update_time} type="text" id="timeAvailable" name="timeAvailable" value={(*time_state).clone()} />
            </form>
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
                if !props.available.is_empty() {
                    <strong>{"Kann"}</strong>
                    <p>{props.available.clone()}</p>
                }
                if !props.unavailable.is_empty() {
                    <strong>{"Kann nicht"}</strong>
                    <p>{props.unavailable.clone()}</p>
                }
            </details>
            if *modal_open_state {
                <UpdateDayModal date={props.date} time={props.time.clone()} available={props.me_available} on_close={on_close} />
            }
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
        return html!(<Redirect<SheefRoute> to={SheefRoute::Home} />);
    };

    let prev_month = date - Months::new(1);
    let next_month = date + Months::new(1);

    let state = use_state_eq(|| vec![] as Vec<DayProps>);
    let initially_loaded_state = use_state_eq(|| false);
    let current_user = use_atom_value::<CurrentUser>();

    use_event_source("/sse/calendar".to_string(), |_| {});

    html!(
        <>
            <h1>{"Event Kalender"}</h1>
            <div class="calendar-header">
                <a>{month_to_german(prev_month.month())}</a>
                <h4>{format!("{} {}", month_to_german(date.month()), date.year())}</h4>
                <a>{month_to_german(next_month.month())}</a>
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

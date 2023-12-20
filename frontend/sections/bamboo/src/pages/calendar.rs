use std::ops::Deref;

use bounce::query::use_query_value;
use chrono::prelude::*;
use chrono::{Days, Months};
use date_range::DateRange;
use stylist::yew::use_style;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventSource, MessageEvent};
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::prelude::{use_bool_toggle, use_effect_update, use_list, use_unmount};
use yew_icons::Icon;

use bamboo_entities::prelude::Event;
use bamboo_frontend_base_error as error;

use crate::api;
use crate::models;
use crate::props::calendar::*;

enum ColorYiqResult {
    Light,
    Dark,
}

impl ToString for ColorYiqResult {
    fn to_string(&self) -> String {
        match self {
            ColorYiqResult::Light => "#ffffff",
            ColorYiqResult::Dark => "#333333",
        }
        .to_string()
    }
}

#[derive(Clone)]
struct CalendarEventSource {
    event_source: Option<web_sys::EventSource>,
}

impl CalendarEventSource {
    fn new() -> Self {
        let event_source = if let Ok(event_source) = EventSource::new("/sse/event").map_err(|err| {
            log::warn!(
                "Failed to start event source, automatic calendar updates disabled: {err:?}"
            );
        }) {
            let open_handler: Closure<dyn Fn()> = Closure::new(|| {
                log::debug!("Calendar connected");
            });
            event_source.set_onopen(Some(open_handler.as_ref().unchecked_ref()));
            open_handler.forget();

            Some(event_source)
        } else {
            None
        };

        Self { event_source }
    }

    fn register_handler(&self, event: impl Into<String>, callback: Callback<Event>) {
        let message_handler: Closure<dyn Fn(MessageEvent)> =
            Closure::new(move |evt: MessageEvent| {
                log::debug!("New message received");
                let data = evt.data();
                if let Some(data) = data.as_string() {
                    log::debug!("The data received: {data:?}");
                    if let Ok(event) = serde_json::from_str::<Event>(data.as_str()) {
                        log::debug!("Decoded the message {:#?}", event.clone());
                        callback.emit(event);
                    }
                }
            });

        if let Some(source) = self.event_source.clone() {
            let event = event.into();
            log::debug!("Register handler for event {}", event.clone());
            if let Err(err) = source.add_event_listener_with_callback(
                event.as_str(),
                message_handler.as_ref().unchecked_ref(),
            ) {
                log::error!("Failed to register event listener {err:#?}");
            }
            message_handler.forget();
        }
    }

    fn close(&self) {
        if let Some(source) = self.event_source.clone() {
            source.close();
        }
    }
}

fn color_yiq(color: Color) -> ColorYiqResult {
    let yiq =
        ((color.red() as u32 * 299) + (color.green() as u32 * 587) + (color.blue() as u32 * 114))
            / 1000;

    if yiq >= 128 {
        ColorYiqResult::Dark
    } else {
        ColorYiqResult::Light
    }
}

#[function_component(AddEventDialog)]
fn add_event_dialog(props: &AddEventDialogProps) -> Html {
    let title_state = use_state_eq(|| AttrValue::from(""));
    let description_state = use_state_eq(|| AttrValue::from(""));

    let end_date_state = use_state_eq(|| props.start_date);

    let color_state = use_state_eq(Color::random);

    let is_private_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    {
        let error_state = error_state.clone();
        let is_private_state = is_private_state.clone();

        let title_state = title_state.clone();
        let description_state = description_state.clone();

        let color_state = color_state.clone();

        use_unmount(move || {
            error_state.set(false);
            is_private_state.set(false);

            title_state.set("".into());
            description_state.set("".into());

            color_state.set(Color::random())
        })
    }

    let title_input = use_callback(title_state.clone(), |value, state| state.set(value));
    let description_input =
        use_callback(description_state.clone(), |value, state| state.set(value));
    let end_date_input = use_callback(end_date_state.clone(), |value, state| state.set(value));
    let color_input = use_callback(color_state.clone(), |value, state| state.set(value));
    let is_private_checked =
        use_callback(is_private_state.clone(), |value, state| state.set(value));

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unknown_error_state.clone()),
        |_, (bamboo_error_state, unknown_error_state)| {
            error::report_unknown_error(
                "bamboo_calendar",
                "add_event_dialog",
                bamboo_error_state.deref().clone(),
            );
            unknown_error_state.set(false);
        },
    );

    let on_form_submit = {
        let title_state = title_state.clone();
        let description_state = description_state.clone();

        let end_date_state = end_date_state.clone();

        let color_state = color_state.clone();

        let is_private_state = is_private_state.clone();
        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let start_date = props.start_date;

        let on_added = props.on_added.clone();

        Callback::from(move |_| {
            let title_state = title_state.clone();
            let description_state = description_state.clone();

            let end_date_state = end_date_state.clone();

            let color_state = color_state.clone();

            let is_private_state = is_private_state.clone();
            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let on_added = on_added.clone();

            yew::platform::spawn_local(async move {
                match api::create_event(Event::new(
                    (*title_state).to_string(),
                    (*description_state).to_string(),
                    start_date,
                    *end_date_state,
                    *color_state,
                    *is_private_state,
                ))
                .await
                {
                    Ok(evt) => {
                        on_added.emit(evt);
                        unknown_error_state.set(false);
                    }
                    Err(err) => {
                        log::error!("Failed to create event {err}");
                        error_state.set(true);
                        unknown_error_state.set(true);
                        bamboo_error_state.set(err.clone());
                    }
                }
            })
        })
    };

    html!(
        <>
            <CosmoModal title="Event hinzufügen" on_form_submit={on_form_submit} is_form={true} buttons={html!(
                <>
                    <CosmoButton label="Abbrechen" on_click={props.on_cancel.clone()} />
                    <CosmoButton label="Event speichern" is_submit={true} />
                </>
            )}>
                if *error_state {
                    if *unknown_error_state {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Event konnte leider nicht erstellt werden" header="Fehler beim Speichern" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Event konnte leider nicht erstellt werden" header="Fehler beim Speichern" />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox width={CosmoInputWidth::Medium} label="Titel" value={(*title_state).clone()} on_input={title_input} />
                    <CosmoTextArea width={CosmoInputWidth::Medium} label="Beschreibung" value={(*description_state).clone()} on_input={description_input} />
                    <CosmoColorPicker width={CosmoInputWidth::Medium} label="Farbe" value={*color_state} on_input={color_input} />
                    <CosmoDatePicker width={CosmoInputWidth::Medium} label="Von" value={props.start_date} readonly={true} on_input={|_| {}} />
                    <CosmoDatePicker width={CosmoInputWidth::Medium} label="Bis" min={props.start_date} value={*end_date_state} on_input={end_date_input} />
                    <CosmoSwitch label="Nur für mich" checked={*is_private_state} on_check={is_private_checked} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[function_component(EditEventDialog)]
fn edit_event_dialog(props: &EditEventDialogProps) -> Html {
    let title_state = use_state_eq(|| AttrValue::from(props.event.title.clone()));
    let description_state = use_state_eq(|| AttrValue::from(props.event.description.clone()));

    let color_state = use_state_eq(|| props.event.color());

    let end_date_state = use_state_eq(|| props.event.end_date);

    let delete_event_open_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let delete_error_state = use_state_eq(|| false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    {
        let error_state = error_state.clone();

        let title_state = title_state.clone();
        let description_state = description_state.clone();

        use_unmount(move || {
            error_state.set(false);

            title_state.set("".into());
            description_state.set("".into());
        })
    }

    let title_input = use_callback(title_state.clone(), |value, state| state.set(value));
    let end_date_input = use_callback(end_date_state.clone(), |value, state| state.set(value));
    let description_input =
        use_callback(description_state.clone(), |value, state| state.set(value));
    let color_input = use_callback(color_state.clone(), |value, state| state.set(value));

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unknown_error_state.clone()),
        |_, (bamboo_error_state, unknown_error_state)| {
            error::report_unknown_error(
                "bamboo_calendar",
                "edit_event_dialog",
                bamboo_error_state.deref().clone(),
            );
            unknown_error_state.set(false);
        },
    );

    let on_form_submit = {
        let title_state = title_state.clone();
        let description_state = description_state.clone();

        let color_state = color_state.clone();

        let end_date_state = end_date_state.clone();

        let error_state = error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let event = props.event.clone();

        let on_updated = props.on_updated.clone();

        Callback::from(move |_| {
            let title_state = title_state.clone();
            let description_state = description_state.clone();

            let color_state = color_state.clone();

            let end_date_state = end_date_state.clone();

            let error_state = error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let event = event.clone();

            let on_updated = on_updated.clone();

            yew::platform::spawn_local(async move {
                let mut evt = Event::new(
                    (*title_state).to_string(),
                    (*description_state).to_string(),
                    event.start_date,
                    *end_date_state,
                    *color_state,
                    event.is_private,
                );
                evt.id = event.id;

                match api::update_event(event.id, evt.clone()).await {
                    Ok(_) => {
                        on_updated.emit(evt);
                        unknown_error_state.set(false);
                    }
                    Err(err) => {
                        log::error!("Failed to update event {} {err}", event.id);
                        error_state.set(true);
                        unknown_error_state.set(true);
                        bamboo_error_state.set(err.clone());
                    }
                }
            })
        })
    };
    let on_delete_confirm = {
        let id = props.event.id;

        let event = props.event.clone();

        let delete_error_state = delete_error_state.clone();
        let unknown_error_state = unknown_error_state.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let on_deleted = props.on_deleted.clone();

        Callback::from(move |_| {
            let delete_error_state = delete_error_state.clone();
            let unknown_error_state = unknown_error_state.clone();

            let bamboo_error_state = bamboo_error_state.clone();

            let event = event.clone();
            let on_deleted = on_deleted.clone();

            yew::platform::spawn_local(async move {
                match api::delete_event(id).await {
                    Ok(_) => {
                        on_deleted.emit(event);
                        unknown_error_state.set(false);
                    }
                    Err(err) => {
                        log::error!("Failed to update event {id} {err}");
                        delete_error_state.set(true);
                        unknown_error_state.set(true);
                        bamboo_error_state.set(err.clone());
                    }
                }
            })
        })
    };

    let on_open_delete = use_callback(delete_event_open_state.clone(), |_, state| state.set(true));
    let on_delete_decline =
        use_callback(delete_event_open_state.clone(), |_, state| state.set(false));

    log::debug!("Color {}", props.event.color().hex());
    log::debug!("Color string {}", props.event.color.clone());

    html!(
        <>
            <CosmoModal title="Event bearbeiten" on_form_submit={on_form_submit} is_form={true} buttons={html!(
                <>
                    <CosmoButton state={CosmoButtonType::Negative} label="Event löschen" on_click={on_open_delete} />
                    <CosmoButton label="Abbrechen" on_click={props.on_cancel.clone()} />
                    <CosmoButton label="Event speichern" is_submit={true} />
                </>
            )}>
                if *error_state {
                    if *unknown_error_state {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Event konnte leider nicht geändert werden" header="Fehler beim Speichern" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Event konnte leider nicht geändert werden" header="Fehler beim Speichern" />
                    }
                }
                if *delete_error_state {
                    if *unknown_error_state {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Event konnte leider nicht gelöscht werden" header="Fehler beim Löschen" actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                    } else {
                        <CosmoMessage message_type={CosmoMessageType::Negative} message="Das Event konnte leider nicht gelöscht werden" header="Fehler beim Löschen" />
                    }
                }
                <CosmoInputGroup>
                    <CosmoTextBox width={CosmoInputWidth::Medium} label="Titel" value={(*title_state).clone()} on_input={title_input} />
                    <CosmoTextArea width={CosmoInputWidth::Medium} label="Beschreibung" value={(*description_state).clone()} on_input={description_input} />
                    <CosmoColorPicker width={CosmoInputWidth::Medium} label="Farbe" value={*color_state} on_input={color_input} />
                    <CosmoDatePicker width={CosmoInputWidth::Medium} label="Von" value={props.event.start_date} readonly={true} on_input={|_| {}} />
                    <CosmoDatePicker width={CosmoInputWidth::Medium} label="Bis" min={props.event.start_date} value={*end_date_state} on_input={end_date_input} />
                </CosmoInputGroup>
            </CosmoModal>
            if *delete_event_open_state {
                <CosmoConfirm confirm_type={CosmoModalType::Warning} title="Event löschen" message={format!("Soll das Event {} wirklich gelöscht werden?", props.event.title.clone())} confirm_label="Event löschen" decline_label="Nicht löschen" on_confirm={on_delete_confirm} on_decline={on_delete_decline} />
            }
        </>
    )
}

#[function_component(EventEntry)]
fn event_entry(props: &EventEntryProps) -> Html {
    let event_style = use_style!(
        r#"
background-color: ${event_color};
padding: 0.125rem 0.25rem;
box-sizing: border-box;
color: ${color};
font-size: 1rem;
font-weight: var(--font-weight-normal);
cursor: pointer;
position: relative;
display: flex;
justify-content: space-between;
align-items: center;

&:hover .panda-calendar-edit {
    opacity: 1;
}"#,
        event_color = props.event.color().hex(),
        color = color_yiq(props.event.color()).to_string(),
    );
    let hover_style = use_style!(
        r#"
&:hover::before {
    content: attr(data-description);
    position: absolute;
    background-color: ${event_color};
    color: ${color};
    font-weight: var(--font-weight-normal);
    white-space: pre-wrap;
    font-size: 1rem;
    bottom: 2rem;
    left: 50%;
    width: 18.75rem;
    transform: translate(-50%);
    padding: 0.125rem 0.25rem;
    box-sizing: border-box;
    z-index: 2;
}

&:hover::after {
    content: "";
    position: absolute;
    border: 0.5rem solid transparent;
    border-top-color: ${event_color};
    bottom: 1.25rem;
    left: 50%;
    transform: translate(-50%);
    z-index: 2;
}"#,
        event_color = props.event.color().hex(),
        color = color_yiq(props.event.color()).to_string(),
    );
    let edit_style = use_style!(
        r#"
opacity: 0;
transition: all 0.1s;
text-decoration: none;
stroke: ${color};
cursor: pointer;"#,
        color = color_yiq(props.event.color()).to_string(),
    );

    let classes = if props.event.description.is_empty() {
        classes!(event_style)
    } else {
        classes!(event_style, hover_style)
    };

    let edit_open_state = use_state_eq(|| false);
    let on_updated = use_callback(
        (edit_open_state.clone(), props.on_updated.clone()),
        |event, (state, on_updated)| {
            state.set(false);
            on_updated.emit(event);
        },
    );
    let on_deleted = use_callback(
        (
            edit_open_state.clone(),
            props.on_deleted.clone(),
            props.event.clone(),
        ),
        |_, (state, on_deleted, event)| {
            state.set(false);
            on_deleted.emit(event.clone());
        },
    );
    let on_cancel = use_callback(edit_open_state.clone(), |_, state| {
        state.set(false);
    });

    html!(
        <>
            if *edit_open_state {
                <EditEventDialog event={props.event.clone()} on_updated={on_updated} on_deleted={on_deleted} on_cancel={on_cancel} />
            }
            <span class={classes} data-description={props.event.description.clone()}>
                {props.event.title.clone()}
                <a onclick={move |_| edit_open_state.set(true)}>
                    <Icon icon_id={IconId::LucidePencil} width="16px" height="16px" class={classes!(edit_style, "panda-calendar-edit")} />
                </a>
            </span>
        </>
    )
}

#[function_component(Day)]
fn day(props: &DayProps) -> Html {
    let add_event_open_state = use_state_eq(|| false);
    let background_color = if props.selected_month == props.month {
        "transparent"
    } else {
        "var(--day-background-past-month)"
    };
    let today = Local::now().date_naive();
    let day_number_color =
        if today.month() == props.month && today.day() == props.day && today.year() == props.year {
            "var(--black)"
        } else {
            "var(--menu-text-color)"
        };

    let style = use_style!(
        r#"
border-top: 0.0625rem solid var(--primary-color);
border-left: 0.0625rem solid var(--primary-color);
background: ${background_color};
position: relative;
box-sizing: border-box;
padding: 0.125rem;
gap: 0.125rem;
display: grid;
grid-template-rows: auto;
align-content: end;

--day-background-past-month: #0000000F;

@media screen and (prefers-color-scheme: dark) {
    --day-background-past-month: #FFFFFF1D;
}

&:nth-child(7n) {
    border-right: 0.0625rem solid var(--primary-color);
}

&:nth-child(43),
&:nth-child(44),
&:nth-child(45),
&:nth-child(46),
&:nth-child(47),
&:nth-child(48),
&:nth-child(49) {
    border-bottom: 0.0625rem solid var(--primary-color);
}

&::before {
    content: "${day}";
    position: absolute;
    top: 0.25rem;
    right: 0.25rem;
    font-size: 1.75rem;
    color: ${day_number_color};
    font-weight: var(--font-weight-bold);
    z-index: 1;
}

&:hover .panda-calendar-add {
    opacity: 1;
}"#,
        background_color = background_color,
        day = props.day,
        day_number_color = day_number_color,
    );
    let add_style = use_style!(
        r#"
opacity: 0;
transition: all 0.1s;
text-decoration: none;
position: absolute;
top: 0.125rem;
left: 0.125rem;
stroke: var(--black);
cursor: pointer;
z-index: 1;
    "#
    );

    let on_added = use_callback(
        (add_event_open_state.clone(), props.on_added.clone()),
        |event, (state, on_added)| {
            state.set(false);
            on_added.emit(event);
        },
    );
    let on_cancel = use_callback(add_event_open_state.clone(), |_, state| {
        state.set(false);
    });

    html!(
        <>
            if *add_event_open_state {
                <AddEventDialog start_date={NaiveDate::from_ymd_opt(props.year, props.month, props.day).unwrap()} on_added={on_added} on_cancel={on_cancel} />
            }
            <div class={classes!(style)}>
                <Icon onclick={move |_| add_event_open_state.set(true)} icon_id={IconId::LucideCalendarPlus} class={classes!(add_style, "panda-calendar-add")} />
                {for props.events.iter().map(move |evt| html!(
                    <EventEntry on_updated={props.on_updated.clone()} on_deleted={props.on_deleted.clone()} key={evt.id} event={evt.clone()} />
                ))}
            </div>
        </>
    )
}

#[function_component(CalendarData)]
fn calendar_data(props: &CalendarProps) -> Html {
    log::debug!("Render CalendarData");
    let first_day_of_month = props.date;
    log::debug!("First day of month {}", first_day_of_month.clone());

    let first_day_offset = first_day_of_month.weekday() as i64 - 1;
    let first_day_offset = if first_day_offset < 0 {
        0
    } else {
        first_day_offset
    } as u64;

    let last_day_of_month = first_day_of_month
        .checked_add_months(Months::new(1))
        .unwrap()
        .checked_sub_days(Days::new(1))
        .unwrap();
    log::debug!("Last day of month {}", last_day_of_month.clone());

    let last_day_of_prev_month = first_day_of_month.checked_sub_days(Days::new(1)).unwrap();
    log::debug!("Last day of prev month {}", last_day_of_prev_month.clone());

    let offset_days = Days::new(first_day_offset);
    log::debug!("Days to take from last month: {offset_days:#?}");

    let calendar_start_date = last_day_of_prev_month
        .checked_sub_days(offset_days)
        .unwrap();

    let total_days = first_day_offset + last_day_of_month.day() as u64;
    let days_of_next_month = if first_day_offset == 0 {
        40 - total_days + 1
    } else {
        40 - total_days
    };

    let first_day_of_next_month = first_day_of_month
        .checked_add_months(Months::new(1))
        .unwrap();
    let calendar_end_date = first_day_of_next_month
        .checked_add_days(Days::new(days_of_next_month))
        .unwrap();

    let selected_month = first_day_of_month.month();

    log::debug!("First day of month {first_day_of_month:?}");
    log::debug!("Last day of month {last_day_of_month:?}");
    log::debug!("First day of calendar {calendar_start_date:?}");
    log::debug!("Last day of prev month {first_day_of_month:?}");
    log::debug!("First day of next month {first_day_of_next_month:?}");
    log::debug!("Last day of calendar {calendar_end_date:?}");

    let initial_loaded_toggle = use_bool_toggle(false);
    let loaded_toggle = use_bool_toggle(false);
    let event_source_connected_toggle = use_bool_toggle(false);
    let unknown_error_state = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(api::ApiError::default);

    let events_list = use_list(vec![] as Vec<Event>);
    let calendar_event_source_state = use_mut_ref(CalendarEventSource::new);

    let props_date_memo = use_memo(props.date, |date| *date);
    let range_memo = use_memo((calendar_start_date, calendar_end_date), |(start, end)| {
        DateRange::new(*start, *end).unwrap()
    });

    let event_query_state = use_query_value::<models::EventRange>(range_memo.clone());
    let event_created = use_callback(
        (events_list.clone(), range_memo.clone()),
        |event: Event, (events_list, range_memo)| {
            log::debug!(
                "Someone created a new event, adding it to the list if it is in current range"
            );
            log::debug!("Got event {event:?}");
            let until = range_memo.until();
            let since = range_memo.since();

            if (event.start_date >= since && event.start_date <= until)
                || (event.end_date >= since && event.end_date <= until)
            {
                log::debug!("The event is in range, lets add it to the list");
                events_list.push(event.clone());
            }
        },
    );
    let event_updated = use_callback(
        (events_list.clone(), range_memo.clone()),
        |event: Event, (events_list, range_memo)| {
            log::debug!("Someone updated an event, if we have it loaded, lets update it");
            log::debug!("Got event {event:?}");
            let until = range_memo.until();
            let since = range_memo.since();

            if (event.start_date >= since && event.start_date <= until)
                || (event.end_date >= since && event.end_date <= until)
            {
                log::debug!("The event is in range");

                let event_id = event.id;
                log::debug!("First remove the event from the list");
                events_list.retain(|evt| evt.id != event_id);

                log::debug!("Then add it to the list again");
                events_list.push(event.clone());
            }
        },
    );
    let event_deleted = use_callback(events_list.clone(), |event: Event, events_list| {
        log::debug!("Got event {event:?}");
        let event_id = event.id;

        log::debug!(
            "Currently {} events are loaded",
            events_list.current().len()
        );
        events_list.retain(|evt| evt.id != event_id);
        log::debug!(
            "After delete {} events are loaded",
            events_list.current().len()
        );
    });

    {
        let calendar_event_source_state = calendar_event_source_state.clone();
        use_unmount(move || {
            let source = calendar_event_source_state.borrow().clone();
            source.close();
        });
    }

    let error_message_style = use_style!(
        r#"
grid-column: span 7;
grid-row: 3/4;
    "#
    );
    let progress_ring_style = use_style!(
        r#"
grid-column: span 7;
grid-row: 3/4;
    "#
    );

    {
        let event_query_state = event_query_state.clone();
        let props = props.clone();
        let loaded_toggle = loaded_toggle.clone();

        use_effect_update(move || {
            if *props_date_memo != props.date {
                loaded_toggle.set(false);

                yew::platform::spawn_local(async move {
                    let _ = event_query_state.refresh().await;
                });
            }

            || ()
        })
    }

    let on_created = use_callback(
        (event_created.clone(), *event_source_connected_toggle),
        |event, (cb, connected)| {
            if !connected {
                cb.emit(event);
            }
        },
    );
    let on_updated = use_callback(
        (event_updated.clone(), *event_source_connected_toggle),
        |event, (cb, connected)| {
            if !connected {
                cb.emit(event);
            }
        },
    );
    let on_deleted = use_callback(
        (event_deleted.clone(), *event_source_connected_toggle),
        |event, (cb, connected)| {
            if !connected {
                cb.emit(event);
            }
        },
    );

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unknown_error_state.clone()),
        |_, (bamboo_error_state, unknown_error_state)| {
            error::report_unknown_error(
                "bamboo_calendar",
                "calendar_data",
                bamboo_error_state.deref().clone(),
            );
            unknown_error_state.set(false);
        },
    );

    if !*loaded_toggle {
        match event_query_state.result() {
            None => {
                log::debug!("Still loading");
                if !*initial_loaded_toggle {
                    return html!(
                        <div class={progress_ring_style}>
                            <CosmoProgressRing />
                        </div>
                    );
                }
            }
            Some(Ok(res)) => {
                log::debug!("Loaded events");
                if !*initial_loaded_toggle {
                    log::debug!("Start event source for calendar on /sse/event");
                    let source = calendar_event_source_state.borrow().clone();
                    source.register_handler("created", event_created.clone());
                    source.register_handler("updated", event_updated.clone());
                    source.register_handler("deleted", event_deleted.clone());
                    event_source_connected_toggle.set(true);
                }

                events_list.set(res.events.clone());
                initial_loaded_toggle.set(true);
                loaded_toggle.set(true);
            }
            Some(Err(err)) => {
                log::warn!("Failed to load {err}");
                bamboo_error_state.set(err.clone());
                if !*initial_loaded_toggle {
                    unknown_error_state.set(true);
                }
                initial_loaded_toggle.set(true);

                return html!(
                    <div class={error_message_style}>
                        if *unknown_error_state {
                            <CosmoMessage header="Fehler beim Laden" message="Der Event Kalender konnte nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
                        } else {
                            <CosmoMessage header="Fehler beim Laden" message="Der Event Kalender konnte nicht geladen werden" message_type={CosmoMessageType::Negative} />
                        }
                    </div>
                );
            }
        }
    }

    let events_for_day = {
        move |day: NaiveDate| {
            events_list
                .current()
                .iter()
                .filter(move |event| event.start_date <= day && event.end_date >= day)
                .cloned()
                .collect::<Vec<Event>>()
        }
    };

    let render_day = move |day| {
        let on_created = on_created.clone();
        let on_updated = on_updated.clone();
        let on_deleted = on_deleted.clone();

        html!(
            <Day on_updated={on_updated} on_added={on_created} on_deleted={on_deleted} events={events_for_day(day)} key={day.format("%F").to_string()} day={day.day()} month={day.month()} year={day.year()} selected_month={selected_month} />
        )
    };

    html!(
        <>
            if first_day_offset > 0 {
                {for DateRange::new(calendar_start_date, last_day_of_prev_month).unwrap().into_iter().map(render_day.clone())}
            }
            {for DateRange::new(first_day_of_month, last_day_of_month).unwrap().into_iter().map(render_day.clone())}
            {for DateRange::new(first_day_of_next_month, calendar_end_date).unwrap().into_iter().map(render_day.clone())}
        </>
    )
}

#[function_component(CalendarPage)]
pub fn calendar_page() -> Html {
    log::debug!("Render calendar page");
    let date_state = use_state_eq(|| Local::now().date_naive().with_day(1).unwrap());

    let prev_month = *date_state - Months::new(1);
    let next_month = *date_state + Months::new(1);

    let calendar_container_style = use_style!(
        r#"
display: grid;
grid-template-columns: repeat(7, 1fr);
grid-template-rows: auto repeat(6, 1fr);
height: calc(var(--page-height) - var(--title-font-size) - 4.5rem);
    "#
    );
    let calendar_header_style = use_style!(
        r#"
display: flex;
justify-content: space-between;
align-items: baseline;
margin-top: 1rem;
margin-bottom: 1rem;

h2 {
    margin: 0;
    flex: 0 0 calc(100% / 3);
    min-width: calc(100% / 3);
    text-align: center;
}
    "#
    );
    let calendar_action_style = use_style!(
        r#"
font-size: 1.5rem;
font-weight: var(--font-weight-light);
color: var(--primary-color);
text-decoration: none;
cursor: pointer;
flex: 0 0 calc(100% / 3);
min-width: calc(100% / 3);
    "#
    );
    let calendar_action_prev_style = use_style!(
        r#"
text-align: left;
    "#
    );
    let calendar_action_next_style = use_style!(
        r#"
text-align: right;
    "#
    );
    let calendar_weekday_style = use_style!(
        r#"
font-size: 1.25rem;
font-weight: var(--font-weight-light);
color: var(--primary-color);
grid-row: 1/2;
text-align: center;
    "#
    );

    let move_prev = use_callback(date_state.clone(), |_: MouseEvent, date_state| {
        date_state.set((*date_state).checked_sub_months(Months::new(1)).unwrap())
    });
    let move_next = use_callback(date_state.clone(), |_: MouseEvent, date_state| {
        date_state.set((*date_state).checked_add_months(Months::new(1)).unwrap())
    });

    html!(
        <>
            <CosmoTitle title="Event Kalender" />
            <div class={calendar_header_style}>
                <span class={classes!(calendar_action_style.clone(), calendar_action_prev_style)}>
                    <a onclick={move_prev}>{prev_month.format_localized("%B %Y", Locale::de_DE).to_string()}</a>
                </span>
                <CosmoHeader level={CosmoHeaderLevel::H2} header={(*date_state).format_localized("%B %Y", Locale::de_DE).to_string()} />
                <span class={classes!(calendar_action_style.clone(), calendar_action_next_style)}>
                    <a onclick={move_next}>{next_month.format_localized("%B %Y", Locale::de_DE).to_string()}</a>
                </span>
            </div>
            <div class={calendar_container_style}>
                <div class={calendar_weekday_style.clone()}>{"Montag"}</div>
                <div class={calendar_weekday_style.clone()}>{"Dienstag"}</div>
                <div class={calendar_weekday_style.clone()}>{"Mittwoch"}</div>
                <div class={calendar_weekday_style.clone()}>{"Donnerstag"}</div>
                <div class={calendar_weekday_style.clone()}>{"Freitag"}</div>
                <div class={calendar_weekday_style.clone()}>{"Samstag"}</div>
                <div class={calendar_weekday_style}>{"Sonntag"}</div>

                <CalendarData date={*date_state} />
            </div>
        </>
    )
}

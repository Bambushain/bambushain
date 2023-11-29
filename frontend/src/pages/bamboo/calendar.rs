use bounce::query::use_query_value;
use chrono::prelude::*;
use chrono::{Days, Months};
use date_range::DateRange;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::use_effect_update;
use yew_icons::Icon;

use bamboo_entities::prelude::Event;

use crate::api;
use crate::api::event::EventRange;
use crate::hooks::event_source::use_event_source;

#[derive(Properties, PartialEq, Clone, Default)]
struct DayProps {
    day: u32,
    month: u32,
    year: i32,
    selected_month: u32,
    events: Vec<Event>,
    on_reload: Callback<()>,
}

#[derive(Properties, PartialEq, Clone, Default)]
struct EventEntryProps {
    event: Event,
    on_reload: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
struct CalendarProps {
    date: NaiveDate,
}

#[derive(Properties, PartialEq, Clone)]
struct AddEventDialogProps {
    start_date: NaiveDate,
    on_added: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
struct EditEventDialogProps {
    event: Event,
    on_saved: Callback<()>,
}

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

    let error_state = use_state_eq(|| false);

    let title_input = use_callback(title_state.clone(), |value, state| state.set(value));
    let description_input =
        use_callback(description_state.clone(), |value, state| state.set(value));
    let end_date_input = use_callback(end_date_state.clone(), |value, state| state.set(value));
    let color_input = use_callback(color_state.clone(), |value, state| state.set(value));

    let on_form_submit = {
        let title_state = title_state.clone();
        let description_state = description_state.clone();

        let end_date_state = end_date_state.clone();

        let color_state = color_state.clone();

        let error_state = error_state.clone();

        let start_date = props.start_date;

        let on_added = props.on_added.clone();

        Callback::from(move |_| {
            let title_state = title_state.clone();
            let description_state = description_state.clone();

            let end_date_state = end_date_state.clone();

            let color_state = color_state.clone();

            let error_state = error_state.clone();

            let on_added = on_added.clone();

            yew::platform::spawn_local(async move {
                match api::create_event(Event::new(
                    (*title_state).to_string(),
                    (*description_state).to_string(),
                    start_date,
                    *end_date_state,
                    *color_state,
                ))
                .await
                {
                    Ok(_) => on_added.emit(()),
                    Err(err) => {
                        log::error!("Failed to create event {err}");
                        error_state.set(true);
                    }
                }
            })
        })
    };

    html!(
        <>
            <CosmoModal title="Event hinzufügen" on_form_submit={on_form_submit} is_form={true} buttons={html!(
                <>
                    <CosmoButton label="Abbrechen" on_click={props.on_added.clone()} />
                    <CosmoButton label="Event speichern" is_submit={true} />
                </>
            )}>
                <CosmoInputGroup>
                    <CosmoTextBox width={CosmoInputWidth::Medium} label="Titel" value={(*title_state).clone()} on_input={title_input} />
                    <CosmoTextArea width={CosmoInputWidth::Medium} label="Beschreibung" value={(*description_state).clone()} on_input={description_input} />
                    <CosmoColorPicker width={CosmoInputWidth::Medium} label="Farbe" value={*color_state} on_input={color_input} />
                    <CosmoDatePicker width={CosmoInputWidth::Medium} label="Von" value={props.start_date} readonly={true} on_input={|_| {}} />
                    <CosmoDatePicker width={CosmoInputWidth::Medium} label="Bis" min={props.start_date} value={*end_date_state} on_input={end_date_input} />
                </CosmoInputGroup>
            </CosmoModal>
            if *error_state {
                <CosmoAlert alert_type={CosmoModalType::Negative} title="Fehler beim Speichern" message="Das Event konnte leider nicht erstellt werden, bitte wende dich an Azami" close_label="Schließen" on_close={move |_| error_state.set(false)} alert_type={CosmoAlertType::Negative} />
            }
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

    let title_input = use_callback(title_state.clone(), |value, state| state.set(value));
    let end_date_input = use_callback(end_date_state.clone(), |value, state| state.set(value));
    let description_input =
        use_callback(description_state.clone(), |value, state| state.set(value));
    let color_input = use_callback(color_state.clone(), |value, state| state.set(value));

    let on_form_submit = {
        let title_state = title_state.clone();
        let description_state = description_state.clone();

        let color_state = color_state.clone();

        let end_date_state = end_date_state.clone();

        let error_state = error_state.clone();

        let event = props.event.clone();

        let on_saved = props.on_saved.clone();

        Callback::from(move |_| {
            let title_state = title_state.clone();
            let description_state = description_state.clone();

            let color_state = color_state.clone();

            let end_date_state = end_date_state.clone();

            let error_state = error_state.clone();

            let event = event.clone();

            let on_saved = on_saved.clone();

            yew::platform::spawn_local(async move {
                match api::update_event(
                    event.id,
                    Event::new(
                        (*title_state).to_string(),
                        (*description_state).to_string(),
                        event.start_date,
                        *end_date_state,
                        *color_state,
                    ),
                )
                .await
                {
                    Ok(_) => on_saved.emit(()),
                    Err(err) => {
                        log::error!("Failed to update event {} {err}", event.id);
                        error_state.set(true);
                    }
                }
            })
        })
    };
    let on_delete_confirm = {
        let id = props.event.id;
        let on_saved = props.on_saved.clone();

        let delete_error_state = delete_error_state.clone();

        Callback::from(move |_| {
            let on_saved = on_saved.clone();

            let delete_error_state = delete_error_state.clone();

            yew::platform::spawn_local(async move {
                match api::delete_event(id).await {
                    Ok(_) => on_saved.emit(()),
                    Err(err) => {
                        log::error!("Failed to update event {id} {err}");
                        delete_error_state.set(true);
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
                    <CosmoButton label="Abbrechen" on_click={props.on_saved.clone()} />
                    <CosmoButton label="Event speichern" is_submit={true} />
                </>
            )}>
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
            if *error_state {
                <CosmoAlert alert_type={CosmoModalType::Negative} title="Fehler beim Speichern" message="Das Event konnte leider nicht geändert werden, bitte wende dich an Azami" close_label="Schließen" on_close={move |_| error_state.set(false)} alert_type={CosmoAlertType::Negative} />
            }
            if *delete_error_state {
                <CosmoAlert alert_type={CosmoModalType::Negative} title="Fehler beim Löschen" message="Das Event konnte leider nicht gelöscht werden, bitte wende dich an Azami" close_label="Schließen" on_close={move |_| delete_error_state.set(false)} alert_type={CosmoAlertType::Negative} />
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
    let on_saved = use_callback(
        (edit_open_state.clone(), props.on_reload.clone()),
        |_, (state, on_saved)| {
            state.set(false);
            on_saved.emit(());
        },
    );

    html!(
        <>
            if *edit_open_state {
                <EditEventDialog event={props.event.clone()} on_saved={on_saved} />
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
        (add_event_open_state.clone(), props.on_reload.clone()),
        |_, (state, callback)| {
            state.set(false);
            callback.emit(());
        },
    );

    html!(
        <>
            if *add_event_open_state {
                <AddEventDialog start_date={NaiveDate::from_ymd_opt(props.year, props.month, props.day).unwrap()} on_added={on_added} />
            }
            <div class={classes!(style)}>
                <Icon onclick={move |_| add_event_open_state.set(true)} icon_id={IconId::LucideCalendarPlus} class={classes!(add_style, "panda-calendar-add")} />
                {for props.events.iter().map(|evt| html!(<EventEntry key={evt.id} on_reload={props.on_reload.clone()} event={evt.clone()} />))}
            </div>
        </>
    )
}

#[function_component(CalendarData)]
fn calendar_data(props: &CalendarProps) -> Html {
    log::debug!("Render CalendarData");
    let first_day_of_month = props.date;
    let first_day_offset = first_day_of_month.weekday() as u64 - 1;

    let last_day_of_month = first_day_of_month
        .checked_add_months(Months::new(1))
        .unwrap()
        .checked_sub_days(Days::new(1))
        .unwrap();

    let last_day_of_prev_month = first_day_of_month.checked_sub_days(Days::new(1)).unwrap();
    let calendar_start_date = last_day_of_prev_month
        .checked_sub_days(Days::new(first_day_offset))
        .unwrap();

    let total_days = first_day_offset + last_day_of_month.day() as u64;
    let days_of_next_month = 40 - total_days;

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

    let initial_loaded_state = use_state_eq(|| false);

    let events_state = use_state_eq(|| vec![] as Vec<Event>);

    let props_date_memo = use_memo(props.date, |date| *date);
    let range_memo = use_memo((calendar_start_date, calendar_end_date), |(start, end)| {
        DateRange::new(*start, *end).unwrap()
    });

    let event_query_state = use_query_value::<EventRange>(range_memo);

    let event_source_trigger = {
        let event_query = event_query_state.clone();

        move |_| {
            log::debug!("Someone changed data on the server, trigger a refresh");
            let event_query = event_query.clone();

            yew::platform::spawn_local(async move {
                let _ = event_query.refresh().await;
            });
        }
    };

    use_event_source("/sse/event".to_string(), event_source_trigger);

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

        use_effect_update(move || {
            if *props_date_memo != props.date {
                yew::platform::spawn_local(async move {
                    let _ = event_query_state.refresh().await;
                });
            }

            || ()
        })
    }

    let on_reload = {
        let event_query_state = event_query_state.clone();

        Callback::from(move |_| {
            let event_query_state = event_query_state.clone();

            yew::platform::spawn_local(async move {
                let _ = event_query_state.refresh().await;
            })
        })
    };

    match event_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <div class={progress_ring_style}>
                        <CosmoProgressRing />
                    </div>
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded events");
            events_state.set(res.events.clone());
            initial_loaded_state.set(true);
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            return html!(
                <div class={error_message_style}>
                    <CosmoMessage header="Fehler beim Laden" message="Der Event Kalender konnte nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
                </div>
            );
        }
    }

    let events_for_day = {
        move |day: NaiveDate| {
            let all_events = (*events_state).clone();
            let mut events = vec![];
            for event in all_events {
                if event.start_date <= day && event.end_date >= day {
                    events.push(event);
                }
            }

            events
        }
    };

    let render_day = |day| {
        html!(
            <Day on_reload={on_reload.clone()} events={events_for_day(day)} key={day.format("%F").to_string()} day={day.day()} month={day.month()} year={day.year()} selected_month={selected_month} />
        )
    };

    html!(
        <>
            {for DateRange::new(calendar_start_date, last_day_of_prev_month).unwrap().into_iter().map(render_day)}
            {for DateRange::new(first_day_of_month, last_day_of_month).unwrap().into_iter().map(render_day)}
            {for DateRange::new(first_day_of_next_month, calendar_end_date).unwrap().into_iter().map(render_day)}
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

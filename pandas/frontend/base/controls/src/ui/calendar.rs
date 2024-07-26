use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::api;
use bamboo_common::core::entities::{Grove, GroveEvent};
use bamboo_common::frontend::api::ApiError;
use bamboo_pandas_frontend_base_error as error;
use chrono::prelude::*;
use chrono::{Days, Months};
use date_range::DateRange;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::prelude::{use_async, use_bool_toggle, use_mount, use_unmount};
use yew_icons::Icon;

enum ColorYiqResult {
    Light,
    Dark,
}

impl Display for ColorYiqResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ColorYiqResult::Light => "#ffffff",
            ColorYiqResult::Dark => "#333333",
        })
    }
}

fn color_yiq(color: String) -> ColorYiqResult {
    let color = Color::from_hex(color.as_str()).unwrap();
    let yiq =
        ((color.red() as u32 * 299) + (color.green() as u32 * 587) + (color.blue() as u32 * 114))
            / 1000;

    if yiq >= 128 {
        ColorYiqResult::Dark
    } else {
        ColorYiqResult::Light
    }
}

#[autoprops]
#[function_component(AddEventDialog)]
fn add_event_dialog(
    start_date: &NaiveDate,
    grove_id: Option<i32>,
    groves: &Vec<Grove>,
    on_added: &Callback<()>,
    on_cancel: &Callback<()>,
) -> Html {
    let title_state = use_state_eq(|| AttrValue::from(""));
    let description_state = use_state_eq(|| AttrValue::from(""));

    let grove_id_state = use_state_eq(|| grove_id);

    let end_date_state = use_state_eq(|| start_date.clone());

    let color_state = use_state_eq(|| AttrValue::from(Color::random().hex_full()));

    let is_private_state = use_state_eq(|| grove_id.is_none());
    let unreported_error_toggle = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(ApiError::default);

    {
        let is_private_state = is_private_state.clone();

        let title_state = title_state.clone();
        let description_state = description_state.clone();

        let color_state = color_state.clone();

        use_unmount(move || {
            is_private_state.set(false);

            title_state.set("".into());
            description_state.set("".into());

            color_state.set(AttrValue::from(Color::random().hex_full()))
        })
    }

    let save_state = {
        let title_state = title_state.clone();
        let description_state = description_state.clone();

        let end_date_state = end_date_state.clone();

        let color_state = color_state.clone();

        let grove_id_state = grove_id_state.clone();

        let is_private_state = is_private_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let start_date = *start_date;

        let groves = groves.clone();

        let on_added = on_added.clone();

        use_async(async move {
            let grove = if !*is_private_state {
                if let Some(id) = *grove_id_state {
                    groves.iter().cloned().find(|grove| grove.id == id)
                } else {
                    None
                }
            } else {
                None
            };

            api::create_event(GroveEvent::new(
                (*title_state).to_string(),
                (*description_state).to_string(),
                start_date,
                (*end_date_state).clone(),
                (*color_state).to_string(),
                *is_private_state,
                grove,
            ))
            .await
            .map(|_| {
                on_added.emit(());
                unreported_error_toggle.set(false)
            })
            .map_err(|err| {
                log::error!("Failed to create event {err}");
                unreported_error_toggle.set(true);
                bamboo_error_state.set(err.clone());

                err
            })
        })
    };

    let title_input = use_callback(title_state.clone(), |value, state| state.set(value));
    let description_input =
        use_callback(description_state.clone(), |value, state| state.set(value));
    let end_date_input = use_callback(end_date_state.clone(), |value: NaiveDate, state| {
        state.set(value.clone())
    });
    let color_input = use_callback(color_state.clone(), |value: Color, state| {
        state.set(AttrValue::from(value.hex_full()))
    });
    let is_private_checked =
        use_callback(is_private_state.clone(), |value, state| state.set(value));
    let grove_select = use_callback(grove_id_state.clone(), |value: AttrValue, state| {
        state.set(value.to_string().parse::<i32>().ok())
    });
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_calendar",
                "add_event_dialog",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    let form_submit = use_callback(save_state.clone(), |_, state| state.run());

    {
        let grove_id_state = grove_id_state.clone();
        let is_private_state = is_private_state.clone();

        let groves = groves.clone();

        use_mount(move || {
            if let Some(grove) = groves.first() {
                grove_id_state.set(Some(grove.id));
            } else {
                is_private_state.set(true);
            }
        });
    }

    html!(
        <>
            <CosmoModal
                title="Event hinzufügen"
                on_form_submit={form_submit}
                is_form=true
                buttons={html!(
                <>
                    <CosmoButton label="Abbrechen" on_click={on_cancel.clone()} />
                    <CosmoButton label="Event speichern" is_submit={true} />
                </>
            )}
            >
                if save_state.error.is_some() && *unreported_error_toggle {
                    <CosmoMessage
                        message_type={CosmoMessageType::Negative}
                        message="Das Event konnte leider nicht erstellt werden"
                        header="Fehler beim Speichern"
                        actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)}
                    />
                } else if save_state.error.is_some() {
                    <CosmoMessage
                        message_type={CosmoMessageType::Negative}
                        message="Das Event konnte leider nicht erstellt werden"
                        header="Fehler beim Speichern"
                    />
                }
                if grove_id.is_some() {
                    <CosmoInputGroup>
                        <CosmoTextBox
                            width={CosmoInputWidth::Medium}
                            label="Titel"
                            value={(*title_state).clone()}
                            on_input={title_input}
                        />
                        <CosmoTextArea
                            width={CosmoInputWidth::Medium}
                            label="Beschreibung"
                            value={(*description_state).clone()}
                            on_input={description_input}
                        />
                        <CosmoColorPicker
                            width={CosmoInputWidth::Medium}
                            label="Farbe"
                            value={Color::from_hex((*color_state).as_str()).unwrap()}
                            on_input={color_input}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Von"
                            value={*start_date}
                            readonly=true
                            on_input={|_| {}}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Bis"
                            min={*start_date}
                            value={(*end_date_state).clone()}
                            on_input={end_date_input}
                        />
                    </CosmoInputGroup>
                } else if !*is_private_state {
                    <CosmoInputGroup>
                        <CosmoTextBox
                            width={CosmoInputWidth::Medium}
                            label="Titel"
                            value={(*title_state).clone()}
                            on_input={title_input}
                        />
                        <CosmoTextArea
                            width={CosmoInputWidth::Medium}
                            label="Beschreibung"
                            value={(*description_state).clone()}
                            on_input={description_input}
                        />
                        <CosmoColorPicker
                            width={CosmoInputWidth::Medium}
                            label="Farbe"
                            value={Color::from_hex((*color_state).as_str()).unwrap()}
                            on_input={color_input}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Von"
                            value={*start_date}
                            readonly=true
                            on_input={|_| {}}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Bis"
                            min={*start_date}
                            value={(*end_date_state).clone()}
                            on_input={end_date_input}
                        />
                        <CosmoSwitch
                            label="Nur für mich"
                            checked={*is_private_state}
                            on_check={is_private_checked}
                        />
                        <CosmoModernSelect
                            label="Hain"
                            required=true
                            items={groves.iter().map(|grove| CosmoModernSelectItem::new(grove.name.clone(), grove.id.to_string(), grove.id == (*grove_id_state).unwrap_or(-1))).collect::<Vec<CosmoModernSelectItem>>()}
                            on_select={grove_select}
                        />
                    </CosmoInputGroup>
                } else {
                    <CosmoInputGroup>
                        <CosmoTextBox
                            width={CosmoInputWidth::Medium}
                            label="Titel"
                            value={(*title_state).clone()}
                            on_input={title_input}
                        />
                        <CosmoTextArea
                            width={CosmoInputWidth::Medium}
                            label="Beschreibung"
                            value={(*description_state).clone()}
                            on_input={description_input}
                        />
                        <CosmoColorPicker
                            width={CosmoInputWidth::Medium}
                            label="Farbe"
                            value={Color::from_hex((*color_state).as_str()).unwrap()}
                            on_input={color_input}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Von"
                            value={*start_date}
                            readonly=true
                            on_input={|_| {}}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Bis"
                            min={*start_date}
                            value={(*end_date_state).clone()}
                            on_input={end_date_input}
                        />
                        <CosmoSwitch
                            label="Nur für mich"
                            checked={*is_private_state}
                            on_check={is_private_checked}
                        />
                    </CosmoInputGroup>
                }
            </CosmoModal>
        </>
    )
}

#[autoprops]
#[function_component(EditEventDialog)]
fn edit_event_dialog(
    event: &GroveEvent,
    groves: &Vec<Grove>,
    on_updated: &Callback<()>,
    on_deleted: &Callback<()>,
    on_cancel: &Callback<()>,
) -> Html {
    let title_state = use_state_eq(|| AttrValue::from(event.title.clone()));
    let description_state = use_state_eq(|| AttrValue::from(event.description.clone()));

    let color_state = use_state_eq(|| AttrValue::from(event.color.clone()));

    let end_date_state = use_state_eq(|| event.end_date);

    let grove_id_state = use_state_eq(|| event.clone().grove.map(|grove| grove.id));

    let is_private_state = use_state_eq(|| event.is_private);
    let delete_event_open_state = use_state_eq(|| false);
    let unreported_error_toggle = use_state_eq(|| false);

    let bamboo_error_state = use_state_eq(ApiError::default);

    {
        let title_state = title_state.clone();
        let description_state = description_state.clone();

        use_unmount(move || {
            title_state.set("".into());
            description_state.set("".into());
        })
    }

    let save_state = {
        let title_state = title_state.clone();
        let description_state = description_state.clone();
        let color_state = color_state.clone();

        let end_date_state = end_date_state.clone();

        let grove_id_state = grove_id_state.clone();

        let is_private_state = is_private_state.clone();
        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let event = event.clone();

        let groves = groves.clone();

        let on_updated = on_updated.clone();

        use_async(async move {
            let grove = if *is_private_state {
                grove_id_state.map(|id| groves.iter().cloned().find(|grove| grove.id == id))
            } else {
                Some(None)
            }
            .unwrap();

            let mut evt = GroveEvent::new(
                (*title_state).to_string(),
                (*description_state).to_string(),
                event.start_date,
                *end_date_state,
                (*color_state).to_string(),
                *is_private_state,
                grove,
            );
            evt.id = event.id;

            api::update_event(event.id, evt.clone())
                .await
                .map(|_| {
                    on_updated.emit(());
                    unreported_error_toggle.set(false)
                })
                .map_err(|err| {
                    log::error!("Failed to update event {} {err}", event.id);
                    unreported_error_toggle.set(true);
                    bamboo_error_state.set(err.clone());
                    err
                })
        })
    };
    let delete_state = {
        let id = event.id;

        let unreported_error_toggle = unreported_error_toggle.clone();

        let bamboo_error_state = bamboo_error_state.clone();

        let on_deleted = on_deleted.clone();

        use_async(async move {
            api::delete_event(id)
                .await
                .map(|_| {
                    on_deleted.emit(());
                    unreported_error_toggle.set(false)
                })
                .map_err(|err| {
                    log::error!("Failed to update event {id} {err}");
                    unreported_error_toggle.set(true);
                    bamboo_error_state.set(err.clone());
                    err
                })
        })
    };

    let title_input = use_callback(title_state.clone(), |value, state| state.set(value));
    let end_date_input = use_callback(end_date_state.clone(), |value, state| state.set(value));
    let description_input =
        use_callback(description_state.clone(), |value, state| state.set(value));
    let color_input = use_callback(color_state.clone(), |value: Color, state| {
        state.set(AttrValue::from(value.hex_full()))
    });
    let is_private_checked =
        use_callback(is_private_state.clone(), |value, state| state.set(value));
    let grove_select = use_callback(grove_id_state.clone(), |value: AttrValue, state| {
        state.set(Some(value.to_string().parse::<i32>().unwrap()))
    });
    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_calendar",
                "edit_event_dialog",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );
    let form_submit = use_callback(save_state.clone(), |_, state| state.run());
    let delete_confirm = use_callback(delete_state.clone(), |_, state| state.run());
    let open_delete = use_callback(delete_event_open_state.clone(), |_, state| state.set(true));
    let delete_decline = use_callback(delete_event_open_state.clone(), |_, state| state.set(false));

    log::debug!("Color {}", event.color.clone());

    let grove_items = groves
        .iter()
        .map(|grove| {
            CosmoModernSelectItem::new(
                grove.name.clone(),
                grove.id.to_string(),
                grove.id == (*grove_id_state).unwrap_or(-1),
            )
        })
        .collect::<Vec<CosmoModernSelectItem>>();

    html!(
        <>
            <CosmoModal
                title="Event bearbeiten"
                on_form_submit={form_submit}
                is_form=true
                buttons={html!(
                <>
                    <CosmoButton state={CosmoButtonType::Negative} label="Event löschen" on_click={open_delete} />
                    <CosmoButton label="Abbrechen" on_click={on_cancel.clone()} />
                    <CosmoButton label="Event speichern" is_submit={true} />
                </>
            )}
            >
                if save_state.error.is_some() && *unreported_error_toggle {
                    <CosmoMessage
                        message_type={CosmoMessageType::Negative}
                        message="Das Event konnte leider nicht geändert werden"
                        header="Fehler beim Speichern"
                        actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error.clone()} />)}
                    />
                } else if save_state.error.is_some() {
                    <CosmoMessage
                        message_type={CosmoMessageType::Negative}
                        message="Das Event konnte leider nicht geändert werden"
                        header="Fehler beim Speichern"
                    />
                }
                if delete_state.error.is_some() && *unreported_error_toggle {
                    <CosmoMessage
                        message_type={CosmoMessageType::Negative}
                        message="Das Event konnte leider nicht gelöscht werden"
                        header="Fehler beim Löschen"
                        actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)}
                    />
                } else if delete_state.error.is_some() {
                    <CosmoMessage
                        message_type={CosmoMessageType::Negative}
                        message="Das Event konnte leider nicht gelöscht werden"
                        header="Fehler beim Löschen"
                    />
                }
                if event.grove.is_some() && !*is_private_state {
                    <CosmoInputGroup>
                        <CosmoTextBox
                            width={CosmoInputWidth::Medium}
                            label="Titel"
                            value={(*title_state).clone()}
                            on_input={title_input}
                        />
                        <CosmoTextArea
                            width={CosmoInputWidth::Medium}
                            label="Beschreibung"
                            value={(*description_state).clone()}
                            on_input={description_input}
                        />
                        <CosmoColorPicker
                            width={CosmoInputWidth::Medium}
                            label="Farbe"
                            value={Color::from_hex((*color_state).as_str()).unwrap()}
                            on_input={color_input}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Von"
                            value={event.start_date}
                            readonly=true
                            on_input={|_| {}}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Bis"
                            min={event.start_date}
                            value={*end_date_state}
                            on_input={end_date_input}
                        />
                        <CosmoSwitch
                            label="Nur für mich"
                            checked={*is_private_state}
                            on_check={is_private_checked}
                        />
                        <CosmoModernSelect
                            label="Hain"
                            required=true
                            items={grove_items}
                            on_select={grove_select}
                        />
                    </CosmoInputGroup>
                } else {
                    <CosmoInputGroup>
                        <CosmoTextBox
                            width={CosmoInputWidth::Medium}
                            label="Titel"
                            value={(*title_state).clone()}
                            on_input={title_input}
                        />
                        <CosmoTextArea
                            width={CosmoInputWidth::Medium}
                            label="Beschreibung"
                            value={(*description_state).clone()}
                            on_input={description_input}
                        />
                        <CosmoColorPicker
                            width={CosmoInputWidth::Medium}
                            label="Farbe"
                            value={Color::from_hex((*color_state).as_str()).unwrap()}
                            on_input={color_input}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Von"
                            value={event.start_date}
                            readonly=true
                            on_input={|_| {}}
                        />
                        <CosmoDatePicker
                            width={CosmoInputWidth::Medium}
                            label="Bis"
                            min={event.start_date}
                            value={*end_date_state}
                            on_input={end_date_input}
                        />
                        <CosmoSwitch
                            label="Nur für mich"
                            checked={*is_private_state}
                            on_check={is_private_checked}
                        />
                    </CosmoInputGroup>
                }
            </CosmoModal>
            if *delete_event_open_state {
                <CosmoConfirm
                    confirm_type={CosmoModalType::Warning}
                    title="Event löschen"
                    message={format!("Soll das Event {} wirklich gelöscht werden?", event.title.clone())}
                    confirm_label="Event löschen"
                    decline_label="Nicht löschen"
                    on_confirm={delete_confirm}
                    on_decline={delete_decline}
                />
            }
        </>
    )
}

#[autoprops]
#[function_component(EventEntry)]
fn event_entry(event: &GroveEvent, groves: &Vec<Grove>) -> Html {
    let color = event.color.clone();
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
height: 1.75rem;

&:hover .panda-calendar-edit,
&:hover .panda-calendar-description {
    opacity: 1;
    display: flex;
}"#,
        event_color = color.clone(),
        color = color_yiq(color.clone()).to_string(),
    );
    let edit_style = use_style!(
        r#"
opacity: 0;
transition: all 0.1s;
text-decoration: none;
stroke: ${color};
cursor: pointer;"#,
        color = color_yiq(color.clone()).to_string(),
    );
    let description_style = use_style!(
        r#"
opacity: 0;
transition: opacity 0.3s;
position: absolute;
background-color: ${event_color}cc;
color: ${color};
font-weight: var(--font-weight-normal);
white-space: pre-wrap;
font-size: 1rem;
bottom: 2.25rem;
padding: 0.25rem 0.5rem;
box-sizing: border-box;
z-index: 2;
margin: 0;
border: none;
display: none;
flex-flow: column;
backdrop-filter: var(--modal-backdrop-filter);
border-radius: var(--border-radius);
box-shadow: 0 0.5rem 1rem -0.25rem ${event_color}ee;
left: 50%;
transform: translate(-50%);

h3,
h5  ,
p,
span {
    margin: 0;
    padding: 0;
}
        "#,
        event_color = color.clone(),
        color = color_yiq(color.clone()).to_string(),
    );
    let arrow_style = use_style!(
        r#"
position: absolute;
overflow: hidden;
width: 1rem;
height: 1rem;
left: 50%;
transform: translate(-50%);
top: 100%;

&::after {
    content: '';
    background: ${event_color}cc;
    backdrop-filter: var(--modal-backdrop-filter);
    position: relative;
    left: 50%;
    bottom: 0.75rem;
    width: 1rem;
    height: 1rem;
    transform: translate(-50%) rotate(45deg);
    display: block;
}
        "#,
        event_color = color.clone(),
    );
    let event_header = use_style!(
        r#"
display: flex;
gap: 0.5rem;
flex-flow: row nowrap;
justify-content: start;
align-items: baseline;
white-space: nowrap;
text-overflow: ellipsis;
        "#
    );

    let edit_open_toggle = use_bool_toggle(false);
    let on_updated = use_callback(edit_open_toggle.clone(), |_, state| {
        state.set(false);
    });
    let on_deleted = use_callback(edit_open_toggle.clone(), |_, state| {
        state.set(false);
    });
    let on_cancel = use_callback(edit_open_toggle.clone(), |_, state| {
        state.set(false);
    });

    html!(
        <>
            if *edit_open_toggle {
                <EditEventDialog
                    groves={groves.clone()}
                    event={event.clone()}
                    on_updated={on_updated}
                    on_deleted={on_deleted}
                    on_cancel={on_cancel}
                />
            }
            <span class={event_style}>
                { event.title.clone() }
                <a onclick={move |_| edit_open_toggle.set(true)}>
                    <Icon
                        icon_id={IconId::LucidePencil}
                        width="16px"
                        height="16px"
                        class={classes!(edit_style, "panda-calendar-edit")}
                    />
                </a>
                <div class={classes!("panda-calendar-description", description_style.clone())}>
                    <hgroup class={event_header}>
                        <CosmoHeader level={CosmoHeaderLevel::H3} header={event.title.clone()} />
                        <CosmoHeader
                            level={CosmoHeaderLevel::H5}
                            header={if let Some(grove) = event.grove.clone() {
                                {grove.name}
                            } else {
                                "Privates Event".to_string()
                            }}
                        />
                    </hgroup>
                    <p>{ event.description.clone() }</p>
                    <span class={arrow_style} />
                </div>
            </span>
        </>
    )
}

#[autoprops]
#[function_component(Day)]
fn day(
    day: u32,
    month: u32,
    year: i32,
    selected_month: u32,
    events: &Vec<GroveEvent>,
    groves: &Vec<Grove>,
    grove_id: Option<i32>,
) -> Html {
    let add_event_open_toggle = use_bool_toggle(false);
    let background_color = if selected_month == month {
        "transparent"
    } else {
        "var(--day-background-past-month)"
    };
    let today = Local::now().date_naive();
    let day_number_color = if today.month() == month && today.day() == day && today.year() == year {
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
        day = day,
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

    let on_added = use_callback(add_event_open_toggle.clone(), |_, state| {
        state.set(false);
    });
    let on_cancel = use_callback(add_event_open_toggle.clone(), |_, state| {
        state.set(false);
    });

    html!(
        <>
            if *add_event_open_toggle {
                <AddEventDialog
                    grove_id={grove_id}
                    groves={groves.clone()}
                    start_date={NaiveDate::from_ymd_opt(year, month, day).unwrap()}
                    on_added={on_added}
                    on_cancel={on_cancel}
                />
            }
            <div class={classes!(style)}>
                <Icon
                    onclick={move |_| add_event_open_toggle.set(true)}
                    icon_id={IconId::LucideCalendarPlus}
                    class={classes!(add_style, "panda-calendar-add")}
                />
                { for events.iter().map(move |evt| html!(
                    <EventEntry groves={groves.clone()} key={evt.id} event={evt.clone()} />
                )) }
            </div>
        </>
    )
}

#[autoprops]
#[function_component(CalendarData)]
fn calendar_data(
    events: &Vec<GroveEvent>,
    groves: &Vec<Grove>,
    date: &NaiveDate,
    grove_id: Option<i32>,
) -> Html {
    log::debug!("Render CalendarData");
    let first_day_offset = date.weekday() as i64 - 1;
    let first_day_offset = if first_day_offset < 0 {
        0
    } else {
        first_day_offset
    } as u64;

    let first_day_of_month = date.clone();

    let last_day_of_month = date
        .checked_add_months(Months::new(1))
        .unwrap()
        .checked_sub_days(Days::new(1))
        .unwrap();
    log::debug!("Last day of month {}", last_day_of_month.clone());

    let last_day_of_prev_month = date.checked_sub_days(Days::new(1)).unwrap();
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

    let first_day_of_next_month = date.checked_add_months(Months::new(1)).unwrap();
    let calendar_end_date = first_day_of_next_month
        .checked_add_days(Days::new(days_of_next_month))
        .unwrap();

    let events_for_day = {
        move |day: NaiveDate| {
            events
                .iter()
                .filter(move |event| event.start_date <= day && event.end_date >= day)
                .cloned()
                .collect::<Vec<GroveEvent>>()
        }
    };

    let render_day = move |day: NaiveDate| {
        let events = events_for_day(day);
        let groves = groves.clone();

        log::debug!("{grove_id:#?}");

        html!(
            <Day
                grove_id={grove_id}
                groves={groves}
                events={events}
                key={day.format("%F").to_string()}
                day={day.day()}
                month={day.month()}
                year={day.year()}
                selected_month={date.month()}
            />
        )
    };

    html!(
        <>
            if first_day_offset > 0 {
                { for DateRange::new(calendar_start_date, last_day_of_prev_month).unwrap().into_iter().map(render_day.clone()) }
            }
            { for DateRange::new(first_day_of_month, last_day_of_month).unwrap().into_iter().map(render_day.clone()) }
            { for DateRange::new(first_day_of_next_month, calendar_end_date).unwrap().into_iter().map(render_day.clone()) }
        </>
    )
}

#[autoprops]
#[function_component(Calendar)]
pub fn calendar(
    events: &Vec<GroveEvent>,
    date: &NaiveDate,
    #[prop_or_default] grove_id: Option<i32>,
    on_navigate: Callback<NaiveDate>,
) -> Html {
    log::debug!("Render calendar page");
    let prev_month = date.clone() - Months::new(1);
    let next_month = date.clone() + Months::new(1);

    let groves_state = use_state_eq(|| vec![] as Vec<Grove>);

    {
        let groves_state = groves_state.clone();

        use_mount(move || {
            yew::platform::spawn_local(async move {
                if let Ok(groves) = api::get_groves().await {
                    groves_state.set(groves)
                }
            });
        });
    }

    let calendar_style = use_style!(
        r#"
display: grid;
height: 100%;
grid-template-rows: auto 1fr;
        "#
    );
    let calendar_container_style = use_style!(
        r#"
display: grid;
grid-template-columns: repeat(7, 1fr);
grid-template-rows: auto repeat(6, 1fr);
height: 100%;
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

    let navigate = use_callback(on_navigate, |date, on_navigate| {
        log::debug!("Navigate to month {date}");
        on_navigate.emit(date);
    });

    let prev = navigate.clone();
    let next = navigate.clone();

    html!(
        <div class={calendar_style}>
            <div class={calendar_header_style}>
                <span class={classes!(calendar_action_style.clone(), calendar_action_prev_style)}>
                    <a onclick={move |_| prev.emit(prev_month)}>
                        { prev_month.format_localized("%B %Y", Locale::de_DE).to_string() }
                    </a>
                </span>
                <CosmoHeader
                    level={CosmoHeaderLevel::H2}
                    header={date.format_localized("%B %Y", Locale::de_DE).to_string()}
                />
                <span class={classes!(calendar_action_style.clone(), calendar_action_next_style)}>
                    <a onclick={move |_| next.emit(next_month)}>
                        { next_month.format_localized("%B %Y", Locale::de_DE).to_string() }
                    </a>
                </span>
            </div>
            <div class={calendar_container_style}>
                <div class={calendar_weekday_style.clone()}>{ "Montag" }</div>
                <div class={calendar_weekday_style.clone()}>{ "Dienstag" }</div>
                <div class={calendar_weekday_style.clone()}>{ "Mittwoch" }</div>
                <div class={calendar_weekday_style.clone()}>{ "Donnerstag" }</div>
                <div class={calendar_weekday_style.clone()}>{ "Freitag" }</div>
                <div class={calendar_weekday_style.clone()}>{ "Samstag" }</div>
                <div class={calendar_weekday_style}>{ "Sonntag" }</div>
                <CalendarData
                    date={date.clone()}
                    events={events.clone()}
                    groves={(*groves_state).clone()}
                    grove_id={grove_id}
                />
            </div>
        </div>
    )
}

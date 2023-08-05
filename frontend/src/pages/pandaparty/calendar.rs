use chrono::{Days, Months};
use chrono::prelude::*;
use date_range::DateRange;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
struct DayProps {
    day: u32,
    month: u32,
    year: i32,
    selected_month: u32,
}

#[function_component(Day)]
fn day(props: &DayProps) -> Html {
    let background_color = if props.selected_month == props.month {
        "transparent"
    } else {
        "var(--day-background-past-month)"
    };
    let today = Local::now().date_naive();
    let (day_number_color, day_number_weight) = if today.month() == props.month && today.day() == props.day && today.year() == props.year {
        ("var(--primary-color)", "var(--font-weight-bold)")
    } else {
        ("var(--control-border-color)", "var(--font-weight-light)")
    };

    let style = use_style!(r#"
border-top: 1px solid var(--primary-color);
border-left: 1px solid var(--primary-color);
background: ${background_color};
position: relative;
box-sizing: border-box;

--day-background-past-month: #0000000F;

@media screen and (prefers-color-scheme: dark) {
    --day-background-past-month: #FFFFFF1D;
}

&:nth-child(7n) {
    border-right: 1px solid var(--primary-color);
}

&:nth-child(43),
&:nth-child(44),
&:nth-child(45),
&:nth-child(46),
&:nth-child(47),
&:nth-child(48),
&:nth-child(49) {
    border-bottom: 1px solid var(--primary-color);
}

&::before {
    content: "${day}";
    position: absolute;
    top: 4px;
    right: 4px;
    font-size: 28px;
    color: ${day_number_color};
    font-weight: ${day_number_weight};
}"#,
        background_color = background_color,
        day = props.day,
        day_number_color = day_number_color,
        day_number_weight = day_number_weight,
    );

    html!(
        <div class={classes!(style)}>
        </div>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct CalendarProps {
    #[prop_or_default]
    days: Vec<DayProps>,
    date: NaiveDate,
}

#[function_component(CalendarData)]
fn calendar_data(props: &CalendarProps) -> Html {
    log::debug!("Render CalendarData");
    let first_day_of_month = props.date;
    let first_day_offset = first_day_of_month.weekday() as u64 - 1;

    let last_day_of_month = first_day_of_month.checked_add_months(Months::new(1)).unwrap().checked_sub_days(Days::new(1)).unwrap();

    let last_day_of_prev_month = first_day_of_month.checked_sub_days(Days::new(1)).unwrap();
    let calendar_start_date = last_day_of_prev_month.checked_sub_days(Days::new(first_day_offset)).unwrap();

    let total_days = first_day_offset + last_day_of_month.day() as u64;
    let days_of_next_month = 40 - total_days;

    let first_day_of_next_month = first_day_of_month.checked_add_months(Months::new(1)).unwrap();
    let calendar_end_date = first_day_of_next_month.checked_add_days(Days::new(days_of_next_month)).unwrap();

    let selected_month = first_day_of_month.month();

    log::debug!("First day of month {first_day_of_month:?}");
    log::debug!("Last day of month {last_day_of_month:?}");
    log::debug!("First day of calendar {calendar_start_date:?}");
    log::debug!("Last day of prev month {first_day_of_month:?}");
    log::debug!("First day of next month {first_day_of_next_month:?}");
    log::debug!("Last day of calendar {calendar_end_date:?}");

    html!(
        <>
            {for DateRange::new(calendar_start_date, last_day_of_prev_month).unwrap().into_iter().map(|day| html!(
                <Day key={day.format("%F").to_string()} day={day.day()} month={day.month()} year={day.year()} selected_month={selected_month} />
            ))}

            {for DateRange::new(first_day_of_month, last_day_of_month).unwrap().into_iter().map(|day| html!(
                <Day key={day.format("%F").to_string()} day={day.day()} month={day.month()} year={day.year()} selected_month={selected_month} />
            ))}

            {for DateRange::new(first_day_of_next_month, calendar_end_date).unwrap().into_iter().map(|day| html!(
                <Day key={day.format("%F").to_string()} day={day.day()} month={day.month()} year={day.year()} selected_month={selected_month} />
            ))}
        </>
    )
}

#[function_component(CalendarPage)]
pub fn calendar_page() -> Html {
    log::debug!("Render calendar page");
    let date_state = use_state_eq(|| Local::now().date_naive().with_day(1).unwrap());

    let prev_month = *date_state - Months::new(1);
    let next_month = *date_state + Months::new(1);

    let calendar_container_style = use_style!(r#"
display: grid;
grid-template-columns: repeat(7, 1fr);
grid-template-rows: auto repeat(6, 1fr);
height: calc(100vh - 64px - 32px - 80px - 28px - 68px - 44px - 16px - 34px - 16px);
    "#);
    let calendar_header_style = use_style!(r#"
display: flex;
justify-content: space-between;
align-items: baseline;
margin-top: 16px;
margin-bottom: 16px;

h2 {
    margin: 0;
    flex: 0 0 33%;
    min-width: 33%;
    text-align: center;
}
    "#);
    let calendar_action_style = use_style!(r#"
font-size: 24px;
font-weight: var(--font-weight-light);
color: var(--primary-color);
text-decoration: none;
cursor: pointer;
flex: 0 0 33%;
min-width: 33%;
    "#);
    let calendar_action_prev_style = use_style!(r#"
text-align: left;
    "#);
    let calendar_action_next_style = use_style!(r#"
text-align: right;
    "#);
    let calendar_weekday_style = use_style!(r#"
font-size: 20px;
font-weight: var(--font-weight-light);
color: var(--primary-color);
grid-row: 1/2;
text-align: center;
    "#);

    let move_prev = use_callback(|_, state| state.set((*state).checked_sub_months(Months::new(1)).unwrap()), date_state.clone());
    let move_next = use_callback(|_, state| state.set((*state).checked_add_months(Months::new(1)).unwrap()), date_state.clone());

    html!(
        <>
            <CosmoTitle title="Event Kalender" />
            <div class={calendar_header_style}>
                <a onclick={move_prev} class={classes!(calendar_action_style.clone(), calendar_action_prev_style)}>{prev_month.format_localized("%B %Y", Locale::de_DE)}</a>
                <CosmoHeader level={CosmoHeaderLevel::H2} header={(*date_state).format_localized("%B %Y", Locale::de_DE).to_string()} />
                <a onclick={move_next} class={classes!(calendar_action_style, calendar_action_next_style)}>{next_month.format_localized("%B %Y", Locale::de_DE)}</a>
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

use bamboo_pandas_frontend_base::controls::{use_events, Calendar};
use chrono::Datelike;
use std::ops::Deref;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_cosmo::prelude::*;

#[function_component(CalendarPage)]
pub fn calendar_page() -> Html {
    log::debug!("Render calendar page");
    let calendar_container_style = use_style!(
        r#"
height: calc(var(--page-height) - var(--title-font-size) - 0.5rem);
    "#
    );

    let today = chrono::Local::now().date_naive().with_day(1).unwrap();
    log::info!("Load for today {today}");

    let events = use_events(today, None);

    html!(
        <>
            <CosmoTitle title="Event Kalender" />
            <div class={calendar_container_style}>
                <Calendar events={events.events_list.current().deref().clone()} date={*events.date_state} on_navigate={events.on_navigate} />
            </div>
        </>
    )
}

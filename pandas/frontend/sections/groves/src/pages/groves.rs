use crate::api;
use bamboo_pandas_frontend_base::controls::{use_events, Calendar};
use bamboo_pandas_frontend_base::routing::GroveRoute;
use bounce::helmet::Helmet;
use chrono::Datelike;
use std::ops::Deref;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::use_route;

#[autoprops]
#[function_component(GroveCalendar)]
fn grove_calendar(id: i32) -> Html {
    log::debug!("Render calendar page");
    let calendar_container_style = use_style!(
        r#"
height: calc(var(--page-height) - var(--title-font-size) - var(--tab-links-height) - var(--tab-gap) - 0.5rem);
    "#
    );

    let today = chrono::Local::now().date_naive().with_day(1).unwrap();
    log::info!("Load for today {today}");

    let events = use_events(today, Some(id));

    html!(
        <>
            <div class={calendar_container_style}>
                <Calendar grove_id={Some(id)} events={events.events_list.current().deref().clone()} date={*events.date_state} on_navigate={events.on_navigate} />
            </div>
        </>
    )
}

#[autoprops]
#[function_component(GroveDetails)]
pub fn grove_details(id: i32) -> Html {
    let id_state = use_state_eq(|| id);

    let grove_state = {
        let id_state = id_state.clone();

        use_async(async move {
            api::get_grove(*id_state).await
        })
    };

    let route = use_route::<GroveRoute>();
    if let Some(route) = route {
        if let GroveRoute::Grove { id } = route {
            if id != *id_state {
                grove_state.run();
                id_state.set(id);
            }
        }
    }

    {
        let grove_state = grove_state.clone();

        use_mount(move || {
            grove_state.run();
        });
    }

    html!(
        <>
            if let Some(grove) = &grove_state.data {
                <>
                    <Helmet>
                        <title>{grove.name.clone()}</title>
                    </Helmet>
                    <CosmoTitle title={grove.name.clone()} />
                </>
            }
            <CosmoTabControl>
                <CosmoTabItem label="Hainkalender">
                    <GroveCalendar id={id} />
                </CosmoTabItem>
                <CosmoTabItem label="Pandas"></CosmoTabItem>
                <CosmoTabItem label="Verwaltung"></CosmoTabItem>
            </CosmoTabControl>
        </>
    )
}

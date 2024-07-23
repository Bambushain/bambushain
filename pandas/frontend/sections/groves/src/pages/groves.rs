use crate::api;
use bamboo_common::frontend::api::ApiError;
use bamboo_common::frontend::ui::{BambooCard, BambooCardList};
use bamboo_pandas_frontend_base::controls::{use_events, Calendar};
use bamboo_pandas_frontend_base::error;
use bamboo_pandas_frontend_base::routing::GroveRoute;
use bounce::helmet::Helmet;
use chrono::Datelike;
use std::ops::Deref;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount};
use yew_router::prelude::use_route;

#[autoprops]
#[function_component(Users)]
fn users(id: i32) -> Html {
    log::debug!("Render users page");
    log::debug!("Initialize state and callbacks");
    let bamboo_error_state = use_state_eq(ApiError::default);

    let unreported_error_toggle = use_bool_toggle(false);

    let users_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            unreported_error_toggle.set(false);

            api::get_users(id).await.map_err(|err| {
                bamboo_error_state.set(err.clone());
                unreported_error_toggle.set(true);

                err
            })
        })
    };

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_user",
                "users_page",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    {
        let users_state = users_state.clone();

        use_effect_with(id, move |_| {
            users_state.run();

            || {}
        });
    }

    html!(
        if users_state.loading {
            <CosmoProgressRing />
        } else if users_state.error.is_some() {
            if *unreported_error_toggle {
                <CosmoMessage header="Fehler beim Laden" message="Die Pandas konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
            } else {
                <CosmoMessage header="Fehler beim Laden" message="Die Pandas konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
            }
        } else if let Some(data) = &users_state.data {
            <>
                <CosmoToolbar>
                </CosmoToolbar>
                <BambooCardList>
                    {for data.iter().map(|user|
                        {
                            let profile_picture = format!(
                                "/api/user/{}/picture#time={}",
                                user.id,
                                chrono::offset::Local::now().timestamp_millis()
                            );
                            html!(
                                <BambooCard title={user.display_name.clone()} prepend={html!(<img style="max-height:7rem;" src={profile_picture} />)}>
                                    <CosmoAnchor href={format!("mailto:{}", user.email.clone())}>{user.email.clone()}</CosmoAnchor>
                                    if !user.discord_name.is_empty() {
                                        <span>{"Auf Discord bekannt als "}<CosmoStrong>{user.discord_name.clone()}</CosmoStrong></span>
                                    }
                                </BambooCard>
                            )
                        }
                    )}
                </BambooCardList>
            </>
        }
    )
}

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

    {
        let events = events.clone();

        use_effect_with(id, move |_| {
            events.grove_id_state.set(Some(id));

            || {}
        });
    }

    html!(
        <div class={calendar_container_style}>
            <Calendar grove_id={Some(id)} events={events.events_list.current().deref().clone()} date={*events.date_state} on_navigate={events.on_navigate} />
        </div>
    )
}

#[autoprops]
#[function_component(GroveDetailsPage)]
pub fn grove_details(id: i32) -> Html {
    let id_state = use_state_eq(|| id);

    let grove_state = {
        let id_state = id_state.clone();

        use_async(async move { api::get_grove(*id_state).await })
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
                    <GroveCalendar id={*id_state} />
                </CosmoTabItem>
                <CosmoTabItem label="Pandas">
                    <Users id={*id_state} />
                </CosmoTabItem>
                <CosmoTabItem label="Verwaltung"></CosmoTabItem>
            </CosmoTabControl>
        </>
    )
}

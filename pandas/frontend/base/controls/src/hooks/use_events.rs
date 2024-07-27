use crate::api;
use bamboo_common::core::entities::event::GroveEvent;
use chrono::{Datelike, Days, Months, NaiveDate};
use date_range::DateRange;
use futures::channel::mpsc;
use futures::Stream;
use gloo_events::EventListener;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{EventSource, EventTarget, MessageEvent};
use yew::{
    hook, use_callback, use_effect_with, use_mut_ref, use_state_eq, Callback, UseStateHandle,
};
use yew_hooks::{use_async, use_list, use_mount, use_unmount, UseListHandle};

struct CalendarEventSourceEvent {
    receiver: mpsc::UnboundedReceiver<()>,
    #[allow(dead_code)]
    listener: EventListener,
}

impl CalendarEventSourceEvent {
    pub fn new(target: EventTarget, event: String, callback: Callback<GroveEvent>) -> Self {
        let (sender, receiver) = mpsc::unbounded();

        let listener = EventListener::new(&target, event, move |evt| {
            log::debug!("New message received");
            let evt = evt.dyn_ref::<MessageEvent>().unwrap_throw();
            let data = evt.data();
            if let Some(data) = data.as_string() {
                log::debug!("The data received: {data:?}");
                if let Ok(event) = serde_json::from_str::<GroveEvent>(data.as_str()) {
                    log::debug!("Decoded the message {:#?}", event.clone());
                    callback.emit(event);
                }
            }

            sender.unbounded_send(()).unwrap_throw();
        });

        Self { receiver, listener }
    }
}

impl Stream for CalendarEventSourceEvent {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}

pub(crate) struct CalendarEventSource {
    event_source: Option<web_sys::EventSource>,
    listeners: Vec<CalendarEventSourceEvent>,
}

impl CalendarEventSource {
    fn new() -> Self {
        let event_source = if let Ok(event_source) = EventSource::new("/sse/event").map_err(|err| {
            log::warn!(
                "Failed to start event source, automatic calendar updates disabled: {err:?}"
            );
        }) {
            Some(event_source)
        } else {
            None
        };

        Self {
            event_source,
            listeners: vec![],
        }
    }

    fn register_handler(&mut self, event: impl Into<String>, callback: Callback<GroveEvent>) {
        if let Some(source) = self.event_source.clone() {
            self.listeners.push(CalendarEventSourceEvent::new(
                source.into(),
                event.into(),
                callback,
            ));
        }
    }

    fn close(&self) {
        if let Some(source) = self.event_source.clone() {
            source.close();
        }
    }
}

#[derive(Clone)]
pub struct UseEventsHandle {
    pub events_list: UseListHandle<GroveEvent>,
    pub date_state: UseStateHandle<NaiveDate>,
    pub grove_id_state: UseStateHandle<Option<i32>>,
    pub(crate) calendar_event_source_state: Rc<RefCell<CalendarEventSource>>,
    pub on_navigate: Callback<NaiveDate>,
}

impl UseEventsHandle {
    pub(crate) fn new(
        events_list: UseListHandle<GroveEvent>,
        date_state: UseStateHandle<NaiveDate>,
        grove_id_state: UseStateHandle<Option<i32>>,
        calendar_event_source_state: Rc<RefCell<CalendarEventSource>>,
        on_navigate: Callback<NaiveDate>,
    ) -> Self {
        Self {
            events_list,
            date_state,
            grove_id_state,
            calendar_event_source_state,
            on_navigate,
        }
    }
}

fn get_dates(date: NaiveDate) -> DateRange {
    let first_day_offset = date.weekday() as i64 - 1;
    let first_day_offset = if first_day_offset < 0 {
        0
    } else {
        first_day_offset
    } as u64;

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

    DateRange::new(calendar_start_date, calendar_end_date).unwrap()
}

#[hook]
pub fn use_events(first_day: NaiveDate, grove_id: Option<i32>) -> UseEventsHandle {
    let date_state = use_state_eq(|| first_day);

    let grove_id_state = use_state_eq(|| grove_id);

    let events_list = use_list(vec![] as Vec<GroveEvent>);

    let calendar_event_source_ref = use_mut_ref(CalendarEventSource::new);

    let navigate = use_callback(date_state.clone(), |date, date_state| date_state.set(date));

    let handle = UseEventsHandle::new(
        events_list.clone(),
        date_state.clone(),
        grove_id_state.clone(),
        calendar_event_source_ref,
        navigate,
    );
    let events_state = {
        let events_list = handle.events_list.clone();

        let date_state = date_state.clone();

        let grove_id_state = grove_id_state.clone();

        use_async(async move {
            let range = get_dates(*date_state);

            api::get_events(range.into(), *grove_id_state)
                .await
                .map(|data| events_list.set(data))
                .map_err(|err| format!("{err}"))
        })
    };

    let event_created = use_callback(
        (handle.events_list.clone(), handle.date_state.clone()),
        |event: GroveEvent, (events_list, date_state)| {
            log::debug!(
                "Someone created a new event, adding it to the list if it is in current range"
            );
            log::debug!("Got event {event:?}");
            let range = get_dates(**date_state);
            if (event.start_date >= range.since() && event.start_date <= range.until())
                || (event.end_date >= range.since() && event.end_date <= range.until())
            {
                log::debug!("The event is in range, lets add it to the list");
                events_list.push(event.clone());
            }
        },
    );
    let event_updated = use_callback(
        (handle.events_list.clone(), handle.date_state.clone()),
        |event: GroveEvent, (events_list, date_state)| {
            log::debug!("Someone updated an event, if we have it loaded, lets update it");
            log::debug!("Got event {event:?}");
            let range = get_dates(**date_state);
            if (event.start_date >= range.since() && event.start_date <= range.until())
                || (event.end_date >= range.since() && event.end_date <= range.until())
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
    let event_deleted = use_callback(events_list.clone(), |event: GroveEvent, events_list| {
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
        let handle = handle.clone();
        use_unmount(move || {
            let source = handle.calendar_event_source_state.borrow();
            source.close();
        });
    }
    {
        let mount_handle = handle.clone();
        let date_events_state = events_state.clone();
        let grove_events_state = events_state.clone();

        let event_created = event_created.clone();
        let event_updated = event_updated.clone();
        let event_deleted = event_deleted.clone();

        use_effect_with(handle.date_state.clone(), move |_| {
            date_events_state.run();

            || {}
        });

        use_effect_with(handle.grove_id_state.clone(), move |_| {
            grove_events_state.run();

            || {}
        });

        use_mount(move || {
            log::debug!("Start event source for calendar on /sse/event");
            let mut source = mount_handle.calendar_event_source_state.borrow_mut();
            source.register_handler("created", event_created.clone());
            source.register_handler("updated", event_updated.clone());
            source.register_handler("deleted", event_deleted.clone());
        })
    }

    handle
}

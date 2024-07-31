use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use actix_web::rt::time::interval;
use actix_web::Responder;
use actix_web_lab::sse;
use futures_util::StreamExt;
use parking_lot::Mutex;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

use bamboo_common::backend::notification::EventAction;
use bamboo_common::core::entities::{Grove, GroveEvent, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
enum EventType {
    Created,
    Updated,
    Deleted,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Deleted => "deleted",
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Event {
    event: GroveEvent,
    action: EventType,
}

impl Into<sse::Event> for Event {
    fn into(self) -> sse::Event {
        let mut data = sse::Data::new_json(self.event.clone()).unwrap();
        data.set_event(self.action.to_string());

        sse::Event::Data(data)
    }
}

impl Event {
    fn new(event: GroveEvent, action: EventType) -> Self {
        Self { event, action }
    }

    fn from_event_action(event_action: EventAction) -> Self {
        match event_action {
            EventAction::Created(evt) => Event::new(evt, EventType::Created),
            EventAction::Updated(evt) => Event::new(evt, EventType::Updated),
            EventAction::Deleted(evt) => Event::new(evt, EventType::Deleted),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Comment {
    Connected,
    Ping,
}

pub struct EventBroadcaster {
    inner: Mutex<EventBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct EventBroadcasterInner {
    clients: Vec<(Sender<sse::Event>, User)>,
}

impl EventBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(EventBroadcaster {
            inner: Mutex::new(EventBroadcasterInner::default()),
        });
        EventBroadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    fn spawn_ping(this: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        let mut ok_clients = Vec::new();
        for (client, user) in clients {
            if let Err(err) = Self::send_comment(client.clone(), Comment::Ping).await {
                log::info!("Failed to send ping {err}");
            } else {
                ok_clients.push((client.clone(), user));
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(&self, user: User) -> impl Responder {
        log::debug!("Open channel using tokio");
        let (tx, rx) = tokio::sync::mpsc::channel::<sse::Event>(10);

        log::debug!("Send connected message");
        if let Err(err) = Self::send_comment(tx.clone(), Comment::Connected).await {
            log::error!("Failed to send message {err}")
        }
        self.inner.lock().clients.push((tx, user));

        sse::Sse::from_infallible_receiver(rx).with_keep_alive(Duration::from_secs(60))
    }

    pub async fn send_event(&self, evt: EventAction, groves: Vec<Grove>) {
        let clients = self.inner.lock().clients.clone();
        log::debug!("Has {} clients registered", clients.len());
        futures::stream::iter(clients)
            .map(|(client, user)| {
                tokio::spawn(Self::send_message(
                    client,
                    user,
                    evt.clone(),
                    groves.clone(),
                ))
            })
            .collect::<Vec<_>>()
            .await;
    }

    async fn send_message(
        client: Sender<sse::Event>,
        user: User,
        evt: EventAction,
        groves: Vec<Grove>,
    ) {
        let event = match evt.clone() {
            EventAction::Created(event) => event,
            EventAction::Updated(event) => event,
            EventAction::Deleted(event) => event,
        };

        let is_private_event_of_current_user =
            event.is_private && Some(user.id) == event.user.map(|u| u.id);
        let is_in_same_grove = !event.is_private
            && groves
                .iter()
                .any(|g| g.id == event.grove.clone().map(|g| g.id).unwrap_or(-1));
        if is_private_event_of_current_user || is_in_same_grove {
            log::debug!("Send event data");
            if let Err(err) = client
                .send(Event::from_event_action(evt.clone()).into())
                .await
            {
                log::error!("Failed to send message {err}");
            }
        }
    }

    async fn send_comment(
        client: Sender<sse::Event>,
        evt: Comment,
    ) -> Result<(), SendError<sse::Event>> {
        client
            .send(sse::Event::Comment(bytestring::ByteString::from(
                serde_json::to_string(&evt).unwrap(),
            )))
            .await
    }
}

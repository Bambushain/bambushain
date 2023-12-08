use std::sync::Arc;
use std::time::Duration;

use actix_web::Responder;
use actix_web::rt::time::interval;
use actix_web_lab::sse::{Data, Event, Sse};
use parking_lot::Mutex;
use tokio::sync::mpsc::Sender;

pub struct EventBroadcaster {
    inner: Mutex<EventBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct EventBroadcasterInner {
    clients: Vec<Sender<Event>>,
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
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        log::info!("Active event client {:?}", clients);

        let mut ok_clients = Vec::new();

        for client in clients {
            if client.send(Event::Comment("ping".into())).await.is_ok() {
                ok_clients.push(client.clone());
            }
        }

        log::info!("Okay event active client {:?}", ok_clients);

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(&self) -> impl Responder {
        log::info!("Starting creation of event broadcaster");
        let (tx, rx) = tokio::sync::mpsc::channel::<Event>(10);

        tx.send(Event::Data(Data::new("connected"))).await.unwrap();
        log::info!("Creating new clients success {tx:?}");
        self.inner.lock().clients.push(tx);

        Sse::from_infallible_receiver(rx).with_keep_alive(Duration::from_secs(60))
    }

    pub async fn notify_change(&self) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients
            .iter()
            .map(|client| client.send(Event::Data(Data::new("new data"))));

        let _ = futures_util::future::join_all(send_futures).await;
    }
}

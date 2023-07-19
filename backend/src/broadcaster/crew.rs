use std::sync::Arc;
use std::time::Duration;

use actix_web::rt::time::interval;
use actix_web_lab::sse::{ChannelStream, Sse};
use parking_lot::Mutex;

pub struct CrewBroadcaster {
    inner: Mutex<CrewBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct CrewBroadcasterInner {
    clients: Vec<actix_web_lab::sse::Sender>,
}

impl CrewBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(CrewBroadcaster {
            inner: Mutex::new(CrewBroadcasterInner::default()),
        });
        CrewBroadcaster::spawn_ping(Arc::clone(&this));

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
        log::info!("Active crew client {:?}", clients);

        let mut ok_clients = Vec::new();

        for client in clients {
            if client
                .send(actix_web_lab::sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        log::info!("Okay crew active client {:?}", ok_clients);

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(&self) -> Sse<ChannelStream> {
        log::info!("Starting creation of crew broadcaster");
        let (tx, rx) = actix_web_lab::sse::channel(10);

        tx.send(actix_web_lab::sse::Data::new("connected")).await.unwrap();
        log::info!("Creating new clients success {:?}", tx);
        self.inner.lock().clients.push(tx);
        rx
    }

    pub async fn notify_change(&self) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients
            .iter()
            .map(|client| client.send(actix_web_lab::sse::Data::new("new data")));

        let _ = futures_util::future::join_all(send_futures).await;
    }
}
// use std::net::SocketAddr;
use std::thread;
use std::time;
// use std::sync::{Arc, RwLock};

// use tokio::net::TcpListener;

// use crate::events;
use crate::events::Events;
use crate::events::EventsBuilder;

pub struct Proxy {
    events: Events
}

impl Proxy {
    pub fn new(events: EventsBuilder) -> (Proxy, Events) {
        (
            Proxy {
                events: events.0
            },
            Events {
                request: events.1.request,
                response: events.1.response
            }
        )
    }

    pub async fn listen(&mut self) {
        // let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
        // let listener = TcpListener::bind(addr).await;

        loop {
            let process_request = self.events.request.0.clone();
            let mut modified = self.events.request.1.resubscribe();

            tokio::spawn(async move {
                thread::sleep(time::Duration::from_secs(1));
                process_request.send("request incoming".to_string()).unwrap();

                let modified = modified.recv().await.unwrap();
                println!("incoming modified {}", modified);
            });

            thread::sleep(time::Duration::from_secs(10));
        }
    }
}
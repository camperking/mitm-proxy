use std::{thread, time};
use mitmProxy::proxy::Proxy;
use mitmProxy::events::{EventsBuilder};

#[tokio::main]
async fn main() {

    let events = EventsBuilder::new();
    let (mut proxy, mut events) = Proxy::new(events);

    tokio::spawn(async move {
        proxy.listen().await;
    });

    tokio::spawn(async move {
        loop {
            let request = events.request.1.recv().await;
            let modified = events.request.0.clone();

            tokio::spawn(async move {
                // println!("request: {:#?}", request);
                // edit request here
                modified.send("modified request".to_string()).unwrap();
            });
        }
    });

    loop {
        thread::sleep(time::Duration::from_secs(3));
    }
}
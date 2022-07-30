use std::{thread, time};
// use std::sync::{Arc, RwLock};
use proxy::proxy::Proxy;
use proxy::events::EventsBuilder;

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
                println!("request: {:#?}", request);
                modified.send("modified request".to_string()).unwrap();
            });
        }
    });

    loop {
        thread::sleep(time::Duration::from_secs(3));
    }
    
    // let requests = reqs.clone();
    // loop {
    //     let r = requests.read().unwrap();
    //     if r.len() > 0 {
    //         println!("{:#?}", r)
    //     };
    //     drop(r);
    //     let mut w = requests.write().unwrap();
    //     w.clear();
    //     drop(w);
    //     thread::sleep(time::Duration::from_secs(3));
    // }
}
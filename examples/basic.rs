use std::{thread, time};
use std::sync::{Arc, RwLock};
use proxy::proxy::{Proxy};

#[tokio::main]
async fn main() {
    let reqs = Arc::new(RwLock::new(Vec::new()));    

    let requests = reqs.clone();
    tokio::spawn(async move {
        let mut proxy = Proxy {
            requests
        };

        proxy.listen().await;
    });

    let requests = reqs.clone();
    loop {
        let r = requests.read().unwrap();
        if r.len() > 0 {
            println!("{:#?}", r)
        };
        drop(r);
        let mut w = requests.write().unwrap();
        w.clear();
        drop(w);
        thread::sleep(time::Duration::from_secs(3));
    }
}
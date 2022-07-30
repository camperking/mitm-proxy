use std::thread;
use std::time;
use std::sync::{Arc, RwLock};

pub struct Proxy {
    pub requests: Arc<RwLock<Vec<String>>>
}

impl Proxy {
    pub async fn listen(&mut self) {

        loop {
            let requests = self.requests.clone();

            tokio::spawn(async move {
                thread::sleep(time::Duration::from_secs(1));
                let mut w = requests.write().unwrap();
                w.push("req".to_string());
                drop(w);

            });

            thread::sleep(time::Duration::from_secs(10));
        }
    }
}
use tokio::sync::broadcast;

pub struct Transceiver (
    pub broadcast::Sender<String>, pub broadcast::Receiver<String>
);

impl Clone for Transceiver {
    fn clone(&self) -> Self {
        Transceiver (self.0.clone(), self.1.resubscribe())
    }
}

pub struct EventHandler {
    alice: Transceiver,
    bob: Transceiver
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let (tx0, rx0) = broadcast::channel(32);
        let (tx1, rx1) = broadcast::channel(32);
        let alice = Transceiver (tx0, rx1);
        let bob = Transceiver (tx1, rx0);
        EventHandler {
            alice,
            bob
        }
    }
}

#[derive(Clone)]
pub struct Events {
    pub request: Transceiver,
    pub response: Transceiver
}

pub struct EventsBuilder (
    pub Events, pub Events
);

impl EventsBuilder {
    pub fn new() -> EventsBuilder {
        let request = EventHandler::new();
        let response = EventHandler::new();

        EventsBuilder ( 
            Events { request: request.alice, response: response.alice },
            Events { request: request.bob, response: response.bob}
        )
    }
}
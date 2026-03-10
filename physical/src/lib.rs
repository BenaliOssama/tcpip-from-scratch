use std::sync::mpsc::{self, Receiver, Sender};

pub struct Endpoint {
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Vec<u8>>,
}

impl Endpoint {
    pub fn send(&self, data: Vec<u8>) {
        self.sender.send(data).unwrap();
    }

    pub fn receive(&self) -> Option<Vec<u8>> {
        self.receiver.try_recv().ok()
    }
}

pub fn create_wire() -> (Endpoint, Endpoint) {
    let (a_tx, a_rx) = mpsc::channel();
    let (b_tx, b_rx) = mpsc::channel();

    let endpoint_a = Endpoint {
        sender: a_tx,
        receiver: b_rx,
    };

    let endpoint_b = Endpoint {
        sender: b_tx,
        receiver: a_rx,
    };

    (endpoint_a, endpoint_b)
}


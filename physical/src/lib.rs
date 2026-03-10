use std::sync::mpsc::{self, Receiver, Sender};
use rand::Rng;

#[derive(Clone)]
pub struct ChaosConfig {
    /// 0.0 = never drop, 1.0 = always drop
    pub loss_rate: f64,
    /// 0.0 = never corrupt, 1.0 = always corrupt
    pub corrupt_rate: f64,
    /// 0.0 = never duplicate, 1.0 = always duplicate
    pub duplicate_rate: f64,
}

impl ChaosConfig {
    pub fn perfect() -> Self {
        ChaosConfig { loss_rate: 0.0, corrupt_rate: 0.0, duplicate_rate: 0.0 }
    }

    pub fn noisy() -> Self {
        ChaosConfig { loss_rate: 0.1, corrupt_rate: 0.1, duplicate_rate: 0.05 }
    }
}



pub struct Endpoint {
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Vec<u8>>,
    chaos: ChaosConfig,
}

impl Endpoint {
    pub fn send(&self, data: Vec<u8>) {
        let mut rng = rand::thread_rng();

        // 1. Maybe drop it
        if rng.gen_bool(self.chaos.loss_rate) {
            return; // bytes are gone, no error, no notification
        }

        // 2. Maybe corrupt it
        let data = if rng.gen_bool(self.chaos.corrupt_rate) {
            let mut corrupted = data.clone();
            let index = rng.gen_range(0..corrupted.len());
            let bit = 1u8 << rng.gen_range(0..8);
            corrupted[index] ^= bit; // flip a random bit
            corrupted
        } else {
            data
        };

        // 3. Maybe duplicate it
        if rng.gen_bool(self.chaos.duplicate_rate) {
            self.sender.send(data.clone()).unwrap();
        }

        self.sender.send(data).unwrap();
    }

    pub fn receive(&self) -> Option<Vec<u8>> {
        self.receiver.try_recv().ok()
    }
}

pub fn create_wire(chaos: ChaosConfig) -> (Endpoint, Endpoint) {
    let (a_sender, a_receiver) = mpsc::channel();
    let (b_sender, b_receiver) = mpsc::channel();

    let endpoint_a = Endpoint {
        sender: a_sender,
        receiver: b_receiver,
        chaos: chaos.clone(),
    };

    let endpoint_b = Endpoint {
        sender: b_sender,
        receiver: a_receiver,
        chaos: chaos,
    };

    (endpoint_a, endpoint_b)
}

use crate::uinput::types::Token;
use crate::uinput::devices::Devices;
use std::sync::mpsc::{channel, Sender};

pub fn spawn_worker() -> Sender<Vec<Token>> {
    let (tx, rx) = channel::<Vec<Token>>();

    std::thread::spawn(move || {
        let mut devices = match Devices::new() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Worker failed to init uinput: {:?}", e);
                return;
            }
        };

        while let Ok(tokens) = rx.recv() {
            for token in tokens {
                if let Err(e) = devices.execute(token) {
                    eprintln!("Execution error: {:?}", e);
                }
            }
        }
    });

    tx
}

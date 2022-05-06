use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::{JoinHandle, Thread};
use crate::ffxiv::manager::FFXIVPacketManager;

enum ClientMessageControl {
    Exit
}

struct Channel<T> {
    rx: Arc<Mutex<Receiver<T>>>,
    tx: Arc<Mutex<Sender<T>>>
}

impl<T> Channel<T> {
    fn new() -> Self {
        let (tx, rx): (Sender<T>, Receiver<T>) = channel();
        Self {
            rx: Arc::new(Mutex::new(rx)),
            tx: Arc::new(Mutex::new(tx))
        }
    }

    fn reciever(&self) -> Arc<Mutex<Receiver<T>>> {
        self.rx.clone()
    }

    fn sender(&self) -> Arc<Mutex<Sender<T>>> {
        self.tx.clone()
    }
}

struct FFXIVClient {
    client: FFXIVPacketManager,
    channel: Channel<ClientMessageControl>,
    thread: Option<JoinHandle<()>>
}

impl FFXIVClient {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: FFXIVPacketManager::new()?,
            thread: None,
            channel: Channel::new()
        })
    }

    fn connect(&mut self) {
        let rx = self.channel.reciever();

        self.thread = Some(std::thread::spawn(move || {
            let rx = rx;

            loop {
                let reciever = rx.try_lock().unwrap();
                if let Ok(message) = reciever.try_recv() {
                    match message {
                        ClientMessageControl::Exit => break
                    }
                }
            }
        }));
    }

    fn disconnect(&mut self) {
    }
}

impl Default for FFXIVClient {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl Drop for FFXIVClient {
    fn drop(&mut self) {
        self.disconnect();

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
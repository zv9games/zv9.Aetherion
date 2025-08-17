// threading.rs

//! 🧵 Threading Module
//! Provides safe concurrency utilities for EchoEngine.
//! Used for async audit logging, overlay rendering, and generator tasks.

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

/// Thread-safe echo queue
pub struct EchoQueue<T> {
    sender: Sender<T>,
    receiver: Arc<Mutex<Receiver<T>>>,
}

impl<T: Send + 'static> EchoQueue<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    /// Spawn a background thread to process echoes
    pub fn spawn_worker<F>(&self, handler: F)
    where
        F: Fn(T) + Send + 'static + Clone,
    {
        let rx = Arc::clone(&self.receiver);
        thread::spawn(move || {
            while let Ok(msg) = rx.lock().unwrap().recv() {
                handler.clone()(msg);
            }
        });
    }

    /// Send echo to queue
    pub fn send(&self, msg: T) {
        let _ = self.sender.send(msg);
    }
}

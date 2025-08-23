//! 🧵 Threading Module
//! Provides safe concurrency utilities for EchoEngine.
//!
//! Used for async audit logging, overlay rendering, and generator tasks.
//! Designed to support backend thread separation and echo processing without blocking the host thread.
//!
//! Contributors may use `EchoQueue` to dispatch messages to background workers safely.

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

/// Thread-safe echo queue
/// Wraps a sender/receiver pair for dispatching messages to background workers
pub struct EchoQueue<T> {
    sender: Sender<T>,
    receiver: Arc<Mutex<Receiver<T>>>,
}

impl<T: Send + 'static> EchoQueue<T> {
    /// Create a new echo queue
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    /// Spawn a background thread to process echoes
    /// Handler is cloned per message; ensure it is lightweight and thread-safe
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
    /// Non-blocking; returns immediately
    pub fn send(&self, msg: T) {
        let _ = self.sender.send(msg);
    }
}

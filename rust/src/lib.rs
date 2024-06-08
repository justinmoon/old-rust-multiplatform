uniffi::setup_scaffolding!();

mod ffi;
mod wasm;

use std::sync::{Arc, RwLock};

use crossbeam::channel::{unbounded, Receiver, Sender};
use ffi::FfiUpdater;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use wasm::WasmUpdater;

// globals.rs
static APP: OnceCell<RwLock<App>> = OnceCell::new();
static UPDATER: OnceCell<Updater> = OnceCell::new();

// events.rs
#[derive(uniffi::Enum, Serialize, Deserialize)]
pub enum Event {
    Increment,
    Decrement,
}

#[derive(uniffi::Enum, Serialize, Deserialize)]
pub enum Update {
    CountChanged { count: i32 },
}

// FIXME(justin): this is more of an "event bus"
struct Updater(pub Sender<Update>);

impl Updater {
    /// Initialize global instance of the updater with a sender
    pub fn init(sender: Sender<Update>) {
        UPDATER.get_or_init(|| Updater(sender));
    }

    pub fn send_update(update: Update) {
        UPDATER
            .get()
            .expect("updater is not initialized")
            .0
            .send(update)
            .expect("failed to send update");
    }
}

/// Our application
pub struct App {
    /// Count is the only state in our app
    count: i32,
    // /// Updater is a callback to the frontend
    // updater: Box<dyn Updater>,
    update_receiver: Arc<Receiver<Update>>,
}

impl App {
    /// Create a new instance of the app
    pub fn new() -> Self {
        let (sender, receiver): (Sender<Update>, Receiver<Update>) = unbounded();
        Updater::init(sender);
        Self {
            count: 0,
            update_receiver: Arc::new(receiver),
        }
    }

    /// Fetch global instance of the app, or create one if it doesn't exist
    pub fn global() -> &'static RwLock<App> {
        APP.get_or_init(|| RwLock::new(App::new()))
    }

    /// Handle event received from frontend
    pub fn handle_event(&mut self, event: Event) {
        // Handle event
        match event {
            Event::Increment => self.count += 1,
            Event::Decrement => self.count -= 1,
        }
        // Reflect state update to the frontend
        Updater::send_update(Update::CountChanged { count: self.count });
    }

    // FIXME(justin): if we only need one subscriber, we could move this logic to
    // the constructor
    pub fn listen_for_updates(&self, updater: Box<dyn FfiUpdater>) {
        let update_receiver = self.update_receiver.clone();
        std::thread::spawn(move || {
            while let Ok(update) = update_receiver.recv() {
                updater.update(update);
            }
        });
    }

    /// Wasm uses a different updater ...
    pub fn listen_for_updates_wasm(&self, updater: WasmUpdater) {
        let updater_arc = Arc::new(updater);
        let update_receiver = self.update_receiver.clone();
        // std::thread::spawn(move || {
        tokio::spawn(async move {
            while let Ok(update) = update_receiver.recv() {
                let update = serde_json::to_string(&update).expect("fixme");
                updater_arc.update(update);
            }
        });
    }
}

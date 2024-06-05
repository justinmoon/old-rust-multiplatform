uniffi::setup_scaffolding!();

use std::sync::{Arc, RwLock};

use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;

#[uniffi::export]
pub fn say_hi() -> String {
    "Hello v2".to_string()
}

// globals.rs
static APP: OnceCell<RwLock<App>> = OnceCell::new();
static UPDATER: OnceCell<Updater> = OnceCell::new();

// events.rs
#[derive(uniffi::Enum)]
pub enum Event {
    Increment,
    Decrement,
}

#[derive(uniffi::Enum)]
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

// FIXME(justin): seems like this should be called FFiListener or something like
// that. Maybe the callback should be `handle_update`?
// #[uniffi::export(with_foreign)]
#[uniffi::export(callback_interface)]
pub trait FfiUpdater: Send + Sync + 'static {
    /// Essentially a callback to the frontend
    fn update(&self, update: Update);
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
            while let Ok(field) = update_receiver.recv() {
                updater.update(field);
            }
        });
    }
}

/// Representation of our app over FFI. Essentially a wrapper of [`App`].
#[derive(uniffi::Object)]
pub struct FfiApp;

#[uniffi::export]
impl FfiApp {
    /// FFI constructor which wraps in an Arc
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    /// Frontend calls this method to send events to the rust application logic
    pub fn dispatch(&self, event: Event) {
        // FIXME: this won't be able to handle concurrent events ...
        self.inner().write().expect("fixme").handle_event(event);
    }

    pub fn listen_for_updates(&self, updater: Box<dyn FfiUpdater>) {
        self.inner()
            .read()
            .expect("fixme")
            .listen_for_updates(updater);
    }
}

impl FfiApp {
    /// Fetch global instance of the app, or create one if it doesn't exist
    fn inner(&self) -> &RwLock<App> {
        App::global()
    }
}

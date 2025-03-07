use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};

use crate::database::{Database, DATABASE};
use crate::navigation::Router;
use crate::updater::{FfiUpdater, Update, Updater};
use crate::Route;

// Global static APP instance
static APP: OnceCell<RwLock<App>> = OnceCell::new();

// Event enum represents actions that can be dispatched to the app
#[derive(uniffi::Enum)]
pub enum Event {
    PushRoute { route: Route },
    PopRoute,
    ResetRouter,
}

#[derive(Clone)]
pub struct App {
    update_receiver: Arc<Receiver<Update>>,
    pub data_dir: String,
}

impl App {
    /// Create a new instance of the app
    pub fn new(ffi_app: &FfiApp) -> Self {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Info),
        );

        let (sender, receiver): (Sender<Update>, Receiver<Update>) = unbounded();
        Updater::init(sender);

        // Store data_dir for future use
        let data_dir = ffi_app.data_dir.clone();

        Self {
            update_receiver: Arc::new(receiver),
            data_dir,
        }
    }

    /// Fetch global instance of the app, or create one if it doesn't exist
    pub fn global(ffi_app: &FfiApp) -> &'static RwLock<App> {
        APP.get_or_init(|| RwLock::new(App::new(ffi_app)))
    }

    /// Handle event received from frontend
    pub fn handle_event(&self, event: Event) {
        // Handle event
        match event {
            Event::PushRoute { route } => {
                // Use the global database
                let db = Database::global();
                db.push_route(&route).expect("Failed to push route");
            }
            Event::PopRoute => {
                // Use the global database
                let db = Database::global();
                db.pop_route().expect("Failed to pop route");
            }
            Event::ResetRouter => {
                // Use the global database
                let db = Database::global();
                db.reset_router().expect("Failed to reset router");
            }
        }
    }

    /// Set up listener for database updates
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
pub struct FfiApp {
    // FIXME: this is database path currently, not actually data dir
    pub data_dir: String,
}

#[uniffi::export]
impl FfiApp {
    /// FFI constructor which wraps in an Arc
    #[uniffi::constructor]
    pub fn new(data_dir: String) -> Arc<Self> {
        // Ensure DATABASE initialized. We can now assume DATABASE exists everywhere in our code.
        if DATABASE.get().is_none() {
            let db = Database::new(data_dir.clone()).expect("Failed to initialize database");
            let _ = DATABASE.set(RwLock::new(db));
        }

        Arc::new(Self { data_dir })
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

    /// Get the router
    pub fn get_router(&self) -> Router {
        // Use Router's static method directly
        Router::get()
    }

    /// Get the current route (or None if router is empty)
    pub fn get_current_route(&self) -> Option<Route> {
        // Use Router's static method directly
        Router::get().current_route()
    }
}

impl FfiApp {
    /// Fetch global instance of the app, or create one if it doesn't exist
    fn inner(&self) -> &RwLock<App> {
        App::global(self)
    }
}

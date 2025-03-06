use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};

use crate::database::Database;
use crate::navigation::Router;
use crate::updater::{FfiUpdater, Update, Updater};
use crate::Route;

// Global static APP instance
static APP: OnceCell<RwLock<App>> = OnceCell::new();

// Event enum represents actions that can be dispatched to the app
#[derive(uniffi::Enum)]
pub enum Event {
    Increment,
    Decrement,
    PushRoute { route: Route },
    PopRoute,
    ResetRouter,
}

#[derive(Clone)]
pub struct App {
    update_receiver: Arc<Receiver<Update>>,
    db: Database,
}

impl App {
    /// Create a new instance of the app
    pub fn new(ffi_app: &FfiApp) -> Self {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Info),
        );

        let (sender, receiver): (Sender<Update>, Receiver<Update>) = unbounded();
        Updater::init(sender);

        // Create the database
        let db = Database::new(ffi_app.data_dir.clone()).expect("Failed to initialize database");

        Self {
            update_receiver: Arc::new(receiver),
            db,
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
            Event::Increment => {
                self.db.increment_state();
            }
            Event::Decrement => {
                self.db.decrement_state();
            }
            Event::PushRoute { route } => {
                self.db.push_route(&route).expect("Failed to push route");
            }
            Event::PopRoute => {
                self.db.pop_route().expect("Failed to pop route");
            }
            Event::ResetRouter => {
                self.db.reset_router().expect("Failed to reset router");
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

    /// Get the router by querying the database directly
    pub fn get_router(&self) -> Router {
        // Use Router's from_database method
        Router::from_database(&self.db).unwrap()
    }

    /// Get the current route by querying the database
    pub fn get_current_route(&self) -> Option<Route> {
        // Get router and return the last route (if any)
        self.get_router().current_route()
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
        // Query directly from the database each time
        self.inner()
            .read()
            .expect("Failed to read app state")
            .get_router()
    }

    /// Get the current route (or None if router is empty)
    pub fn get_current_route(&self) -> Option<Route> {
        // Query directly from the database each time
        self.inner()
            .read()
            .expect("Failed to read app state")
            .get_current_route()
    }
}

impl FfiApp {
    /// Fetch global instance of the app, or create one if it doesn't exist
    fn inner(&self) -> &RwLock<App> {
        App::global(self)
    }
}

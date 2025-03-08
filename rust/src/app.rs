use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use nostr_sdk::prelude::*;
use tokio::runtime::Runtime;

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
    SendPost { text: String },
}

#[derive(Clone)]
pub struct App {
    update_receiver: Arc<Receiver<Update>>,
    #[allow(dead_code)]
    data_dir: String,
    runtime: Arc<Runtime>,
}

impl App {
    /// Create a new instance of the app
    pub fn new(ffi_app: &FfiApp) -> Self {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Info),
        );

        let (sender, receiver): (Sender<Update>, Receiver<Update>) = unbounded();
        Updater::init(sender);

        // Create tokio runtime for async operations
        let runtime = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));

        Self {
            update_receiver: Arc::new(receiver),
            data_dir: ffi_app.data_dir.clone(),
            runtime,
        }
    }

    /// Fetch global instance of the app, or create one if it doesn't exist
    pub fn global(ffi_app: &FfiApp) -> &'static RwLock<App> {
        APP.get_or_init(|| RwLock::new(App::new(ffi_app)))
    }

    async fn send_nostr_post(text: String) -> Result<String, Box<dyn std::error::Error>> {
        // Generate new random keys
        let keys = Keys::generate();

        // Create new client
        let client = Client::new(keys);

        // Add relays
        client.add_relay("wss://relay.damus.io").await?;
        
        // Connect to relays
        client.connect().await;

        // Publish text note
        let builder = EventBuilder::text_note(text);
        let event_id = client.send_event_builder(builder).await?;
        
        // Disconnect from relays (no need for ? since we don't care about result)
        let _ = client.disconnect().await;
        
        Ok(event_id.to_hex())
    }

    /// Handle event received from frontend
    pub fn handle_event(&self, event: Event) {
        let db = Database::global();
        match event {
            Event::PushRoute { route } => db.push_route(&route).unwrap(),
            Event::PopRoute => db.pop_route().unwrap(),
            Event::ResetRouter => db.reset_router().unwrap(),
            Event::SendPost { text } => {
                let runtime = self.runtime.clone();
                
                // Spawn a new thread for async operations
                std::thread::spawn(move || {
                    runtime.block_on(async {
                        match Self::send_nostr_post(text.clone()).await {
                            Ok(event_id) => {
                                log::info!("Post sent successfully with event ID: {}", event_id);
                                crate::updater::Updater::send_update(
                                    crate::updater::Update::PostSendSuccess { 
                                        message: "Post sent successfully!".to_string(),
                                        event_id,
                                    }
                                );
                            },
                            Err(e) => {
                                log::error!("Failed to send post: {}", e);
                                // TODO: Add error handling update variant
                            }
                        }
                    });
                });
            },
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

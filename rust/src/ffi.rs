use std::sync::{Arc, RwLock};

use crate::{App, Event, FfiUpdater};

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

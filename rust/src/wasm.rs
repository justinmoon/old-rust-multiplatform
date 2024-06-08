use std::sync::{Arc, RwLock};

use wasm_bindgen::prelude::*;

use crate::{App, Event, FfiUpdater, Update};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type WasmUpdater;

    #[wasm_bindgen(method)]
    pub fn update(this: &WasmUpdater, event: String);
}

// impl WasmUpdater {
//     pub fn update(&self, update: Update) {
//         let update = serde_json::to_string(&update).expect("fixme");
//         self.update(update)
//     }
// }

/// Representation of our app over FFI. Essentially a wrapper of [`App`].
#[wasm_bindgen]
pub struct WasmApp;

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmApp {
        Self
    }

    // pub fn new() -> Arc<WasmApp> {
    //     Arc::new(Self)
    // }

    /// Frontend calls this method to send events to the rust application logic
    pub fn dispatch(&self, event: String) {
        let event: Event = serde_json::from_str(&event).expect("fixme");
        // FIXME: this won't be able to handle concurrent events ...
        self.inner().write().expect("fixme").handle_event(event);
    }

    pub fn listen_for_updates(&self, updater: WasmUpdater) {
        self.inner()
            .read()
            .expect("fixme")
            .listen_for_updates_wasm(updater);
    }
}

impl WasmApp {
    /// Fetch global instance of the app, or create one if it doesn't exist
    fn inner(&self) -> &RwLock<App> {
        App::global()
    }
}

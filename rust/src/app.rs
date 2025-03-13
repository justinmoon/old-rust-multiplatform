use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};

use crate::view_model::{FfiViewModel, ModelUpdate, ViewModel};

// Global static APP instance
static GLOBAL_MODEL: OnceCell<RwLock<Action>> = OnceCell::new();

// Event enum represents actions that can be dispatched to the app
#[derive(uniffi::Enum)]
pub enum Event {}

// TODO: derive RmpApp which adds global() method and generates FfiApp?
#[derive(Clone)]
pub struct Action {
    model_update_rx: Arc<Receiver<ModelUpdate>>,
    #[allow(dead_code)]
    data_dir: String,
}

impl Action {
    /// Create a new instance of the app
    pub fn new(singleton: &FfiModel) -> Self {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Info),
        );

        let (sender, receiver): (Sender<ModelUpdate>, Receiver<ModelUpdate>) = unbounded();
        ViewModel::init(sender);

        Self {
            model_update_rx: Arc::new(receiver),
            data_dir: singleton.data_dir.clone(),
        }
    }

    /// Fetch global instance of the app, or create one if it doesn't exist
    pub fn get_or_set_global_model(ffi_model: &FfiModel) -> &'static RwLock<Action> {
        GLOBAL_MODEL.get_or_init(|| RwLock::new(Action::new(ffi_model)))
    }

    /// Handle event received from frontend
    pub fn handle_event(&self, event: Event) {
        match event {}
    }

    /// Set up listener for database updates
    pub fn listen_for_updates(&self, updater: Box<dyn FfiViewModel>) {
        let update_receiver = self.model_update_rx.clone();
        std::thread::spawn(move || {
            while let Ok(field) = update_receiver.recv() {
                updater.dispatch(field);
            }
        });
    }
}

/// Representation of our app over FFI. Essentially a wrapper of [`App`].
#[derive(uniffi::Object)]
pub struct FfiModel {
    // FIXME: this is database path currently, not actually data dir
    #[allow(unused_variables)]
    pub data_dir: String,
}

#[uniffi::export]
impl FfiModel {
    #[uniffi::constructor]
    pub fn new(data_dir: String) -> Arc<Self> {
        Arc::new(Self { data_dir })
    }

    /// Frontend calls this method to send events to the rust application logic
    pub fn dispatch(&self, event: Event) {
        self.get_or_set_global_model()
            .write()
            .expect("fixme")
            .handle_event(event);
    }

    pub fn listen_for_updates(&self, updater: Box<dyn FfiViewModel>) {
        self.get_or_set_global_model()
            .read()
            .expect("fixme")
            .listen_for_updates(updater);
    }
}

impl FfiModel {
    /// Fetch global instance of the app, or create one if it doesn't exist
    fn get_or_set_global_model(&self) -> &RwLock<Action> {
        Action::get_or_set_global_model(self)
    }
}

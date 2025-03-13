use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};

use crate::view_model::{ModelUpdate, RmpViewModel, ViewModel};

// Global static APP instance
static GLOBAL_MODEL: OnceCell<RwLock<Model>> = OnceCell::new();

// Event enum represents actions that can be dispatched to the app
#[derive(uniffi::Enum)]
pub enum Action {
    Increment,
    Decrement,
}

// TODO: derive RmpApp which adds global() method and generates FfiApp?
#[derive(Clone)]
pub struct Model {
    model_update_rx: Arc<Receiver<ModelUpdate>>,
    #[allow(dead_code)]
    data_dir: String,
    count: i32,
}

impl Model {
    /// Create a new instance of the app
    pub fn new(singleton: &RmpModel) -> Self {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Info),
        );

        let (sender, receiver): (Sender<ModelUpdate>, Receiver<ModelUpdate>) = unbounded();
        ViewModel::init(sender);

        Self {
            model_update_rx: Arc::new(receiver),
            data_dir: singleton.data_dir.clone(),
            count: 0,
        }
    }

    /// Fetch global instance of the app, or create one if it doesn't exist
    pub fn get_or_set_global_model(ffi_model: &RmpModel) -> &'static RwLock<Model> {
        GLOBAL_MODEL.get_or_init(|| RwLock::new(Model::new(ffi_model)))
    }

    /// Handle event received from frontend
    pub fn action(&mut self, action: Action) {
        match action {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
        }
        ViewModel::model_update(ModelUpdate::CountChanged { count: self.count });
    }

    /// Set up listener for database updates
    pub fn listen_for_model_updates(&self, rmp_view_model: Box<dyn RmpViewModel>) {
        let model_update_rx = self.model_update_rx.clone();
        std::thread::spawn(move || {
            while let Ok(field) = model_update_rx.recv() {
                rmp_view_model.model_update(field);
            }
        });
    }
}

/// Representation of our app over FFI. Essentially a wrapper of [`App`].
#[derive(uniffi::Object)]
pub struct RmpModel {
    // FIXME: this is database path currently, not actually data dir
    #[allow(unused_variables)]
    pub data_dir: String,
}

#[uniffi::export]
impl RmpModel {
    #[uniffi::constructor]
    pub fn new(data_dir: String) -> Arc<Self> {
        Arc::new(Self { data_dir })
    }

    /// Frontend calls this method to send events to the rust application logic
    pub fn action(&self, action: Action) {
        self.get_or_set_global_model()
            .write()
            .expect("fixme")
            .action(action);
    }

    // TODO: should it be
    pub fn listen_for_model_updates(&self, updater: Box<dyn RmpViewModel>) {
        self.get_or_set_global_model()
            .read()
            .expect("fixme")
            .listen_for_model_updates(updater);
    }

    // FIXME: could have just a `get_initial_state` which just returns a big struct.
    // then we wouldn't need to update this over time.
    pub fn get_count(&self) -> i32 {
        self.get_or_set_global_model().read().expect("fixme").count
    }
}

impl RmpModel {
    /// Fetch global instance of the app, or create one if it doesn't exist
    fn get_or_set_global_model(&self) -> &RwLock<Model> {
        Model::get_or_set_global_model(self)
    }
}

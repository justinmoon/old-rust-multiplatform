use rust_multiplatform::{
    RmpAppModel, BuildableApp,
    create_model_update_channel, create_app_builder,
    crossbeam::channel::Receiver,
};
use std::sync::Arc;
use log; // for android_logger

#[derive(uniffi::Enum)]
pub enum Action {
    Increment,
    Decrement,
}

#[derive(uniffi::Enum, Clone)]
pub enum ModelUpdate {
    CountChanged { count: i32 },
}

#[derive(Clone)]
pub struct ViewModel(pub crossbeam::channel::Sender<ModelUpdate>);

pub struct Model {
    count: i32,
    builder: rust_multiplatform::traits::AppBuilder<Self, ModelUpdate>,
}

impl RmpAppModel for Model {
    type ActionType = Action;
    type UpdateType = ModelUpdate;

    fn create(data_dir: String) -> Self {
        // Optional: init logging here
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Info)
        );

        // Set up the channel for Model->View updates
        let (tx, rx) = create_model_update_channel::<ModelUpdate>();

        // Initialize our local ViewModel so it has a valid Sender
        ViewModel::init(tx);

        // Create the builder with the receiver
        let builder = create_app_builder::<Self, ModelUpdate>(data_dir, rx);

        Self {
            count: 0,
            builder,
        }
    }

    fn action(&mut self, action: Action) {
        match action {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
        }
        // Send the updated count to the ViewModel
        ViewModel::model_update(ModelUpdate::CountChanged { count: self.count });
    }

    /// Ties into the framework so it can spawn the listener thread
    fn get_update_receiver(&self) -> Option<Arc<Receiver<ModelUpdate>>> {
        Some(self.builder.model_update_rx.clone())
    }
}

// Required so the framework can call `builder()` to get the receiver, data_dir, etc.
impl BuildableApp<ModelUpdate> for Model {
    fn builder(&self) -> &rust_multiplatform::traits::AppBuilder<Self, ModelUpdate> {
        &self.builder
    }
}

// Invoke register_app! to generate the FFI
use rust_multiplatform::register_app;

register_app!(Model, ViewModel, Action, ModelUpdate);
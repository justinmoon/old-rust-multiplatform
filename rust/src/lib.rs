rust_multiplatform::register_app!(Model, ViewModel, Action, ModelUpdate);

use crossbeam::channel::{unbounded, Receiver, Sender};
use std::sync::Arc;

mod logging;

#[derive(Debug, PartialEq, Clone, uniffi::Enum)]
pub enum ModelUpdate {
    CountChanged { count: i32 },
}

#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum Action {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub struct Model {
    pub count: i32,
    pub data_dir: String,
    update_receiver: Arc<Receiver<ModelUpdate>>,
}

// Implement RmpAppModel for the model
impl rust_multiplatform::traits::RmpAppModel for Model {
    type ActionType = Action;
    type UpdateType = ModelUpdate;

    fn create(data_dir: String) -> Self {
        // Create a channel, give sender to ViewModel and receiver to Model
        let (sender, receiver) = unbounded();
        ViewModel::init(sender);
        Model {
            count: 0,
            data_dir,
            update_receiver: Arc::new(receiver),
        }
    }

    fn action(&mut self, action: Self::ActionType) {
        match action {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
        }
        ViewModel::model_update(ModelUpdate::CountChanged { count: self.count });
    }

    fn get_update_receiver(&self) -> Arc<Receiver<Self::UpdateType>> {
        self.update_receiver.clone()
    }
}

#[derive(Clone)]
struct ViewModel(pub Sender<ModelUpdate>);

// Use the register_app macro to generate the FFI code

#[uniffi::export]
impl RmpModel {
    pub fn get_count(&self) -> i32 {
        self.get_or_set_global_model()
            .read()
            .expect("Failed to acquire read lock on model")
            .count
    }

    /// Initialize platform-specific logging
    pub fn setup_logging(&self) {
        logging::init_logging();
    }
}

#[test]
fn test_model_creation() {
    // Create an RmpModel instance
    let model = RmpModel::new("test_dir".to_string());

    // Verify it has the right data_dir
    assert_eq!(model.data_dir, "test_dir");
}

#[test]
fn test_action_handling() {
    // Create an RmpModel instance
    let model = RmpModel::new("test_dir".to_string());

    // Call the action method
    model.action(Action::Action);

    // Get the global model
    let global_model = model.get_or_set_global_model().read().unwrap();

    // Verify the action was handled
    assert_eq!(global_model.count, 1);
}

#[test]
fn test_view_model() {
    // Create a channel for the view model
    let (sender, receiver) = unbounded();

    // Initialize the view model
    ViewModel::init(sender);

    // Send a model update
    ViewModel::model_update(ModelUpdate::Update { value: 42 });

    // Verify the update was sent
    if let Ok(update) = receiver.try_recv() {
        match update {
            ModelUpdate::Update { value } => assert_eq!(value, 42),
        }
    } else {
        panic!("No update received");
    }
}

uniffi::setup_scaffolding!();

use crossbeam::channel::{unbounded, Sender};

// Define a model update type
#[derive(Debug, PartialEq, Clone, uniffi::Enum)]
pub enum ModelUpdate {
    Update { value: i32 },
}

// Define an action type
#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum Action {
    Action,
}

// Define a model with an app builder field for the update receiver
#[derive(Debug)]
pub struct Model {
    pub count: i32,
    pub data_dir: String,
}

// Implement RmpAppModel for the model
impl rust_multiplatform::traits::RmpAppModel for Model {
    type ActionType = Action;
    type UpdateType = ModelUpdate;

    fn create(data_dir: String) -> Self {
        Model { count: 0, data_dir }
    }

    fn action(&mut self, action: Self::ActionType) {
        match action {
            Action::Action => self.count += 1,
        }
    }
}

// Define a view model
#[derive(Clone)]
struct ViewModel(pub Sender<ModelUpdate>);

// Use the register_app macro to generate the FFI code
rust_multiplatform::register_app!(Model, ViewModel, Action, ModelUpdate);

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

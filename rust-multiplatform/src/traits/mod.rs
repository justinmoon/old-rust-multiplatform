//! Traits for application models to implement
//!
//! These traits define the interface between the user's app code and the framework.

use crossbeam::channel::Receiver;
use std::sync::Arc;

/// Trait for view models that can receive updates from the model
///
/// This trait is implemented by the framework's generated code and used as a callback
/// interface for sending updates from the model to the view.
pub trait RmpViewModel: Send + Sync + 'static {
    /// The type of updates that can be received from the model
    type UpdateType;
    
    /// Handle a model update
    ///
    /// This function is called when the model has an update to send to the view.
    fn model_update(&self, model_update: Self::UpdateType);
}

/// Trait for application models that can be managed by the framework
///
/// By implementing this trait, a model can be initialized and managed by the framework,
/// with automatic integration into the FFI layer.
pub trait RmpAppModel {
    /// The type of actions that can be dispatched to the model
    type ActionType;
    
    /// The type of updates that can be sent from the model to the view
    type UpdateType;
    
    /// Create a new instance of the model
    ///
    /// This function is called by the framework to initialize the model.
    /// The `data_dir` parameter provides a location for storing app data.
    fn create(data_dir: String) -> Self;
    
    /// Handle an action dispatched to the model
    ///
    /// This function is called when an action is dispatched from the frontend.
    fn action(&mut self, action: Self::ActionType);
    
    /// Get access to the model update receiver
    ///
    /// This is used by the framework to access the receiver for model updates.
    /// App developers should not need to implement this method manually as
    /// the app builder helper will implement it.
    fn get_update_receiver(&self) -> Option<Arc<Receiver<Self::UpdateType>>> {
        None
    }
}
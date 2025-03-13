//! Traits for application models to implement
//!
//! These traits define the interface between the user's app code and the framework.

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
}
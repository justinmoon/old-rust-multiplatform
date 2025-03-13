//! Utility functions for the rust-multiplatform framework

use crate::traits::{RmpAppModel, RmpViewModel};
use crossbeam::channel::Receiver;
use std::sync::Arc;
use std::thread;

/// Set up a listener for model updates
///
/// This function creates a thread that listens for model updates and forwards them to the view model.
/// It's used by the framework's generated code to handle the boilerplate of setting up the listener.
pub fn listen_for_model_updates<M, V>(
    model: &M,
    view_model: Box<V>,
) where
    M: RmpAppModel,
    V: RmpViewModel<UpdateType = M::UpdateType> + 'static,
{
    // Get the receiver from the model
    if let Some(model_update_rx) = model.get_update_receiver() {
        let model_update_rx = model_update_rx.clone();
        
        // Spawn a thread to listen for updates
        thread::spawn(move || {
            while let Ok(update) = model_update_rx.recv() {
                view_model.model_update(update);
            }
        });
    } else {
        log::warn!("Model does not have an update receiver set up. Updates will not be forwarded to the view.");
    }
}
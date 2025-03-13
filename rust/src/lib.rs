uniffi::setup_scaffolding!();

// Module declarations
mod app;
mod view_model;

// Re-exports for FFI
pub use app::{Event, RmpModel};
pub use view_model::{ModelUpdate, RmpViewModel};

// Internal re-exports
pub(crate) use view_model::ViewModel;

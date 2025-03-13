uniffi::setup_scaffolding!();

// Module declarations
mod app;
mod view_model;

// Re-exports for FFI
pub use app::{Event, FfiModel};
pub use view_model::{FfiViewModel, ModelUpdate};

// Internal re-exports
pub(crate) use view_model::ViewModel;

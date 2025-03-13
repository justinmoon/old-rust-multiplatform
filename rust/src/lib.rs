uniffi::setup_scaffolding!();

// Module declarations
mod app;
mod view_model;

// Re-exports for FFI
pub use app::{Action, RmpModel};
pub use view_model::{ModelUpdate, RmpViewModel};

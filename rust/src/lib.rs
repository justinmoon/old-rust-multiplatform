uniffi::setup_scaffolding!();

// Module declarations
mod app;
mod updater;

// Re-exports for FFI
pub use app::{Event, FfiApp};
pub use updater::{FfiUpdater, Update};

// Internal re-exports
pub(crate) use updater::Updater;

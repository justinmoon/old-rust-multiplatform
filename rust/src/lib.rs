uniffi::setup_scaffolding!();

// Module declarations
mod app;
mod database;
mod navigation;
mod updater;

// Re-exports for FFI
pub use app::{Event, FfiApp};
pub use database::FfiDatabase;
pub use navigation::{Route, Router};
pub use updater::{FfiUpdater, Update};

// Internal re-exports
pub(crate) use database::Database;
pub(crate) use updater::Updater;

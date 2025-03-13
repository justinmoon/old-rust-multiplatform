//! Rust Multiplatform Framework
//!
//! This crate provides utilities for building cross-platform applications with Rust,
//! focusing on simplifying the FFI layer and eliminating boilerplate.

// Re-export uniffi for convenience in user crates
pub use uniffi;

// Internal modules
mod macros;
mod utils;
mod generated_code_template;

// Public exports
pub use macros::register_app;
pub use utils::listen_for_model_updates;

/// Traits that can be implemented by app models to integrate with the framework
pub mod traits;

// Re-export frequently used types to make it easier for app developers
pub use crossbeam;
pub use once_cell;
//! Rust Multiplatform Framework
//!
//! This crate provides utilities for building cross-platform applications with Rust,
//! focusing on simplifying the FFI layer and eliminating boilerplate.

// Re-export uniffi for convenience in user crates
pub use uniffi;

mod macros;

// Export the register_app macro as the main entry point for users
pub use macros::register_app;

/// Traits that can be implemented by app models to integrate with the framework
pub mod traits;
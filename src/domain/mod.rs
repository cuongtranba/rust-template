//! Domain layer - Core business logic
//!
//! This module contains the heart of the application:
//! - **entities**: Business objects and value objects
//! - **ports**: Trait definitions (interfaces) for external dependencies
//! - **services**: Business logic and use cases
//! - **errors**: Domain-specific error types
//!
//! ## Key Principle
//!
//! The domain layer has ZERO external dependencies. It only depends on
//! the Rust standard library and basic utilities (uuid, chrono, etc.).
//! All infrastructure concerns are abstracted behind traits in `ports`.

pub mod entities;
pub mod errors;
pub mod ports;
pub mod services;

// Re-export commonly used types
pub use entities::*;
pub use errors::DomainError;

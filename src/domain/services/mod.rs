//! Domain services - Business logic and use cases
//!
//! Services contain the business logic and orchestrate domain entities.
//! They depend on ports (traits) for external functionality.
//!
//! ## Key Principle
//!
//! Services are generic over their dependencies (ports), making them
//! easily testable with mock implementations.

mod user_service;

pub use user_service::UserService;

//! Ports - Interface definitions (traits)
//!
//! Ports define the boundaries between the domain and external world.
//! They are implemented by adapters.
//!
//! ## Types of Ports
//!
//! - **Repository ports**: Data persistence abstractions
//! - **Service ports**: External service abstractions (email, payments, etc.)
//!
//! ## Key Principle
//!
//! The domain defines WHAT it needs (traits), adapters define HOW to provide it.

pub mod repositories;
pub mod services;

pub use repositories::UserRepository;
pub use services::EmailService;

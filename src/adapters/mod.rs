//! Adapters - Infrastructure implementations
//!
//! Adapters implement the ports defined in the domain layer.
//! They handle all external concerns like HTTP, databases, email, etc.
//!
//! ## Structure
//!
//! - **inbound**: Driving adapters (how external world interacts with our app)
//!   - HTTP handlers
//!   - CLI commands
//!   - gRPC services
//!   - Message queue consumers
//!
//! - **outbound**: Driven adapters (how our app interacts with external world)
//!   - Database repositories
//!   - External API clients
//!   - Cache implementations
//!   - Email service clients
//!
//! ## Key Principle
//!
//! Adapters depend on the domain layer (implement its ports).
//! The domain NEVER depends on adapters.

pub mod inbound;
pub mod outbound;

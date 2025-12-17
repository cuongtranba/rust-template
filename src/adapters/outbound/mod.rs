//! Outbound adapters - Driven adapters
//!
//! These adapters handle how our application interacts with the external world.
//! They implement the ports (traits) defined in the domain layer.
//!
//! ## Examples
//!
//! - **persistence**: Database implementations (PostgreSQL, SQLite, etc.)
//! - **external**: External API clients (payment gateways, etc.)
//! - **cache**: Caching implementations (Redis, in-memory)
//! - **email**: Email service implementations (SendGrid, SMTP)

pub mod external;
pub mod persistence;

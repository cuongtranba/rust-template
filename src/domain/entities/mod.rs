//! Domain entities and value objects
//!
//! Entities are objects with identity that persist over time.
//! Value objects are immutable objects defined by their attributes.
//!
//! ## Example Entity
//!
//! ```rust,ignore
//! use uuid::Uuid;
//! use chrono::{DateTime, Utc};
//!
//! pub struct User {
//!     pub id: Uuid,
//!     pub email: Email,
//!     pub name: String,
//!     pub created_at: DateTime<Utc>,
//!     pub updated_at: DateTime<Utc>,
//! }
//! ```
//!
//! ## Example Value Object
//!
//! ```rust,ignore
//! pub struct Email(String);
//!
//! impl Email {
//!     pub fn new(value: &str) -> Result<Self, ValidationError> {
//!         // Validation logic
//!     }
//! }
//! ```

mod user;

pub use user::{Email, User, UserId};

//! Persistence adapters
//!
//! Database implementations for the repository ports.
//! See `examples/web-api` for SQLx PostgreSQL implementation.
//!
//! ## In-Memory Implementation (for testing/development)
//!
//! ```rust,ignore
//! use std::collections::HashMap;
//! use std::sync::RwLock;
//!
//! pub struct InMemoryUserRepository {
//!     users: RwLock<HashMap<UserId, User>>,
//! }
//!
//! #[async_trait]
//! impl UserRepository for InMemoryUserRepository {
//!     async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
//!         Ok(self.users.read().unwrap().get(id).cloned())
//!     }
//!     // ... other methods
//! }
//! ```
//!
//! ## PostgreSQL Implementation
//!
//! ```rust,ignore
//! pub struct PostgresUserRepository {
//!     pool: PgPool,
//! }
//!
//! #[async_trait]
//! impl UserRepository for PostgresUserRepository {
//!     async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
//!         let result = sqlx::query_as!(
//!             UserRow,
//!             "SELECT * FROM users WHERE id = $1",
//!             id.0
//!         )
//!         .fetch_optional(&self.pool)
//!         .await
//!         .map_err(|e| DomainError::Infrastructure(e.into()))?;
//!
//!         Ok(result.map(|row| row.into()))
//!     }
//! }
//! ```

mod in_memory;

pub use in_memory::InMemoryUserRepository;

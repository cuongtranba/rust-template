//! Repository port definitions
//!
//! Repositories abstract data persistence. The domain defines what operations
//! it needs; adapters implement how to perform them (PostgreSQL, SQLite, etc.).

use async_trait::async_trait;

use crate::domain::{
    entities::{Email, User, UserId},
    errors::DomainError,
};

/// User repository port
///
/// Defines operations for persisting and retrieving users.
/// Implement this trait for your specific storage backend.
///
/// # Example Implementation
///
/// ```rust,ignore
/// pub struct PostgresUserRepository {
///     pool: PgPool,
/// }
///
/// #[async_trait]
/// impl UserRepository for PostgresUserRepository {
///     async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
///         // PostgreSQL implementation
///     }
/// }
/// ```
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find a user by their ID
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;

    /// Find a user by their email
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;

    /// Save a user (insert or update)
    async fn save(&self, user: &User) -> Result<(), DomainError>;

    /// Delete a user by their ID
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;

    /// List all users (with optional pagination in real implementations)
    async fn list(&self) -> Result<Vec<User>, DomainError>;
}

// Generate mock for testing (when mockall feature is enabled in tests)
#[cfg(test)]
mockall::mock! {
    pub UserRepository {}

    #[async_trait]
    impl UserRepository for UserRepository {
        async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
        async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
        async fn save(&self, user: &User) -> Result<(), DomainError>;
        async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
        async fn list(&self) -> Result<Vec<User>, DomainError>;
    }
}

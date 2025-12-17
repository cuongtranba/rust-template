//! Application state for dependency injection

use std::sync::Arc;

use async_trait::async_trait;

// Domain types (in real project, import from main crate)
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

// =============================================================================
// Domain Layer (copied from main crate for example independence)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.is_empty() || !value.contains('@') {
            return Err(DomainError::ValidationError("Invalid email".into()));
        }
        Ok(Self(value.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            name: name.into(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: &'static str, id: Uuid },
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] anyhow::Error),
}

impl DomainError {
    pub fn not_found<T>(id: Uuid) -> Self {
        Self::NotFound {
            entity_type: std::any::type_name::<T>(),
            id,
        }
    }
}

// =============================================================================
// Ports
// =============================================================================

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
    async fn list(&self) -> Result<Vec<User>, DomainError>;
}

// =============================================================================
// In-Memory Repository
// =============================================================================

pub struct InMemoryUserRepository {
    users: RwLock<HashMap<UserId, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let users = self.users.read().unwrap();
        Ok(users.get(id).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|u| &u.email == email).cloned())
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut users = self.users.write().unwrap();
        users.insert(user.id, user.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        let mut users = self.users.write().unwrap();
        users.remove(id);
        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>, DomainError> {
        let users = self.users.read().unwrap();
        Ok(users.values().cloned().collect())
    }
}

// =============================================================================
// Application State
// =============================================================================

/// Shared application state
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
}

impl AppState {
    /// Create state with in-memory repository (for development)
    pub fn new_in_memory() -> Self {
        Self {
            user_repository: Arc::new(InMemoryUserRepository::new()),
        }
    }

    // In production, add:
    // pub async fn new_postgres(database_url: &str) -> Result<Self> {
    //     let pool = PgPool::connect(database_url).await?;
    //     Ok(Self {
    //         user_repository: Arc::new(PostgresUserRepository::new(pool)),
    //     })
    // }
}

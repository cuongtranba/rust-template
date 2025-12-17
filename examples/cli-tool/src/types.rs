//! Domain types for CLI example
//!
//! These are simplified versions from the main crate.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// Value Objects
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
    pub fn new(value: impl Into<String>) -> anyhow::Result<Self> {
        let value = value.into();
        if value.is_empty() {
            anyhow::bail!("Email cannot be empty");
        }
        if !value.contains('@') {
            anyhow::bail!("Email must contain @");
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

// =============================================================================
// Entities
// =============================================================================

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

// =============================================================================
// Repository Port
// =============================================================================

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> anyhow::Result<Option<User>>;
    async fn save(&self, user: &User) -> anyhow::Result<()>;
    async fn delete(&self, id: &UserId) -> anyhow::Result<()>;
    async fn list(&self) -> anyhow::Result<Vec<User>>;
}

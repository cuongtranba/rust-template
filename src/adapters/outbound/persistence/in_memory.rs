//! In-memory repository implementations
//!
//! Useful for testing and development without a database.

use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::domain::{
    entities::{Email, User, UserId},
    errors::DomainError,
    ports::UserRepository,
};

/// In-memory user repository for testing and development
pub struct InMemoryUserRepository {
    users: RwLock<HashMap<UserId, User>>,
}

impl InMemoryUserRepository {
    /// Create a new empty in-memory repository
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let users = self
            .users
            .read()
            .map_err(|e| DomainError::Infrastructure(anyhow::anyhow!("Lock poisoned: {}", e)))?;
        Ok(users.get(id).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let users = self
            .users
            .read()
            .map_err(|e| DomainError::Infrastructure(anyhow::anyhow!("Lock poisoned: {}", e)))?;
        Ok(users.values().find(|u| &u.email == email).cloned())
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut users = self
            .users
            .write()
            .map_err(|e| DomainError::Infrastructure(anyhow::anyhow!("Lock poisoned: {}", e)))?;
        users.insert(user.id, user.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        let mut users = self
            .users
            .write()
            .map_err(|e| DomainError::Infrastructure(anyhow::anyhow!("Lock poisoned: {}", e)))?;
        users.remove(id);
        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>, DomainError> {
        let users = self
            .users
            .read()
            .map_err(|e| DomainError::Infrastructure(anyhow::anyhow!("Lock poisoned: {}", e)))?;
        Ok(users.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_find_by_id() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "Test User");
        let user_id = user.id;

        repo.save(&user).await.unwrap();

        let found = repo.find_by_id(&user_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test User");
    }

    #[tokio::test]
    async fn test_find_by_email() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email.clone(), "Test User");

        repo.save(&user).await.unwrap();

        let found = repo.find_by_email(&email).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test User");
    }

    #[tokio::test]
    async fn test_delete() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "Test User");
        let user_id = user.id;

        repo.save(&user).await.unwrap();
        repo.delete(&user_id).await.unwrap();

        let found = repo.find_by_id(&user_id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_list() {
        let repo = InMemoryUserRepository::new();

        let user1 = User::new(Email::new("user1@example.com").unwrap(), "User 1");
        let user2 = User::new(Email::new("user2@example.com").unwrap(), "User 2");

        repo.save(&user1).await.unwrap();
        repo.save(&user2).await.unwrap();

        let users = repo.list().await.unwrap();
        assert_eq!(users.len(), 2);
    }
}

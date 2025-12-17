//! User repository integration tests
//!
//! Tests the repository implementations with real operations.

use std::sync::Arc;

// Import from main crate (use actual crate name in real project)
// use {{project-name}}::{...};

// For now, we'll use local types since the crate name is templated

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

// =============================================================================
// Types (simplified for test independence)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Email(String);

impl Email {
    fn new(value: &str) -> Result<Self, String> {
        if value.is_empty() || !value.contains('@') {
            return Err("Invalid email".to_string());
        }
        Ok(Self(value.to_lowercase()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: UserId,
    email: Email,
    name: String,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl User {
    fn new(email: Email, name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            name: name.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

// =============================================================================
// In-Memory Repository
// =============================================================================

struct InMemoryUserRepository {
    users: RwLock<HashMap<UserId, User>>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }

    async fn find_by_id(&self, id: &UserId) -> Option<User> {
        self.users.read().unwrap().get(id).cloned()
    }

    async fn find_by_email(&self, email: &Email) -> Option<User> {
        self.users
            .read()
            .unwrap()
            .values()
            .find(|u| &u.email == email)
            .cloned()
    }

    async fn save(&self, user: &User) {
        self.users.write().unwrap().insert(user.id, user.clone());
    }

    async fn delete(&self, id: &UserId) {
        self.users.write().unwrap().remove(id);
    }

    async fn list(&self) -> Vec<User> {
        self.users.read().unwrap().values().cloned().collect()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[tokio::test]
async fn test_create_and_find_user() {
    let repo = InMemoryUserRepository::new();

    let email = Email::new("test@example.com").unwrap();
    let user = User::new(email, "Test User");
    let user_id = user.id;

    repo.save(&user).await;

    let found = repo.find_by_id(&user_id).await;
    assert!(found.is_some());

    let found_user = found.unwrap();
    assert_eq!(found_user.name, "Test User");
    assert_eq!(found_user.email.0, "test@example.com");
}

#[tokio::test]
async fn test_find_by_email() {
    let repo = InMemoryUserRepository::new();

    let email = Email::new("unique@example.com").unwrap();
    let user = User::new(email.clone(), "Unique User");

    repo.save(&user).await;

    let found = repo.find_by_email(&email).await;
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Unique User");
}

#[tokio::test]
async fn test_user_not_found() {
    let repo = InMemoryUserRepository::new();

    let random_id = UserId::new();
    let found = repo.find_by_id(&random_id).await;

    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_user() {
    let repo = InMemoryUserRepository::new();

    let email = Email::new("delete@example.com").unwrap();
    let user = User::new(email, "Delete Me");
    let user_id = user.id;

    repo.save(&user).await;
    assert!(repo.find_by_id(&user_id).await.is_some());

    repo.delete(&user_id).await;
    assert!(repo.find_by_id(&user_id).await.is_none());
}

#[tokio::test]
async fn test_list_users() {
    let repo = InMemoryUserRepository::new();

    let user1 = User::new(Email::new("user1@example.com").unwrap(), "User 1");
    let user2 = User::new(Email::new("user2@example.com").unwrap(), "User 2");
    let user3 = User::new(Email::new("user3@example.com").unwrap(), "User 3");

    repo.save(&user1).await;
    repo.save(&user2).await;
    repo.save(&user3).await;

    let users = repo.list().await;
    assert_eq!(users.len(), 3);
}

#[tokio::test]
async fn test_update_user() {
    let repo = InMemoryUserRepository::new();

    let email = Email::new("update@example.com").unwrap();
    let mut user = User::new(email, "Original Name");
    let user_id = user.id;

    repo.save(&user).await;

    // Update user
    user.name = "Updated Name".to_string();
    user.updated_at = Utc::now();
    repo.save(&user).await;

    let found = repo.find_by_id(&user_id).await.unwrap();
    assert_eq!(found.name, "Updated Name");
}

#[tokio::test]
async fn test_concurrent_access() {
    let repo = Arc::new(InMemoryUserRepository::new());

    let mut handles = vec![];

    // Spawn multiple tasks writing concurrently
    for i in 0..10 {
        let repo_clone = repo.clone();
        let handle = tokio::spawn(async move {
            let email = Email::new(&format!("user{}@example.com", i)).unwrap();
            let user = User::new(email, &format!("User {}", i));
            repo_clone.save(&user).await;
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all users were saved
    let users = repo.list().await;
    assert_eq!(users.len(), 10);
}

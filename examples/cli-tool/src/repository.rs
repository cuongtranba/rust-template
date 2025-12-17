//! File-based repository implementation
//!
//! Stores data in a JSON file for simplicity.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::types::{Email, User, UserId, UserRepository};

/// File-based user repository
///
/// Stores users in a JSON file. Suitable for CLI tools and simple use cases.
pub struct FileUserRepository {
    file_path: PathBuf,
    cache: RwLock<HashMap<UserId, User>>,
}

impl FileUserRepository {
    /// Create a new file repository
    pub fn new(file_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let file_path = file_path.into();

        // Load existing data if file exists
        let cache = if file_path.exists() {
            let content = std::fs::read_to_string(&file_path)?;
            let users: Vec<User> = serde_json::from_str(&content)?;
            users.into_iter().map(|u| (u.id, u)).collect()
        } else {
            HashMap::new()
        };

        Ok(Self {
            file_path,
            cache: RwLock::new(cache),
        })
    }

    /// Persist cache to file
    fn persist(&self) -> anyhow::Result<()> {
        let cache = self.cache.read().unwrap();
        let users: Vec<&User> = cache.values().collect();
        let content = serde_json::to_string_pretty(&users)?;
        std::fs::write(&self.file_path, content)?;
        Ok(())
    }
}

#[async_trait]
impl UserRepository for FileUserRepository {
    async fn find_by_id(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        let cache = self.cache.read().unwrap();
        Ok(cache.get(id).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> anyhow::Result<Option<User>> {
        let cache = self.cache.read().unwrap();
        Ok(cache.values().find(|u| &u.email == email).cloned())
    }

    async fn save(&self, user: &User) -> anyhow::Result<()> {
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(user.id, user.clone());
        }
        self.persist()
    }

    async fn delete(&self, id: &UserId) -> anyhow::Result<()> {
        {
            let mut cache = self.cache.write().unwrap();
            cache.remove(id);
        }
        self.persist()
    }

    async fn list(&self) -> anyhow::Result<Vec<User>> {
        let cache = self.cache.read().unwrap();
        Ok(cache.values().cloned().collect())
    }
}

//! User entity and related value objects
//!
//! This is an example entity to demonstrate the pattern.
//! Replace or extend with your own domain entities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Strongly-typed user identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    /// Create a new random user ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create from an existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
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

/// Email value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Create a new validated email
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic email validation
        if value.is_empty() {
            return Err(DomainError::validation("Email cannot be empty"));
        }

        if !value.contains('@') {
            return Err(DomainError::validation("Email must contain @"));
        }

        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::validation("Invalid email format"));
        }

        if !parts[1].contains('.') {
            return Err(DomainError::validation("Email domain must contain a dot"));
        }

        Ok(Self(value.to_lowercase()))
    }

    /// Get the email as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier
    pub id: UserId,
    /// User's email address
    pub email: Email,
    /// User's display name
    pub name: String,
    /// When the user was created
    pub created_at: DateTime<Utc>,
    /// When the user was last updated
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Create a new user
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

    /// Update the user's name
    pub fn update_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
        self.updated_at = Utc::now();
    }

    /// Update the user's email
    pub fn update_email(&mut self, email: Email) {
        self.email = email;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation_valid() {
        let email = Email::new("test@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().as_str(), "test@example.com");
    }

    #[test]
    fn test_email_validation_uppercase_normalized() {
        let email = Email::new("Test@Example.COM").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn test_email_validation_empty() {
        let email = Email::new("");
        assert!(email.is_err());
    }

    #[test]
    fn test_email_validation_no_at() {
        let email = Email::new("invalid-email");
        assert!(email.is_err());
    }

    #[test]
    fn test_email_validation_no_domain_dot() {
        let email = Email::new("test@localhost");
        assert!(email.is_err());
    }

    #[test]
    fn test_user_creation() {
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "Test User");

        assert_eq!(user.name, "Test User");
        assert_eq!(user.email.as_str(), "test@example.com");
    }

    #[test]
    fn test_user_update_name() {
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(email, "Old Name");
        let original_updated = user.updated_at;

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(10));

        user.update_name("New Name");

        assert_eq!(user.name, "New Name");
        assert!(user.updated_at > original_updated);
    }
}

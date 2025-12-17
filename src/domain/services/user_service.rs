//! User domain service
//!
//! Contains business logic for user operations.

use std::sync::Arc;

use crate::domain::{
    entities::{Email, User, UserId},
    errors::DomainError,
    ports::{EmailService, UserRepository},
};

/// User service containing business logic
///
/// This service is generic over its dependencies, allowing easy testing
/// with mock implementations.
///
/// # Example Usage
///
/// ```rust,ignore
/// let repo = Arc::new(PostgresUserRepository::new(pool));
/// let email = Arc::new(SendGridEmailService::new(api_key));
/// let service = UserService::new(repo, email);
///
/// let user = service.register("test@example.com", "Test User").await?;
/// ```
pub struct UserService<R, E>
where
    R: UserRepository,
    E: EmailService,
{
    repository: Arc<R>,
    email_service: Arc<E>,
}

impl<R, E> UserService<R, E>
where
    R: UserRepository,
    E: EmailService + 'static,
{
    /// Create a new user service
    pub fn new(repository: Arc<R>, email_service: Arc<E>) -> Self {
        Self {
            repository,
            email_service,
        }
    }

    /// Register a new user
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Email validation fails
    /// - User with email already exists
    /// - Repository operation fails
    pub async fn register(&self, email: &str, name: &str) -> Result<User, DomainError> {
        // Validate email
        let email = Email::new(email)?;

        // Check if user already exists
        if self.repository.find_by_email(&email).await?.is_some() {
            return Err(DomainError::conflict(format!(
                "User with email {} already exists",
                email
            )));
        }

        // Create new user
        let user = User::new(email.clone(), name);

        // Save to repository
        self.repository.save(&user).await?;

        // Send welcome email (fire and forget, log errors)
        let email_clone = email.clone();
        let email_service = self.email_service.clone();
        tokio::spawn(async move {
            if let Err(e) = email_service
                .send(
                    &email_clone,
                    "Welcome!",
                    "Thank you for registering with us.",
                )
                .await
            {
                tracing::warn!("Failed to send welcome email: {}", e);
            }
        });

        Ok(user)
    }

    /// Get a user by ID
    pub async fn get_by_id(&self, id: &UserId) -> Result<User, DomainError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::not_found::<User>(id.0))
    }

    /// Get a user by email
    pub async fn get_by_email(&self, email: &str) -> Result<User, DomainError> {
        let email = Email::new(email)?;
        self.repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| DomainError::validation("User not found"))
    }

    /// Update a user's name
    pub async fn update_name(&self, id: &UserId, new_name: &str) -> Result<User, DomainError> {
        let mut user = self.get_by_id(id).await?;
        user.update_name(new_name);
        self.repository.save(&user).await?;
        Ok(user)
    }

    /// Delete a user
    pub async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        // Verify user exists
        let _ = self.get_by_id(id).await?;
        self.repository.delete(id).await
    }

    /// List all users
    pub async fn list(&self) -> Result<Vec<User>, DomainError> {
        self.repository.list().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::repositories::MockUserRepository;
    use crate::domain::ports::services::MockEmailService;

    #[tokio::test]
    async fn test_register_success() {
        let mut mock_repo = MockUserRepository::new();
        let mut mock_email = MockEmailService::new();

        // Expect find_by_email to return None (user doesn't exist)
        mock_repo.expect_find_by_email().returning(|_| Ok(None));

        // Expect save to succeed
        mock_repo.expect_save().returning(|_| Ok(()));

        // Expect email to be sent
        mock_email.expect_send().returning(|_, _, _| Ok(()));

        let service = UserService::new(Arc::new(mock_repo), Arc::new(mock_email));

        let result = service.register("test@example.com", "Test User").await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email.as_str(), "test@example.com");
    }

    #[tokio::test]
    async fn test_register_duplicate_email() {
        let mut mock_repo = MockUserRepository::new();
        let mock_email = MockEmailService::new();

        let existing_user = User::new(Email::new("test@example.com").unwrap(), "Existing User");

        // Expect find_by_email to return existing user
        mock_repo
            .expect_find_by_email()
            .returning(move |_| Ok(Some(existing_user.clone())));

        let service = UserService::new(Arc::new(mock_repo), Arc::new(mock_email));

        let result = service.register("test@example.com", "New User").await;

        assert!(result.is_err());
        match result {
            Err(DomainError::Conflict(_)) => {}
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_register_invalid_email() {
        let mock_repo = MockUserRepository::new();
        let mock_email = MockEmailService::new();

        let service = UserService::new(Arc::new(mock_repo), Arc::new(mock_email));

        let result = service.register("invalid-email", "Test User").await;

        assert!(result.is_err());
        match result {
            Err(DomainError::ValidationError(_)) => {}
            _ => panic!("Expected ValidationError"),
        }
    }
}

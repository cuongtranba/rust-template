//! External service port definitions
//!
//! These traits abstract external services like email providers,
//! payment gateways, notification systems, etc.

use async_trait::async_trait;

use crate::domain::{entities::Email, errors::DomainError};

/// Email service port
///
/// Abstracts email sending functionality. Implement this trait
/// for your specific email provider (SendGrid, AWS SES, SMTP, etc.).
///
/// # Example Implementation
///
/// ```rust,ignore
/// pub struct SendGridEmailService {
///     api_key: String,
///     from_address: String,
/// }
///
/// #[async_trait]
/// impl EmailService for SendGridEmailService {
///     async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), DomainError> {
///         // SendGrid API implementation
///     }
/// }
/// ```
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send an email
    async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), DomainError>;

    /// Send an email with HTML body
    async fn send_html(
        &self,
        to: &Email,
        subject: &str,
        html_body: &str,
    ) -> Result<(), DomainError>;
}

// Generate mock for testing
#[cfg(test)]
mockall::mock! {
    pub EmailService {}

    #[async_trait]
    impl EmailService for EmailService {
        async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), DomainError>;
        async fn send_html(&self, to: &Email, subject: &str, html_body: &str) -> Result<(), DomainError>;
    }
}

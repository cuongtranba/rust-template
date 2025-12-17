//! Console email service - prints emails to stdout
//!
//! Useful for development and testing without a real email provider.

use async_trait::async_trait;

use crate::domain::{entities::Email, errors::DomainError, ports::EmailService};

/// Email service that prints to console
///
/// Use this for development and testing. Replace with a real
/// implementation (SendGrid, AWS SES, etc.) in production.
pub struct ConsoleEmailService;

impl ConsoleEmailService {
    /// Create a new console email service
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleEmailService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailService for ConsoleEmailService {
    async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), DomainError> {
        println!("========== EMAIL ==========");
        println!("To: {}", to);
        println!("Subject: {}", subject);
        println!("Body:");
        println!("{}", body);
        println!("===========================");
        Ok(())
    }

    async fn send_html(
        &self,
        to: &Email,
        subject: &str,
        html_body: &str,
    ) -> Result<(), DomainError> {
        println!("========== HTML EMAIL ==========");
        println!("To: {}", to);
        println!("Subject: {}", subject);
        println!("HTML Body:");
        println!("{}", html_body);
        println!("================================");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_email() {
        let service = ConsoleEmailService::new();
        let email = Email::new("test@example.com").unwrap();

        let result = service.send(&email, "Test Subject", "Test body").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_html_email() {
        let service = ConsoleEmailService::new();
        let email = Email::new("test@example.com").unwrap();

        let result = service
            .send_html(&email, "Test Subject", "<h1>Hello</h1>")
            .await;
        assert!(result.is_ok());
    }
}

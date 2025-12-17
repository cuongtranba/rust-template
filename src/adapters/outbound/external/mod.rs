//! External service adapters
//!
//! Implementations for external API clients.
//! See `examples/web-api` for real implementations.
//!
//! ## Console Email Service (for development)
//!
//! Prints emails to console instead of sending them.
//!
//! ## Real Implementation Example
//!
//! ```rust,ignore
//! pub struct SendGridEmailService {
//!     client: reqwest::Client,
//!     api_key: String,
//!     from_address: String,
//! }
//!
//! #[async_trait]
//! impl EmailService for SendGridEmailService {
//!     async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), DomainError> {
//!         // SendGrid API call
//!     }
//! }
//! ```

mod console_email;

pub use console_email::ConsoleEmailService;

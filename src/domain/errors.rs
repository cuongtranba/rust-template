//! Domain-specific error types
//!
//! These errors represent business rule violations and domain-level failures.
//! They use `thiserror` for ergonomic error handling.

use thiserror::Error;
use uuid::Uuid;

/// Domain-level errors representing business rule violations
#[derive(Debug, Error)]
pub enum DomainError {
    /// Entity was not found
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: &'static str, id: Uuid },

    /// Validation failed
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Business rule violation
    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    /// Conflict (e.g., duplicate entity)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Infrastructure error (wrapped from adapters)
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] anyhow::Error),
}

impl DomainError {
    /// Create a not found error for a specific entity type
    pub fn not_found<T>(id: Uuid) -> Self {
        Self::NotFound {
            entity_type: std::any::type_name::<T>(),
            id,
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    /// Create a business rule violation error
    pub fn business_rule(message: impl Into<String>) -> Self {
        Self::BusinessRuleViolation(message.into())
    }

    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }
}

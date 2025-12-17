//! HTTP error handling
//!
//! Maps domain errors to HTTP responses.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::app_state::DomainError;

/// Application error wrapper
pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the error
        tracing::error!("Application error: {:?}", self.0);

        // Check if it's a domain error
        if let Some(domain_error) = self.0.downcast_ref::<DomainError>() {
            return domain_error_to_response(domain_error);
        }

        // Default to internal server error
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
                message: None,
            }),
        )
            .into_response()
    }
}

fn domain_error_to_response(error: &DomainError) -> Response {
    match error {
        DomainError::NotFound { entity_type, id } => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Not found".to_string(),
                message: Some(format!("{} with id {} not found", entity_type, id)),
            }),
        )
            .into_response(),

        DomainError::ValidationError(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Validation error".to_string(),
                message: Some(msg.clone()),
            }),
        )
            .into_response(),

        DomainError::Conflict(msg) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Conflict".to_string(),
                message: Some(msg.clone()),
            }),
        )
            .into_response(),

        DomainError::Infrastructure(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
                message: None,
            }),
        )
            .into_response(),
    }
}

/// Error response format
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// Allow `?` operator to work with AppError
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

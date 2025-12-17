//! HTTP request handlers

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::{AppState, DomainError, Email, User, UserId, UserRepository};
use crate::error::AppError;

// =============================================================================
// Request/Response DTOs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.0,
            email: user.email.as_str().to_string(),
            name: user.name,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

// =============================================================================
// Handlers
// =============================================================================

/// Health check endpoint
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Create a new user
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // Validate email
    let email = Email::new(&req.email).map_err(|e| AppError(e.into()))?;

    // Check for existing user
    if state.user_repository.find_by_email(&email).await?.is_some() {
        return Err(AppError(
            DomainError::Conflict(format!("User with email {} already exists", email)).into(),
        ));
    }

    // Create and save user
    let user = User::new(email, &req.name);
    state.user_repository.save(&user).await?;

    tracing::info!("Created user: {}", user.id);

    Ok((StatusCode::CREATED, Json(user.into())))
}

/// Get a user by ID
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user_id = UserId(id);

    let user = state
        .user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| AppError(DomainError::not_found::<User>(id).into()))?;

    Ok(Json(user.into()))
}

/// List all users
pub async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = state.user_repository.list().await?;
    Ok(Json(users.into_iter().map(|u| u.into()).collect()))
}

/// Delete a user
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = UserId(id);

    // Verify user exists
    state
        .user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| AppError(DomainError::not_found::<User>(id).into()))?;

    state.user_repository.delete(&user_id).await?;

    tracing::info!("Deleted user: {}", id);

    Ok(StatusCode::NO_CONTENT)
}

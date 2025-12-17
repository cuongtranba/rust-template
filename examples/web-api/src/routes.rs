//! Route definitions

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::app_state::AppState;
use crate::handlers;

/// Create the application router
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health))
        // User routes
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::list_users))
        .route("/users/{id}", get(handlers::get_user))
        .route("/users/{id}", delete(handlers::delete_user))
        // Add state
        .with_state(state)
}

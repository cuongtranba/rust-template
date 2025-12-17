//! HTTP adapter
//!
//! REST API handlers using axum.
//! See `examples/web-api` for a complete implementation.
//!
//! ## Example Handler
//!
//! ```rust,ignore
//! use axum::{extract::State, Json};
//! use std::sync::Arc;
//!
//! pub async fn create_user(
//!     State(service): State<Arc<UserService>>,
//!     Json(req): Json<CreateUserRequest>,
//! ) -> Result<Json<UserResponse>, AppError> {
//!     let user = service.register(&req.email, &req.name).await?;
//!     Ok(Json(user.into()))
//! }
//! ```
//!
//! ## Setting Up Routes
//!
//! ```rust,ignore
//! use axum::{routing::{get, post}, Router};
//!
//! pub fn create_router<S>(service: Arc<UserService>) -> Router {
//!     Router::new()
//!         .route("/users", post(create_user))
//!         .route("/users/:id", get(get_user))
//!         .with_state(service)
//! }
//! ```

// HTTP handlers will be implemented in examples/web-api
// This module provides the structure and documentation

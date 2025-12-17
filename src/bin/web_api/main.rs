//! Web API Binary
//!
//! A complete REST API using axum with hexagonal architecture.
//!
//! ## Running
//!
//! ```bash
//! cargo run --bin web-api
//! ```
//!
//! ## Endpoints
//!
//! - `POST /users` - Create a new user
//! - `GET /users/:id` - Get a user by ID
//! - `GET /users` - List all users
//! - `DELETE /users/:id` - Delete a user
//! - `GET /health` - Health check

mod app_state;
mod error;
mod handlers;
mod routes;

use std::sync::Arc;

use anyhow::Result;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app_state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create application state with in-memory repository
    // In production, replace with PostgresUserRepository
    let state = Arc::new(AppState::new_in_memory());

    // Build router
    let app = routes::create_router(state).layer(TraceLayer::new_for_http());

    // Start server
    let addr = "127.0.0.1:3000";
    tracing::info!("Starting server on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

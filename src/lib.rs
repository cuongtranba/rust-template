//! # Rust Hexagonal Template
//!
//! A Rust project using hexagonal architecture.
//!
//! This project follows Hexagonal Architecture (Ports & Adapters) pattern:
//!
//! - **Domain**: Core business logic with zero external dependencies
//! - **Ports**: Trait definitions that define boundaries
//! - **Adapters**: Infrastructure implementations (HTTP, CLI, Database, etc.)
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                      Inbound Adapters                       │
//! │                    (HTTP, CLI, gRPC...)                     │
//! └─────────────────────────┬───────────────────────────────────┘
//!                           │
//!                           ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                         Domain                              │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │  Entities   │  │  Services   │  │   Ports (Traits)    │  │
//! │  └─────────────┘  └─────────────┘  └─────────────────────┘  │
//! └─────────────────────────┬───────────────────────────────────┘
//!                           │
//!                           ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     Outbound Adapters                       │
//! │               (Database, External APIs, Cache...)           │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod adapters;
pub mod config;
pub mod domain;

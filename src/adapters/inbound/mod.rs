//! Inbound adapters - Driving adapters
//!
//! These adapters handle how the external world interacts with our application.
//! They translate external requests into domain operations.
//!
//! ## Examples
//!
//! - **HTTP**: REST API handlers (see `examples/web-api`)
//! - **CLI**: Command-line interface (see `examples/cli-tool`)
//! - **gRPC**: RPC service handlers
//! - **Message Queue**: Event consumers

pub mod cli;
pub mod http;

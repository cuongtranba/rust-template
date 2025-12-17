//! CLI adapter
//!
//! Command-line interface using clap.
//! See `examples/cli-tool` for a complete implementation.
//!
//! ## Example CLI Definition
//!
//! ```rust,ignore
//! use clap::{Parser, Subcommand};
//!
//! #[derive(Parser)]
//! #[command(name = "myapp")]
//! #[command(about = "My application CLI")]
//! pub struct Cli {
//!     #[command(subcommand)]
//!     pub command: Commands,
//! }
//!
//! #[derive(Subcommand)]
//! pub enum Commands {
//!     /// Create a new user
//!     CreateUser {
//!         #[arg(short, long)]
//!         email: String,
//!         #[arg(short, long)]
//!         name: String,
//!     },
//!     /// List all users
//!     ListUsers,
//! }
//! ```
//!
//! ## Running Commands
//!
//! ```rust,ignore
//! let cli = Cli::parse();
//!
//! match cli.command {
//!     Commands::CreateUser { email, name } => {
//!         let user = service.register(&email, &name).await?;
//!         println!("Created user: {}", user.id);
//!     }
//!     Commands::ListUsers => {
//!         let users = service.list().await?;
//!         for user in users {
//!             println!("{}: {}", user.id, user.name);
//!         }
//!     }
//! }
//! ```

// CLI handlers will be implemented in examples/cli-tool
// This module provides the structure and documentation

//! CLI definition using clap

use clap::{Parser, Subcommand};

/// A CLI tool demonstrating hexagonal architecture
#[derive(Parser)]
#[command(name = "cli-tool")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new user
    #[command(name = "create-user")]
    CreateUser {
        /// User's email address
        #[arg(short, long)]
        email: String,

        /// User's display name
        #[arg(short, long)]
        name: String,
    },

    /// Get a user by ID
    #[command(name = "get-user")]
    GetUser {
        /// User ID (UUID)
        #[arg(short, long)]
        id: String,
    },

    /// List all users
    #[command(name = "list-users")]
    ListUsers,

    /// Delete a user
    #[command(name = "delete-user")]
    DeleteUser {
        /// User ID to delete (UUID)
        #[arg(short, long)]
        id: String,
    },
}

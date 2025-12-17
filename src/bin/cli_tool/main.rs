//! CLI Tool Binary
//!
//! A command-line interface demonstrating hexagonal architecture with clap.
//!
//! ## Running
//!
//! ```bash
//! cargo run --bin cli-tool -- --help
//! cargo run --bin cli-tool -- create-user --email user@example.com --name "John Doe"
//! cargo run --bin cli-tool -- list-users
//! ```

mod cli;
mod repository;
mod types;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use crate::cli::{Cli, Commands};
use crate::repository::FileUserRepository;
use crate::types::{Email, User, UserId, UserRepository};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize file-based repository
    let repo = FileUserRepository::new("users.json")?;

    match cli.command {
        Commands::CreateUser { email, name } => {
            create_user(&repo, &email, &name).await?;
        }
        Commands::GetUser { id } => {
            get_user(&repo, &id).await?;
        }
        Commands::ListUsers => {
            list_users(&repo).await?;
        }
        Commands::DeleteUser { id } => {
            delete_user(&repo, &id).await?;
        }
    }

    Ok(())
}

async fn create_user(repo: &FileUserRepository, email: &str, name: &str) -> Result<()> {
    let email = Email::new(email)?;

    // Check for existing user
    if repo.find_by_email(&email).await?.is_some() {
        eprintln!("{} User with email {} already exists", "Error:".red(), email);
        std::process::exit(1);
    }

    let user = User::new(email, name);
    repo.save(&user).await?;

    println!("{} Created user", "Success:".green());
    print_user(&user);

    Ok(())
}

async fn get_user(repo: &FileUserRepository, id: &str) -> Result<()> {
    let uuid = uuid::Uuid::parse_str(id)?;
    let user_id = UserId(uuid);

    match repo.find_by_id(&user_id).await? {
        Some(user) => {
            print_user(&user);
        }
        None => {
            eprintln!("{} User not found", "Error:".red());
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn list_users(repo: &FileUserRepository) -> Result<()> {
    let users = repo.list().await?;

    if users.is_empty() {
        println!("No users found.");
        return Ok(());
    }

    println!("{}", "Users:".bold());
    println!("{}", "-".repeat(60));

    for user in users {
        println!(
            "{}: {} <{}>",
            user.id.0.to_string().dimmed(),
            user.name.bold(),
            user.email
        );
    }

    Ok(())
}

async fn delete_user(repo: &FileUserRepository, id: &str) -> Result<()> {
    let uuid = uuid::Uuid::parse_str(id)?;
    let user_id = UserId(uuid);

    // Verify user exists
    if repo.find_by_id(&user_id).await?.is_none() {
        eprintln!("{} User not found", "Error:".red());
        std::process::exit(1);
    }

    repo.delete(&user_id).await?;
    println!("{} User deleted", "Success:".green());

    Ok(())
}

fn print_user(user: &User) {
    println!("  {}: {}", "ID".dimmed(), user.id);
    println!("  {}: {}", "Email".dimmed(), user.email);
    println!("  {}: {}", "Name".dimmed(), user.name);
    println!("  {}: {}", "Created".dimmed(), user.created_at);
}

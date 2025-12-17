# Rust Hexagonal Template - Design Document

**Date:** 2025-12-17
**Status:** Draft

## Overview

A general-purpose Rust project template implementing Hexagonal Architecture (Ports & Adapters). Designed for the community - beginner-friendly with sensible defaults while teaching clean architecture principles.

### Goals

- Provide a clean starting point for any Rust project type
- Demonstrate hexagonal architecture in idiomatic Rust
- Include concrete examples (Web API, CLI) users can learn from and copy
- Support both `cargo-generate` and GitHub template workflows
- Always use latest stable library versions

### Non-Goals (YAGNI)

- Not a framework - just structure and patterns
- Not prescriptive about specific libraries beyond examples
- No runtime abstractions or "magic"
- No Docker/containerization (users add if needed)
- No gRPC, messaging adapters (can add later)

### Target Users

- Rust developers wanting clean project structure
- Teams adopting hexagonal architecture
- Developers coming from Go/Java hexagonal backgrounds

## Architecture

### Key Principles

- Domain logic has zero external dependencies
- Adapters depend on domain, never reverse
- Traits define boundaries (ports)
- Easy to test via trait mocking
- Start simple, scale to workspace when needed

### Folder Structure

```
rust-template/
├── src/
│   ├── main.rs                 # Entry point, wires everything together
│   ├── lib.rs                  # Library root, exports public API
│   │
│   ├── domain/                 # Core business logic (no external deps)
│   │   ├── mod.rs
│   │   ├── entities/           # Business objects (User, Order, etc.)
│   │   ├── services/           # Domain services, business rules
│   │   ├── errors.rs           # Domain-specific errors (thiserror)
│   │   └── ports/              # Trait definitions (interfaces)
│   │       ├── mod.rs
│   │       ├── repositories.rs # Data persistence traits
│   │       └── services.rs     # External service traits
│   │
│   ├── adapters/               # Infrastructure implementations
│   │   ├── mod.rs
│   │   ├── inbound/            # Driving adapters (how users interact)
│   │   │   ├── http/           # REST API handlers
│   │   │   └── cli/            # CLI commands
│   │   └── outbound/           # Driven adapters (what domain needs)
│   │       ├── persistence/    # Database implementations
│   │       └── external/       # External API clients
│   │
│   └── config/                 # Configuration loading
│       └── mod.rs
│
├── tests/                      # Integration tests
│   └── api/
│
├── examples/                   # Pluggable example projects
│   ├── web-api/                # Full axum + SQLx example
│   └── cli-tool/               # Full clap + file storage example
│
├── docs/
│   ├── architecture.md         # Hexagonal explanation
│   └── workspace-migration.md  # Guide to split into workspace
│
├── .github/workflows/          # CI/CD
├── justfile                    # Task runner commands
├── Cargo.toml
└── cargo-generate.toml         # Template configuration
```

## Domain Layer

The domain layer is the heart of hexagonal architecture - pure business logic with zero external dependencies.

### Entities (`domain/entities/`)

```rust
// Simple structs representing business concepts
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub name: String,
    pub created_at: DateTime,
}

// Value objects with validation
pub struct Email(String);
impl Email {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        // validation logic
    }
}
```

### Ports (`domain/ports/`)

```rust
// Repository trait - domain defines WHAT it needs, not HOW
#[trait_variant::make(Send)]
pub trait UserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}

// External service trait
#[trait_variant::make(Send)]
pub trait EmailService {
    async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), DomainError>;
}
```

### Domain Services (`domain/services/`)

```rust
// Business logic using ports - testable via mocks
pub struct UserService<R: UserRepository, E: EmailService> {
    repo: R,
    email: E,
}

impl<R: UserRepository, E: EmailService> UserService<R, E> {
    pub async fn register(&self, email: Email, name: String) -> Result<User, DomainError> {
        // Pure business logic here
    }
}
```

### Errors (`domain/errors.rs`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("User not found: {0}")]
    UserNotFound(UserId),
    #[error("Invalid email format")]
    InvalidEmail,
}
```

## Adapters Layer

Adapters implement the ports and handle all external concerns. They depend on the domain, never the reverse.

### Inbound Adapters (HTTP)

```rust
// Axum handler - translates HTTP to domain calls
pub async fn create_user(
    State(service): State<Arc<dyn UserService>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let email = Email::new(&req.email)?;
    let user = service.register(email, req.name).await?;
    Ok(Json(user.into()))
}

// AppError maps DomainError to HTTP responses
pub struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Map to appropriate status codes
    }
}
```

### Inbound Adapters (CLI)

```rust
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateUser { email: String, name: String },
}
```

### Outbound Adapters (Database)

```rust
// Implements the port trait
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        sqlx::query_as!(...)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Infrastructure(e.into()))
    }
}
```

### Error Mapping Pattern

- Domain errors use `thiserror` (typed, explicit)
- Adapter errors wrap with `anyhow` for context
- Inbound adapters map to response types (HTTP status, CLI exit codes)

## Testing Strategy

### Unit Tests (in `src/` modules)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    mock! {
        UserRepo {}
        impl UserRepository for UserRepo {
            async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
            async fn save(&self, user: &User) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn register_creates_user_with_valid_email() {
        let mut mock_repo = MockUserRepo::new();
        mock_repo.expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let service = UserService::new(mock_repo);
        let result = service.register(
            Email::new("test@example.com").unwrap(),
            "Test".into()
        ).await;

        assert!(result.is_ok());
    }
}
```

### Integration Tests (`tests/`)

```rust
#[tokio::test]
async fn create_user_returns_201() {
    let app = TestApp::spawn().await;

    let response = app.post("/users")
        .json(&json!({"email": "test@example.com", "name": "Test"}))
        .await;

    assert_eq!(response.status(), 201);
}
```

### Test Organization

- `#[cfg(test)]` modules: Unit tests with mocks for domain logic
- `tests/`: Integration tests hitting real adapters
- Examples include their own test patterns
- `TestApp` builder for integration tests
- Mock implementations in `src/domain/ports/mocks.rs` (feature-gated)

## Developer Tooling

### justfile

```just
default:
    @just --list

# Development
run *ARGS:
    cargo run -- {{ARGS}}

dev:
    cargo watch -x run

# Quality
check:
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo test

fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings

# Testing
test:
    cargo test

test-unit:
    cargo test --lib

test-integration:
    cargo test --test '*'

# Build
build:
    cargo build --release

# Database (for web-api example)
db-migrate:
    sqlx migrate run

db-reset:
    sqlx database reset
```

### GitHub Actions

```yaml
name: CI
on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
```

### Additional Config Files

- `.gitignore` - Rust defaults + IDE files
- `rust-toolchain.toml` - Pin Rust version
- `clippy.toml` - Lint configuration
- `rustfmt.toml` - Format settings
- `dependabot.yml` - Automated dependency updates (weekly)

## Template Distribution

### cargo-generate.toml

```toml
[template]
cargo_generate_version = ">=0.18.0"

[placeholders.project-name]
type = "string"
prompt = "Project name?"
regex = "^[a-z][a-z0-9_-]*$"

[placeholders.description]
type = "string"
prompt = "Project description?"
default = "A Rust project using hexagonal architecture"

[placeholders.author]
type = "string"
prompt = "Author name?"

[placeholders.with_web_example]
type = "bool"
prompt = "Include web API example?"
default = true

[placeholders.with_cli_example]
type = "bool"
prompt = "Include CLI example?"
default = true
```

### Distribution Methods

1. **cargo-generate**: `cargo generate --git <repo>`
2. **GitHub template**: "Use this template" button

## Dependencies

All dependencies will use latest stable versions at time of creation.

### Core

| Crate | Purpose |
|-------|---------|
| tokio | Async runtime |
| thiserror | Domain error types |
| anyhow | Adapter error handling |
| serde | Serialization |

### Web API Example

| Crate | Purpose |
|-------|---------|
| axum | Web framework |
| sqlx | Database (PostgreSQL) |
| tower | Middleware |
| tower-http | HTTP middleware (cors, tracing) |

### CLI Example

| Crate | Purpose |
|-------|---------|
| clap | CLI parsing |
| serde_json | JSON file storage |

### Development

| Crate | Purpose |
|-------|---------|
| mockall | Mock generation |
| tokio-test | Async test utilities |

## Files to Create

### Core Template

- `src/main.rs`
- `src/lib.rs`
- `src/domain/mod.rs`
- `src/domain/entities/mod.rs`
- `src/domain/services/mod.rs`
- `src/domain/errors.rs`
- `src/domain/ports/mod.rs`
- `src/domain/ports/repositories.rs`
- `src/domain/ports/services.rs`
- `src/adapters/mod.rs`
- `src/adapters/inbound/mod.rs`
- `src/adapters/outbound/mod.rs`
- `src/config/mod.rs`

### Tests

- `tests/api/mod.rs`

### Examples

- `examples/web-api/` (full axum + SQLx setup)
- `examples/cli-tool/` (full clap setup)

### Documentation

- `README.md`
- `docs/architecture.md`
- `docs/workspace-migration.md`

### Configuration

- `Cargo.toml`
- `cargo-generate.toml`
- `justfile`
- `.github/workflows/ci.yml`
- `.github/dependabot.yml`
- `.gitignore`
- `rust-toolchain.toml`
- `clippy.toml`
- `rustfmt.toml`

## Summary

| Aspect | Decision |
|--------|----------|
| Purpose | General-purpose Rust template for community |
| Architecture | Hexagonal (Ports & Adapters) |
| Structure | Single crate (hybrid - docs for workspace migration) |
| Examples | Web API (axum + SQLx), CLI (clap + file storage) |
| Error handling | thiserror (domain) + anyhow (adapters) |
| Testing | Unit tests with mockall + integration tests |
| Distribution | cargo-generate + GitHub template |
| Tooling | just + GitHub Actions + dependabot |
| Dependencies | Always latest stable versions |

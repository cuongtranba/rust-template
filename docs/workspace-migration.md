# Migrating to a Cargo Workspace

As your project grows, you may want to split it into separate crates. This guide shows how to migrate from a single crate to a Cargo workspace.

## When to Migrate

Consider migrating when:

- Build times become slow (separate crates enable parallel compilation)
- You want to enforce architectural boundaries at compile time
- Multiple binaries share the same domain logic
- You want to publish the domain as a separate library

## Target Structure

```
my-project/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── domain/             # Pure domain logic
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── entities/
│   │       ├── services/
│   │       ├── ports/
│   │       └── errors.rs
│   │
│   ├── adapters/           # All adapters
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── inbound/
│   │       └── outbound/
│   │
│   └── app/                # Application entry point
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── config/
├── tests/
└── justfile
```

## Migration Steps

### Step 1: Create Workspace Structure

```bash
mkdir -p crates/{domain,adapters,app}/src
```

### Step 2: Create Root Cargo.toml

```toml
# Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
    "crates/domain",
    "crates/adapters",
    "crates/app",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT"

[workspace.dependencies]
# Shared dependencies - define versions once
tokio = { version = "1.48", features = ["full"] }
thiserror = "2.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.18", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
async-trait = "0.1"
tracing = "0.1"

# Internal crates
domain = { path = "crates/domain" }
adapters = { path = "crates/adapters" }
```

### Step 3: Create Domain Crate

```toml
# crates/domain/Cargo.toml
[package]
name = "domain"
version.workspace = true
edition.workspace = true

[dependencies]
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
serde.workspace = true
async-trait.workspace = true
```

Move domain files:

```bash
mv src/domain/* crates/domain/src/
```

Update `crates/domain/src/lib.rs`:

```rust
pub mod entities;
pub mod errors;
pub mod ports;
pub mod services;

pub use entities::*;
pub use errors::DomainError;
```

### Step 4: Create Adapters Crate

```toml
# crates/adapters/Cargo.toml
[package]
name = "adapters"
version.workspace = true
edition.workspace = true

[dependencies]
domain.workspace = true
tokio.workspace = true
anyhow.workspace = true
async-trait.workspace = true
tracing.workspace = true

# HTTP adapter (optional feature)
axum = { version = "0.8", optional = true }
tower = { version = "0.5", optional = true }
tower-http = { version = "0.6", features = ["cors", "trace"], optional = true }

# Database adapter (optional feature)
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"], optional = true }

[features]
default = []
http = ["axum", "tower", "tower-http"]
postgres = ["sqlx"]
full = ["http", "postgres"]
```

Move adapter files:

```bash
mv src/adapters/* crates/adapters/src/
```

### Step 5: Create App Crate

```toml
# crates/app/Cargo.toml
[package]
name = "app"
version.workspace = true
edition.workspace = true

[[bin]]
name = "my-app"
path = "src/main.rs"

[dependencies]
domain.workspace = true
adapters = { workspace = true, features = ["full"] }
tokio.workspace = true
anyhow.workspace = true
tracing.workspace = true
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
config = "0.14"
```

Move main.rs:

```bash
mv src/main.rs crates/app/src/
```

Update imports in `crates/app/src/main.rs`:

```rust
use domain::{User, UserService};
use adapters::{
    inbound::http::create_router,
    outbound::persistence::PostgresUserRepository,
};
```

### Step 6: Update Imports

Throughout your codebase, update imports:

```rust
// Before (single crate)
use crate::domain::entities::User;

// After (workspace)
use domain::entities::User;
// or
use domain::User;  // if re-exported in lib.rs
```

### Step 7: Update Tests

Integration tests move to the app crate or stay in the workspace root:

```toml
# Root Cargo.toml addition
[workspace]
members = [
    "crates/domain",
    "crates/adapters",
    "crates/app",
]

# Tests can import from any workspace crate
[dev-dependencies]
domain.workspace = true
adapters = { workspace = true, features = ["full"] }
```

### Step 8: Update justfile

```just
# Build all crates
build:
    cargo build --workspace

# Test all crates
test:
    cargo test --workspace

# Test specific crate
test-domain:
    cargo test -p domain

test-adapters:
    cargo test -p adapters

# Run the app
run:
    cargo run -p app
```

## Dependency Rules

Enforce these at compile time:

| Crate | Can depend on |
|-------|---------------|
| domain | Nothing (only std + utilities) |
| adapters | domain |
| app | domain, adapters |

If `domain` accidentally imports from `adapters`, compilation fails.

## Feature Flags

Use features to make adapters optional:

```rust
// crates/adapters/src/lib.rs
pub mod inbound {
    #[cfg(feature = "http")]
    pub mod http;

    pub mod cli;
}

pub mod outbound {
    #[cfg(feature = "postgres")]
    pub mod postgres;

    pub mod in_memory;
}
```

## Common Issues

### Circular Dependencies

If you get circular dependency errors:
1. Move shared types to domain
2. Use traits (ports) to break cycles
3. Consider a separate `common` crate for utilities

### Feature Propagation

Ensure features propagate correctly:

```toml
# In app/Cargo.toml
adapters = { workspace = true, features = ["http", "postgres"] }
```

### Test Organization

- Unit tests: Stay in each crate's `src/` files
- Integration tests: In `crates/app/tests/` or workspace root `tests/`

## Rollback Plan

If the migration causes issues, you can always:

1. Keep the workspace Cargo.toml
2. Move all code back to a single `src/` directory
3. Update the workspace to have a single member

The beauty of Cargo workspaces is they're just a way to organize code - the actual Rust is the same.

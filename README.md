# {{project-name}}

{{description}}

A Rust project template implementing **Hexagonal Architecture** (Ports & Adapters).

## Quick Start

### Using cargo-generate (Recommended)

```bash
cargo generate --git https://github.com/YOUR_USERNAME/rust-template
```

### Using GitHub Template

Click "Use this template" on GitHub, then:

```bash
git clone https://github.com/YOUR_USERNAME/your-new-project
cd your-new-project
# Find and replace {{project-name}} with your actual project name
```

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── lib.rs                  # Library root
│
├── domain/                 # Core business logic (no external deps)
│   ├── entities/           # Business objects and value objects
│   ├── services/           # Domain services with business rules
│   ├── ports/              # Interface definitions (traits)
│   └── errors.rs           # Domain-specific errors
│
├── adapters/               # Infrastructure implementations
│   ├── inbound/            # Driving adapters (HTTP, CLI, etc.)
│   └── outbound/           # Driven adapters (DB, external APIs)
│
└── config/                 # Configuration management

examples/
├── web-api/                # Complete REST API example (axum + SQLx)
└── cli-tool/               # Complete CLI example (clap + file storage)

tests/
└── api/                    # Integration tests
```

## Architecture

This template follows **Hexagonal Architecture** (also known as Ports & Adapters):

```
┌─────────────────────────────────────────────────────────────┐
│                      Inbound Adapters                       │
│                    (HTTP, CLI, gRPC...)                     │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                         Domain                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Entities   │  │  Services   │  │   Ports (Traits)    │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                     Outbound Adapters                       │
│               (Database, External APIs, Cache...)           │
└─────────────────────────────────────────────────────────────┘
```

### Key Principles

1. **Domain is pure** - No external dependencies in domain layer
2. **Dependencies point inward** - Adapters depend on domain, never reverse
3. **Ports define boundaries** - Traits abstract external concerns
4. **Easily testable** - Mock ports for isolated unit tests

See [docs/architecture.md](docs/architecture.md) for detailed explanation.

## Development

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- [just](https://github.com/casey/just) (optional, for task running)

### Commands

```bash
# Show all available commands
just

# Run the application
just run

# Run all quality checks
just check

# Run tests
just test

# Format code
just fmt

# Run clippy lints
just lint
```

### Running Examples

```bash
# Web API example
just web-api
# Then: curl http://localhost:3000/health

# CLI tool example
just cli-tool create-user --email user@example.com --name "John Doe"
just cli-tool list-users
```

## Testing

```bash
# Run all tests
just test

# Run unit tests only
just test-unit

# Run integration tests only
just test-integration

# Run with coverage (requires cargo-llvm-cov)
just test-coverage
```

## Configuration

Configuration is loaded from (in order of precedence):

1. Environment variables (prefixed with `APP_`)
2. `config/local.toml` (not in git)
3. `config/{environment}.toml`
4. `config/default.toml`

Example environment variables:

```bash
APP_ENVIRONMENT=production
APP_SERVER_HOST=0.0.0.0
APP_SERVER_PORT=8080
APP_DATABASE_URL=postgres://user:pass@localhost/db
```

## Scaling Up

As your project grows, you may want to split into a workspace. See [docs/workspace-migration.md](docs/workspace-migration.md) for guidance.

## License

MIT

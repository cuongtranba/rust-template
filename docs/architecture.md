# Architecture Guide

This project implements **Hexagonal Architecture** (also known as Ports & Adapters), a software design pattern that promotes separation of concerns and testability.

## Overview

```
                    ┌─────────────────────────────────────┐
                    │          INBOUND ADAPTERS           │
                    │    (HTTP, CLI, gRPC, WebSocket)     │
                    └─────────────────┬───────────────────┘
                                      │
                                      │ calls
                                      ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                              DOMAIN                                     │
│  ┌──────────────┐    ┌──────────────┐    ┌───────────────────────────┐  │
│  │   Entities   │    │   Services   │    │    Ports (Traits)         │  │
│  │              │    │              │    │                           │  │
│  │  - User      │◄───│  - UserSvc   │───►│  - UserRepository         │  │
│  │  - Email     │    │              │    │  - EmailService           │  │
│  │  - UserId    │    │              │    │                           │  │
│  └──────────────┘    └──────────────┘    └───────────────────────────┘  │
│                                                     ▲                   │
└─────────────────────────────────────────────────────│───────────────────┘
                                                      │
                                                      │ implements
                                                      │
                    ┌─────────────────────────────────┴───────────────────┐
                    │                OUTBOUND ADAPTERS                    │
                    │      (PostgreSQL, Redis, SendGrid, Stripe)          │
                    └─────────────────────────────────────────────────────┘
```

## Core Concepts

### 1. Domain Layer

The domain layer is the heart of your application. It contains:

- **Entities**: Business objects with identity (e.g., `User`, `Order`)
- **Value Objects**: Immutable objects defined by attributes (e.g., `Email`, `Money`)
- **Domain Services**: Business logic that doesn't belong to a single entity
- **Ports**: Traits that define what the domain needs from the outside world

**Key Rule**: The domain has ZERO external dependencies. It only uses standard library and basic utilities.

```rust
// domain/entities/user.rs
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub name: String,
}

// domain/ports/repositories.rs
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}
```

### 2. Ports (Interfaces)

Ports are traits that define the boundaries between the domain and the outside world:

- **Inbound Ports**: How the outside world interacts with your domain (defined by service methods)
- **Outbound Ports**: What your domain needs from the outside world (repository traits, external service traits)

```rust
// Outbound port - domain defines what it needs
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
}

// Outbound port - external service abstraction
pub trait EmailService: Send + Sync {
    async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), DomainError>;
}
```

### 3. Adapters

Adapters implement the ports and handle all external concerns:

#### Inbound Adapters (Driving)

These translate external requests into domain operations:

- **HTTP Handlers**: REST API endpoints using axum/actix
- **CLI Commands**: Command-line interface using clap
- **gRPC Services**: RPC handlers using tonic
- **Message Consumers**: Queue consumers for async processing

```rust
// adapters/inbound/http/handlers.rs
pub async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = service.register(&req.email, &req.name).await?;
    Ok(Json(user.into()))
}
```

#### Outbound Adapters (Driven)

These implement what the domain needs:

- **Database Repositories**: PostgreSQL, SQLite, MongoDB
- **External API Clients**: Payment gateways, email providers
- **Cache Implementations**: Redis, in-memory
- **Message Publishers**: Kafka, RabbitMQ

```rust
// adapters/outbound/persistence/postgres.rs
pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        sqlx::query_as!(...)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Infrastructure(e.into()))
    }
}
```

## Dependency Flow

Dependencies ALWAYS point inward:

```
Adapters → Domain ← Adapters
         (center)
```

- Adapters depend on Domain (implement its ports)
- Domain NEVER depends on Adapters
- Domain defines interfaces, Adapters provide implementations

## Benefits

### 1. Testability

The domain is easily testable with mocks:

```rust
#[tokio::test]
async fn test_register_user() {
    let mut mock_repo = MockUserRepository::new();
    mock_repo.expect_save().returning(|_| Ok(()));

    let service = UserService::new(Arc::new(mock_repo));
    let result = service.register("test@example.com", "Test").await;

    assert!(result.is_ok());
}
```

### 2. Flexibility

Swap implementations without changing domain:

```rust
// Development
let repo = InMemoryUserRepository::new();

// Production
let repo = PostgresUserRepository::new(pool);

// Both implement UserRepository trait
```

### 3. Clear Boundaries

Each layer has clear responsibilities:
- Domain: Business logic
- Inbound Adapters: Translation from external to domain
- Outbound Adapters: Translation from domain to external

## Adding New Features

### Adding a New Entity

1. Create entity in `domain/entities/`
2. Add repository port in `domain/ports/repositories.rs`
3. Create domain service in `domain/services/`
4. Implement repository adapter in `adapters/outbound/persistence/`
5. Add HTTP handlers in `adapters/inbound/http/`

### Adding a New External Service

1. Define port trait in `domain/ports/services.rs`
2. Implement adapter in `adapters/outbound/external/`
3. Inject into domain services that need it

### Adding a New Inbound Adapter

1. Create new module in `adapters/inbound/`
2. Translate external format to domain calls
3. Handle errors appropriately for the protocol

## Error Handling

### Domain Errors

Use `thiserror` for typed, explicit errors:

```rust
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User not found: {0}")]
    UserNotFound(UserId),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
```

### Adapter Errors

Use `anyhow` for contextual errors:

```rust
// In adapters, wrap infrastructure errors
.map_err(|e| DomainError::Infrastructure(e.into()))?
```

### Error Mapping

Inbound adapters map domain errors to protocol-specific responses:

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self.0.downcast_ref::<DomainError>() {
            Some(DomainError::NotFound { .. }) => StatusCode::NOT_FOUND,
            Some(DomainError::ValidationError(_)) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
```

## Further Reading

- [Hexagonal Architecture by Alistair Cockburn](https://alistair.cockburn.us/hexagonal-architecture/)
- [Clean Architecture by Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Domain-Driven Design by Eric Evans](https://www.domainlanguage.com/ddd/)

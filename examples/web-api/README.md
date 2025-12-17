# Web API Example

A complete REST API example demonstrating hexagonal architecture with axum.

## Running

```bash
cd examples/web-api
cargo run
```

The server will start at `http://127.0.0.1:3000`.

## Endpoints

### Health Check

```bash
curl http://localhost:3000/health
```

### Create User

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "name": "John Doe"}'
```

### Get User

```bash
curl http://localhost:3000/users/{id}
```

### List Users

```bash
curl http://localhost:3000/users
```

### Delete User

```bash
curl -X DELETE http://localhost:3000/users/{id}
```

## Architecture

This example shows:

1. **Handlers** (`handlers.rs`) - HTTP request handlers that translate HTTP to domain operations
2. **Routes** (`routes.rs`) - Route definitions using axum
3. **Error Handling** (`error.rs`) - Mapping domain errors to HTTP responses
4. **App State** (`app_state.rs`) - Dependency injection using shared state

## Production Considerations

For production use:

1. Replace `InMemoryUserRepository` with `PostgresUserRepository`
2. Add proper configuration loading
3. Add authentication/authorization middleware
4. Add request validation
5. Add rate limiting
6. Add proper logging and metrics

# CLI Tool Example

A command-line interface demonstrating hexagonal architecture with clap.

## Running

```bash
cd examples/cli-tool
cargo run -- --help
```

## Commands

### Create User

```bash
cargo run -- create-user --email user@example.com --name "John Doe"
```

### Get User

```bash
cargo run -- get-user --id <uuid>
```

### List Users

```bash
cargo run -- list-users
```

### Delete User

```bash
cargo run -- delete-user --id <uuid>
```

## Architecture

This example shows:

1. **CLI Definition** (`cli.rs`) - Command-line argument parsing with clap
2. **Domain Types** (`types.rs`) - Entities and value objects
3. **Repository** (`repository.rs`) - File-based storage implementing the repository port

## Storage

User data is stored in `users.json` in the current directory. This is a simple JSON file that persists between runs.

## Production Considerations

For production use:

1. Replace file-based storage with a proper database
2. Add proper error messages and exit codes
3. Add configuration file support
4. Add logging
5. Consider using a TUI library for interactive mode

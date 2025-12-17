# Rust Template - Task Runner
# https://github.com/casey/just

# Default task - show available commands
default:
    @just --list

# =============================================================================
# Development
# =============================================================================

# Run the application
run *ARGS:
    cargo run -- {{ARGS}}

# Run with auto-reload on changes (requires cargo-watch)
dev:
    cargo watch -x run

# Run a specific example
example NAME:
    cargo run --example {{NAME}}

# =============================================================================
# Quality Assurance
# =============================================================================

# Run all checks (format, lint, test)
check: fmt-check lint test
    @echo "All checks passed!"

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Format code
fmt:
    cargo fmt --all

# Run clippy lints
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Fix clippy warnings automatically
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty

# =============================================================================
# Testing
# =============================================================================

# Run all tests
test:
    cargo test --all-features

# Run unit tests only
test-unit:
    cargo test --lib

# Run integration tests only
test-integration:
    cargo test --test '*'

# Run tests with output
test-verbose:
    cargo test --all-features -- --nocapture

# Run tests and generate coverage (requires cargo-llvm-cov)
test-coverage:
    cargo llvm-cov --all-features --lcov --output-path lcov.info

# =============================================================================
# Building
# =============================================================================

# Build debug
build:
    cargo build

# Build release
build-release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean

# =============================================================================
# Documentation
# =============================================================================

# Generate and open documentation
docs:
    cargo doc --open --no-deps

# Generate documentation without opening
docs-build:
    cargo doc --no-deps

# =============================================================================
# Dependencies
# =============================================================================

# Check for outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# Update dependencies
update:
    cargo update

# Audit dependencies for security vulnerabilities (requires cargo-audit)
audit:
    cargo audit

# =============================================================================
# Examples
# =============================================================================

# Run web-api example
web-api:
    cd examples/web-api && cargo run

# Run cli-tool example
cli-tool *ARGS:
    cd examples/cli-tool && cargo run -- {{ARGS}}

# =============================================================================
# Setup
# =============================================================================

# Install development tools
setup:
    rustup component add rustfmt clippy
    cargo install cargo-watch cargo-outdated cargo-audit cargo-llvm-cov

# Initialize git hooks
hooks:
    @echo "Setting up git hooks..."
    @echo '#!/bin/sh\njust check' > .git/hooks/pre-commit
    @chmod +x .git/hooks/pre-commit
    @echo "Pre-commit hook installed!"

# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

### Building and Testing
```bash
# Build the entire workspace
cargo build

# Run tests for the SDK
cargo test

# Run a specific example
cd examples/messages/messages && cargo run

# Set API key for running examples
export ANTHROPIC_API_KEY="your-api-key"
```

### Development Workflow
```bash
# Check all workspace members
cargo check --workspace

# Format code
cargo fmt

# Run clippy for linting
cargo clippy

# Build documentation
cargo doc --open
```

## Architecture Overview

This is a Rust SDK for the Anthropic API organized as a Cargo workspace with the main SDK in `anthropic-ai-sdk/` and examples demonstrating usage patterns.

### Core Structure

- **`anthropic-ai-sdk/src/`**: Main SDK implementation
  - `client.rs`: Core AnthropicClient with authentication and HTTP handling
  - `admin_client.rs`: Administrative API functionality
  - `messages.rs`: Message creation and streaming
  - `message_batches.rs`: Batch operations
  - `models.rs`: Model listing and retrieval
  - `types/`: Type definitions organized by API domain

### Key Design Patterns

- **Builder Pattern**: Used extensively for request construction (e.g., `CreateMessageParams::new().with_temperature(0.7)`)
- **Trait-based Architecture**: Core functionality defined through traits (`MessageClient`, `ModelClient`, etc.)
- **Domain-specific Error Types**: Each module has its own error enum using `thiserror`
- **Type-state Pattern**: Ensures correct parameter usage through the type system

### Client Architecture

The `AnthropicClient` is the main entry point supporting:
- Multiple API versions
- Custom base URLs
- Custom HTTP clients via builder pattern
- Authentication header management

### Examples Organization

Examples are categorized by API domain:
- `examples/messages/`: Message creation, streaming, token counting
- `examples/models/`: Model listing and retrieval
- `examples/message-batches/`: Batch operations
- `examples/admin/`: Administrative API operations (API keys, users, workspaces, invites)

Each example is a standalone Cargo project demonstrating specific API usage patterns.

### Testing Strategy

- Unit tests for individual components
- Integration tests for API interactions
- Documentation tests for usage examples
- Examples serve as functional tests

### Dependencies

Core runtime dependencies:
- `tokio`: Async runtime with full features
- `reqwest`: HTTP client with JSON and streaming support
- `serde`: Serialization with derive features
- `async-trait`: For async trait definitions
- `futures-util`: Stream processing utilities

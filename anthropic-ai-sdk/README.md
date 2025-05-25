# Anthropic Rust SDK

[![Crates.io](https://img.shields.io/crates/v/anthropic-ai-sdk.svg)](https://crates.io/crates/anthropic-ai-sdk)
[![Documentation](https://docs.rs/anthropic-ai-sdk/badge.svg)](https://docs.rs/anthropic-ai-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An unofficial Rust SDK for the [Anthropic API](https://docs.anthropic.com/claude/reference/getting-started).

## Features

- Robust async/await implementation using Tokio
- Comprehensive error handling with detailed error types
- Built-in pagination support for list operations
- Token counting utilities for accurate message length estimation
- Type-safe API with full Rust type definitions
- Easy-to-use builder patterns for request construction

## Installation

```bash
cargo add anthropic-ai-sdk
```

## Quick Start

```rust
use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    CreateMessageParams, Message, MessageClient, MessageError, RequiredMessageParams, Role,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY").unwrap();
    let client = AnthropicClient::new::<MessageError>(anthropic_api_key, "2023-06-01").unwrap();

    // stream(false)
    let body = CreateMessageParams::new(RequiredMessageParams {
        model: "claude-3-7-sonnet-latest".to_string(),
        messages: vec![Message::new_text(Role::User, "Hello, Claude")],
        max_tokens: 1024,
    });

    match client.create_message(Some(&body)).await {
        Ok(message) => {
            println!("Successfully created message: {:?}", message.content);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // stream(true)
    let body = CreateMessageParams::new(RequiredMessageParams {
        model: "claude-3-7-sonnet-latest".to_string(),
        messages: vec![Message::new_text(Role::User, "Hello, Claude")],
        max_tokens: 1024,
    })
    .with_stream(true);

    match client.create_message_streaming(&body).await {
        Ok(mut stream) => {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => info!("Received event: {:?}", event),
                    Err(e) => error!("Stream error: {}", e),
                }
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }

    Ok(())
}
```

## Examples

Check out the [examples](https://github.com/e-bebe/anthropic-sdk-rs/tree/main/examples) directory for more usage examples:

- Models
  - [List Models](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/models/list-models/src/main.rs) - How to retrieve a list of available models
  - [Get a Model](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/models/get-a-model/src/main.rs) - How to get a model
- Messages
  - [Message](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/messages/messages/src/main.rs) - How to create a message
  - [Count Message Tokens](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/messages/count-message-tokens/src/main.rs) - How to count tokens in a message
- Message Batch
  - [Create a Message Batch](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/message-batches/create-a-message-batch/src/main.rs) - How to create a message batch
- Admin Invites
  - [Get Invite](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/admin/organization-invites/get-invite/src/main.rs) - How to retrieve an organization invite
  - [List Invites](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/admin/organization-invites/list-invites/src/main.rs) - How to list organization invites
  - [Create Invite](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/admin/organization-invites/create-invite/src/main.rs) - How to create an organization invite
  - [Delete Invite](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/admin/organization-invites/delete-invite/src/main.rs) - How to delete an organization invite

> **Note:** The examples listed above are only a subset. For additional detailed usage examples, please refer to the [examples directory](https://github.com/e-bebe/anthropic-sdk-rs/tree/main/examples).

## API Coverage

- Models
  - [x] List Models
  - [x] Get a Model
- Messages
  - [x] Messages
  - [x] Count Message Tokens
- Message Batches
  - [x] Create a Message Batch
  - [x] Retrieve a Message Batch
  - [x] Retrieve Message Batch Results
  - [x] List Message Batches
  - [x] Cancel a Message Batch
  - [x] Delete a Message Batch
- Admin API
  - Organization Member Management
    - [x] Get User
    - [x] List Users
    - [x] Update User
    - [x] Remove User
  - Organization Invites
    - [x] Get Invite
    - [x] List Invites
    - [x] Create Invite
    - [x] Delete Invite
  - Workspace Management
    - [x] Get Workspace
    - [x] List Workspaces
    - [x] Update Workspace
    - [x] Create Workspace
    - [ ] Archive Workspace
  - Workspace Member Management
    - [x] Get Workspace Member
    - [x] List Workspace Members
    - [x] Add Workspace Member
    - [x] Update Workspace Member
    - [x] Delete Workspace Member
  - API Keys
    - [x] Get API Key
    - [x] List API Keys
    - [x] Update API Keys

## Development

### Prerequisites

- Rust 1.85.0 or later
- An Anthropic API key

### Running Tests

```bash
cargo test
```

### Running Examples

Set your API key

```bash
export ANTHROPIC_API_KEY="your-api-key"
```

Run an example

```bash
cd examples/models/list-models
cargo run 
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Anthropic API Documentation](https://docs.anthropic.com/claude/reference/getting-started)

## Security

If you discover a security vulnerability within this package, please send an e-mail to the maintainers. All security vulnerabilities will be promptly addressed.

## Support

For support questions, please use the [GitHub Issues](https://github.com/e-bebe/anthropic-sdk-rs/issues).

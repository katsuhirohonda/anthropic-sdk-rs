# Anthropic SDK for Rust

A Rust SDK for interacting with Anthropic's AI services.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anthropic-sdk = { git = "https://github.com/e-bebe/anthropic-sdk-rs" }
```

## Basic Usage

Here's a simple example of how to use the SDK:

```rust
use anthropic_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let client = Client::new("your-api-key")?;

    // Example of making a completion request
    let response = client.complete()
        .prompt("Hello, tell me a short story.")
        .max_tokens(100)
        .send()
        .await?;

    println!("{}", response.completion);

    Ok(())
}
```

## Features

- Simple and intuitive API
- Async support
- Comprehensive error handling

## License

MIT License

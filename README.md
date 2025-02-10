# Anthropic SDK for Rust

A Rust SDK for interacting with Anthropic's AI services.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anthropic-sdk = { git = "https://github.com/e-bebe/anthropic-sdk-rs" }
```

## Usage Example

```rust
use anthropic_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let client = Client::new("your-api-key")?;

    // Create a chat completion request
    let response = client.complete_chat()
        .model("claude-v1")
        .prompt("Hello, how are you?")
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
- Full type safety

## License

MIT License
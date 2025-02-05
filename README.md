# Anthropic SDK for Rust

A Rust SDK for interacting with Anthropic's API.

## Features

- Client setup
- Message handling
- Model interactions
- Message batch operations

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
anthropic-ai-sdk = { git = "https://github.com/e-bebe/anthropic-sdk-rs" }
```

## Sample Usage

### Creating a Client

```rust
use anthropic_ai_sdk::Client;

async fn example() {
    let client = Client::new("your-api-key");
    // Perform API operations
}
```

## Testing

Run tests using:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

See the LICENSE file for details.
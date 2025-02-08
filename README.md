# Anthropic SDK for Rust

## Sample Usage

Here's a simple example of how to use the Anthropic SDK:

```rust
use anthropic_sdk::Client;

async fn example() {
    let client = Client::new("your_api_key");
    let response = client.complete("Hello, how are you?").await;
    println!("{}", response);
}
```

Feel free to explore more features of the SDK in the documentation.
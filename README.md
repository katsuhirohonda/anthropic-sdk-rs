# Anthropic SDK for Rust

## Sample Usage

Here's a simple example of how to use the Anthropic SDK:

```rust
use anthropic_sdk::Client;

async fn example() {
    let client = Client::new("your_api_key");
    
    // Create a completion
    let response = client.complete()
        .model("claude-2")
        .prompt("Hello, Claude!")
        .max_tokens_to_sample(300)
        .send()
        .await?;
    
    println!("{}", response.completion);
    
    // Handle chat messages
    let chat_response = client.messages()
        .model("claude-2")
        .system("You are a helpful assistant")
        .user("Tell me a joke")
        .send()
        .await?;
    
    println!("{}", chat_response.content[0].text);
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anthropic-sdk = "0.1.0"
```

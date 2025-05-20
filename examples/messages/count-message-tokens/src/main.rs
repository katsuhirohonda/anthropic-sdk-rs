use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    CountMessageTokensParams, Message, MessageClient, MessageError, Role,
};
use std::env;
use tracing::{error, info};

/// cd anthropic-sdk-rs/examples/messages
/// cargo run

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new::<MessageError>(api_key, api_version).unwrap();

    let body = CountMessageTokensParams {
        model: "claude-3-7-sonnet-latest".to_string(),
        messages: vec![Message::new_text(Role::User, "Hello, Claude")],
    };

    info!("body: {:?}", body);

    match client.count_tokens(Some(&body)).await {
        Ok(message) => {
            info!("Successfully created message: {:?}", message.input_tokens);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}

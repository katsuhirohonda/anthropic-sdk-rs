use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::message::{CreateMessageParams, Message, MessageClient, MessageError, RequiredMessageParams, Role, Thinking, ThinkingType};
use futures_util::StreamExt;
use std::env;
use tracing::{error, info};

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

    let body = CreateMessageParams::new(RequiredMessageParams {
        model: "claude-3-7-sonnet-latest".to_string(),
        messages: vec![Message::new_text(Role::User, "Hello, Claude")],
        max_tokens: 2048,
    })
    .with_stream(true)
    .with_thinking(Thinking {
        budget_tokens: 1024,
        type_: ThinkingType::Enabled
    });

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
}

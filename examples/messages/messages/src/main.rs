use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    CreateMessageParams, Message, MessageClient, MessageError, RequiredMessageParams, Role,
};
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
        max_tokens: 1024,
    });

    // Or with some optional parameters
    // let params_with_options = CreateMessageParams::new(RequiredMessageParams {
    //     model: "claude-3-7-sonnet-latest".to_string(),
    //     messages: vec![Message::new_text(Role::User, "Hello, Claude")],
    //     max_tokens: 1024,
    // })
    // .with_temperature(0.7)
    // .with_system("You are a helpful assistant");

    info!("body: {:?}", body);

    match client.create_message(Some(&body)).await {
        Ok(message) => {
            info!("Successfully created message: {:?}", message.content);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}

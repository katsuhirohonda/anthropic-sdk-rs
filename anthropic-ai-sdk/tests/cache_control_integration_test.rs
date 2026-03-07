//! Integration test for prompt caching via cache_control.
//!
//! Requires the `ANTHROPIC_API_KEY` environment variable to be set.
//! Ignored by default to avoid burning tokens on every `cargo test` run.
//!
//! Run with:
//!   cargo test --test cache_control_integration_test -- --ignored --nocapture
//!
//! On success you will see a summary line like:
//!   "Tokens: 12 sent, 5702 cache write, 5702 cache read, 5 received."

use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    CacheControl, ContentBlock, CreateMessageParams, Message, MessageClient, MessageError,
    RequiredMessageParams, Role, SystemBlock, SystemPrompt,
};

/// A large block of text to use as a system prompt so it crosses the minimum
/// token threshold Anthropic requires before caching kicks in (~1024 tokens).
fn large_system_prompt() -> SystemPrompt {
    // ~6 tokens per sentence * 300 = ~1800 tokens, well above the 1024 minimum
    let text = "You are a helpful assistant who always tries to give accurate, detailed, and thoughtful responses. ".repeat(300);
    SystemPrompt::Blocks(vec![
        SystemBlock::new(text).with_cache_control(CacheControl::ephemeral()),
    ])
}

#[tokio::test]
#[ignore = "makes live API calls that cost tokens; run explicitly with -- --ignored"]
async fn test_cache_control_creates_and_reads_cache() {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY must be set to run this test");

    let client = AnthropicClient::new::<MessageError>(&api_key, "2023-06-01")
        .expect("Failed to create client");

    let build_params = || {
        CreateMessageParams::new(RequiredMessageParams {
            model: "claude-haiku-4-5-20251001".to_string(),
            max_tokens: 64,
            messages: vec![Message::new_blocks(
                Role::User,
                vec![ContentBlock::text("Reply with one word: ready")],
            )],
        })
        .with_system(large_system_prompt())
    };

    let params = build_params();
    let json = serde_json::to_string_pretty(&params).expect("Failed to serialize params");
    println!("Request JSON:\n{}", json);

    // First call — should write to cache
    let resp1 = client
        .create_message(Some(&params))
        .await
        .expect("First API call failed");

    println!(
        "Call 1 — sent: {}, cache_write: {}, cache_read: {}, received: {}",
        resp1.usage.input_tokens,
        resp1.usage.cache_creation_input_tokens,
        resp1.usage.cache_read_input_tokens,
        resp1.usage.output_tokens,
    );

    assert!(
        resp1.usage.cache_creation_input_tokens > 0 || resp1.usage.cache_read_input_tokens > 0,
        "Expected cache to be written or read on first call, but both were 0"
    );

    // Second call — should read from cache
    let resp2 = client
        .create_message(Some(&build_params()))
        .await
        .expect("Second API call failed");

    println!(
        "Call 2 — sent: {}, cache_write: {}, cache_read: {}, received: {}",
        resp2.usage.input_tokens,
        resp2.usage.cache_creation_input_tokens,
        resp2.usage.cache_read_input_tokens,
        resp2.usage.output_tokens,
    );

    assert!(
        resp2.usage.cache_read_input_tokens > 0,
        "Expected cache_read_input_tokens > 0 on second call, got {}",
        resp2.usage.cache_read_input_tokens
    );

    println!(
        "Tokens: {} sent, {} cache write, {} cache read, {} received.",
        resp1.usage.input_tokens,
        resp1.usage.cache_creation_input_tokens,
        resp2.usage.cache_read_input_tokens,
        resp2.usage.output_tokens,
    );
}

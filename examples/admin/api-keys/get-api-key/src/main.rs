use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), AdminError> {
    let admin_api_key = env::var("ANTHROPIC_ADMIN_KEY").expect("ANTHROPIC_ADMIN_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new_admin::<AdminError>(admin_api_key, api_version)?;

    // Get the API key ID from command line arguments
    let args: Vec<String> = env::args().collect();
    let api_key_id = args
        .get(1)
        .expect("Please provide an API key ID as argument");

    // Use the AdminClient trait explicitly to call get_api_key_by_id
    let api_key = AdminClient::get_api_key_by_id(&client, api_key_id).await?;
    println!("API Key Details:");
    println!("  ID: {}", api_key.id);
    println!("  Name: {}", api_key.name);
    println!("  Status: {:?}", api_key.status);
    println!("  Partial Key Hint: {}", api_key.partial_key_hint);
    println!("  Created At: {}", api_key.created_at);
    println!(
        "  Created By: {} (ID: {})",
        api_key.created_by.type_, api_key.created_by.id
    );
    if let Some(workspace_id) = api_key.workspace_id {
        println!("  Workspace ID: {}", workspace_id);
    } else {
        println!("  Workspace ID: default workspace");
    }

    Ok(())
}

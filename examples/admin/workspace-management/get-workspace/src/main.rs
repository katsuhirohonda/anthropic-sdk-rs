use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), AdminError> {
    let admin_api_key = env::var("ANTHROPIC_ADMIN_KEY").expect("ANTHROPIC_ADMIN_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new_admin::<AdminError>(admin_api_key, api_version)?;

    let args: Vec<String> = env::args().collect();
    let workspace_id = args
        .get(1)
        .expect("Please provide a workspace ID as argument");

    let workspace = AdminClient::get_workspace(&client, workspace_id).await?;
    println!("Workspace: {} ({})", workspace.name, workspace.id);
    println!("Display Color: {}", workspace.display_color);
    println!("Created At: {}", workspace.created_at);
    if let Some(archived_at) = workspace.archived_at {
        println!("Archived At: {}", archived_at);
    }

    Ok(())
}

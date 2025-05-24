use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use anthropic_ai_sdk::types::admin::workspaces::AdminUpdateWorkspaceParams;
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), AdminError> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let admin_api_key = env::var("ANTHROPIC_ADMIN_KEY").expect("ANTHROPIC_ADMIN_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new_admin::<AdminError>(admin_api_key, api_version)?;

    let args: Vec<String> = env::args().collect();
    let workspace_id = args.get(1).expect("Please provide a workspace ID as argument");
    let new_name = args.get(2).expect("Please provide the new workspace name");

    let params = AdminUpdateWorkspaceParams::new(new_name);

    match AdminClient::update_workspace(&client, workspace_id, &params).await {
        Ok(ws) => {
            info!("Successfully updated workspace!");
            info!("  ID: {}", ws.id);
            info!("  Name: {}", ws.name);
            info!("  Display Color: {}", ws.display_color);
            info!("  Created At: {}", ws.created_at);
            if let Some(archived_at) = ws.archived_at {
                info!("  Archived At: {}", archived_at);
            }
        }
        Err(e) => {
            error!("Error updating workspace: {}", e);
        }
    }

    Ok(())
}

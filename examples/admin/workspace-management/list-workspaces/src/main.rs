use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use anthropic_ai_sdk::types::admin::workspaces::ListWorkspacesParams;
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

    let params = ListWorkspacesParams::new().limit(20);

    match AdminClient::list_workspaces(&client, Some(&params)).await {
        Ok(resp) => {
            info!("Workspaces:");
            for ws in resp.data {
                info!("- {} ({})", ws.name, ws.id);
            }
        }
        Err(e) => {
            error!("Error listing workspaces: {}", e);
        }
    }

    Ok(())
}

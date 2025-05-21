use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use anthropic_ai_sdk::types::admin::workspace_members::ListWorkspaceMembersParams;
use std::env;
use tracing::{info, error};

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
    let workspace_id = args.get(1).expect("Provide workspace ID as argument");

    let params = ListWorkspaceMembersParams::new().limit(20);

    match AdminClient::list_workspace_members(&client, workspace_id, Some(&params)).await {
        Ok(resp) => {
            for member in resp.data {
                info!("{} -> {:?}", member.user_id, member.workspace_role);
            }
        }
        Err(e) => {
            error!("Error listing members: {}", e);
        }
    }

    Ok(())
}

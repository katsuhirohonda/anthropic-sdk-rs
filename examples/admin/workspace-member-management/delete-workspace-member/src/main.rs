use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
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
    let workspace_id = args.get(1).expect("Provide workspace ID as first argument");
    let user_id = args.get(2).expect("Provide user ID as second argument");

    match AdminClient::delete_workspace_member(&client, workspace_id, user_id).await {
        Ok(resp) => {
            info!("Deleted workspace member: {} - {}", resp.obj_type, resp.user_id);
        }
        Err(e) => {
            error!("Error deleting workspace member: {}", e);
        }
    }

    Ok(())
}

use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use anthropic_ai_sdk::types::admin::workspace_members::{
    AdminUpdateWorkspaceMemberParams, WorkspaceRole,
};
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
    let role_arg = args
        .get(3)
        .expect("Provide workspace role: user, developer, admin or billing");

    let role = match role_arg.as_str() {
        "user" => WorkspaceRole::WorkspaceUser,
        "developer" => WorkspaceRole::WorkspaceDeveloper,
        "admin" => WorkspaceRole::WorkspaceAdmin,
        "billing" => WorkspaceRole::WorkspaceBilling,
        _ => {
            error!("Invalid role. Valid options: user, developer, admin, billing");
            return Ok(());
        }
    };

    let params = AdminUpdateWorkspaceMemberParams::new(role);

    match AdminClient::update_workspace_member(&client, workspace_id, user_id, &params).await {
        Ok(member) => {
            info!(
                "Updated member {} -> {:?}",
                member.user_id, member.workspace_role
            );
        }
        Err(e) => {
            error!("Error updating workspace member: {}", e);
        }
    }

    Ok(())
}

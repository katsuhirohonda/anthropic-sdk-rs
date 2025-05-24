use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use anthropic_ai_sdk::types::admin::invites::CreateInviteParams;
use anthropic_ai_sdk::types::admin::users::UserRole;
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
    let email = args.get(1).expect("Please provide an email as argument");
    let role_arg = args.get(2).expect("Please provide a role: user, developer, or billing");

    let role = match role_arg.as_str() {
        "user" => UserRole::User,
        "developer" => UserRole::Developer,
        "billing" => UserRole::Billing,
        _ => {
            error!("Invalid role. Valid options: user, developer, billing");
            return Ok(());
        }
    };

    let params = CreateInviteParams::new(email, role);

    match AdminClient::create_invite(&client, &params).await {
        Ok(invite) => {
            info!("Successfully created invite: {} -> {}", invite.id, invite.email);
        }
        Err(e) => {
            error!("Error creating invite: {}", e);
        }
    }

    Ok(())
}

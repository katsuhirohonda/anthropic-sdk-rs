use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
use std::env;

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

    // Get the user ID from command line arguments
    let args: Vec<String> = env::args().collect();
    let user_id = args.get(1).expect("Please provide a user ID as argument");

    let user = AdminClient::get_user(&client, user_id).await?;

    println!("User Details:");
    println!("  ID: {}", user.id);
    println!("  Name: {}", user.name);
    println!("  Email: {}", user.email);
    println!("  Role: {:?}", user.role);
    println!("  Added At: {}", user.added_at);

    Ok(())
}

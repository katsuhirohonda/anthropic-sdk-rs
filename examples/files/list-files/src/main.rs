use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::files::FileClient;
use anthropic_ai_sdk::types::files::{FileError, ListFilesParams};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new::<FileError>(api_key, api_version).unwrap();

    info!("Listing files with default parameters...");

    // List files with default parameters
    match client.list_files(None).await {
        Ok(response) => {
            info!("Successfully retrieved files:");
            info!("Total files: {}", response.data.len());
            info!("Has more: {}", response.has_more);

            for file in &response.data {
                info!("  - {} ({})", file.filename, file.id);
                info!("    Size: {} bytes", file.size_bytes);
                info!("    MIME type: {}", file.mime_type);
                info!("    Created: {}", file.created_at);
                info!("    Downloadable: {}", file.downloadable);
            }

            if response.has_more {
                info!("More files available. Use pagination to retrieve them.");

                // Example of pagination
                if let Some(last_id) = response.last_id {
                    info!("\nFetching next page...");

                    let params = ListFilesParams::new().after_id(&last_id).limit(10);

                    match client.list_files(Some(&params)).await {
                        Ok(next_page) => {
                            info!("Next page contains {} files", next_page.data.len());
                            for file in &next_page.data {
                                info!("  - {} ({})", file.filename, file.id);
                            }
                        }
                        Err(e) => {
                            error!("Error fetching next page: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("Error listing files: {}", e);
        }
    }

    // Example with custom parameters
    info!("\nListing files with custom limit...");

    let params = ListFilesParams::new().limit(5);

    match client.list_files(Some(&params)).await {
        Ok(response) => {
            info!("Retrieved {} files (limited to 5)", response.data.len());
            for file in &response.data {
                info!("  - {}", file.filename);
            }
        }
        Err(e) => {
            error!("Error listing files with limit: {}", e);
        }
    }
}

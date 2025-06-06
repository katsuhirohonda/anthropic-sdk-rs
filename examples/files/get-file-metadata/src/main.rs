use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::files::FileClient;
use anthropic_ai_sdk::types::files::FileError;
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

    // Get the file ID from command line argument or use a default
    let file_id = env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: {} [FILE_ID]", env::args().next().unwrap());
            eprintln!("No file ID provided. Using example ID.");
            "file_example123".to_string()
        });

    info!("Getting metadata for file: {}", file_id);
    
    match client.get_file_metadata(&file_id).await {
        Ok(file) => {
            info!("Successfully retrieved file metadata:");
            info!("  File ID: {}", file.id);
            info!("  Filename: {}", file.filename);
            info!("  Size: {} bytes", file.size_bytes);
            info!("  MIME Type: {}", file.mime_type);
            info!("  Created: {}", file.created_at);
            info!("  Downloadable: {}", file.downloadable);
            info!("  Type: {}", file.file_type);
            
            // Calculate human-readable file size
            let size_mb = file.size_bytes as f64 / (1024.0 * 1024.0);
            if size_mb >= 1.0 {
                info!("  Size (MB): {:.2} MB", size_mb);
            } else {
                let size_kb = file.size_bytes as f64 / 1024.0;
                info!("  Size (KB): {:.2} KB", size_kb);
            }
        }
        Err(e) => {
            error!("Error retrieving file metadata: {}", e);
            
            // Provide helpful error messages
            match e {
                FileError::ApiError(ref msg) => {
                    if msg.contains("not found") {
                        error!("The file with ID '{}' was not found.", file_id);
                        error!("Please ensure the file ID is correct and the file exists.");
                    } else if msg.contains("unauthorized") || msg.contains("forbidden") {
                        error!("Access denied. Please check your API key permissions.");
                    }
                }
                _ => {}
            }
        }
    }
}
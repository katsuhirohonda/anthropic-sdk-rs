use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::files::FileClient;
use anthropic_ai_sdk::types::files::FileError;
use std::env;
use tracing::{error, info, warn};

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

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <FILE_ID>", args[0]);
        eprintln!("Example: {} file_abc123", args[0]);
        std::process::exit(1);
    }

    let file_id = &args[1];
    
    info!("Attempting to delete file: {}", file_id);
    
    // First, try to get file metadata to verify it exists
    match client.get_file_metadata(file_id).await {
        Ok(metadata) => {
            info!("File found:");
            info!("  Filename: {}", metadata.filename);
            info!("  Size: {} bytes", metadata.size_bytes);
            info!("  MIME Type: {}", metadata.mime_type);
            info!("  Created at: {}", metadata.created_at);
            
            // Warn if the file is downloadable
            if metadata.downloadable {
                warn!("This file is downloadable. Deletion will be permanent!");
            }
        }
        Err(e) => {
            error!("Failed to get file metadata: {}", e);
            info!("The file may not exist or you may not have access to it.");
            info!("Proceeding with deletion attempt anyway...");
        }
    }
    
    // Add a confirmation prompt in production environments
    if env::var("SKIP_CONFIRMATION").is_err() {
        println!("\nAre you sure you want to delete this file? This action cannot be undone.");
        println!("Type 'yes' to confirm: ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        if input.trim() != "yes" {
            info!("Deletion cancelled by user");
            std::process::exit(0);
        }
    }
    
    info!("Deleting file...");
    
    match client.delete_file(file_id).await {
        Ok(deleted_file) => {
            info!("File deleted successfully!");
            info!("  Deleted file ID: {}", deleted_file.id);
            info!("  Type: {}", deleted_file.deleted_type);
            
            println!("\nFile '{}' has been permanently deleted.", deleted_file.id);
        }
        Err(e) => {
            error!("Failed to delete file: {}", e);
            
            // Provide helpful error messages
            match e {
                FileError::ApiError(ref msg) => {
                    if msg.contains("not found") {
                        error!("The file with ID '{}' was not found.", file_id);
                        error!("It may have already been deleted or never existed.");
                    } else if msg.contains("unauthorized") || msg.contains("forbidden") {
                        error!("Access denied. Please check your API key permissions.");
                    } else if msg.contains("in use") || msg.contains("referenced") {
                        error!("The file may be in use by another resource and cannot be deleted.");
                    }
                }
                _ => {}
            }
            std::process::exit(1);
        }
    }
}
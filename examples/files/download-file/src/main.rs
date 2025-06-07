use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::files::FileClient;
use anthropic_ai_sdk::types::files::FileError;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <FILE_ID> [OUTPUT_PATH]", args[0]);
        eprintln!("Example: {} file_abc123 downloaded_file.pdf", args[0]);
        std::process::exit(1);
    }

    let file_id = &args[1];
    
    info!("Fetching metadata for file: {}", file_id);
    
    // First, get file metadata to know the filename
    let metadata = match client.get_file_metadata(file_id).await {
        Ok(metadata) => {
            info!("File metadata retrieved:");
            info!("  Filename: {}", metadata.filename);
            info!("  Size: {} bytes", metadata.size_bytes);
            info!("  MIME Type: {}", metadata.mime_type);
            info!("  Downloadable: {}", metadata.downloadable);
            
            if !metadata.downloadable {
                error!("File is not downloadable!");
                std::process::exit(1);
            }
            
            metadata
        }
        Err(e) => {
            error!("Failed to get file metadata: {}", e);
            std::process::exit(1);
        }
    };
    
    // Determine output filename
    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        // Use the original filename from metadata
        metadata.filename.clone()
    };
    
    info!("Downloading file content...");
    
    match client.download_file(file_id).await {
        Ok(file_content) => {
            info!("Successfully downloaded {} bytes", file_content.len());
            
            // Save to file
            let path = Path::new(&output_path);
            
            // Create parent directories if they don't exist
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).expect("Failed to create directories");
                }
            }
            
            match File::create(&path) {
                Ok(mut file) => {
                    match file.write_all(&file_content) {
                        Ok(_) => {
                            info!("File saved successfully to: {}", output_path);
                            
                            // Verify file size
                            if let Ok(saved_metadata) = std::fs::metadata(&path) {
                                info!("Saved file size: {} bytes", saved_metadata.len());
                                
                                if saved_metadata.len() as u64 != metadata.size_bytes {
                                    error!(
                                        "Warning: Saved file size ({}) doesn't match expected size ({})",
                                        saved_metadata.len(),
                                        metadata.size_bytes
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to write file content: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to create file '{}': {}", output_path, e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            error!("Failed to download file: {}", e);
            
            // Provide helpful error messages
            match e {
                FileError::ApiError(ref msg) => {
                    if msg.contains("not found") {
                        error!("The file with ID '{}' was not found.", file_id);
                    } else if msg.contains("unauthorized") || msg.contains("forbidden") {
                        error!("Access denied. Please check your API key permissions.");
                    }
                }
                _ => {}
            }
            std::process::exit(1);
        }
    }
}
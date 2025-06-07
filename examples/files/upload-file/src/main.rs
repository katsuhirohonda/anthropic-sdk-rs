use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::files::FileClient;
use anthropic_ai_sdk::types::files::FileError;
use std::env;
use std::fs;
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
        eprintln!("Usage: {} <FILE_PATH>", args[0]);
        eprintln!("Example: {} document.pdf", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let path = Path::new(file_path);
    
    // Check if file exists
    if !path.exists() {
        error!("File not found: {}", file_path);
        std::process::exit(1);
    }
    
    // Get the filename from the path
    let file_name = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    
    info!("Reading file: {}", file_path);
    
    // Read the file
    let file_content = match fs::read(path) {
        Ok(content) => {
            info!("File size: {} bytes", content.len());
            content
        }
        Err(e) => {
            error!("Failed to read file: {}", e);
            std::process::exit(1);
        }
    };
    
    info!("Uploading file '{}' to Anthropic...", file_name);
    
    match client.upload_file(file_name, file_content).await {
        Ok(uploaded_file) => {
            info!("File uploaded successfully!");
            info!("  File ID: {}", uploaded_file.id);
            info!("  Filename: {}", uploaded_file.filename);
            info!("  Size: {} bytes", uploaded_file.size_bytes);
            info!("  MIME Type: {}", uploaded_file.mime_type);
            info!("  Created at: {}", uploaded_file.created_at);
            info!("  Downloadable: {}", uploaded_file.downloadable);
            
            println!("\nFile ID for future use: {}", uploaded_file.id);
        }
        Err(e) => {
            error!("Failed to upload file: {}", e);
            
            // Provide helpful error messages
            match e {
                FileError::ApiError(ref msg) => {
                    if msg.contains("too large") || msg.contains("size") {
                        error!("The file may be too large. Please check Anthropic's file size limits.");
                    } else if msg.contains("unauthorized") || msg.contains("forbidden") {
                        error!("Access denied. Please check your API key permissions.");
                    } else if msg.contains("unsupported") || msg.contains("type") {
                        error!("The file type may not be supported. Please check Anthropic's supported file types.");
                    }
                }
                _ => {}
            }
            std::process::exit(1);
        }
    }
}
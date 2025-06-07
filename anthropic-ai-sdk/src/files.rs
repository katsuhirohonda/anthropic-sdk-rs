//! Files API for managing files in Anthropic
//!
//! This module provides functionality for listing files, retrieving file metadata, and downloading file content in the Anthropic API.
//! The Files API is currently in beta and requires the `anthropic-beta: files-api-2025-04-14` header.
//!
//! # Example
//!
//! ```no_run
//! use anthropic_ai_sdk::client::AnthropicClient;
//! use anthropic_ai_sdk::files::FileClient;
//! use anthropic_ai_sdk::types::files::ListFilesParams;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AnthropicClient::new::<anthropic_ai_sdk::types::files::FileError>(
//!         "api-key".to_string(),
//!         "2023-06-01".to_string()
//!     )?;
//!
//!     // List files with default parameters
//!     let files = client.list_files(None).await?;
//!     
//!     for file in files.data {
//!         println!("File: {} ({})", file.filename, file.id);
//!     }
//!
//!     // List files with pagination
//!     let params = ListFilesParams::new()
//!         .limit(50)
//!         .after_id("file_abc123");
//!     
//!     let files = client.list_files(Some(&params)).await?;
//!
//!     Ok(())
//! }
//! ```

use crate::client::AnthropicClient;
use crate::types::files::{FileError, ListFilesParams, ListFilesResponse};
use async_trait::async_trait;

/// Trait for file-related operations in the Anthropic API
///
/// This trait provides methods for managing files.
/// All methods require appropriate authentication.
#[async_trait]
pub trait FileClient {
    /// List files
    ///
    /// Returns a paginated list of files.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use anthropic_ai_sdk::client::AnthropicClient;
    /// # use anthropic_ai_sdk::files::FileClient;
    /// # use anthropic_ai_sdk::types::files::ListFilesParams;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<anthropic_ai_sdk::types::files::FileError>(
    ///     "api-key".to_string(),
    ///     "2023-06-01".to_string()
    /// )?;
    ///
    /// // List all files
    /// let files = client.list_files(None).await?;
    ///
    /// // List with pagination
    /// let params = ListFilesParams::new()
    ///     .limit(20)
    ///     .after_id("file_123");
    /// let files = client.list_files(Some(&params)).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn list_files<'a>(
        &'a self,
        params: Option<&'a ListFilesParams>,
    ) -> Result<ListFilesResponse, FileError>;

    /// Get file metadata
    ///
    /// Retrieves metadata for a specific file by its ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use anthropic_ai_sdk::client::AnthropicClient;
    /// # use anthropic_ai_sdk::files::FileClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<anthropic_ai_sdk::types::files::FileError>(
    ///     "api-key".to_string(),
    ///     "2023-06-01".to_string()
    /// )?;
    ///
    /// // Get metadata for a specific file
    /// let file = client.get_file_metadata("file_abc123").await?;
    /// println!("File: {} ({} bytes)", file.filename, file.size_bytes);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_file_metadata<'a>(&'a self, file_id: &'a str) -> Result<crate::types::files::File, FileError>;

    /// Download file content
    ///
    /// Downloads the raw content of a file by its ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file to download
    ///
    /// # Returns
    ///
    /// Returns the raw file content as bytes on success.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use anthropic_ai_sdk::client::AnthropicClient;
    /// # use anthropic_ai_sdk::files::FileClient;
    /// # use std::fs::File;
    /// # use std::io::Write;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<anthropic_ai_sdk::types::files::FileError>(
    ///     "api-key".to_string(),
    ///     "2023-06-01".to_string()
    /// )?;
    ///
    /// // Download file content
    /// let file_content = client.download_file("file_abc123").await?;
    /// 
    /// // Save to disk
    /// let mut file = File::create("downloaded_file.pdf")?;
    /// file.write_all(&file_content)?;
    /// # Ok(())
    /// # }
    /// ```
    async fn download_file<'a>(&'a self, file_id: &'a str) -> Result<Vec<u8>, FileError>;
}

#[async_trait]
impl FileClient for AnthropicClient {
    async fn list_files<'a>(
        &'a self,
        params: Option<&'a ListFilesParams>,
    ) -> Result<ListFilesResponse, FileError> {
        // Validate parameters if provided
        if let Some(params) = params {
            params.validate()?;
        }

        // Files API requires the beta header
        const FILES_BETA_HEADER: &str = "files-api-2025-04-14";

        self.get_with_beta("/files", params, FILES_BETA_HEADER)
            .await
    }

    async fn get_file_metadata<'a>(&'a self, file_id: &'a str) -> Result<crate::types::files::File, FileError> {
        // Files API requires the beta header
        const FILES_BETA_HEADER: &str = "files-api-2025-04-14";
        
        self.get_with_beta(
            &format!("/files/{}", file_id),
            Option::<&()>::None,
            FILES_BETA_HEADER,
        )
        .await
    }

    async fn download_file<'a>(&'a self, file_id: &'a str) -> Result<Vec<u8>, FileError> {
        // Files API requires the beta header
        const FILES_BETA_HEADER: &str = "files-api-2025-04-14";
        
        self.download_with_beta(
            &format!("/files/{}/content", file_id),
            FILES_BETA_HEADER,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_files_params_validation() {
        // Valid params
        let params = ListFilesParams::new().limit(100);
        assert!(params.validate().is_ok());

        // Clamped limit still passes validation (clamped to 1000)
        let params = ListFilesParams::new().limit(1001);
        assert!(params.validate().is_ok());
        assert_eq!(params.limit, Some(1000));

        // Zero limit gets clamped to 1 and passes validation
        let params = ListFilesParams::new().limit(0);
        assert!(params.validate().is_ok());
        assert_eq!(params.limit, Some(1));
    }
}

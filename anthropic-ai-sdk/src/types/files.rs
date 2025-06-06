//! Types for the Files API

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur when using the Files API
#[derive(Debug, Error)]
pub enum FileError {
    /// Invalid pagination parameters
    #[error("Invalid pagination parameters")]
    InvalidPagination,

    /// Invalid limit value
    #[error("Invalid limit value: {0}")]
    InvalidLimit(u16),

    /// API request failed
    #[error("API request failed: {0}")]
    RequestFailed(String),

    /// API returned an error
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for FileError {
    fn from(error: String) -> Self {
        FileError::ApiError(error)
    }
}

/// Parameters for listing files
#[derive(Debug, Serialize, Default)]
pub struct ListFilesParams {
    /// ID of the item to use as the starting point for pagination (exclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,

    /// ID of the item to use as the ending point for pagination (exclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,

    /// Number of items to return per page (1-1000, default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl ListFilesParams {
    /// Create a new ListFilesParams with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit for the number of items to return
    ///
    /// The limit will be clamped to the valid range (1-1000)
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(1, 1000));
        self
    }

    /// Set the before_id for pagination
    pub fn before_id(mut self, before_id: impl Into<String>) -> Self {
        self.before_id = Some(before_id.into());
        self
    }

    /// Set the after_id for pagination
    pub fn after_id(mut self, after_id: impl Into<String>) -> Self {
        self.after_id = Some(after_id.into());
        self
    }

    /// Validate the parameters
    pub fn validate(&self) -> Result<(), FileError> {
        // Validate limit
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err(FileError::InvalidLimit(limit));
            }
        }

        // Validate pagination parameters
        if self.before_id.is_some() && self.after_id.is_some() {
            return Err(FileError::InvalidPagination);
        }

        Ok(())
    }
}

/// File object representing a file in the Anthropic system
#[derive(Debug, Deserialize, Clone)]
pub struct File {
    /// RFC 3339 timestamp of when the file was created
    pub created_at: String,

    /// Whether the file can be downloaded
    pub downloadable: bool,

    /// Original filename
    pub filename: String,

    /// Unique file identifier
    pub id: String,

    /// MIME type of the file
    pub mime_type: String,

    /// File size in bytes
    pub size_bytes: u64,

    /// Object type (always "file")
    #[serde(rename = "type")]
    pub file_type: String,
}

/// Response from listing files
#[derive(Debug, Deserialize)]
pub struct ListFilesResponse {
    /// List of files
    pub data: Vec<File>,

    /// ID of the first item in the data list
    pub first_id: Option<String>,

    /// ID of the last item in the data list
    pub last_id: Option<String>,

    /// Whether there are more items available
    pub has_more: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_files_params_builder() {
        let params = ListFilesParams::new().limit(50).after_id("file_abc123");

        assert_eq!(params.limit, Some(50));
        assert_eq!(params.after_id, Some("file_abc123".to_string()));
        assert_eq!(params.before_id, None);
    }

    #[test]
    fn test_list_files_params_limit_clamping() {
        // Test upper bound clamping
        let params = ListFilesParams::new().limit(2000);
        assert_eq!(params.limit, Some(1000));

        // Test lower bound clamping (0 becomes 1)
        let params = ListFilesParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }

    #[test]
    fn test_list_files_params_validation() {
        // Valid params
        let params = ListFilesParams::new().limit(100);
        assert!(params.validate().is_ok());

        // Both before_id and after_id set should fail
        let params = ListFilesParams::new()
            .before_id("file_1")
            .after_id("file_2");
        assert!(matches!(
            params.validate(),
            Err(FileError::InvalidPagination)
        ));
    }

    #[test]
    fn test_file_error_from_string() {
        let error = FileError::from("Test error".to_string());
        assert!(matches!(error, FileError::ApiError(_)));
    }
}

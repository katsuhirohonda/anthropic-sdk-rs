//! Admin API
//!
//! This module contains the types and functions for the Anthropic Admin API.
//!
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;
use time::serde::rfc3339;

/// Error types for the Admin API
#[derive(Debug, Error)]
pub enum AdminError {
    #[error("Invalid pagination parameters")]
    InvalidPagination,
    #[error("Invalid limit value: {0}")]
    InvalidLimit(u16),
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for AdminError {
    fn from(error: String) -> Self {
        AdminError::ApiError(error)
    }
}

#[async_trait]
pub trait AdminClient {
    async fn list_api_keys<'a>(
        &'a self,
        params: Option<&'a ListApiKeysParams>,
    ) -> Result<ListApiKeysResponse, AdminError>;
}

/// Parameters for listing API keys
#[derive(Debug, Serialize, Default)]
pub struct ListApiKeysParams {
    /// Cursor for pagination (before)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
    /// Cursor for pagination (after)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
    /// Number of items per page (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Filter by API key status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ApiKeyStatus>,
    /// Filter by Workspace ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// Filter by the ID of the User who created the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
}

impl ListApiKeysParams {
    /// Create a new ListApiKeysParams with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the before_id parameter
    pub fn before_id(mut self, before_id: impl Into<String>) -> Self {
        self.before_id = Some(before_id.into());
        self
    }

    /// Set the after_id parameter
    pub fn after_id(mut self, after_id: impl Into<String>) -> Self {
        self.after_id = Some(after_id.into());
        self
    }

    /// Set the limit parameter (1-1000)
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(1, 1000));
        self
    }

    /// Set the status parameter
    pub fn status(mut self, status: ApiKeyStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the workspace_id parameter
    pub fn workspace_id(mut self, workspace_id: impl Into<String>) -> Self {
        self.workspace_id = Some(workspace_id.into());
        self
    }

    /// Set the created_by_user_id parameter
    pub fn created_by_user_id(mut self, created_by_user_id: impl Into<String>) -> Self {
        self.created_by_user_id = Some(created_by_user_id.into());
        self
    }
}

/// API key status
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyStatus {
    Active,
    Inactive,
    Archived,
}

/// Response structure for listing API keys
#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    /// List of API keys
    pub data: Vec<ApiKey>,
    /// First ID in the data list
    pub first_id: Option<String>,
    /// Indicates if there are more results
    pub has_more: bool,
    /// Last ID in the data list
    pub last_id: Option<String>,
}

/// User information
#[derive(Debug, Deserialize)]
pub struct User {
    /// Unique identifier for the user
    pub id: String,
    /// Type of the resource (always "user")
    #[serde(rename = "type")]
    pub type_: String,
}

/// Represents an API key
#[derive(Debug, Deserialize)]
pub struct ApiKey {
    /// Unique identifier for the API key
    pub id: String,
    /// Type of the resource (always "api_key")
    #[serde(rename = "type")]
    pub type_: String,
    /// Status of the API key
    pub status: ApiKeyStatus,
    /// Name of the API key
    pub name: String,
    /// Creation timestamp
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    /// Information about the user who created the API key
    pub created_by: User,
    /// ID of the workspace this API key belongs to
    #[serde(default)]
    pub workspace_id: Option<String>,
    /// Partial key hint for display purposes
    pub partial_key_hint: String,
}

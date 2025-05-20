//! Models API
//!
//! This module contains the types and functions for the Anthropic Models API.
//!
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::serde::rfc3339;
use time::OffsetDateTime;

/// Error types for the Models API
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Invalid pagination parameters")]
    InvalidPagination,
    #[error("Invalid limit value: {0}")]
    InvalidLimit(u16),
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for ModelError {
    fn from(error: String) -> Self {
        ModelError::ApiError(error)
    }
}

#[async_trait]
pub trait ModelClient {
    async fn list_models<'a>(
        &'a self,
        params: Option<&'a ListModelsParams>,
    ) -> Result<ListModelsResponse, ModelError>;

    async fn get_model<'a>(&'a self, model_id: &'a str) -> Result<Model, ModelError>;
}

/// Response structure for the List Models API endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// List of model objects
    pub data: Vec<Model>,
    /// First ID in the data list
    pub first_id: Option<String>,
    /// Indicates if there are more results
    pub has_more: bool,
    /// Last ID in the data list
    pub last_id: Option<String>,
}

/// Represents an Anthropic model
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// Type of the resource (always "model")
    #[serde(rename = "type")]
    pub type_: String,

    /// Unique identifier for the model
    pub id: String,

    /// Display name of the model
    #[serde(rename = "display_name")]
    pub display_name: String,

    /// Creation timestamp of the model
    #[serde(rename = "created_at")]
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

/// Parameters for listing models
#[derive(Debug, Serialize, Default)]
pub struct ListModelsParams {
    /// Cursor for pagination (before)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
    /// Cursor for pagination (after)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
    /// Number of items per page (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl ListModelsParams {
    /// Create a new ListModelsParams with default values
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

    /// Set the limit parameter
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.clamp(1, 1000));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ListModelsParams;

    #[test]
    fn limit_clamps_to_upper_bound() {
        let params = ListModelsParams::new().limit(1500);
        assert_eq!(params.limit, Some(1000));
    }

    #[test]
    fn limit_clamps_to_lower_bound() {
        let params = ListModelsParams::new().limit(0);
        assert_eq!(params.limit, Some(1));
    }
}

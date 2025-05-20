//! Admin API
//!
//! This module contains the implementations for the Anthropic Admin API endpoints.
//! It provides functionality for managing API keys and other administrative tasks.

use crate::client::AnthropicClient;
use crate::types::admin::api_keys::{
    AdminClient, AdminError, AdminUpdateApiKeyParams, ApiKey, ListApiKeysParams,
    ListApiKeysResponse,
};
use async_trait::async_trait;

#[async_trait]
impl AdminClient for AnthropicClient {
    /// Lists API keys
    ///
    /// Retrieves a list of API keys with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a list of API keys and pagination information on success.
    ///
    /// # Errors
    ///
    /// Returns an `AdminError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AdminError> {
    ///     let client = AnthropicClient::new::<AdminError>(
    ///         "your-admin-api-key",
    ///         "2023-06-01",
    ///     )?;
    ///
    ///     // List all API keys
    ///     let api_keys = client.list_api_keys(None).await?;
    ///     for api_key in api_keys.data {
    ///         println!("API Key: {} ({})", api_key.name, api_key.id);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn list_api_keys<'a>(
        &'a self,
        params: Option<&'a ListApiKeysParams>,
    ) -> Result<ListApiKeysResponse, AdminError> {
        self.get("/organizations/api_keys", params).await
    }

    /// Gets a specific API key
    ///
    /// Retrieves details for a specific API key by its ID.
    ///
    /// # Arguments
    ///
    /// * `api_key_id` - The ID of the API key to retrieve
    ///
    /// # Returns
    ///
    /// Returns the API key details on success.
    ///
    /// # Errors
    ///
    /// Returns an `AdminError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    /// - The API key is not found
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AdminError> {
    ///     let client = AnthropicClient::new::<AdminError>(
    ///         "your-admin-api-key",
    ///         "2023-06-01",
    ///     )?;
    ///
    ///     // Get a specific API key
    ///     let api_key = client.get_api_key_by_id("api_key_xyz").await?;
    ///     println!("API Key: {} ({})", api_key.name, api_key.id);
    ///     println!("Status: {:?}", api_key.status);
    ///     println!("Partial Hint: {}", api_key.partial_key_hint);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_api_key_by_id<'a>(&'a self, api_key_id: &'a str) -> Result<ApiKey, AdminError> {
        self.get(
            &format!("/organizations/api_keys/{}", api_key_id),
            Option::<&()>::None,
        )
        .await
    }

    /// Updates an API key
    ///
    /// Updates properties of an API key by its ID.
    ///
    /// # Arguments
    ///
    /// * `api_key_id` - The ID of the API key to update
    /// * `params` - Parameters for updating the API key
    ///
    /// # Returns
    ///
    /// Returns the updated API key details on success.
    ///
    /// # Errors
    ///
    /// Returns an `AdminError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    /// - The API key is not found
    /// - Invalid parameters are provided
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::api_keys::{AdminClient, AdminError, ApiKeyStatus, AdminUpdateApiKeyParams};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AdminError> {
    ///     let client = AnthropicClient::new::<AdminError>(
    ///         "your-admin-api-key",
    ///         "2023-06-01",
    ///     )?;
    ///
    ///     // Update an API key
    ///     let params = AdminUpdateApiKeyParams::new()
    ///         .name("Updated API Key")
    ///         .status(ApiKeyStatus::Inactive);
    ///
    ///     let api_key = client.update_api_key("api_key_xyz", &params).await?;
    ///     println!("Updated API Key: {} ({})", api_key.name, api_key.id);
    ///     println!("New Status: {:?}", api_key.status);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn update_api_key<'a>(
        &'a self,
        api_key_id: &'a str,
        params: &'a AdminUpdateApiKeyParams,
    ) -> Result<ApiKey, AdminError> {
        self.post(
            &format!("/organizations/api_keys/{}", api_key_id),
            Some(params),
        )
        .await
    }
}

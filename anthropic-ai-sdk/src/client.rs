//! Anthropic API client implementation
//!
//! This module provides the main client for interacting with the Anthropic API.
//! It handles authentication, request construction, and response parsing.

use reqwest::Client as ReqwestClient;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::error::Error as StdError;

/// Anthropic API client
///
/// The main client for making requests to the Anthropic API.
/// Handles authentication and provides methods for making API requests.
///
/// # Examples
///
/// ```no_run
/// use anthropic_ai_sdk::client::AnthropicClient;
/// use anthropic_ai_sdk::types::model::ModelError;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Basic usage
/// let client = AnthropicClient::new::<ModelError>(
///     "your-api-key",
///     "2023-06-01",
/// )?;
///
/// // Using the builder pattern
/// let client_with_custom_url = AnthropicClient::builder("your-api-key", "2023-06-01")
///     .with_api_base_url("https://custom-anthropic-endpoint.com/v1")
///     .build::<ModelError>()?;
///
/// // Using a custom HTTP client
/// let reqwest_client = reqwest::Client::builder().build()?;
/// let client_with_custom_http = AnthropicClient::builder("your-api-key", "2023-06-01")
///     .with_http_client(reqwest_client)
///     .build::<ModelError>()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct AnthropicClient {
    /// The underlying HTTP client for making requests
    client: ReqwestClient,
    /// The API key used for authentication with Anthropic's services
    api_key: String,
    /// The API version used for authentication with Anthropic's services
    api_version: String,
    /// The base URL for the Anthropic API
    api_base_url: String,
}

/// Builder for AnthropicClient
///
/// Provides a flexible way to configure and create an AnthropicClient.
pub struct AnthropicClientBuilder {
    api_key: String,
    api_version: String,
    api_base_url: String,
    client: Option<ReqwestClient>,
}

impl AnthropicClientBuilder {
    /// Creates a new builder with required parameters
    pub fn new(api_key: impl Into<String>, api_version: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            api_version: api_version.into(),
            api_base_url: AnthropicClient::DEFAULT_API_BASE_URL.to_string(),
            client: None,
        }
    }

    /// Sets a custom API base URL
    pub fn with_api_base_url(mut self, api_base_url: impl Into<String>) -> Self {
        self.api_base_url = api_base_url.into();
        self
    }

    /// Sets a custom HTTP client
    pub fn with_http_client(mut self, client: ReqwestClient) -> Self {
        self.client = Some(client);
        self
    }

    /// Set the API version
    pub fn with_api_version(mut self, api_version: impl Into<String>) -> Self {
        self.api_version = api_version.into();
        self
    }

    /// Builds the AnthropicClient with the specified configuration
    pub fn build<E>(self) -> Result<AnthropicClient, E>
    where
        E: StdError + From<String>,
    {
        // Use provided client or create a new one
        let client = if let Some(client) = self.client {
            client
        } else {
            ReqwestClient::builder()
                .user_agent(AnthropicClient::DEFAULT_USER_AGENT)
                .build()
                .map_err(|e| E::from(e.to_string()))?
        };

        Ok(AnthropicClient {
            client,
            api_key: self.api_key,
            api_version: self.api_version,
            api_base_url: self.api_base_url,
        })
    }
}

impl AnthropicClient {
    /// Base URL for the Anthropic API
    pub const DEFAULT_API_BASE_URL: &str = "https://api.anthropic.com/v1";

    /// Default API version for the Anthropic API
    ///
    /// see https://docs.anthropic.com/en/api/versioning
    pub const DEFAULT_API_VERSION: &str = "2023-06-01";

    /// Our user agent.
    pub const DEFAULT_USER_AGENT: &'static str =
        concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"));

    pub fn get_client(&self) -> &ReqwestClient {
        &self.client
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_api_version(&self) -> &str {
        &self.api_version
    }

    pub fn get_api_base_url(&self) -> &str {
        &self.api_base_url
    }

    /// Creates a new AnthropicClient builder
    pub fn builder(
        api_key: impl Into<String>,
        api_version: impl Into<String>,
    ) -> AnthropicClientBuilder {
        AnthropicClientBuilder::new(api_key, api_version)
    }

    /// Creates a new Anthropic API client with the specified credentials
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Anthropic API key for authentication
    /// * `api_version` - The API version to use (e.g., "2023-06-01")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API version header cannot be created
    /// - The HTTP client cannot be initialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anthropic_ai_sdk::client::AnthropicClient;
    /// # use anthropic_ai_sdk::types::model::ModelError;
    /// let client = AnthropicClient::new::<ModelError>(
    ///     "your-api-key",
    ///     "2023-06-01",
    /// ).unwrap();
    /// ```
    pub fn new<E>(api_key: impl Into<String>, api_version: impl Into<String>) -> Result<Self, E>
    where
        E: StdError + From<String>,
    {
        Self::builder(api_key, api_version).build()
    }

    /// Creates a new Anthropic Admin API client with the specified credentials
    ///
    /// # Arguments
    ///
    /// * `admin_api_key` - Your Anthropic Admin API key for authentication
    /// * `api_version` - The API version to use (e.g., "2023-06-01")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API version header cannot be created
    /// - The HTTP client cannot be initialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::api_keys::AdminError;
    /// let client = AnthropicClient::new_admin::<AdminError>(
    ///     "your-admin-api-key",
    ///     "2023-06-01",
    /// ).unwrap();
    /// ```
    pub fn new_admin<E>(
        admin_api_key: impl Into<String>,
        api_version: impl Into<String>,
    ) -> Result<Self, E>
    where
        E: StdError + From<String>,
    {
        Self::builder(admin_api_key, api_version).build()
    }

    /// Sends a request to the Anthropic API with the specified parameters
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type that can be deserialized from JSON
    /// * `Q` - The query parameters type that can be serialized
    /// * `B` - The request body type that can be serialized
    /// * `E` - The error type that can be created from a string
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use for the request
    /// * `path` - The API endpoint path (will be appended to the base URL)
    /// * `query` - Optional query parameters to include in the URL
    /// * `body` - Optional request body to send
    ///
    /// # Returns
    ///
    /// Returns the deserialized response on success, or an error if:
    /// - The request fails to send
    /// - The response indicates an error (non-2xx status)
    /// - The response body cannot be parsed
    pub(crate) async fn send_request<T, Q, B, E>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        let url = format!("{}{}", self.api_base_url, path);

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version);

        // Add query parameters if provided
        if let Some(q) = query {
            request = request.query(q);
        }

        // Add request body if provided
        if let Some(b) = body {
            let _json = serde_json::to_string_pretty(b)
                .map_err(|e| E::from(format!("Failed to serialize body: {}", e)))?;
            request = request.json(b);
        }

        let response = request.send().await.map_err(|e| E::from(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| E::from(format!("Failed to get response body: {}", e)))?;

        if !status.is_success() {
            return Err(E::from(body));
        }

        // Parse the JSON response
        serde_json::from_str(&body).map_err(|e| {
            E::from(format!(
                "JSON parsing error: {}. Response body: {}",
                e, body
            ))
        })
    }

    /// Sends a GET request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `Q` - The query parameters type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    pub(crate) async fn get<T, Q, E>(&self, path: &str, query: Option<&Q>) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, Q, (), E>(reqwest::Method::GET, path, query, None)
            .await
    }

    /// Sends a POST request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `B` - The request body type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `body` - Optional request body
    pub(crate) async fn post<T, B, E>(&self, path: &str, body: Option<&B>) -> Result<T, E>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, (), B, E>(reqwest::Method::POST, path, None, body)
            .await
    }

    /// Sends a DELETE request to the specified endpoint.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `Q` - The query parameters type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    pub(crate) async fn delete<T, Q, E>(&self, path: &str, query: Option<&Q>) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, Q, (), E>(reqwest::Method::DELETE, path, query, None)
            .await
    }

    /// Sends a request with a beta header to the Anthropic API
    ///
    /// This method is similar to `send_request` but adds the `anthropic-beta` header
    /// required for beta features like the Files API.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    /// * `body` - Optional request body
    /// * `beta_header` - The beta header value (e.g., "files-api-2025-04-14")
    pub(crate) async fn send_request_with_beta<T, Q, B, E>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        beta_header: &str,
    ) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        let url = format!("{}{}", self.api_base_url, path);

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version)
            .header("anthropic-beta", beta_header);

        // Add query parameters if provided
        if let Some(q) = query {
            request = request.query(q);
        }

        // Add request body if provided
        if let Some(b) = body {
            let _json = serde_json::to_string_pretty(b)
                .map_err(|e| E::from(format!("Failed to serialize body: {}", e)))?;
            request = request.json(b);
        }

        let response = request.send().await.map_err(|e| E::from(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| E::from(format!("Failed to get response body: {}", e)))?;

        if !status.is_success() {
            return Err(E::from(body));
        }

        // Parse the JSON response
        serde_json::from_str(&body).map_err(|e| {
            E::from(format!(
                "JSON parsing error: {}. Response body: {}",
                e, body
            ))
        })
    }

    /// Sends a GET request with a beta header
    ///
    /// Used for beta APIs that require the `anthropic-beta` header.
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    /// * `beta_header` - The beta header value
    pub(crate) async fn get_with_beta<T, Q, E>(
        &self,
        path: &str,
        query: Option<&Q>,
        beta_header: &str,
    ) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request_with_beta::<T, Q, (), E>(
            reqwest::Method::GET,
            path,
            query,
            None,
            beta_header,
        )
        .await
    }

    /// Sends a request with a beta header and returns raw bytes
    ///
    /// This method is used for endpoints that return binary data instead of JSON.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    /// * `beta_header` - The beta header value
    pub(crate) async fn send_request_with_beta_bytes<Q, E>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Q>,
        beta_header: &str,
    ) -> Result<Vec<u8>, E>
    where
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        let url = format!("{}{}", self.api_base_url, path);

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version)
            .header("anthropic-beta", beta_header);

        // Add query parameters if provided
        if let Some(q) = query {
            request = request.query(q);
        }

        let response = request.send().await.map_err(|e| E::from(e.to_string()))?;

        let status = response.status();
        
        if !status.is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error response".to_string());
            return Err(E::from(error_body));
        }

        // Get the response as bytes
        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| E::from(format!("Failed to get response bytes: {}", e)))
    }

    /// Downloads a file with a beta header
    ///
    /// Used for the Files API download endpoint that returns binary content.
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `beta_header` - The beta header value
    pub(crate) async fn download_with_beta<E>(
        &self,
        path: &str,
        beta_header: &str,
    ) -> Result<Vec<u8>, E>
    where
        E: StdError + From<String>,
    {
        self.send_request_with_beta_bytes::<(), E>(
            reqwest::Method::GET,
            path,
            None,
            beta_header,
        )
        .await
    }
}

//! Messages API
//!
//! This module contains the implementations for the Anthropic Messages API endpoints.
//! It provides functionality for creating messages and counting tokens.

use eventsource_stream::Eventsource;
use futures_util::Stream;
use reqwest::header::HeaderValue;

use crate::client::AnthropicClient;
use crate::types::message::{
    CountMessageTokensParams, CountMessageTokensResponse, CreateMessageParams,
    CreateMessageResponse, MessageClient, MessageError, StreamEvent,
};
use async_trait::async_trait;
use futures_util::StreamExt;

#[async_trait]
impl MessageClient for AnthropicClient {
    /// Creates a message using the specified model
    ///
    /// Creates a message with the given parameters and returns the model's response.
    /// The message can include system prompts, user messages, and other parameters
    /// that control the model's behavior.
    ///
    /// # Arguments
    ///
    /// * `body` - Parameters for creating the message, including the model to use,
    ///   the messages to send, and any additional options
    ///
    /// # Returns
    ///
    /// Returns the model's response on success, including the generated message
    /// and any additional metadata.
    ///
    /// # Errors
    ///
    /// Returns a `MessageError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::message::{MessageClient, MessageError};
    /// use anthropic_ai_sdk::types::message::{
    ///     CreateMessageParams, CreateMessageResponse
    /// };
    /// use tokio;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<MessageError>(
    ///     "your-api-key",
    ///     "2023-06-01",
    /// )?;
    ///
    /// let params = CreateMessageParams::default();
    /// let response = client.create_message(Some(&params)).await?;
    ///
    /// println!("Response: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    async fn create_message<'a>(
        &'a self,
        body: Option<&'a CreateMessageParams>,
    ) -> Result<CreateMessageResponse, MessageError> {
        self.post("/messages", body).await
    }

    /// Counts the number of tokens in a message
    ///
    /// Returns a count of the tokens that would be used by a message with the
    /// given parameters. This can be used to ensure messages stay within token limits.
    ///
    /// # Arguments
    ///
    /// * `body` - Parameters containing the message content to count tokens for
    ///
    /// # Returns
    ///
    /// Returns the token count information on success.
    ///
    /// # Errors
    ///
    /// Returns a `MessageError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::message::{MessageClient, MessageError};
    /// use tokio;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # Ok(())
    /// # }
    /// ```
    async fn count_tokens<'a>(
        &'a self,
        body: Option<&'a CountMessageTokensParams>,
    ) -> Result<CountMessageTokensResponse, MessageError> {
        self.post("/messages/count_tokens", body).await
    }

    /// Creates a message with streaming enabled
    ///
    /// # Arguments
    ///
    /// * `body` - Parameters for creating the message, including the model to use,
    ///   the messages to send, and any additional options
    ///
    /// # Returns
    ///
    /// Returns a stream of events on success.
    ///
    /// # Errors
    ///
    /// Returns a `MessageError` if:
    /// - The request fails to send
    async fn create_message_streaming<'a>(
        &'a self,
        body: &'a CreateMessageParams,
    ) -> Result<impl Stream<Item = Result<StreamEvent, MessageError>> + 'a, MessageError> {
        // Ensure that stream parameter is set to true
        if body.stream.is_none() || !body.stream.unwrap() {
            return Err(MessageError::ApiError(
                "Stream parameter must be set to true for streaming".to_string(),
            ));
        }

        let url = format!("{}/messages", self.get_api_base_url());

        let client = &self.get_client();
        let request = client
            .request(reqwest::Method::POST, &url)
            .header(
                "x-api-key",
                HeaderValue::from_str(self.get_api_key()).unwrap(),
            )
            .header(
                "anthropic-version",
                HeaderValue::from_str(self.get_api_version()).unwrap(),
            )
            .json(body);

        let response = request
            .send()
            .await
            .map_err(|e| MessageError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.map_err(|e| {
                MessageError::RequestFailed(format!("Failed to read error response: {}", e))
            })?;
            return Err(MessageError::ApiError(error_text));
        }

        // Get the bytes stream and convert it to EventSource stream
        let bytes_stream = response.bytes_stream();
        let event_stream = bytes_stream.eventsource();

        // Map SSE events to our StreamEvent type
        Ok(event_stream.map(|event_result| {
            event_result
                .map_err(|e| MessageError::RequestFailed(e.to_string()))
                .and_then(|event| {
                    serde_json::from_str::<StreamEvent>(&event.data).map_err(|e| {
                        MessageError::ApiError(format!(
                            "Failed to parse SSE event: {}. Event data: {}",
                            e, event.data
                        ))
                    })
                })
        }))
    }
}

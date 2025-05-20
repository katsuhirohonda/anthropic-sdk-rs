use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for the Messages API
#[derive(Debug, Error)]
pub enum MessageError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for MessageError {
    fn from(error: String) -> Self {
        MessageError::ApiError(error)
    }
}

#[async_trait]
pub trait MessageClient {
    async fn create_message<'a>(
        &'a self,
        params: Option<&'a CreateMessageParams>,
    ) -> Result<CreateMessageResponse, MessageError>;

    async fn count_tokens<'a>(
        &'a self,
        params: Option<&'a CountMessageTokensParams>,
    ) -> Result<CountMessageTokensResponse, MessageError>;

    async fn create_message_streaming<'a>(
        &'a self,
        body: &'a CreateMessageParams,
    ) -> Result<
        impl futures_util::Stream<Item = Result<StreamEvent, MessageError>> + 'a,
        MessageError,
    >;
}

#[derive(Debug)]
pub struct RequiredMessageParams {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
}

/// Parameters for creating a message
#[derive(Debug, Serialize, Default)]
pub struct CreateMessageParams {
    /// Maximum number of tokens to generate
    pub max_tokens: u32,
    /// Input messages for the conversation
    pub messages: Vec<Message>,
    /// Model to use
    pub model: String,
    /// System prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Temperature for response generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Custom stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Top-k sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// Top-p sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Tools that the model may use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// How the model should use tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Configuration for enabling Claude's extended thinking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
    /// Request metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl From<RequiredMessageParams> for CreateMessageParams {
    fn from(required: RequiredMessageParams) -> Self {
        Self {
            model: required.model,
            messages: required.messages,
            max_tokens: required.max_tokens,
            ..Default::default()
        }
    }
}

impl CreateMessageParams {
    /// Create new parameters with only required fields
    pub fn new(required: RequiredMessageParams) -> Self {
        required.into()
    }

    // Builder methods for optional parameters
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn with_tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    pub fn with_thinking(mut self, thinking: Thinking) -> Self {
        self.thinking = Some(thinking);
        self
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Message in a conversation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    /// Role of the message sender
    pub role: Role,
    /// Content of the message (either string or array of content blocks)
    #[serde(flatten)]
    pub content: MessageContent,
}

/// Role of a message sender
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// Content of a message
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content
    Text { content: String },
    /// Structured content blocks
    Blocks { content: Vec<ContentBlock> },
}

/// Content block in a message
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ContentBlock {
    /// Text content
    #[serde(rename = "text")]
    Text { text: String },
    /// Image content
    #[serde(rename = "image")]
    Image { source: ImageSource },
    /// Tool use content
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    /// Tool result content
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
    /// Thinking content
    #[serde(rename = "thinking")]
    Thinking { thinking: String, signature: String },
    /// Redacted thinking
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
}

/// Source of an image
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ImageSource {
    /// Type of image source
    #[serde(rename = "type")]
    pub type_: String,
    /// Media type of the image
    pub media_type: String,
    /// Base64-encoded image data
    pub data: String,
}

/// Tool definition
#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    /// Name of the tool
    pub name: String,
    /// Description of the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON schema for tool input
    pub input_schema: serde_json::Value,
}

/// Tool choice configuration
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolChoice {
    /// Let model choose whether to use tools
    #[serde(rename = "auto")]
    Auto,
    /// Model must use one of the provided tools
    #[serde(rename = "any")]
    Any,
    /// Model must use a specific tool
    #[serde(rename = "tool")]
    Tool { name: String },
}

/// Configuration for extended thinking
#[derive(Debug, Serialize)]
pub struct Thinking {
    /// Must be at least 1024 tokens
    pub budget_tokens: usize,
    #[serde(rename = "type")]
    pub type_: ThinkingType,
}

#[derive(Debug, Serialize)]
pub enum ThinkingType {
    #[serde(rename = "enabled")]
    Enabled
}
/// Message metadata
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// Custom metadata fields
    #[serde(flatten)]
    pub fields: std::collections::HashMap<String, String>,
}

/// Response from creating a message
#[derive(Debug, Deserialize)]
pub struct CreateMessageResponse {
    /// Content blocks in the response
    pub content: Vec<ContentBlock>,
    /// Unique message identifier
    pub id: String,
    /// Model that handled the request
    pub model: String,
    /// Role of the message (always "assistant")
    pub role: Role,
    /// Reason for stopping generation
    pub stop_reason: Option<StopReason>,
    /// Stop sequence that was generated
    pub stop_sequence: Option<String>,
    /// Type of the message
    #[serde(rename = "type")]
    pub type_: String,
    /// Usage statistics
    pub usage: Usage,
}

/// Reason for stopping message generation
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
}

/// Token usage statistics
#[derive(Debug, Deserialize)]
pub struct Usage {
    /// Input tokens used
    pub input_tokens: u32,
    /// Output tokens used
    pub output_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct StreamUsage {
    /// Input tokens used (may be missing in some events)
    #[serde(default)]
    pub input_tokens: u32,
    /// Output tokens used
    pub output_tokens: u32,
}

impl Message {
    /// Create a new message with simple text content
    pub fn new_text(role: Role, text: impl Into<String>) -> Self {
        Self {
            role,
            content: MessageContent::Text {
                content: text.into(),
            },
        }
    }

    /// Create a new message with content blocks
    pub fn new_blocks(role: Role, blocks: Vec<ContentBlock>) -> Self {
        Self {
            role,
            content: MessageContent::Blocks { content: blocks },
        }
    }
}

// Helper methods for content blocks
impl ContentBlock {
    /// Create a new text block
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    /// Create a new image block
    pub fn image(
        type_: impl Into<String>,
        media_type: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        Self::Image {
            source: ImageSource {
                type_: type_.into(),
                media_type: media_type.into(),
                data: data.into(),
            },
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct CountMessageTokensParams {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct CountMessageTokensResponse {
    pub input_tokens: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum StreamEvent {
    #[serde(rename = "message_start")]
    MessageStart { message: MessageStartContent },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: usize,
        delta: ContentBlockDelta,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: MessageDeltaContent,
        usage: Option<StreamUsage>,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "error")]
    Error { error: StreamError },
}

#[derive(Debug, Deserialize)]
pub struct MessageStartContent {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub role: Role,
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlockDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "input_json_delta")]
    InputJsonDelta { partial_json: String },
    #[serde(rename = "thinking_delta")]
    ThinkingDelta { thinking: String },
    #[serde(rename = "signature_delta")]
    SignatureDelta { signature: String },
}

#[derive(Debug, Deserialize)]
pub struct MessageDeltaContent {
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamError {
    #[serde(rename = "type")]
    pub type_: String,
    pub message: String,
}
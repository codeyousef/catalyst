//! AI Assistant Plugin API
//!
//! This module defines the plugin interface for AI assistants that can be
//! integrated into Catalyst IDE. This allows for modular AI capabilities
//! while keeping the core editor agnostic to specific AI providers.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait that AI assistant plugins must implement
pub trait AiAssistantPlugin: Send + Sync {
    /// Initialize the AI assistant plugin
    fn initialize(&mut self) -> Result<()>;

    /// Get plugin information
    fn plugin_info(&self) -> AiPluginInfo;

    /// Check if the assistant is authenticated and ready
    fn is_authenticated(&self) -> bool;

    /// Send a message to the AI assistant
    fn send_message(&self, request: AiMessageRequest) -> Result<AiMessageResponse>;

    /// Stream a message from the AI assistant
    fn stream_message(
        &self,
        request: AiMessageRequest,
        callback: Box<dyn Fn(AiStreamChunk) + Send>,
    ) -> Result<()>;

    /// Get available tools/capabilities
    fn get_capabilities(&self) -> Vec<AiCapability>;

    /// Handle authentication flow
    fn authenticate(&mut self, auth_data: AiAuthData) -> Result<AiAuthResult>;

    /// Get current usage/cost information
    fn get_usage_info(&self) -> Option<AiUsageInfo>;
}

/// Information about an AI assistant plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub provider: String,
    pub supports_streaming: bool,
    pub supports_tools: bool,
    pub supports_vision: bool,
}

/// Request to send to an AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessageRequest {
    pub messages: Vec<AiMessage>,
    pub context: Option<EditorContext>,
    pub tools: Option<Vec<ToolDefinition>>,
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

/// A message in the AI conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: Option<std::time::SystemTime>,
}

/// Role of the message sender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Context from the editor to provide to AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorContext {
    pub current_file: Option<FileContext>,
    pub selection: Option<SelectionContext>,
    pub project: Option<ProjectContext>,
    pub open_files: Vec<String>,
}

/// Context about the current file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContext {
    pub path: String,
    pub content: String,
    pub language: String,
    pub cursor_position: Position,
}

/// Context about selected text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionContext {
    pub start: Position,
    pub end: Position,
    pub selected_text: String,
}

/// Context about the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub root_path: String,
    pub name: String,
    pub language: Option<String>,
    pub dependencies: Vec<String>,
}

/// Position in a file
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// Tool definition for AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Response from AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessageResponse {
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub usage: Option<UsageInfo>,
    pub model: String,
    pub finish_reason: Option<String>,
}

/// Tool call from AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Usage information from AI provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost: Option<f64>,
}

/// Streaming chunk from AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStreamChunk {
    pub content: Option<String>,
    pub tool_call: Option<ToolCall>,
    pub finished: bool,
}

/// AI assistant capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCapability {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

/// Authentication data for AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAuthData {
    pub auth_type: AiAuthType,
    pub credentials: HashMap<String, String>,
}

/// Type of authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiAuthType {
    ApiKey,
    OAuth,
    BearerToken,
    Custom(String),
}

/// Result of authentication attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAuthResult {
    pub success: bool,
    pub message: String,
    pub expires_at: Option<std::time::SystemTime>,
}

/// Current usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiUsageInfo {
    pub total_tokens_used: u64,
    pub total_cost: f64,
    pub current_session_tokens: u32,
    pub current_session_cost: f64,
    pub rate_limit_info: Option<RateLimitInfo>,
}

/// Rate limiting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub requests_remaining: u32,
    pub tokens_remaining: u32,
    pub reset_time: std::time::SystemTime,
}

//! Plugin API for Catalyst IDE
//!
//! This module defines the plugin interfaces that allow external integrations
//! to extend Catalyst IDE functionality. This provides a clean separation
//! between the open-source editor and proprietary extensions.

use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Trait for AI assistant providers
/// 
/// This allows different AI assistants to integrate with Catalyst IDE
/// without hardcoding specific implementations.
pub trait AssistantProvider: Send + Sync {
    /// Get the name of this assistant provider
    fn name(&self) -> &str;
    
    /// Send a message to the assistant and get a response
    async fn send_message(
        &self, 
        message: &str, 
        context: &EditorContext
    ) -> Result<AssistantResponse>;
    
    /// Stream a response from the assistant
    async fn stream_message(
        &self,
        message: &str,
        context: &EditorContext,
        callback: Box<dyn Fn(String) + Send>,
    ) -> Result<()>;
    
    /// Get available tools/capabilities
    fn get_capabilities(&self) -> Vec<AssistantCapability>;
}

/// Context information about the current editor state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorContext {
    /// Currently active file
    pub current_file: Option<PathBuf>,
    /// Text selection, if any
    pub selection: Option<TextRange>,
    /// Cursor position
    pub cursor_position: Position,
    /// Project root directory
    pub project_root: PathBuf,
    /// List of open files
    pub open_files: Vec<PathBuf>,
    /// Git repository status
    pub git_status: Option<GitStatus>,
    /// Language server diagnostics
    pub diagnostics: Vec<Diagnostic>,
    /// Available language servers
    pub language_servers: Vec<String>,
}

/// Text range in the editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRange {
    pub start: Position,
    pub end: Position,
}

/// Position in a text document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// Git repository status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub branch: String,
    pub modified_files: Vec<PathBuf>,
    pub untracked_files: Vec<PathBuf>,
    pub staged_files: Vec<PathBuf>,
}

/// Diagnostic information from language servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: TextRange,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
}

/// Diagnostic severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// Response from an AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantResponse {
    /// The main response content
    pub content: String,
    /// Metadata about the response
    pub metadata: ResponseMetadata,
    /// Suggested actions the user can take
    pub actions: Vec<SuggestedAction>,
}

/// Metadata about an assistant response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Model used to generate response
    pub model: String,
    /// Tokens used in the request
    pub tokens_used: Option<u32>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Whether this was a streamed response
    pub streamed: bool,
}

/// Actions suggested by the assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    /// Display name for the action
    pub label: String,
    /// Type of action
    pub action_type: ActionType,
    /// Data associated with the action
    pub data: serde_json::Value,
}

/// Types of actions that can be suggested
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Apply a code change
    ApplyEdit,
    /// Open a file
    OpenFile,
    /// Run a command
    RunCommand,
    /// Create a file
    CreateFile,
    /// Explain code
    Explain,
}

/// Capabilities of an assistant provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantCapability {
    /// Name of the capability
    pub name: String,
    /// Description of what it does
    pub description: String,
    /// Whether it's currently available
    pub available: bool,
    /// Required parameters
    pub parameters: Vec<CapabilityParameter>,
}

/// Parameter for a capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

/// Trait for external tool providers
///
/// This allows integration of external tools and services that can be
/// called from within Catalyst IDE.
pub trait ToolProvider: Send + Sync {
    /// Get the name of this tool provider
    fn name(&self) -> &str;
    
    /// List available tools
    fn list_tools(&self) -> Vec<ToolDefinition>;
    
    /// Execute a tool
    async fn execute_tool(
        &self,
        tool_name: &str,
        arguments: HashMap<String, serde_json::Value>,
        context: &EditorContext,
    ) -> Result<ToolResult>;
    
    /// Check if a tool is available
    fn is_tool_available(&self, tool_name: &str) -> bool;
}

/// Definition of an external tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool identifier
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Input schema for the tool
    pub input_schema: serde_json::Value,
    /// Whether the tool requires confirmation
    pub requires_confirmation: bool,
    /// Security level required
    pub security_level: SecurityLevel,
}

/// Security levels for tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Safe operations that don't modify system state
    Safe,
    /// Operations that modify files in the workspace
    Workspace,
    /// Operations that can modify system state
    System,
    /// Operations requiring network access
    Network,
}

/// Result of tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Whether execution was successful
    pub success: bool,
    /// Result data
    pub data: serde_json::Value,
    /// Error message if execution failed
    pub error: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Trait for context providers
///
/// Context providers supply additional information about the workspace
/// and project that can be used by assistants and tools.
pub trait ContextProvider: Send + Sync {
    /// Get the name of this context provider
    fn name(&self) -> &str;
    
    /// Get context information for the current state
    async fn get_context(&self, editor_context: &EditorContext) -> Result<ContextData>;
    
    /// Check if this provider can provide context for the given file
    fn supports_file(&self, path: &PathBuf) -> bool;
}

/// Context data from a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextData {
    /// Provider that generated this context
    pub provider: String,
    /// Context information
    pub data: serde_json::Value,
    /// Relevance score (0.0 to 1.0)
    pub relevance: f64,
    /// Timestamp when context was generated
    pub timestamp: std::time::SystemTime,
}

/// Plugin registry for managing all registered plugins
pub struct PluginRegistry {
    assistant_providers: HashMap<String, Box<dyn AssistantProvider>>,
    tool_providers: HashMap<String, Box<dyn ToolProvider>>,
    context_providers: HashMap<String, Box<dyn ContextProvider>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            assistant_providers: HashMap::new(),
            tool_providers: HashMap::new(),
            context_providers: HashMap::new(),
        }
    }
    
    /// Register an assistant provider
    pub fn register_assistant(&mut self, provider: Box<dyn AssistantProvider>) {
        let name = provider.name().to_string();
        self.assistant_providers.insert(name, provider);
    }
    
    /// Register a tool provider
    pub fn register_tool_provider(&mut self, provider: Box<dyn ToolProvider>) {
        let name = provider.name().to_string();
        self.tool_providers.insert(name, provider);
    }
    
    /// Register a context provider
    pub fn register_context_provider(&mut self, provider: Box<dyn ContextProvider>) {
        let name = provider.name().to_string();
        self.context_providers.insert(name, provider);
    }
    
    /// Get an assistant provider by name
    pub fn get_assistant(&self, name: &str) -> Option<&dyn AssistantProvider> {
        self.assistant_providers.get(name).map(|p| p.as_ref())
    }
    
    /// Get a tool provider by name
    pub fn get_tool_provider(&self, name: &str) -> Option<&dyn ToolProvider> {
        self.tool_providers.get(name).map(|p| p.as_ref())
    }
    
    /// Get a context provider by name
    pub fn get_context_provider(&self, name: &str) -> Option<&dyn ContextProvider> {
        self.context_providers.get(name).map(|p| p.as_ref())
    }
    
    /// List all registered providers
    pub fn list_providers(&self) -> PluginInfo {
        PluginInfo {
            assistants: self.assistant_providers.keys().cloned().collect(),
            tool_providers: self.tool_providers.keys().cloned().collect(),
            context_providers: self.context_providers.keys().cloned().collect(),
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about registered plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub assistants: Vec<String>,
    pub tool_providers: Vec<String>, 
    pub context_providers: Vec<String>,
}

/// Hook points in the editor where plugins can integrate
pub enum PluginHook {
    /// Called when a file is opened
    FileOpened(PathBuf),
    /// Called when text is changed
    TextChanged(PathBuf, TextRange),
    /// Called when saving a file
    FileSaving(PathBuf),
    /// Called when a project is opened
    ProjectOpened(PathBuf),
    /// Called when language server provides diagnostics
    DiagnosticsReceived(PathBuf, Vec<Diagnostic>),
}

/// Plugin hook handler trait
pub trait PluginHookHandler: Send + Sync {
    /// Handle a plugin hook event
    async fn handle_hook(&self, hook: PluginHook, context: &EditorContext) -> Result<()>;
}

/// Global plugin registry instance
/// This would be initialized in the main application
pub static mut GLOBAL_PLUGIN_REGISTRY: Option<PluginRegistry> = None;
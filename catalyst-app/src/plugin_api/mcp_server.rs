//! MCP Server Plugin API
//!
//! This module defines the plugin interface for Model Context Protocol (MCP) servers
//! that can be integrated into Catalyst IDE.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait that MCP server plugins must implement
pub trait McpServerPlugin: Send + Sync {
    /// Initialize the MCP server plugin
    fn initialize(&mut self) -> Result<()>;
    
    /// Get server information
    fn server_info(&self) -> McpServerInfo;
    
    /// Start the MCP server
    fn start(&mut self) -> Result<()>;
    
    /// Stop the MCP server
    fn stop(&mut self) -> Result<()>;
    
    /// Check if server is running
    fn is_running(&self) -> bool;
    
    /// Get server health status
    fn health_check(&self) -> McpServerHealth;
    
    /// Send a request to the MCP server
    fn send_request(&self, request: McpRequest) -> Result<McpResponse>;
    
    /// Get available tools from the server
    fn get_tools(&self) -> Result<Vec<McpTool>>;
    
    /// Get available resources from the server
    fn get_resources(&self) -> Result<Vec<McpResource>>;
    
    /// Call a tool on the server
    fn call_tool(&self, tool_name: &str, arguments: serde_json::Value) -> Result<McpToolResult>;
    
    /// Read a resource from the server
    fn read_resource(&self, resource_uri: &str) -> Result<McpResourceContent>;
    
    /// Subscribe to resource changes
    fn subscribe_to_resource(&self, resource_uri: &str) -> Result<()>;
    
    /// Unsubscribe from resource changes
    fn unsubscribe_from_resource(&self, resource_uri: &str) -> Result<()>;
}

/// Information about an MCP server plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub command: Vec<String>,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub working_directory: Option<String>,
    pub auto_start: bool,
    pub capabilities: McpServerCapabilities,
}

/// Capabilities that an MCP server supports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerCapabilities {
    pub tools: bool,
    pub resources: bool,
    pub prompts: bool,
    pub logging: bool,
    pub experimental: HashMap<String, serde_json::Value>,
}

/// Health status of an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerHealth {
    pub status: McpServerStatus,
    pub last_error: Option<String>,
    pub uptime: Option<std::time::Duration>,
    pub request_count: u64,
    pub error_count: u64,
}

/// Status of an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpServerStatus {
    Stopped,
    Starting,
    Running,
    Error,
    Restarting,
}

/// Request to send to an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

/// Response from an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
}

/// Error from an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Tool available from an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
}

/// Resource available from an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

/// Result of calling a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolResult {
    pub content: Vec<McpContent>,
    pub is_error: bool,
}

/// Content returned by MCP operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpContent {
    pub content_type: String,
    pub data: serde_json::Value,
}

/// Content of a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceContent {
    pub uri: String,
    pub mime_type: Option<String>,
    pub text: Option<String>,
    pub blob: Option<Vec<u8>>,
}

/// Registry for managing MCP servers
pub struct McpServerRegistry {
    servers: HashMap<String, Box<dyn McpServerPlugin>>,
}

impl McpServerRegistry {
    /// Create a new MCP server registry
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }
    
    /// Register a new MCP server
    pub fn register_server(&mut self, id: String, server: Box<dyn McpServerPlugin>) -> Result<()> {
        if self.servers.contains_key(&id) {
            return Err(anyhow::anyhow!("MCP server with id '{}' is already registered", id));
        }
        
        self.servers.insert(id, server);
        Ok(())
    }
    
    /// Unregister an MCP server
    pub fn unregister_server(&mut self, id: &str) -> Result<()> {
        self.servers.remove(id)
            .ok_or_else(|| anyhow::anyhow!("MCP server with id '{}' is not registered", id))?;
        Ok(())
    }
    
    /// Get an MCP server by id
    pub fn get_server(&self, id: &str) -> Option<&dyn McpServerPlugin> {
        self.servers.get(id).map(|server| server.as_ref())
    }
    
    /// Get a mutable MCP server by id
    pub fn get_server_mut(&mut self, id: &str) -> Option<&mut dyn McpServerPlugin> {
        self.servers.get_mut(id).map(|server| server.as_mut())
    }
    
    /// Get all registered server IDs
    pub fn get_server_ids(&self) -> Vec<String> {
        self.servers.keys().cloned().collect()
    }
    
    /// Get server info for all registered servers
    pub fn get_all_server_info(&self) -> Vec<McpServerInfo> {
        self.servers.values()
            .map(|server| server.server_info())
            .collect()
    }
    
    /// Start all auto-start servers
    pub fn start_auto_start_servers(&mut self) -> Result<()> {
        for server in self.servers.values_mut() {
            if server.server_info().auto_start && !server.is_running() {
                server.start()?;
            }
        }
        Ok(())
    }
    
    /// Stop all running servers
    pub fn stop_all_servers(&mut self) -> Result<()> {
        for server in self.servers.values_mut() {
            if server.is_running() {
                server.stop()?;
            }
        }
        Ok(())
    }
    
    /// Get health status for all servers
    pub fn get_all_health_status(&self) -> HashMap<String, McpServerHealth> {
        self.servers.iter()
            .map(|(id, server)| (id.clone(), server.health_check()))
            .collect()
    }
}

impl Default for McpServerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
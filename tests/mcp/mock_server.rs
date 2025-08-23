/// MCP Server Mock Infrastructure
/// 
/// Provides mock implementations of MCP servers for testing purposes.
/// This allows testing MCP protocol compliance and functionality without
/// requiring actual external server installations.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde_json::{json, Value};

/// Mock MCP Server implementation for testing
#[derive(Debug, Clone)]
pub struct MockMcpServer {
    pub name: String,
    pub capabilities: Vec<String>,
    pub tools: HashMap<String, MockTool>,
    pub resources: HashMap<String, MockResource>,
    pub response_delay: Duration,
    pub should_fail: bool,
    pub call_count: Arc<Mutex<u32>>,
}

#[derive(Debug, Clone)]
pub struct MockTool {
    pub name: String,
    pub description: String,
    pub schema: Value,
    pub handler: fn(&Value) -> Result<Value, String>,
}

#[derive(Debug, Clone)]
pub struct MockResource {
    pub uri: String,
    pub mime_type: String,
    pub content: String,
}

impl MockMcpServer {
    /// Create a new mock MCP server
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            capabilities: vec![
                "tools".to_string(),
                "resources".to_string(),
                "logging".to_string(),
            ],
            tools: HashMap::new(),
            resources: HashMap::new(),
            response_delay: Duration::from_millis(0),
            should_fail: false,
            call_count: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Add a mock tool to the server
    pub fn with_tool(mut self, tool: MockTool) -> Self {
        self.tools.insert(tool.name.clone(), tool);
        self
    }
    
    /// Add a mock resource to the server
    pub fn with_resource(mut self, resource: MockResource) -> Self {
        self.resources.insert(resource.uri.clone(), resource);
        self
    }
    
    /// Set response delay for testing latency
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.response_delay = delay;
        self
    }
    
    /// Configure server to fail requests for error testing
    pub fn with_failure(mut self, should_fail: bool) -> Self {
        self.should_fail = should_fail;
        self
    }
    
    /// Handle MCP initialize request
    pub fn handle_initialize(&self, request: &Value) -> Result<Value, String> {
        self.increment_call_count();
        
        if self.should_fail {
            return Err("Mock server configured to fail".to_string());
        }
        
        std::thread::sleep(self.response_delay);
        
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {
                    "listChanged": true
                },
                "resources": {
                    "subscribe": true,
                    "listChanged": true
                },
                "logging": {}
            },
            "serverInfo": {
                "name": self.name,
                "version": "1.0.0-mock"
            }
        }))
    }
    
    /// Handle tools/list request
    pub fn handle_tools_list(&self, _request: &Value) -> Result<Value, String> {
        self.increment_call_count();
        
        if self.should_fail {
            return Err("Mock server configured to fail".to_string());
        }
        
        std::thread::sleep(self.response_delay);
        
        let tools: Vec<Value> = self.tools.values()
            .map(|tool| json!({
                "name": tool.name,
                "description": tool.description,
                "inputSchema": tool.schema
            }))
            .collect();
        
        Ok(json!({
            "tools": tools
        }))
    }
    
    /// Handle tools/call request
    pub fn handle_tools_call(&self, request: &Value) -> Result<Value, String> {
        self.increment_call_count();
        
        if self.should_fail {
            return Err("Mock server configured to fail".to_string());
        }
        
        std::thread::sleep(self.response_delay);
        
        let tool_name = request.get("params")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .ok_or("Missing tool name")?;
        
        let arguments = request.get("params")
            .and_then(|p| p.get("arguments"))
            .cloned()
            .unwrap_or(Value::Null);
        
        if let Some(tool) = self.tools.get(tool_name) {
            let result = (tool.handler)(&arguments)?;
            Ok(json!({
                "content": [{
                    "type": "text",
                    "text": result.to_string()
                }]
            }))
        } else {
            Err(format!("Unknown tool: {}", tool_name))
        }
    }
    
    /// Handle resources/list request
    pub fn handle_resources_list(&self, _request: &Value) -> Result<Value, String> {
        self.increment_call_count();
        
        if self.should_fail {
            return Err("Mock server configured to fail".to_string());
        }
        
        std::thread::sleep(self.response_delay);
        
        let resources: Vec<Value> = self.resources.values()
            .map(|resource| json!({
                "uri": resource.uri,
                "name": resource.uri,
                "mimeType": resource.mime_type
            }))
            .collect();
        
        Ok(json!({
            "resources": resources
        }))
    }
    
    /// Handle resources/read request
    pub fn handle_resources_read(&self, request: &Value) -> Result<Value, String> {
        self.increment_call_count();
        
        if self.should_fail {
            return Err("Mock server configured to fail".to_string());
        }
        
        std::thread::sleep(self.response_delay);
        
        let uri = request.get("params")
            .and_then(|p| p.get("uri"))
            .and_then(|u| u.as_str())
            .ok_or("Missing resource URI")?;
        
        if let Some(resource) = self.resources.get(uri) {
            Ok(json!({
                "contents": [{
                    "uri": resource.uri,
                    "mimeType": resource.mime_type,
                    "text": resource.content
                }]
            }))
        } else {
            Err(format!("Resource not found: {}", uri))
        }
    }
    
    /// Get the number of calls made to this mock server
    pub fn get_call_count(&self) -> u32 {
        *self.call_count.lock().unwrap()
    }
    
    /// Reset the call count
    pub fn reset_call_count(&self) {
        *self.call_count.lock().unwrap() = 0;
    }
    
    fn increment_call_count(&self) {
        *self.call_count.lock().unwrap() += 1;
    }
}

/// Factory for creating pre-configured mock servers for each MCP server type
pub struct MockMcpServerFactory;

impl MockMcpServerFactory {
    /// Create mock filesystem server
    pub fn filesystem_server() -> MockMcpServer {
        MockMcpServer::new("filesystem")
            .with_tool(MockTool {
                name: "read_file".to_string(),
                description: "Read a file from the filesystem".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File path to read" }
                    },
                    "required": ["path"]
                }),
                handler: |args| {
                    let path = args.get("path")
                        .and_then(|p| p.as_str())
                        .unwrap_or("unknown");
                    Ok(json!(format!("Mock file content for: {}", path)))
                },
            })
            .with_tool(MockTool {
                name: "write_file".to_string(),
                description: "Write content to a file".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "content": { "type": "string" }
                    },
                    "required": ["path", "content"]
                }),
                handler: |args| {
                    let path = args.get("path").and_then(|p| p.as_str()).unwrap_or("unknown");
                    let content_len = args.get("content")
                        .and_then(|c| c.as_str())
                        .map(|s| s.len())
                        .unwrap_or(0);
                    Ok(json!(format!("Wrote {} bytes to {}", content_len, path)))
                },
            })
    }
    
    /// Create mock git server
    pub fn git_server() -> MockMcpServer {
        MockMcpServer::new("git")
            .with_tool(MockTool {
                name: "git_status".to_string(),
                description: "Get git repository status".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Repository path" }
                    },
                    "required": ["path"]
                }),
                handler: |_args| {
                    Ok(json!("Modified: 5 files, Untracked: 2 files"))
                },
            })
            .with_tool(MockTool {
                name: "git_commit".to_string(),
                description: "Create a git commit".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "message": { "type": "string" },
                        "files": { "type": "array", "items": { "type": "string" } }
                    },
                    "required": ["message"]
                }),
                handler: |args| {
                    let message = args.get("message").and_then(|m| m.as_str()).unwrap_or("Empty commit");
                    Ok(json!(format!("Created commit: {}", message)))
                },
            })
    }
    
    /// Create mock docker server
    pub fn docker_server() -> MockMcpServer {
        MockMcpServer::new("docker")
            .with_tool(MockTool {
                name: "list_containers".to_string(),
                description: "List Docker containers".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "all": { "type": "boolean", "description": "Include stopped containers" }
                    }
                }),
                handler: |_args| {
                    Ok(json!([
                        {"id": "container1", "name": "web-app", "status": "running"},
                        {"id": "container2", "name": "database", "status": "stopped"}
                    ]))
                },
            })
            .with_tool(MockTool {
                name: "run_container".to_string(),
                description: "Run a Docker container".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "image": { "type": "string" },
                        "name": { "type": "string" },
                        "ports": { "type": "array", "items": { "type": "string" } }
                    },
                    "required": ["image"]
                }),
                handler: |args| {
                    let image = args.get("image").and_then(|i| i.as_str()).unwrap_or("unknown");
                    Ok(json!(format!("Started container from image: {}", image)))
                },
            })
    }
    
    /// Create mock GitHub server
    pub fn github_server() -> MockMcpServer {
        MockMcpServer::new("github")
            .with_tool(MockTool {
                name: "create_issue".to_string(),
                description: "Create a GitHub issue".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "body": { "type": "string" },
                        "labels": { "type": "array", "items": { "type": "string" } }
                    },
                    "required": ["title"]
                }),
                handler: |args| {
                    let title = args.get("title").and_then(|t| t.as_str()).unwrap_or("Untitled");
                    Ok(json!(format!("Created issue #{}: {}", 1234, title)))
                },
            })
            .with_tool(MockTool {
                name: "create_pr".to_string(),
                description: "Create a pull request".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "body": { "type": "string" },
                        "head": { "type": "string" },
                        "base": { "type": "string" }
                    },
                    "required": ["title", "head", "base"]
                }),
                handler: |args| {
                    let title = args.get("title").and_then(|t| t.as_str()).unwrap_or("Untitled PR");
                    Ok(json!(format!("Created PR #{}: {}", 567, title)))
                },
            })
    }
    
    /// Create all standard mock servers
    pub fn all_servers() -> Vec<MockMcpServer> {
        vec![
            Self::filesystem_server(),
            Self::git_server(),
            Self::docker_server(),
            Self::github_server(),
            // Add more servers as needed
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_mock_server_creation() {
        let server = MockMcpServer::new("test-server");
        assert_eq!(server.name, "test-server");
        assert!(server.capabilities.contains(&"tools".to_string()));
        assert_eq!(server.get_call_count(), 0);
    }
    
    #[test]
    fn test_mock_server_initialize() {
        let server = MockMcpServer::new("test-server");
        let request = json!({
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05"
            }
        });
        
        let response = server.handle_initialize(&request).unwrap();
        assert_eq!(response["serverInfo"]["name"], "test-server");
        assert_eq!(server.get_call_count(), 1);
    }
    
    #[test]
    fn test_filesystem_mock_server() {
        let server = MockMcpServerFactory::filesystem_server();
        
        // Test tools list
        let tools_response = server.handle_tools_list(&json!({})).unwrap();
        let tools = tools_response["tools"].as_array().unwrap();
        assert!(tools.len() >= 2); // read_file and write_file
        
        // Test file read
        let read_request = json!({
            "params": {
                "name": "read_file",
                "arguments": { "path": "/test/file.txt" }
            }
        });
        
        let read_response = server.handle_tools_call(&read_request).unwrap();
        assert!(read_response["content"][0]["text"].as_str().unwrap().contains("/test/file.txt"));
    }
    
    #[test]
    fn test_mock_server_failure_mode() {
        let server = MockMcpServer::new("failing-server").with_failure(true);
        
        let request = json!({});
        let result = server.handle_initialize(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("configured to fail"));
    }
    
    #[test]
    fn test_mock_server_response_delay() {
        let server = MockMcpServer::new("slow-server")
            .with_delay(Duration::from_millis(100));
        
        let start = Instant::now();
        let _response = server.handle_initialize(&json!({})).unwrap();
        let duration = start.elapsed();
        
        assert!(duration >= Duration::from_millis(100));
    }
}
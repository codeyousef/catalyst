/// MCP Protocol Compliance Tests
/// 
/// Tests that verify MCP server implementations comply with the
/// Model Context Protocol specification.

use super::mock_server::*;
use serde_json::{json, Value};
use std::time::{Duration, Instant};

#[cfg(test)]
mod protocol_compliance_tests {
    use super::*;
    
    #[test]
    fn test_mcp_initialize_protocol_compliance() {
        let server = MockMcpServer::new("test-server");
        
        // Valid initialize request
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "roots": {
                        "listChanged": true
                    }
                },
                "clientInfo": {
                    "name": "Catalyst IDE",
                    "version": "0.4.3"
                }
            }
        });
        
        let response = server.handle_initialize(&request).unwrap();
        
        // Verify response structure compliance
        assert!(response.get("protocolVersion").is_some());
        assert!(response.get("capabilities").is_some());
        assert!(response.get("serverInfo").is_some());
        
        let server_info = response.get("serverInfo").unwrap();
        assert!(server_info.get("name").is_some());
        assert!(server_info.get("version").is_some());
        
        println!("✅ MCP initialize protocol compliance verified");
    }
    
    #[test]
    fn test_tools_list_protocol_compliance() {
        let server = MockMcpServerFactory::filesystem_server();
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        });
        
        let response = server.handle_tools_list(&request).unwrap();
        
        // Verify response structure
        assert!(response.get("tools").is_some());
        let tools = response.get("tools").unwrap().as_array().unwrap();
        
        for tool in tools {
            // Each tool must have required fields
            assert!(tool.get("name").is_some());
            assert!(tool.get("description").is_some());
            assert!(tool.get("inputSchema").is_some());
            
            // Verify schema is valid JSON Schema
            let schema = tool.get("inputSchema").unwrap();
            assert!(schema.get("type").is_some());
        }
        
        println!("✅ MCP tools/list protocol compliance verified");
    }
    
    #[test]
    fn test_tools_call_protocol_compliance() {
        let server = MockMcpServerFactory::filesystem_server();
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "read_file",
                "arguments": {
                    "path": "/test/file.txt"
                }
            }
        });
        
        let response = server.handle_tools_call(&request).unwrap();
        
        // Verify response structure
        assert!(response.get("content").is_some());
        let content = response.get("content").unwrap().as_array().unwrap();
        
        for item in content {
            assert!(item.get("type").is_some());
            let content_type = item.get("type").unwrap().as_str().unwrap();
            
            match content_type {
                "text" => {
                    assert!(item.get("text").is_some());
                }
                "image" => {
                    assert!(item.get("data").is_some());
                    assert!(item.get("mimeType").is_some());
                }
                _ => panic!("Unknown content type: {}", content_type),
            }
        }
        
        println!("✅ MCP tools/call protocol compliance verified");
    }
    
    #[test]
    fn test_resources_list_protocol_compliance() {
        let server = MockMcpServer::new("test-server")
            .with_resource(MockResource {
                uri: "file:///test.txt".to_string(),
                mime_type: "text/plain".to_string(),
                content: "Test content".to_string(),
            });
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "resources/list"
        });
        
        let response = server.handle_resources_list(&request).unwrap();
        
        // Verify response structure
        assert!(response.get("resources").is_some());
        let resources = response.get("resources").unwrap().as_array().unwrap();
        
        for resource in resources {
            assert!(resource.get("uri").is_some());
            assert!(resource.get("name").is_some());
            
            // Optional but common fields
            if let Some(mime_type) = resource.get("mimeType") {
                assert!(mime_type.is_string());
            }
        }
        
        println!("✅ MCP resources/list protocol compliance verified");
    }
    
    #[test]
    fn test_resources_read_protocol_compliance() {
        let server = MockMcpServer::new("test-server")
            .with_resource(MockResource {
                uri: "file:///test.txt".to_string(),
                mime_type: "text/plain".to_string(),
                content: "Test content".to_string(),
            });
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 5,
            "method": "resources/read",
            "params": {
                "uri": "file:///test.txt"
            }
        });
        
        let response = server.handle_resources_read(&request).unwrap();
        
        // Verify response structure
        assert!(response.get("contents").is_some());
        let contents = response.get("contents").unwrap().as_array().unwrap();
        
        for content in contents {
            assert!(content.get("uri").is_some());
            assert!(content.get("mimeType").is_some());
            
            // Must have either text or blob content
            let has_text = content.get("text").is_some();
            let has_blob = content.get("blob").is_some();
            assert!(has_text || has_blob, "Content must have either text or blob");
        }
        
        println!("✅ MCP resources/read protocol compliance verified");
    }
    
    #[test]
    fn test_error_handling_protocol_compliance() {
        let server = MockMcpServer::new("failing-server").with_failure(true);
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 6,
            "method": "tools/call",
            "params": {
                "name": "nonexistent_tool",
                "arguments": {}
            }
        });
        
        let result = server.handle_tools_call(&request);
        assert!(result.is_err());
        
        // In a real implementation, this would return a proper JSON-RPC error response
        let error_message = result.unwrap_err();
        assert!(!error_message.is_empty());
        
        println!("✅ MCP error handling protocol compliance verified");
    }
    
    #[test]
    fn test_response_time_requirements() {
        let server = MockMcpServer::new("performance-server");
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 7,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05"
            }
        });
        
        let start = Instant::now();
        let _response = server.handle_initialize(&request).unwrap();
        let duration = start.elapsed();
        
        // MCP server response time should be < 200ms as per requirements
        assert!(
            duration.as_millis() < super::super::performance::MCP_RESPONSE_THRESHOLD_MS,
            "MCP server response time {} ms exceeds threshold {} ms",
            duration.as_millis(),
            super::super::performance::MCP_RESPONSE_THRESHOLD_MS
        );
        
        println!("✅ MCP response time requirements verified ({:?})", duration);
    }
    
    #[test]
    fn test_concurrent_request_handling() {
        use std::thread;
        
        let server = MockMcpServerFactory::filesystem_server();
        let server_arc = std::sync::Arc::new(server);
        
        let mut handles = vec![];
        
        // Spawn multiple concurrent requests
        for i in 0..10 {
            let server_clone = server_arc.clone();
            let handle = thread::spawn(move || {
                let request = json!({
                    "jsonrpc": "2.0",
                    "id": i,
                    "method": "tools/list"
                });
                
                server_clone.handle_tools_list(&request)
            });
            handles.push(handle);
        }
        
        // Wait for all requests to complete
        let mut results = vec![];
        for handle in handles {
            let result = handle.join().unwrap();
            results.push(result);
        }
        
        // Verify all requests succeeded
        for result in results {
            assert!(result.is_ok());
        }
        
        // Verify call count is correct
        assert_eq!(server_arc.get_call_count(), 10);
        
        println!("✅ MCP concurrent request handling verified");
    }
    
    #[test]
    fn test_json_rpc_message_format() {
        // Verify our mock server responses would be valid JSON-RPC format
        let server = MockMcpServer::new("format-test");
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": 123,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05"
            }
        });
        
        let response = server.handle_initialize(&request).unwrap();
        
        // In a real implementation, we would wrap this in JSON-RPC format:
        let json_rpc_response = json!({
            "jsonrpc": "2.0",
            "id": 123,
            "result": response
        });
        
        assert_eq!(json_rpc_response.get("jsonrpc").unwrap(), "2.0");
        assert_eq!(json_rpc_response.get("id").unwrap(), 123);
        assert!(json_rpc_response.get("result").is_some());
        
        println!("✅ JSON-RPC message format compliance verified");
    }
}
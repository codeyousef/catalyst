/// MCP Server Health Check System
/// 
/// Provides health monitoring and diagnostics for MCP servers to ensure
/// they are functioning correctly and meeting performance requirements.

use super::mock_server::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct McpServerHealth {
    pub server_name: String,
    pub is_healthy: bool,
    pub last_check: Instant,
    pub response_time_ms: u64,
    pub error_count: u32,
    pub success_count: u32,
    pub last_error: Option<String>,
}

impl McpServerHealth {
    pub fn new(server_name: String) -> Self {
        Self {
            server_name,
            is_healthy: false,
            last_check: Instant::now(),
            response_time_ms: 0,
            error_count: 0,
            success_count: 0,
            last_error: None,
        }
    }
    
    pub fn success_rate(&self) -> f64 {
        let total = self.success_count + self.error_count;
        if total == 0 {
            0.0
        } else {
            self.success_count as f64 / total as f64
        }
    }
}

pub struct McpHealthChecker {
    server_health: Arc<Mutex<HashMap<String, McpServerHealth>>>,
    check_interval: Duration,
    response_timeout: Duration,
    max_acceptable_response_time: Duration,
}

impl McpHealthChecker {
    pub fn new() -> Self {
        Self {
            server_health: Arc::new(Mutex::new(HashMap::new())),
            check_interval: Duration::from_secs(30),
            response_timeout: Duration::from_secs(5),
            max_acceptable_response_time: Duration::from_millis(200),
        }
    }
    
    pub fn with_check_interval(mut self, interval: Duration) -> Self {
        self.check_interval = interval;
        self
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.response_timeout = timeout;
        self
    }
    
    /// Perform health check on a single MCP server
    pub fn check_server_health(&self, server: &MockMcpServer) -> McpServerHealth {
        let start_time = Instant::now();
        let mut health = McpServerHealth::new(server.name.clone());
        
        // Test basic initialization
        let init_result = self.test_initialize(server);
        let response_time = start_time.elapsed();
        
        health.last_check = Instant::now();
        health.response_time_ms = response_time.as_millis() as u64;
        
        match init_result {
            Ok(_) => {
                health.success_count += 1;
                health.is_healthy = response_time <= self.max_acceptable_response_time;
                
                if !health.is_healthy {
                    health.last_error = Some(format!(
                        "Response time {}ms exceeds maximum {}ms", 
                        response_time.as_millis(),
                        self.max_acceptable_response_time.as_millis()
                    ));
                }
            }
            Err(e) => {
                health.error_count += 1;
                health.is_healthy = false;
                health.last_error = Some(e);
            }
        }
        
        // Additional health checks
        if health.is_healthy {
            if let Err(e) = self.test_tools_functionality(server) {
                health.is_healthy = false;
                health.error_count += 1;
                health.last_error = Some(e);
            }
        }
        
        health
    }
    
    /// Test server initialization
    fn test_initialize(&self, server: &MockMcpServer) -> Result<(), String> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05"
            }
        });
        
        match server.handle_initialize(&request) {
            Ok(response) => {
                if response.get("protocolVersion").is_none() {
                    return Err("Missing protocolVersion in response".to_string());
                }
                if response.get("serverInfo").is_none() {
                    return Err("Missing serverInfo in response".to_string());
                }
                Ok(())
            }
            Err(e) => Err(format!("Initialize failed: {}", e)),
        }
    }
    
    /// Test tools functionality
    fn test_tools_functionality(&self, server: &MockMcpServer) -> Result<(), String> {
        // Test tools/list
        let list_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        });
        
        let list_response = server.handle_tools_list(&list_request)
            .map_err(|e| format!("tools/list failed: {}", e))?;
        
        let tools = list_response.get("tools")
            .and_then(|t| t.as_array())
            .ok_or("Invalid tools/list response")?;
        
        // Test calling the first available tool if any
        if let Some(first_tool) = tools.first() {
            let tool_name = first_tool.get("name")
                .and_then(|n| n.as_str())
                .ok_or("Tool missing name")?;
            
            let call_request = json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {
                    "name": tool_name,
                    "arguments": {}
                }
            });
            
            server.handle_tools_call(&call_request)
                .map_err(|e| format!("tools/call failed for {}: {}", tool_name, e))?;
        }
        
        Ok(())
    }
    
    /// Check health of multiple servers
    pub fn check_multiple_servers(&self, servers: &[MockMcpServer]) -> Vec<McpServerHealth> {
        servers.iter()
            .map(|server| self.check_server_health(server))
            .collect()
    }
    
    /// Generate health report
    pub fn generate_health_report(&self, health_checks: &[McpServerHealth]) -> String {
        let mut report = String::new();
        report.push_str("MCP Server Health Report\n");
        report.push_str("========================\n\n");
        
        let total_servers = health_checks.len();
        let healthy_servers = health_checks.iter().filter(|h| h.is_healthy).count();
        let unhealthy_servers = total_servers - healthy_servers;
        
        report.push_str(&format!("Total Servers: {}\n", total_servers));
        report.push_str(&format!("Healthy: {} ({}%)\n", 
            healthy_servers, 
            (healthy_servers * 100) / total_servers.max(1)
        ));
        report.push_str(&format!("Unhealthy: {} ({}%)\n\n", 
            unhealthy_servers, 
            (unhealthy_servers * 100) / total_servers.max(1)
        ));
        
        for health in health_checks {
            let status = if health.is_healthy { "✅ HEALTHY" } else { "❌ UNHEALTHY" };
            
            report.push_str(&format!("Server: {} - {}\n", health.server_name, status));
            report.push_str(&format!("  Response Time: {}ms\n", health.response_time_ms));
            report.push_str(&format!("  Success Rate: {:.1}%\n", health.success_rate() * 100.0));
            report.push_str(&format!("  Success/Error Count: {}/{}\n", 
                health.success_count, health.error_count));
            
            if let Some(ref error) = health.last_error {
                report.push_str(&format!("  Last Error: {}\n", error));
            }
            
            report.push_str("\n");
        }
        
        report
    }
}

#[cfg(test)]
mod health_check_tests {
    use super::*;
    
    #[test]
    fn test_healthy_server_check() {
        let checker = McpHealthChecker::new();
        let server = MockMcpServerFactory::filesystem_server();
        
        let health = checker.check_server_health(&server);
        
        assert!(health.is_healthy, "Server should be healthy");
        assert_eq!(health.server_name, "filesystem");
        assert!(health.response_time_ms > 0);
        assert!(health.success_count > 0);
        assert_eq!(health.error_count, 0);
        assert!(health.success_rate() > 0.0);
        
        println!("✅ Healthy server check passed");
    }
    
    #[test]
    fn test_failing_server_check() {
        let checker = McpHealthChecker::new();
        let server = MockMcpServer::new("failing-server").with_failure(true);
        
        let health = checker.check_server_health(&server);
        
        assert!(!health.is_healthy, "Server should be unhealthy");
        assert_eq!(health.server_name, "failing-server");
        assert!(health.error_count > 0);
        assert!(health.last_error.is_some());
        assert_eq!(health.success_rate(), 0.0);
        
        println!("✅ Failing server check passed");
    }
    
    #[test]
    fn test_slow_server_check() {
        let checker = McpHealthChecker::new()
            .with_timeout(Duration::from_secs(2));
        
        let server = MockMcpServer::new("slow-server")
            .with_delay(Duration::from_millis(300)); // Exceeds 200ms threshold
        
        let health = checker.check_server_health(&server);
        
        // Server responds but is marked unhealthy due to slow response
        assert!(!health.is_healthy, "Slow server should be marked unhealthy");
        assert!(health.response_time_ms >= 300);
        assert!(health.last_error.is_some());
        assert!(health.last_error.as_ref().unwrap().contains("Response time"));
        
        println!("✅ Slow server check passed");
    }
    
    #[test]
    fn test_multiple_server_health_check() {
        let checker = McpHealthChecker::new();
        let servers = vec![
            MockMcpServerFactory::filesystem_server(),
            MockMcpServerFactory::git_server(),
            MockMcpServer::new("failing-server").with_failure(true),
        ];
        
        let health_results = checker.check_multiple_servers(&servers);
        
        assert_eq!(health_results.len(), 3);
        
        // First two should be healthy, third should fail
        assert!(health_results[0].is_healthy);
        assert!(health_results[1].is_healthy);
        assert!(!health_results[2].is_healthy);
        
        println!("✅ Multiple server health check passed");
    }
    
    #[test]
    fn test_health_report_generation() {
        let checker = McpHealthChecker::new();
        let servers = vec![
            MockMcpServerFactory::filesystem_server(),
            MockMcpServer::new("failing-server").with_failure(true),
        ];
        
        let health_results = checker.check_multiple_servers(&servers);
        let report = checker.generate_health_report(&health_results);
        
        assert!(report.contains("MCP Server Health Report"));
        assert!(report.contains("Total Servers: 2"));
        assert!(report.contains("filesystem"));
        assert!(report.contains("failing-server"));
        assert!(report.contains("✅ HEALTHY"));
        assert!(report.contains("❌ UNHEALTHY"));
        
        println!("Health Report:\n{}", report);
        println!("✅ Health report generation passed");
    }
    
    #[test]
    fn test_health_check_performance_requirements() {
        let checker = McpHealthChecker::new();
        let server = MockMcpServerFactory::filesystem_server();
        
        let start = Instant::now();
        let _health = checker.check_server_health(&server);
        let check_duration = start.elapsed();
        
        // Health check itself should be fast
        assert!(
            check_duration.as_millis() < 1000,
            "Health check took too long: {:?}",
            check_duration
        );
        
        println!("✅ Health check performance requirements met ({:?})", check_duration);
    }
    
    #[test]
    fn test_all_standard_mcp_servers_health() {
        let checker = McpHealthChecker::new();
        let servers = MockMcpServerFactory::all_servers();
        
        let health_results = checker.check_multiple_servers(&servers);
        let report = checker.generate_health_report(&health_results);
        
        // All mock servers should be healthy
        let all_healthy = health_results.iter().all(|h| h.is_healthy);
        assert!(all_healthy, "All standard MCP servers should be healthy");
        
        println!("Standard MCP Servers Health Report:\n{}", report);
        println!("✅ All {} standard MCP servers are healthy", servers.len());
    }
}
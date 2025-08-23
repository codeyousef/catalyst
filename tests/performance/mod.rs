/// Performance Testing Module for Catalyst IDE
/// 
/// Following strict performance requirements as defined in catalyst-ide.md:
/// - Startup: < 500ms cold start, < 200ms warm start
/// - Memory: < 40MB idle
/// - Binary Size: < 5MB  
/// - File Search: < 50ms for 100k files
/// - Git Status: < 100ms for 10k files
/// - Syntax Highlighting: < 10ms for 10MB files

use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use std::path::Path;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

pub mod startup;
pub mod memory;
pub mod file_search;
pub mod git_operations;
pub mod syntax_highlighting;

/// Performance thresholds as defined in requirements
pub const COLD_START_THRESHOLD_MS: u128 = 500;
pub const WARM_START_THRESHOLD_MS: u128 = 200;
pub const IDLE_MEMORY_THRESHOLD_MB: u64 = 40;
pub const BINARY_SIZE_THRESHOLD_MB: u64 = 5;
pub const FILE_SEARCH_THRESHOLD_MS: u128 = 50;
pub const GIT_STATUS_THRESHOLD_MS: u128 = 100;
pub const SYNTAX_HIGHLIGHT_THRESHOLD_MS: u128 = 10;

/// Helper function to measure process startup time
pub fn measure_startup_time(binary_path: &str, args: &[&str]) -> Result<Duration, std::io::Error> {
    let start = Instant::now();
    
    let mut child = Command::new(binary_path)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    
    // Wait a moment for the process to fully initialize
    std::thread::sleep(Duration::from_millis(100));
    
    let startup_time = start.elapsed();
    
    // Clean up the process
    let _ = child.kill();
    let _ = child.wait();
    
    Ok(startup_time)
}

/// Helper function to get memory usage of a process
#[cfg(target_os = "linux")]
pub fn get_process_memory_usage(pid: u32) -> Result<u64, Box<dyn std::error::Error>> {
    use std::fs;
    
    let status_path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(status_path)?;
    
    for line in content.lines() {
        if line.starts_with("VmRSS:") {
            let kb_str = line.split_whitespace().nth(1).unwrap_or("0");
            let kb: u64 = kb_str.parse().unwrap_or(0);
            return Ok(kb / 1024); // Convert KB to MB
        }
    }
    
    Err("Could not find VmRSS in /proc/*/status".into())
}

#[cfg(not(target_os = "linux"))]
pub fn get_process_memory_usage(_pid: u32) -> Result<u64, Box<dyn std::error::Error>> {
    // For non-Linux platforms, return a placeholder
    // TODO: Implement for Windows and macOS
    Ok(0)
}

/// Helper function to get binary size
pub fn get_binary_size(binary_path: &str) -> Result<u64, std::io::Error> {
    use std::fs;
    let metadata = fs::metadata(binary_path)?;
    Ok(metadata.len() / (1024 * 1024)) // Convert bytes to MB
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_startup_performance_measurement() {
        // This is a failing test that will pass once we implement proper startup optimization
        let binary_path = "target/debug/catalyst";
        
        if Path::new(binary_path).exists() {
            let startup_time = measure_startup_time(binary_path, &["--version"])
                .expect("Failed to measure startup time");
            
            println!("Current startup time: {:?}", startup_time);
            
            // This test will initially fail - that's expected for TDD
            assert!(
                startup_time.as_millis() < COLD_START_THRESHOLD_MS,
                "Cold start time {} ms exceeds threshold {} ms",
                startup_time.as_millis(),
                COLD_START_THRESHOLD_MS
            );
        } else {
            // Skip test if binary doesn't exist yet
            println!("Binary not found, skipping startup test");
        }
    }
    
    #[test]
    fn test_binary_size_requirement() {
        let binary_path = "target/release-lto/catalyst";
        
        if Path::new(binary_path).exists() {
            let size_mb = get_binary_size(binary_path)
                .expect("Failed to get binary size");
            
            println!("Current binary size: {} MB", size_mb);
            
            assert!(
                size_mb < BINARY_SIZE_THRESHOLD_MB,
                "Binary size {} MB exceeds threshold {} MB",
                size_mb,
                BINARY_SIZE_THRESHOLD_MB
            );
        } else {
            println!("Release binary not found, skipping size test");
        }
    }
}
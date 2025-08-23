/// Memory Usage Performance Tests
/// 
/// Tests the critical memory performance requirement:
/// - Idle memory usage: < 40MB (leveraging Rust's efficiency)

use super::*;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod memory_tests {
    use super::*;
    
    #[test]
    fn test_idle_memory_usage() {
        let binary_path = "target/debug/catalyst";
        
        if !Path::new(binary_path).exists() {
            println!("Binary not found, skipping memory test");
            return;
        }
        
        // Start catalyst in background
        let mut child = Command::new(binary_path)
            .args(&["--no-gui"]) // Assume we'll add a headless mode flag
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start catalyst");
        
        let pid = child.id();
        
        // Let it settle into idle state
        thread::sleep(Duration::from_secs(5));
        
        // Measure memory usage
        let memory_mb = get_process_memory_usage(pid)
            .unwrap_or_else(|_| {
                println!("Warning: Could not measure memory usage on this platform");
                0 // Skip test on unsupported platforms
            });
        
        // Clean up
        let _ = child.kill();
        let _ = child.wait();
        
        if memory_mb == 0 {
            println!("Memory measurement not available on this platform");
            return;
        }
        
        println!("Idle memory usage: {} MB", memory_mb);
        
        // This test will initially fail - implementing TDD
        assert!(
            memory_mb < IDLE_MEMORY_THRESHOLD_MB,
            "Idle memory usage {} MB exceeds threshold {} MB. Need to optimize memory usage!",
            memory_mb,
            IDLE_MEMORY_THRESHOLD_MB
        );
    }
    
    #[test]
    fn test_memory_usage_with_large_file() {
        use tempfile::NamedTempFile;
        use std::io::Write;
        
        let binary_path = "target/debug/catalyst";
        
        if !Path::new(binary_path).exists() {
            println!("Binary not found, skipping large file memory test");
            return;
        }
        
        // Create a large test file (1MB)
        let mut temp_file = NamedTempFile::new()
            .expect("Failed to create temp file");
        
        let large_content = "A".repeat(1024 * 1024); // 1MB of 'A's
        temp_file.write_all(large_content.as_bytes())
            .expect("Failed to write large content");
        
        // Start catalyst with the large file
        let mut child = Command::new(binary_path)
            .arg(temp_file.path())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start catalyst with large file");
        
        let pid = child.id();
        
        // Let it load and settle
        thread::sleep(Duration::from_secs(3));
        
        let memory_mb = get_process_memory_usage(pid)
            .unwrap_or(0);
        
        // Clean up
        let _ = child.kill();
        let _ = child.wait();
        
        if memory_mb == 0 {
            println!("Memory measurement not available on this platform");
            return;
        }
        
        println!("Memory usage with 1MB file: {} MB", memory_mb);
        
        // Should still be reasonable with large files
        // Allow 2x the idle threshold for large file handling
        let large_file_threshold = IDLE_MEMORY_THRESHOLD_MB * 2;
        
        assert!(
            memory_mb < large_file_threshold,
            "Memory usage with large file {} MB exceeds threshold {} MB",
            memory_mb,
            large_file_threshold
        );
    }
    
    #[test]
    fn test_memory_leak_detection() {
        let binary_path = "target/debug/catalyst";
        
        if !Path::new(binary_path).exists() {
            println!("Binary not found, skipping memory leak test");
            return;
        }
        
        // Start catalyst
        let mut child = Command::new(binary_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start catalyst");
        
        let pid = child.id();
        
        // Initial memory measurement
        thread::sleep(Duration::from_secs(2));
        let initial_memory = get_process_memory_usage(pid).unwrap_or(0);
        
        if initial_memory == 0 {
            println!("Memory measurement not available on this platform");
            let _ = child.kill();
            let _ = child.wait();
            return;
        }
        
        // Simulate some activity and wait
        thread::sleep(Duration::from_secs(5));
        
        // Final memory measurement
        let final_memory = get_process_memory_usage(pid).unwrap_or(0);
        
        // Clean up
        let _ = child.kill();
        let _ = child.wait();
        
        println!("Initial memory: {} MB, Final memory: {} MB", 
                initial_memory, final_memory);
        
        // Memory should not increase significantly during idle
        let memory_growth = final_memory.saturating_sub(initial_memory);
        let max_growth_mb = 5; // Allow 5MB growth during idle
        
        assert!(
            memory_growth < max_growth_mb,
            "Memory grew by {} MB during idle, indicating possible leak. Max allowed: {} MB",
            memory_growth,
            max_growth_mb
        );
    }
    
    #[cfg(target_os = "linux")]
    #[test]
    fn test_memory_measurement_utility() {
        // Test our memory measurement utility works
        let current_pid = std::process::id();
        let memory = get_process_memory_usage(current_pid);
        
        assert!(memory.is_ok(), "Memory measurement should work on Linux");
        
        let memory_mb = memory.unwrap();
        println!("Current test process memory: {} MB", memory_mb);
        
        // Sanity check - test process should use some memory but not too much
        assert!(memory_mb > 0, "Memory usage should be greater than 0");
        assert!(memory_mb < 100, "Test process should not use excessive memory");
    }
}
/// Startup Performance Tests
/// 
/// Tests the critical startup performance requirements:
/// - Cold start: < 500ms
/// - Warm start: < 200ms

use super::*;
use std::time::Instant;

#[cfg(test)]
mod startup_tests {
    use super::*;
    
    #[test]
    fn test_cold_start_performance() {
        let binary_path = "target/debug/catalyst";
        
        if !Path::new(binary_path).exists() {
            // Build first if binary doesn't exist
            let output = Command::new("cargo")
                .args(&["build", "--bin", "catalyst"])
                .output()
                .expect("Failed to build catalyst");
                
            if !output.status.success() {
                panic!("Failed to build catalyst: {}", 
                       String::from_utf8_lossy(&output.stderr));
            }
        }
        
        // Cold start test - first launch after build
        let start = Instant::now();
        let mut child = Command::new(binary_path)
            .args(&["--help"])  // Quick command that exits immediately
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start catalyst");
        
        // Wait for process to complete
        let status = child.wait().expect("Failed to wait for catalyst");
        let cold_start_time = start.elapsed();
        
        println!("Cold start time: {:?}", cold_start_time);
        
        // This test will initially fail - implementing TDD
        assert!(
            cold_start_time.as_millis() < COLD_START_THRESHOLD_MS,
            "Cold start time {} ms exceeds threshold {} ms. Need to optimize startup!",
            cold_start_time.as_millis(),
            COLD_START_THRESHOLD_MS
        );
    }
    
    #[test]
    fn test_warm_start_performance() {
        let binary_path = "target/debug/catalyst";
        
        if !Path::new(binary_path).exists() {
            println!("Binary not found, skipping warm start test");
            return;
        }
        
        // First run to "warm up" the system
        let _ = Command::new(binary_path)
            .args(&["--version"])
            .output();
        
        // Now measure the warm start
        let start = Instant::now();
        let mut child = Command::new(binary_path)
            .args(&["--version"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start catalyst");
        
        let status = child.wait().expect("Failed to wait for catalyst");
        let warm_start_time = start.elapsed();
        
        println!("Warm start time: {:?}", warm_start_time);
        
        // This test will initially fail - implementing TDD
        assert!(
            warm_start_time.as_millis() < WARM_START_THRESHOLD_MS,
            "Warm start time {} ms exceeds threshold {} ms. Need to optimize warm startup!",
            warm_start_time.as_millis(),
            WARM_START_THRESHOLD_MS
        );
    }
    
    #[test]
    fn test_startup_with_empty_project() {
        use tempfile::TempDir;
        
        let binary_path = "target/debug/catalyst";
        if !Path::new(binary_path).exists() {
            println!("Binary not found, skipping project startup test");
            return;
        }
        
        // Create temporary empty project
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        
        let start = Instant::now();
        let mut child = Command::new(binary_path)
            .arg(temp_dir.path())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start catalyst with project");
        
        // Let it initialize for a moment
        std::thread::sleep(Duration::from_millis(200));
        
        let _ = child.kill();
        let _ = child.wait();
        
        let project_start_time = start.elapsed();
        
        println!("Project startup time: {:?}", project_start_time);
        
        // Project startup should be under cold start threshold
        assert!(
            project_start_time.as_millis() < COLD_START_THRESHOLD_MS,
            "Project startup time {} ms exceeds cold start threshold {} ms",
            project_start_time.as_millis(),
            COLD_START_THRESHOLD_MS
        );
    }
}
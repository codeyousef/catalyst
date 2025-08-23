/// Git Operations Performance Tests
/// 
/// Tests the critical git performance requirement:
/// - Git Status: < 100ms for 10k files

use super::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;
use std::time::Instant;

#[cfg(test)]
mod git_operations_tests {
    use super::*;
    
    #[test]
    fn test_git_status_performance_10k_files() {
        // Create a temporary git repository with 10k files
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo_path = temp_dir.path();
        
        // Initialize git repository
        let init_result = Command::new("git")
            .args(&["init"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to initialize git repository");
            
        if !init_result.status.success() {
            println!("Git not available, skipping git performance test");
            return;
        }
        
        // Configure git user for the test repo
        Command::new("git")
            .args(&["config", "user.name", "Catalyst Test"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git user name");
            
        Command::new("git")
            .args(&["config", "user.email", "test@catalyst-ide.dev"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git user email");
        
        println!("Creating 10,000 files for git status test...");
        let creation_start = Instant::now();
        
        // Create 10k files in various directories
        for i in 0..10_000 {
            let dir_path = repo_path.join(format!("src/module_{}", i / 100));
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path).expect("Failed to create directory");
            }
            
            let file_path = dir_path.join(format!("file_{}.rs", i));
            fs::write(&file_path, format!("// File {} content\npub fn function_{}() {{}}", i, i))
                .expect("Failed to write test file");
        }
        
        let creation_time = creation_start.elapsed();
        println!("Created 10k files in {:?}", creation_time);
        
        // Add files to git
        let add_start = Instant::now();
        let add_result = Command::new("git")
            .args(&["add", "."])
            .current_dir(repo_path)
            .output()
            .expect("Failed to add files to git");
            
        if !add_result.status.success() {
            println!("Failed to add files to git: {}", String::from_utf8_lossy(&add_result.stderr));
            return;
        }
        
        let add_time = add_start.elapsed();
        println!("Added files to git in {:?}", add_time);
        
        // Initial commit
        Command::new("git")
            .args(&["commit", "-m", "Initial commit with 10k files"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to make initial commit");
        
        // Modify some files to create changes
        for i in 0..1000 {
            let dir_path = repo_path.join(format!("src/module_{}", i / 100));
            let file_path = dir_path.join(format!("file_{}.rs", i));
            
            fs::write(&file_path, format!("// Modified file {} content\npub fn function_{}() {{ println!(\"modified\"); }}", i, i))
                .expect("Failed to modify test file");
        }
        
        // Test git status performance
        let status_start = Instant::now();
        
        let status_result = Command::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to run git status");
            
        let git_status_time = status_start.elapsed();
        
        let modified_files = String::from_utf8_lossy(&status_result.stdout)
            .lines()
            .count();
            
        println!("Git status found {} modified files in {:?}", modified_files, git_status_time);
        
        // This test will initially fail - implementing TDD
        assert!(
            git_status_time.as_millis() < GIT_STATUS_THRESHOLD_MS,
            "Git status time {} ms exceeds threshold {} ms for repository with 10k files. Need to optimize git operations!",
            git_status_time.as_millis(),
            GIT_STATUS_THRESHOLD_MS
        );
    }
    
    #[test]
    fn test_git_diff_performance() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo_path = temp_dir.path();
        
        // Initialize git repository
        let init_result = Command::new("git")
            .args(&["init"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to initialize git repository");
            
        if !init_result.status.success() {
            println!("Git not available, skipping git diff test");
            return;
        }
        
        // Configure git user
        Command::new("git")
            .args(&["config", "user.name", "Catalyst Test"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git user name");
            
        Command::new("git")
            .args(&["config", "user.email", "test@catalyst-ide.dev"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git user email");
        
        // Create and commit a large file
        let large_file = repo_path.join("large_file.rs");
        let large_content: String = (0..10000)
            .map(|i| format!("fn function_{}() {{ /* implementation */ }}\n", i))
            .collect();
            
        fs::write(&large_file, &large_content).expect("Failed to write large file");
        
        Command::new("git")
            .args(&["add", "large_file.rs"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to add large file");
            
        Command::new("git")
            .args(&["commit", "-m", "Add large file"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to commit large file");
        
        // Modify the large file
        let modified_content = large_content.replace("/* implementation */", "println!(\"modified\");");
        fs::write(&large_file, &modified_content).expect("Failed to modify large file");
        
        // Test git diff performance
        let diff_start = Instant::now();
        
        let diff_result = Command::new("git")
            .args(&["diff"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to run git diff");
            
        let git_diff_time = diff_start.elapsed();
        
        let diff_lines = String::from_utf8_lossy(&diff_result.stdout)
            .lines()
            .count();
            
        println!("Git diff processed {} lines in {:?}", diff_lines, git_diff_time);
        
        // Git diff should be reasonably fast
        let diff_threshold_ms = 200; // More generous threshold for diff operations
        
        assert!(
            git_diff_time.as_millis() < diff_threshold_ms,
            "Git diff time {} ms exceeds threshold {} ms. Need to optimize git diff operations!",
            git_diff_time.as_millis(),
            diff_threshold_ms
        );
    }
    
    #[test]
    fn test_git_log_performance() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo_path = temp_dir.path();
        
        // Initialize git repository
        let init_result = Command::new("git")
            .args(&["init"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to initialize git repository");
            
        if !init_result.status.success() {
            println!("Git not available, skipping git log test");
            return;
        }
        
        // Configure git user
        Command::new("git")
            .args(&["config", "user.name", "Catalyst Test"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git user name");
            
        Command::new("git")
            .args(&["config", "user.email", "test@catalyst-ide.dev"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to set git user email");
        
        // Create multiple commits to test log performance
        for i in 0..100 {
            let file_path = repo_path.join(format!("commit_{}.txt", i));
            fs::write(&file_path, format!("Content for commit {}", i))
                .expect("Failed to write commit file");
                
            Command::new("git")
                .args(&["add", &format!("commit_{}.txt", i)])
                .current_dir(repo_path)
                .output()
                .expect("Failed to add commit file");
                
            Command::new("git")
                .args(&["commit", "-m", &format!("Commit {}: Add file {}", i, i)])
                .current_dir(repo_path)
                .output()
                .expect("Failed to make commit");
        }
        
        // Test git log performance
        let log_start = Instant::now();
        
        let log_result = Command::new("git")
            .args(&["log", "--oneline", "-n", "50"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to run git log");
            
        let git_log_time = log_start.elapsed();
        
        let log_entries = String::from_utf8_lossy(&log_result.stdout)
            .lines()
            .count();
            
        println!("Git log retrieved {} entries in {:?}", log_entries, git_log_time);
        
        // Git log should be very fast
        let log_threshold_ms = 50; // Fast threshold for log operations
        
        assert!(
            git_log_time.as_millis() < log_threshold_ms,
            "Git log time {} ms exceeds threshold {} ms. Need to optimize git log operations!",
            git_log_time.as_millis(),
            log_threshold_ms
        );
    }
}
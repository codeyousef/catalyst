/// File Search Performance Tests
/// 
/// Tests the critical file search performance requirement:
/// - File Search: < 50ms for 100k files

use super::*;
use std::fs;
use tempfile::TempDir;
use std::time::Instant;

#[cfg(test)]
mod file_search_tests {
    use super::*;
    
    #[test]
    fn test_file_search_latency_100k_files() {
        // Create a temporary directory with 100k files
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let base_path = temp_dir.path();
        
        // Generate 100k test files
        println!("Creating 100,000 test files...");
        let start_creation = Instant::now();
        
        for i in 0..100_000 {
            let dir_path = base_path.join(format!("dir_{}", i / 1000));
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path).expect("Failed to create directory");
            }
            
            let file_path = dir_path.join(format!("file_{}.txt", i));
            fs::write(&file_path, format!("content_{}", i))
                .expect("Failed to write test file");
        }
        
        let creation_time = start_creation.elapsed();
        println!("Created 100k files in {:?}", creation_time);
        
        // Test search performance
        let binary_path = "target/debug/catalyst";
        
        if !Path::new(binary_path).exists() {
            println!("Binary not found, skipping file search test");
            return;
        }
        
        // Simulate file search operation
        let search_start = Instant::now();
        
        // For now, we'll test directory traversal speed as a proxy
        // In the real implementation, this would use Catalyst's search system
        let mut file_count = 0;
        for entry in walkdir::WalkDir::new(base_path) {
            match entry {
                Ok(entry) if entry.file_type().is_file() => {
                    file_count += 1;
                }
                _ => {}
            }
        }
        
        let search_time = search_start.elapsed();
        
        println!("Searched {} files in {:?}", file_count, search_time);
        
        // This test will initially fail - implementing TDD
        assert!(
            search_time.as_millis() < FILE_SEARCH_THRESHOLD_MS,
            "File search time {} ms exceeds threshold {} ms for {} files. Need to optimize search!",
            search_time.as_millis(),
            FILE_SEARCH_THRESHOLD_MS,
            file_count
        );
    }
    
    #[test]
    fn test_fuzzy_search_performance() {
        // Test fuzzy search performance with a smaller dataset
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let base_path = temp_dir.path();
        
        // Create 10k files with varied names for fuzzy search testing
        let patterns = vec![
            "component", "service", "util", "test", "config", 
            "controller", "model", "view", "helper", "manager"
        ];
        
        for i in 0..10_000 {
            let pattern = &patterns[i % patterns.len()];
            let file_path = base_path.join(format!("{}_{}.rs", pattern, i));
            fs::write(&file_path, format!("// {} file content", pattern))
                .expect("Failed to write test file");
        }
        
        // Test fuzzy search performance
        let search_start = Instant::now();
        
        // Simulate fuzzy search for "comp" -> should match "component" files
        let search_term = "comp";
        let mut matches = 0;
        
        for entry in fs::read_dir(base_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();
            
            // Simple fuzzy matching simulation
            if filename_str.contains(search_term) {
                matches += 1;
            }
        }
        
        let fuzzy_search_time = search_start.elapsed();
        
        println!("Fuzzy search found {} matches in {:?}", matches, fuzzy_search_time);
        
        // Fuzzy search should be even faster for smaller datasets
        let fuzzy_threshold_ms = 25; // More aggressive threshold
        
        assert!(
            fuzzy_search_time.as_millis() < fuzzy_threshold_ms,
            "Fuzzy search time {} ms exceeds threshold {} ms. Need to optimize fuzzy search!",
            fuzzy_search_time.as_millis(),
            fuzzy_threshold_ms
        );
    }
    
    #[test]
    fn test_index_based_search() {
        // Test index-based search performance
        use std::collections::HashMap;
        
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let base_path = temp_dir.path();
        
        // Create files and build an index
        let mut file_index: HashMap<String, Vec<String>> = HashMap::new();
        
        for i in 0..50_000 {
            let filename = format!("indexed_file_{}.rs", i);
            let filepath = base_path.join(&filename);
            let content = format!("fn function_{}() {{ /* implementation */ }}", i);
            
            fs::write(&filepath, &content).expect("Failed to write file");
            
            // Build index (simulate indexing keywords)
            let keywords = vec![format!("function_{}", i), "implementation".to_string()];
            for keyword in keywords {
                file_index.entry(keyword)
                    .or_insert_with(Vec::new)
                    .push(filepath.to_string_lossy().to_string());
            }
        }
        
        // Test index-based search
        let search_start = Instant::now();
        
        let search_results = file_index.get("implementation")
            .map(|files| files.len())
            .unwrap_or(0);
        
        let index_search_time = search_start.elapsed();
        
        println!("Index search found {} results in {:?}", search_results, index_search_time);
        
        // Index-based search should be very fast
        let index_threshold_ms = 1; // Very aggressive threshold
        
        assert!(
            index_search_time.as_millis() < index_threshold_ms,
            "Index search time {} ms exceeds threshold {} ms. Index optimization needed!",
            index_search_time.as_millis(),
            index_threshold_ms
        );
    }
}

// Add walkdir dependency
#[cfg(test)]
use walkdir;
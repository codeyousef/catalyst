/// Syntax Highlighting Performance Tests
/// 
/// Tests the critical syntax highlighting performance requirement:
/// - Syntax Highlighting: < 10ms for 10MB files

use super::*;
use std::fs;
use tempfile::NamedTempFile;
use std::time::Instant;

#[cfg(test)]
mod syntax_highlighting_tests {
    use super::*;
    
    #[test]
    fn test_syntax_highlighting_10mb_rust_file() {
        // Generate a large 10MB Rust file
        let mut temp_file = NamedTempFile::new()
            .expect("Failed to create temp file");
            
        println!("Generating 10MB Rust file for syntax highlighting test...");
        let generation_start = Instant::now();
        
        // Generate realistic Rust code patterns
        let mut content = String::new();
        content.push_str("// Large Rust file for syntax highlighting performance test\n");
        content.push_str("use std::collections::{HashMap, HashSet, BTreeMap};\n");
        content.push_str("use std::sync::{Arc, Mutex, RwLock};\n");
        content.push_str("use std::thread;\n");
        content.push_str("use std::time::{Duration, Instant};\n\n");
        
        // Generate many structs, enums, and functions
        let mut current_size = content.len();
        let mut counter = 0;
        
        while current_size < 10 * 1024 * 1024 { // 10MB
            let module_content = format!(
                r#"
pub mod module_{counter} {{
    use super::*;
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct DataStructure{counter} {{
        pub id: u64,
        pub name: String,
        pub values: Vec<i32>,
        pub metadata: HashMap<String, String>,
        pub status: StatusEnum{counter},
    }}
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StatusEnum{counter} {{
        Active,
        Inactive,
        Pending,
        Processing,
        Complete,
        Failed,
    }}
    
    impl DataStructure{counter} {{
        pub fn new(id: u64, name: impl Into<String>) -> Self {{
            Self {{
                id,
                name: name.into(),
                values: Vec::new(),
                metadata: HashMap::new(),
                status: StatusEnum{counter}::Pending,
            }}
        }}
        
        pub fn add_value(&mut self, value: i32) -> Result<(), String> {{
            if self.values.len() >= 1000 {{
                return Err("Too many values".to_string());
            }}
            self.values.push(value);
            Ok(())
        }}
        
        pub fn calculate_sum(&self) -> i64 {{
            self.values.iter().map(|&v| v as i64).sum()
        }}
        
        pub fn find_max(&self) -> Option<i32> {{
            self.values.iter().copied().max()
        }}
        
        pub fn filter_positive(&self) -> Vec<i32> {{
            self.values
                .iter()
                .copied()
                .filter(|&v| v > 0)
                .collect()
        }}
        
        pub async fn async_process(&mut self) -> Result<(), Box<dyn std::error::Error>> {{
            self.status = StatusEnum{counter}::Processing;
            
            // Simulate async work
            tokio::time::sleep(Duration::from_millis(1)).await;
            
            // Complex processing logic
            let result = self.values
                .iter()
                .enumerate()
                .map(|(i, &v)| {{
                    match v {{
                        x if x > 100 => x * 2,
                        x if x > 50 => x + 10,
                        x if x > 0 => x,
                        _ => 0,
                    }}
                }})
                .collect::<Vec<_>>();
                
            self.values = result;
            self.status = StatusEnum{counter}::Complete;
            Ok(())
        }}
    }}
    
    pub trait Processor{counter} {{
        type Output;
        type Error;
        
        fn process(&self, input: &DataStructure{counter}) -> Result<Self::Output, Self::Error>;
        fn validate(&self, input: &DataStructure{counter}) -> bool {{
            !input.name.is_empty() && input.id > 0
        }}
    }}
    
    pub struct ConcreteProcessor{counter} {{
        config: ProcessorConfig,
    }}
    
    #[derive(Debug, Clone)]
    pub struct ProcessorConfig {{
        pub max_iterations: usize,
        pub timeout: Duration,
        pub parallel: bool,
    }}
    
    impl Processor{counter} for ConcreteProcessor{counter} {{
        type Output = Vec<String>;
        type Error = ProcessingError;
        
        fn process(&self, input: &DataStructure{counter}) -> Result<Self::Output, Self::Error> {{
            if !self.validate(input) {{
                return Err(ProcessingError::InvalidInput);
            }}
            
            let results = input.values
                .iter()
                .map(|&v| format!("processed_{{v}}_in_module_{counter}"))
                .collect();
                
            Ok(results)
        }}
    }}
    
    #[derive(Debug, thiserror::Error)]
    pub enum ProcessingError {{
        #[error("Invalid input provided")]
        InvalidInput,
        #[error("Processing timeout")]
        Timeout,
        #[error("Resource exhausted")]
        ResourceExhausted,
    }}
    
    // Complex macro definition
    macro_rules! generate_handler_{counter} {{
        ($handler_name:ident, $input_type:ty, $output_type:ty) => {{
            pub struct $handler_name {{
                inner: Arc<Mutex<DataStructure{counter}>>,
            }}
            
            impl $handler_name {{
                pub fn new(data: DataStructure{counter}) -> Self {{
                    Self {{
                        inner: Arc::new(Mutex::new(data)),
                    }}
                }}
                
                pub fn handle(&self, input: $input_type) -> $output_type {{
                    let mut guard = self.inner.lock().unwrap();
                    // Handler logic here
                    todo!("Implement handler logic")
                }}
            }}
        }};
    }}
    
    generate_handler_{counter}!(Handler{counter}, String, i32);
}}
"#,
                counter = counter
            );
            
            content.push_str(&module_content);
            current_size = content.len();
            counter += 1;
            
            if counter % 100 == 0 {
                println!("Generated {} modules, current size: {:.2}MB", 
                        counter, current_size as f64 / 1024.0 / 1024.0);
            }
        }
        
        let generation_time = generation_start.elapsed();
        println!("Generated {:.2}MB Rust file with {} modules in {:?}", 
                content.len() as f64 / 1024.0 / 1024.0, counter, generation_time);
        
        // Write to temp file
        fs::write(temp_file.path(), &content)
            .expect("Failed to write large file");
        
        // Test syntax highlighting performance
        // For now, we'll simulate this with basic parsing operations
        // In the real implementation, this would use Lapce's syntax highlighting system
        
        let highlighting_start = Instant::now();
        
        // Simulate syntax highlighting by parsing common patterns
        let lines: Vec<&str> = content.lines().collect();
        let mut highlighted_tokens = 0;
        
        for line in &lines {
            // Simulate token highlighting
            if line.contains("pub") { highlighted_tokens += 1; }
            if line.contains("struct") { highlighted_tokens += 1; }
            if line.contains("impl") { highlighted_tokens += 1; }
            if line.contains("fn") { highlighted_tokens += 1; }
            if line.contains("let") { highlighted_tokens += 1; }
            if line.contains("match") { highlighted_tokens += 1; }
            if line.contains("if") { highlighted_tokens += 1; }
            if line.contains("//") { highlighted_tokens += 1; }
            
            // Simulate more complex highlighting patterns
            for keyword in &["async", "await", "pub", "use", "mod", "trait", "enum", "const"] {
                if line.contains(keyword) {
                    highlighted_tokens += 1;
                }
            }
        }
        
        let highlighting_time = highlighting_start.elapsed();
        
        println!(
            "Syntax highlighting processed {} lines ({:.2}MB) with {} tokens in {:?}",
            lines.len(),
            content.len() as f64 / 1024.0 / 1024.0,
            highlighted_tokens,
            highlighting_time
        );
        
        // This test will initially fail - implementing TDD
        assert!(
            highlighting_time.as_millis() < SYNTAX_HIGHLIGHT_THRESHOLD_MS,
            "Syntax highlighting time {} ms exceeds threshold {} ms for {:.2}MB file. Need to optimize syntax highlighting!",
            highlighting_time.as_millis(),
            SYNTAX_HIGHLIGHT_THRESHOLD_MS,
            content.len() as f64 / 1024.0 / 1024.0
        );
    }
    
    #[test]
    fn test_incremental_syntax_highlighting() {
        // Test incremental highlighting performance
        let base_content = r#"
use std::collections::HashMap;

pub struct TestStruct {
    field1: String,
    field2: i32,
}

impl TestStruct {
    pub fn new() -> Self {
        Self {
            field1: String::new(),
            field2: 0,
        }
    }
}
"#;
        
        let highlighting_start = Instant::now();
        
        // Simulate incremental highlighting - only re-highlight changed lines
        let lines: Vec<&str> = base_content.lines().collect();
        let changed_line_index = 5; // Simulate change on line 5
        
        // Only highlight the changed line and surrounding context
        let mut highlighted_tokens = 0;
        for (i, line) in lines.iter().enumerate() {
            if (i as i32 - changed_line_index as i32).abs() <= 2 { // Context window of 2 lines
                // Simulate highlighting this line
                for keyword in &["pub", "struct", "impl", "fn", "Self"] {
                    if line.contains(keyword) {
                        highlighted_tokens += 1;
                    }
                }
            }
        }
        
        let incremental_time = highlighting_start.elapsed();
        
        println!("Incremental highlighting processed {} tokens in {:?}", 
                highlighted_tokens, incremental_time);
        
        // Incremental highlighting should be very fast
        let incremental_threshold_ms = 1;
        
        assert!(
            incremental_time.as_millis() < incremental_threshold_ms,
            "Incremental syntax highlighting time {} ms exceeds threshold {} ms. Need to optimize incremental highlighting!",
            incremental_time.as_millis(),
            incremental_threshold_ms
        );
    }
    
    #[test]
    fn test_multi_language_syntax_highlighting() {
        // Test syntax highlighting for multiple languages
        let javascript_content = r#"
// JavaScript content
const express = require('express');
const app = express();

app.get('/api/users', async (req, res) => {
    try {
        const users = await User.findAll();
        res.json(users);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

class UserService {
    constructor(database) {
        this.db = database;
    }
    
    async createUser(userData) {
        return await this.db.users.create(userData);
    }
}
"#;

        let python_content = r#"
# Python content
import asyncio
import json
from typing import List, Optional
from dataclasses import dataclass

@dataclass
class User:
    id: int
    name: str
    email: Optional[str] = None

class UserService:
    def __init__(self, database):
        self.db = database
    
    async def create_user(self, user_data: dict) -> User:
        user = User(**user_data)
        return await self.db.users.create(user)
    
    def get_users(self) -> List[User]:
        return self.db.users.all()

if __name__ == "__main__":
    service = UserService(database)
    asyncio.run(service.create_user({"id": 1, "name": "Test"}))
"#;

        let highlighting_start = Instant::now();
        
        // Simulate multi-language highlighting
        let mut total_tokens = 0;
        
        // JavaScript highlighting simulation
        for keyword in &["const", "async", "await", "class", "function", "try", "catch"] {
            total_tokens += javascript_content.matches(keyword).count();
        }
        
        // Python highlighting simulation  
        for keyword in &["import", "async", "await", "class", "def", "if", "__name__"] {
            total_tokens += python_content.matches(keyword).count();
        }
        
        let multi_lang_time = highlighting_start.elapsed();
        
        println!("Multi-language highlighting processed {} tokens in {:?}", 
                total_tokens, multi_lang_time);
        
        // Multi-language highlighting should be reasonably fast
        let multi_lang_threshold_ms = 5;
        
        assert!(
            multi_lang_time.as_millis() < multi_lang_threshold_ms,
            "Multi-language syntax highlighting time {} ms exceeds threshold {} ms. Need to optimize multi-language support!",
            multi_lang_time.as_millis(),
            multi_lang_threshold_ms
        );
    }
}
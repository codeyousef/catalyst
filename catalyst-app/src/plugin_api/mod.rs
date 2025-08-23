//! Plugin API Module
//!
//! This module contains the plugin interfaces and extension points for Catalyst IDE.
//! It allows for modular functionality to be added without modifying core editor code.

pub mod ai_assistant;
pub mod manager;
pub mod mcp_server;
pub mod sidebar;

pub use ai_assistant::*;
pub use manager::*;
pub use mcp_server::*;
pub use sidebar::*;

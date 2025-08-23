//! Plugin Manager
//!
//! This module manages the loading and lifecycle of all plugins in Catalyst IDE.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::plugin_api::{
    AiAssistantPlugin, McpServerRegistry, SidebarPanelRegistry,
};

/// Main plugin manager for Catalyst IDE
pub struct PluginManager {
    ai_assistants: HashMap<String, Arc<dyn AiAssistantPlugin>>,
    sidebar_registry: SidebarPanelRegistry,
    mcp_registry: McpServerRegistry,
    config: PluginConfig,
}

/// Configuration for the plugin system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled_plugins: Vec<String>,
    pub plugin_directories: Vec<String>,
    pub auto_load_plugins: bool,
    pub max_plugins: usize,
    pub plugin_timeout_seconds: u64,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled_plugins: Vec::new(),
            plugin_directories: vec![
                "plugins".to_string(),
                "~/.catalyst/plugins".to_string(),
            ],
            auto_load_plugins: true,
            max_plugins: 50,
            plugin_timeout_seconds: 30,
        }
    }
}

/// Information about a loaded plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub plugin_type: PluginType,
    pub enabled: bool,
    pub loaded: bool,
}

/// Type of plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginType {
    AiAssistant,
    SidebarPanel,
    McpServer,
    Extension,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new(config: PluginConfig) -> Self {
        Self {
            ai_assistants: HashMap::new(),
            sidebar_registry: SidebarPanelRegistry::new(),
            mcp_registry: McpServerRegistry::new(),
            config,
        }
    }

    /// Initialize the plugin manager
    pub fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing plugin manager");

        if self.config.auto_load_plugins {
            self.load_all_plugins()?;
        }

        // Start auto-start MCP servers
        self.mcp_registry.start_auto_start_servers()?;

        tracing::info!("Plugin manager initialized successfully");
        Ok(())
    }

    /// Load all plugins from configured directories
    pub fn load_all_plugins(&mut self) -> Result<()> {
        let directories = self.config.plugin_directories.clone();
        for directory in directories {
            self.load_plugins_from_directory(&directory)?;
        }
        Ok(())
    }

    /// Load plugins from a specific directory
    pub fn load_plugins_from_directory(&mut self, directory: &str) -> Result<()> {
        tracing::info!("Loading plugins from directory: {}", directory);

        // This is a placeholder - in a real implementation, this would:
        // 1. Scan the directory for plugin files
        // 2. Load plugin metadata
        // 3. Initialize plugins based on their type
        // 4. Register them with appropriate registries

        // For now, we'll just log that we would load plugins
        tracing::info!(
            "Plugin loading from directory '{}' is not yet implemented",
            directory
        );

        Ok(())
    }

    /// Register an AI assistant plugin
    pub fn register_ai_assistant(
        &mut self,
        id: String,
        plugin: Arc<dyn AiAssistantPlugin>,
    ) -> Result<()> {
        if self.ai_assistants.contains_key(&id) {
            return Err(anyhow::anyhow!(
                "AI assistant with id '{}' is already registered",
                id
            ));
        }

        tracing::info!("Registering AI assistant plugin: {}", id);
        self.ai_assistants.insert(id, plugin);
        Ok(())
    }

    /// Get an AI assistant plugin by id
    pub fn get_ai_assistant(&self, id: &str) -> Option<Arc<dyn AiAssistantPlugin>> {
        self.ai_assistants.get(id).cloned()
    }

    /// Get all AI assistant plugin IDs
    pub fn get_ai_assistant_ids(&self) -> Vec<String> {
        self.ai_assistants.keys().cloned().collect()
    }

    /// Get sidebar panel registry
    pub fn get_sidebar_registry(&self) -> &SidebarPanelRegistry {
        &self.sidebar_registry
    }

    /// Get mutable sidebar panel registry
    pub fn get_sidebar_registry_mut(&mut self) -> &mut SidebarPanelRegistry {
        &mut self.sidebar_registry
    }

    /// Get MCP server registry
    pub fn get_mcp_registry(&self) -> &McpServerRegistry {
        &self.mcp_registry
    }

    /// Get mutable MCP server registry
    pub fn get_mcp_registry_mut(&mut self) -> &mut McpServerRegistry {
        &mut self.mcp_registry
    }

    /// Get information about all loaded plugins
    pub fn get_plugin_info(&self) -> Vec<PluginInfo> {
        let mut plugins = Vec::new();

        // Add AI assistant plugins
        for (id, plugin) in &self.ai_assistants {
            let info = plugin.plugin_info();
            plugins.push(PluginInfo {
                id: id.clone(),
                name: info.name,
                version: info.version,
                description: info.description,
                plugin_type: PluginType::AiAssistant,
                enabled: true,
                loaded: true,
            });
        }

        // Add sidebar panel plugins
        for panel_info in self.sidebar_registry.get_all_panel_info() {
            plugins.push(PluginInfo {
                id: panel_info.id.clone(),
                name: panel_info.name,
                version: "1.0.0".to_string(), // Panels don't have version info yet
                description: panel_info.description,
                plugin_type: PluginType::SidebarPanel,
                enabled: true,
                loaded: true,
            });
        }

        // Add MCP server plugins
        for server_info in self.mcp_registry.get_all_server_info() {
            plugins.push(PluginInfo {
                id: server_info.id.clone(),
                name: server_info.name,
                version: server_info.version,
                description: server_info.description,
                plugin_type: PluginType::McpServer,
                enabled: true,
                loaded: true,
            });
        }

        plugins
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<()> {
        tracing::info!("Enabling plugin: {}", plugin_id);

        // Add to enabled plugins list if not already there
        if !self.config.enabled_plugins.contains(&plugin_id.to_string()) {
            self.config.enabled_plugins.push(plugin_id.to_string());
        }

        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<()> {
        tracing::info!("Disabling plugin: {}", plugin_id);

        // Remove from enabled plugins list
        self.config.enabled_plugins.retain(|id| id != plugin_id);

        // TODO: Actually disable the plugin functionality

        Ok(())
    }

    /// Shutdown all plugins
    pub fn shutdown(&mut self) -> Result<()> {
        tracing::info!("Shutting down plugin manager");

        // Stop all MCP servers
        self.mcp_registry.stop_all_servers()?;

        // Clear all registries
        self.ai_assistants.clear();

        tracing::info!("Plugin manager shutdown complete");
        Ok(())
    }

    /// Get plugin configuration
    pub fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    /// Update plugin configuration
    pub fn update_config(&mut self, config: PluginConfig) {
        self.config = config;
    }
}

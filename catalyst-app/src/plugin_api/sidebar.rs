//! Sidebar Panel Plugin API
//!
//! This module defines the plugin interface for custom sidebar panels
//! that can be added to Catalyst IDE.

use anyhow::Result;
use floem::View;
use serde::{Deserialize, Serialize};

/// Trait that sidebar panel plugins must implement
pub trait SidebarPanelPlugin: Send + Sync + 'static {
    /// Initialize the sidebar panel plugin
    fn initialize(&mut self) -> Result<()>;

    /// Get panel information
    fn panel_info(&self) -> SidebarPanelInfo;

    /// Create the panel view
    fn create_view(&self) -> Box<dyn View>;

    /// Handle panel activation
    fn on_activate(&mut self) -> Result<()>;

    /// Handle panel deactivation
    fn on_deactivate(&mut self) -> Result<()>;

    /// Handle panel visibility change
    fn on_visibility_changed(&mut self, visible: bool) -> Result<()>;

    /// Get current panel state
    fn get_state(&self) -> serde_json::Value;

    /// Set panel state
    fn set_state(&mut self, state: serde_json::Value) -> Result<()>;

    /// Handle commands sent to this panel
    fn handle_command(
        &mut self,
        command: PanelCommand,
    ) -> Result<PanelCommandResult>;
}

/// Information about a sidebar panel plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarPanelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub position: SidebarPosition,
    pub default_visible: bool,
    pub resizable: bool,
    pub minimum_width: Option<u32>,
    pub maximum_width: Option<u32>,
}

/// Position where the sidebar panel should be placed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SidebarPosition {
    Left,
    Right,
    Bottom,
}

/// Command that can be sent to a panel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelCommand {
    pub command_id: String,
    pub parameters: serde_json::Value,
}

/// Result of executing a panel command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelCommandResult {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Registry for managing sidebar panels
pub struct SidebarPanelRegistry {
    panels: std::collections::HashMap<String, Box<dyn SidebarPanelPlugin>>,
}

impl SidebarPanelRegistry {
    /// Create a new panel registry
    pub fn new() -> Self {
        Self {
            panels: std::collections::HashMap::new(),
        }
    }

    /// Register a new sidebar panel
    pub fn register_panel(
        &mut self,
        id: String,
        panel: Box<dyn SidebarPanelPlugin>,
    ) -> Result<()> {
        if self.panels.contains_key(&id) {
            return Err(anyhow::anyhow!(
                "Panel with id '{}' is already registered",
                id
            ));
        }

        self.panels.insert(id, panel);
        Ok(())
    }

    /// Unregister a sidebar panel
    pub fn unregister_panel(&mut self, id: &str) -> Result<()> {
        self.panels.remove(id).ok_or_else(|| {
            anyhow::anyhow!("Panel with id '{}' is not registered", id)
        })?;
        Ok(())
    }

    /// Get a panel by id
    pub fn get_panel(&self, id: &str) -> Option<&dyn SidebarPanelPlugin> {
        self.panels.get(id).map(|panel| panel.as_ref())
    }

    /// Get a mutable panel by id
    pub fn get_panel_mut(
        &mut self,
        id: &str,
    ) -> Option<&mut dyn SidebarPanelPlugin> {
        self.panels.get_mut(id).map(|panel| panel.as_mut())
    }

    /// Get all registered panel IDs
    pub fn get_panel_ids(&self) -> Vec<String> {
        self.panels.keys().cloned().collect()
    }

    /// Get panel info for all registered panels
    pub fn get_all_panel_info(&self) -> Vec<SidebarPanelInfo> {
        self.panels
            .values()
            .map(|panel| panel.panel_info())
            .collect()
    }
}

impl Default for SidebarPanelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

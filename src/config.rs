/*!
Configuration Manager
Handles loading and saving application settings
*/

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub last_file: Option<String>,
    pub last_page: Option<usize>,
    pub last_zoom: Option<f32>,
    pub dark_mode: bool,
    pub window_width: f32,
    pub window_height: f32,
    pub current_tool: Option<String>,
    pub annotation_color: [u8; 4], // RGBA
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            last_file: None,
            last_page: None,
            last_zoom: Some(100.0),
            dark_mode: false,
            window_width: 1200.0,
            window_height: 800.0,
            current_tool: None,
            annotation_color: [255, 255, 0, 100], // Yellow with transparency
        }
    }
}

impl Settings {
    /// Get the config directory path
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("doclens");
        
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }
    
    /// Get the settings file path
    pub fn settings_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("settings.json"))
    }
    
    /// Load settings from file
    pub fn load() -> Result<Self> {
        let path = Self::settings_path()?;
        
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let settings: Settings = serde_json::from_str(&contents)?;
            Ok(settings)
        } else {
            Ok(Settings::default())
        }
    }
    
    /// Save settings to file
    pub fn save(&self) -> Result<()> {
        let path = Self::settings_path()?;
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

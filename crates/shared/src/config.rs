use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorConfig {
    pub default_fg: String,
    pub focused_fg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TidalConfig {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tidal: Option<TidalConfig>,
    pub lyrics_folder: String,
    pub colors: ColorConfig,
    pub allowed_players: Option<Vec<String>>,
    pub ignore_players: Option<Vec<String>>,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            focused_fg: "white".to_string(),
            default_fg: "gray".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;

        if !config_path.exists() {
            return Self::create_default_config(&config_path);
        }

        let config_str = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }

    fn create_default_config(config_path: &Path) -> Result<Self> {
        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let default_config = Config {
            tidal: None,
            lyrics_folder: dirs::home_dir()
                .map(|p| p.join("lyrics").to_string_lossy().to_string())
                .unwrap_or_else(|| String::from("./lyrics")),
            colors: ColorConfig::default(),
            allowed_players: None,
            ignore_players: None,
        };

        let config_str = serde_json::to_string_pretty(&default_config)?;
        fs::write(config_path, config_str)?;

        // Let the user know they need to edit the config
        println!("Created default config at: {}", config_path.display());

        Ok(default_config)
    }
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("bard");

    Ok(config_dir.join("config.json"))
}

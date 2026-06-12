use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VyzoConfig {
    pub project_name: String,
    pub strict_mode: bool,
    pub api_endpoint: Option<String>,
}

impl VyzoConfig {
    pub fn load() -> Self {
        let config_path = Path::new("vyzorix.toml");
        if config_path.exists() {
            if let Ok(contents) = fs::read_to_string(config_path) {
                if let Ok(config) = toml::from_str(&contents) {
                    return config;
                }
            }
        }
        
        // Fallback / default
        VyzoConfig {
            project_name: "vyzorix-workspace".to_string(),
            strict_mode: true,
            api_endpoint: None,
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let toml_string = toml::to_string(self).unwrap_or_default();
        fs::write("vyzorix.toml", toml_string)
    }
}

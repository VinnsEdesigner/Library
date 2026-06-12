use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VyzoState {
    pub session_token: Option<String>,
    pub last_deployment_id: Option<String>,
    pub known_workspaces: Vec<String>,
    pub machine_key: Option<String>,
}

impl VyzoState {
    fn global_path() -> std::path::PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".vyzorix")
            .join("state.json")
    }

    pub fn load() -> Self {
        let local_path = Path::new(".vyzo/state.json");
        let global_path = Self::global_path();

        let mut state = if local_path.exists() {
            fs::read_to_string(local_path)
                .ok()
                .and_then(|c| serde_json::from_str(&c).ok())
                .unwrap_or_default()
        } else {
            VyzoState::default()
        };

        // Overlay global values if local ones are missing
        if global_path.exists() {
            if let Ok(c) = fs::read_to_string(&global_path) {
                if let Ok(global_state) = serde_json::from_str::<VyzoState>(&c) {
                    if state.session_token.is_none() {
                        state.session_token = global_state.session_token;
                    }
                    if state.machine_key.is_none() {
                        state.machine_key = global_state.machine_key;
                    }
                }
            }
        }

        // Generate a machine key if it's missing everywhere
        if state.machine_key.is_none() {
            use rand::{RngCore, thread_rng};
            let mut key = [0u8; 32];
            thread_rng().fill_bytes(&mut key);
            state.machine_key = Some(hex::encode(key));
            let _ = state.save(); // Try to persist it immediately
        }

        state
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let global_path = Self::global_path();
        if let Some(parent) = global_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json_string = serde_json::to_string_pretty(self).unwrap_or_default();
        fs::write(global_path, &json_string)?;

        let state_dir = Path::new(".vyzo");
        if state_dir.exists() {
            fs::write(".vyzo/state.json", json_string)?;
        }
        Ok(())
    }
}

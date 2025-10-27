use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_theme")]
    pub theme: String,
    pub github_token: Option<String>,
    pub gitlab_token: Option<String>,
    #[serde(default = "default_repo_path")]
    pub repo_path: String,
    pub github_repo: Option<String>,
    pub gitlab_project_id: Option<String>,
    #[serde(default = "default_timer_minutes")]
    pub timer_default_minutes: u64,
}

fn default_theme() -> String {
    "moonlight".to_string()
}

fn default_repo_path() -> String {
    ".".to_string()
}

fn default_timer_minutes() -> u64 {
    25
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string("config.json") {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_else(|_| {
                    eprintln!("Warning: Invalid config.json, using defaults");
                    Config::default()
                })
            }
            Err(_) => Config::default(),
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("config.json", content)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: "moonlight".to_string(),
            github_token: None,
            gitlab_token: None,
            repo_path: ".".to_string(),
            github_repo: None,
            gitlab_project_id: None,
            timer_default_minutes: 25,
        }
    }
}

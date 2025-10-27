use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerSession {
    pub start: DateTime<Utc>,
    pub duration_min: u64,
    pub tag: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerData {
    pub sessions: Vec<TimerSession>,
}

impl TimerData {
    pub fn load() -> Self {
        if Path::new("timer_sessions.json").exists() {
            match fs::read_to_string("timer_sessions.json") {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => TimerData::default(),
            }
        } else {
            TimerData::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("timer_sessions.json", content)?;
        Ok(())
    }

    pub fn add_session(&mut self, session: TimerSession) -> anyhow::Result<()> {
        self.sessions.push(session);
        self.save()
    }

    pub fn total_today(&self) -> u64 {
        let today = Utc::now().date_naive();
        self.sessions
            .iter()
            .filter(|s| s.start.date_naive() == today && s.completed)
            .map(|s| s.duration_min)
            .sum()
    }
}

impl Default for TimerData {
    fn default() -> Self {
        TimerData { sessions: Vec::new() }
    }
}

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub input: String,
    pub intent: String,
    pub result: String,
    pub success: bool,
}

pub struct CommandHistory {
    file: PathBuf,
    entries: Vec<CommandEntry>,
}

impl CommandHistory {
    pub fn new() -> anyhow::Result<Self> {
        let file = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("No home directory found"))?
            .join(".draco")
            .join("command_history.json");
        
        let entries = if file.exists() {
            let content = fs::read_to_string(&file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        Ok(Self { file, entries })
    }

    pub fn add(&mut self, input: &str, intent: &str, result: &str, success: bool) -> anyhow::Result<()> {
        let entry = CommandEntry {
            timestamp: chrono::Utc::now(),
            input: input.to_string(),
            intent: intent.to_string(),
            result: result.to_string(),
            success,
        };
        
        self.entries.push(entry);
        self.save()?;
        Ok(())
    }

    pub fn recent(&self, count: usize) -> &[CommandEntry] {
        let start = if self.entries.len() > count { self.entries.len() - count } else { 0 };
        &self.entries[start..]
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.entries.clear();
        self.save()
    }

    fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.file.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.entries)?;
        fs::write(&self.file, json)?;
        Ok(())
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self {
            file: dirs::home_dir().unwrap_or_default().join(".draco").join("command_history.json"),
            entries: Vec::new(),
        }
    }
}

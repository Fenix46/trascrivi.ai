use anyhow::{Result, anyhow};
use serde_json;
use std::fs;
use std::path::PathBuf;
use crate::models::{Transcription, AppState};
use std::collections::HashMap;

#[derive(Clone)]
pub struct StorageService {
    data_dir: PathBuf,
}

impl StorageService {
    pub fn new() -> Result<Self> {
        let data_dir = Self::get_app_data_dir()?;
        fs::create_dir_all(&data_dir)?;

        Ok(Self { data_dir })
    }

    fn get_app_data_dir() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not find config directory"))?;
        path.push("trascrivi-ai");
        Ok(path)
    }

    pub async fn save_transcription(&self, transcription: &Transcription) -> Result<()> {
        let file_path = self.data_dir.join(format!("{}.json", transcription.id));
        let json_data = serde_json::to_string_pretty(transcription)?;

        tokio::fs::write(file_path, json_data).await?;
        Ok(())
    }

    pub async fn load_transcription(&self, id: &str) -> Result<Transcription> {
        let file_path = self.data_dir.join(format!("{}.json", id));
        let json_data = tokio::fs::read_to_string(file_path).await?;
        let transcription: Transcription = serde_json::from_str(&json_data)?;
        Ok(transcription)
    }

    pub async fn load_all_transcriptions(&self) -> Result<HashMap<String, Transcription>> {
        let mut transcriptions = HashMap::new();

        if !self.data_dir.exists() {
            return Ok(transcriptions);
        }

        let mut entries = tokio::fs::read_dir(&self.data_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json_data) = tokio::fs::read_to_string(&path).await {
                    if let Ok(transcription) = serde_json::from_str::<Transcription>(&json_data) {
                        transcriptions.insert(transcription.id.clone(), transcription);
                    }
                }
            }
        }

        Ok(transcriptions)
    }

    pub async fn delete_transcription(&self, id: &str) -> Result<()> {
        let file_path = self.data_dir.join(format!("{}.json", id));
        if file_path.exists() {
            tokio::fs::remove_file(file_path).await?;
        }
        Ok(())
    }

    pub async fn save_app_state(&self, state: &AppState) -> Result<()> {
        let file_path = self.data_dir.join("app_state.json");
        let json_data = serde_json::to_string_pretty(state)?;
        tokio::fs::write(file_path, json_data).await?;
        Ok(())
    }

    pub async fn load_app_state(&self) -> Result<AppState> {
        let file_path = self.data_dir.join("app_state.json");

        if !file_path.exists() {
            return Ok(AppState::default());
        }

        let json_data = tokio::fs::read_to_string(file_path).await?;
        let state: AppState = serde_json::from_str(&json_data)?;
        Ok(state)
    }

    pub fn get_export_path(&self, filename: &str) -> PathBuf {
        let mut export_dir = self.data_dir.clone();
        export_dir.push("exports");

        // Create exports directory if it doesn't exist
        let _ = std::fs::create_dir_all(&export_dir);

        export_dir.push(filename);
        export_dir
    }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub id: String,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration: u64,
    pub chapters: Vec<Chapter>,
    pub raw_text: String,
    pub status: TranscriptionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub start_time: f64,
    pub content: String,
    pub confidence: f32,
    pub subsections: Vec<Subsection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subsection {
    pub id: String,
    pub content: String,
    pub start_time: f64,
    pub end_time: f64,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscriptionStatus {
    Recording,
    Processing,
    Completed,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingState {
    pub is_recording: bool,
    pub current_text: String,
    pub duration: f64,
    pub audio_level: f32,
    pub transcription_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFormat {
    pub format_type: ExportType,
    pub include_timestamps: bool,
    pub include_chapters: bool,
    pub custom_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportType {
    Pdf,
    Docx,
    Txt,
    Markdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub transcriptions: HashMap<String, Transcription>,
    pub current_recording: Option<RecordingState>,
    pub gemini_api_key: Option<String>,
    pub selected_model: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            transcriptions: HashMap::new(),
            current_recording: None,
            gemini_api_key: None,
            selected_model: "gemini-2.5-flash".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChunk {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionChunk {
    pub text: String,
    pub confidence: f32,
    pub start_time: f64,
    pub end_time: f64,
    pub is_final: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supports_audio: bool,
    pub context_window: String,
}

pub fn get_available_models() -> Vec<GeminiModel> {
    vec![
        GeminiModel {
            id: "gemini-2.5-flash".to_string(),
            name: "Gemini 2.5 Flash".to_string(),
            description: "Best price/performance, audio support, thinking capabilities".to_string(),
            supports_audio: true,
            context_window: "1M tokens".to_string(),
        },
        GeminiModel {
            id: "gemini-2.5-pro".to_string(),
            name: "Gemini 2.5 Pro".to_string(),
            description: "Most powerful model for complex reasoning and analysis".to_string(),
            supports_audio: true,
            context_window: "2M tokens".to_string(),
        },
        GeminiModel {
            id: "gemini-2.0-flash".to_string(),
            name: "Gemini 2.0 Flash".to_string(),
            description: "Fast with native tool use and improved capabilities".to_string(),
            supports_audio: true,
            context_window: "1M tokens".to_string(),
        },
        GeminiModel {
            id: "gemini-1.5-pro".to_string(),
            name: "Gemini 1.5 Pro (Legacy)".to_string(),
            description: "Available only for existing projects with prior usage".to_string(),
            supports_audio: true,
            context_window: "2M tokens".to_string(),
        },
    ]
}
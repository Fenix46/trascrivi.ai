// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod transcription;
mod storage;
mod export;
mod models;

use audio::AudioCapture;
use transcription::TranscriptionService;
use storage::StorageService;
use export::ExportService;
use models::*;

use std::sync::{Arc, Mutex};
use tauri::{State, Window};
use tokio::sync::mpsc;
use anyhow::Result;

type AppStateType = Arc<Mutex<AppState>>;

#[tauri::command]
async fn start_recording(
    state: State<'_, AppStateType>,
    window: Window,
) -> std::result::Result<String, String> {
    let transcription_id = uuid::Uuid::new_v4().to_string();

    // Initialize audio capture
    let mut audio_capture_instance = AudioCapture::new().map_err(|e| e.to_string())?;
    let audio_rx = audio_capture_instance.start_recording().map_err(|e| e.to_string())?;

    // Update app state
    {
        let mut app_state = state.lock().unwrap();
        app_state.current_recording = Some(RecordingState {
            is_recording: true,
            current_text: String::new(),
            duration: 0.0,
            audio_level: 0.0,
            transcription_id: Some(transcription_id.clone()),
        });
    }

    // Start transcription service if API key is available
    let (api_key, model) = {
        let app_state = state.lock().unwrap();
        (app_state.gemini_api_key.clone(), app_state.selected_model.clone())
    };

    if let Some(api_key) = api_key {
        let transcription_service = TranscriptionService::new(api_key, model);
        let transcription_rx = transcription_service
            .start_streaming_transcription(audio_rx)
            .await
            .map_err(|e| e.to_string())?;

        // Spawn task to handle transcription updates
        let window_clone = window.clone();
        let state_clone = state.inner().clone();
        tokio::spawn(async move {
            handle_transcription_stream(transcription_rx, window_clone, state_clone).await;
        });
    }

    Ok(transcription_id)
}

#[tauri::command]
async fn stop_recording(
    state: State<'_, AppStateType>,
    storage: State<'_, StorageService>,
) -> std::result::Result<Transcription, String> {

    // Create transcription from current state
    let transcription = {
        let mut app_state = state.lock().unwrap();
        if let Some(recording_state) = app_state.current_recording.take() {
            let transcription = Transcription {
                id: recording_state.transcription_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                title: "New Transcription".to_string(),
                created_at: chrono::Utc::now(),
                duration: recording_state.duration as u64,
                chapters: Vec::new(),
                raw_text: recording_state.current_text,
                status: TranscriptionStatus::Completed,
            };

            app_state.transcriptions.insert(transcription.id.clone(), transcription.clone());
            transcription
        } else {
            return Err("No active recording".to_string());
        }
    };

    // Save transcription
    storage.save_transcription(&transcription).await.map_err(|e| e.to_string())?;

    Ok(transcription)
}

#[tauri::command]
async fn get_transcriptions(
    state: State<'_, AppStateType>,
) -> std::result::Result<Vec<Transcription>, String> {
    let app_state = state.lock().unwrap();
    let transcriptions: Vec<Transcription> = app_state.transcriptions.values().cloned().collect();
    Ok(transcriptions)
}

#[tauri::command]
async fn get_transcription(
    id: String,
    storage: State<'_, StorageService>,
) -> std::result::Result<Transcription, String> {
    storage.load_transcription(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_transcription(
    id: String,
    state: State<'_, AppStateType>,
    storage: State<'_, StorageService>,
) -> std::result::Result<(), String> {
    {
        let mut app_state = state.lock().unwrap();
        app_state.transcriptions.remove(&id);
    }

    storage.delete_transcription(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn export_transcription(
    id: String,
    format: ExportFormat,
    storage: State<'_, StorageService>,
) -> std::result::Result<String, String> {
    let transcription = storage.load_transcription(&id).await.map_err(|e| e.to_string())?;
    let export_service = ExportService::new((*storage).clone());
    export_service.export_transcription(&transcription, &format).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_api_key(
    api_key: String,
    state: State<'_, AppStateType>,
    storage: State<'_, StorageService>,
) -> std::result::Result<(), String> {
    {
        let mut app_state = state.lock().unwrap();
        app_state.gemini_api_key = Some(api_key);
    }

    let app_state = state.lock().unwrap().clone();
    storage.save_app_state(&app_state).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_available_models() -> std::result::Result<Vec<models::GeminiModel>, String> {
    Ok(models::get_available_models())
}

#[tauri::command]
async fn set_selected_model(
    model: String,
    state: State<'_, AppStateType>,
    storage: State<'_, StorageService>,
) -> std::result::Result<(), String> {
    {
        let mut app_state = state.lock().unwrap();
        app_state.selected_model = model;
    }

    let app_state = state.lock().unwrap().clone();
    storage.save_app_state(&app_state).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_selected_model(
    state: State<'_, AppStateType>,
) -> std::result::Result<String, String> {
    let app_state = state.lock().unwrap();
    Ok(app_state.selected_model.clone())
}

#[tauri::command]
async fn get_recording_state(
    state: State<'_, AppStateType>,
) -> std::result::Result<Option<RecordingState>, String> {
    let app_state = state.lock().unwrap();
    Ok(app_state.current_recording.clone())
}

#[tauri::command]
async fn analyze_transcription_structure(
    id: String,
    state: State<'_, AppStateType>,
    storage: State<'_, StorageService>,
) -> std::result::Result<Transcription, String> {
    let mut transcription = storage.load_transcription(&id).await.map_err(|e| e.to_string())?;

    let (api_key, model) = {
        let app_state = state.lock().unwrap();
        (app_state.gemini_api_key.clone(), app_state.selected_model.clone())
    };

    if let Some(api_key) = api_key {
        let transcription_service = TranscriptionService::new(api_key, model);
        let chapters = transcription_service
            .analyze_content_structure(&transcription.raw_text)
            .await
            .map_err(|e| e.to_string())?;

        transcription.chapters = chapters;

        // Update in state and storage
        {
            let mut app_state = state.lock().unwrap();
            app_state.transcriptions.insert(transcription.id.clone(), transcription.clone());
        }

        storage.save_transcription(&transcription).await.map_err(|e| e.to_string())?;
    }

    Ok(transcription)
}

async fn handle_transcription_stream(
    mut transcription_rx: mpsc::UnboundedReceiver<TranscriptionChunk>,
    window: Window,
    state: AppStateType,
) {
    while let Some(chunk) = transcription_rx.recv().await {
        // Update current recording state
        {
            let mut app_state = state.lock().unwrap();
            if let Some(ref mut recording) = app_state.current_recording {
                if !chunk.text.trim().is_empty() {
                    recording.current_text.push(' ');
                    recording.current_text.push_str(&chunk.text);
                }
                recording.duration = chunk.end_time;
            }
        }

        // Emit event to frontend
        let _ = window.emit("transcription-chunk", &chunk);
    }
}

#[tokio::main]
async fn main() {
    let storage = StorageService::new().expect("Failed to initialize storage");
    let app_state = storage.load_app_state().await.unwrap_or_default();

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(app_state)))
        .manage(storage)
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            get_transcriptions,
            get_transcription,
            delete_transcription,
            export_transcription,
            set_api_key,
            get_available_models,
            set_selected_model,
            get_selected_model,
            get_recording_state,
            analyze_transcription_structure
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
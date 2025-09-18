use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use crate::models::{AudioChunk, TranscriptionChunk, Chapter};

pub struct TranscriptionService {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl TranscriptionService {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            model,
        }
    }

    pub async fn start_streaming_transcription(
        &self,
        mut audio_rx: mpsc::UnboundedReceiver<AudioChunk>,
    ) -> Result<mpsc::UnboundedReceiver<TranscriptionChunk>> {
        let (tx, rx) = mpsc::unbounded_channel();
        let client = self.client.clone();
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();
        let model = self.model.clone();

        tokio::spawn(async move {
            let mut accumulated_audio = Vec::new();
            let mut last_transcription_time = std::time::Instant::now();
            const TRANSCRIPTION_INTERVAL: std::time::Duration = std::time::Duration::from_millis(2000);

            while let Some(chunk) = audio_rx.recv().await {
                accumulated_audio.extend_from_slice(&chunk.data);

                if last_transcription_time.elapsed() >= TRANSCRIPTION_INTERVAL && !accumulated_audio.is_empty() {
                    let audio_data = accumulated_audio.clone();
                    accumulated_audio.clear();
                    last_transcription_time = std::time::Instant::now();

                    match Self::transcribe_audio_chunk(&client, &api_key, &base_url, &model, audio_data, chunk.sample_rate).await {
                        Ok(text) => {
                            if !text.trim().is_empty() {
                                let transcription_chunk = TranscriptionChunk {
                                    text,
                                    confidence: 0.9, // Placeholder confidence
                                    start_time: chunk.timestamp,
                                    end_time: chunk.timestamp + 2.0,
                                    is_final: false,
                                };

                                if tx.send(transcription_chunk).is_err() {
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Transcription error: {}", e);
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    async fn transcribe_audio_chunk(
        client: &Client,
        api_key: &str,
        base_url: &str,
        model: &str,
        audio_data: Vec<f32>,
        sample_rate: u32,
    ) -> Result<String> {
        // Convert f32 audio data to base64 encoded WAV
        let wav_data = Self::convert_to_wav(&audio_data, sample_rate)?;
        let base64_audio = base64::encode(&wav_data);

        let url = format!("{}/models/{}:generateContent?key={}", base_url, model, api_key);

        let request_body = json!({
            "contents": [{
                "parts": [{
                    "text": "Please transcribe this audio to text. Only return the transcribed text, nothing else."
                }, {
                    "inline_data": {
                        "mime_type": "audio/wav",
                        "data": base64_audio
                    }
                }]
            }]
        });

        let response = client
            .post(&url)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("API error: {}", error_text));
        }

        let response_json: Value = response.json().await?;

        let text = response_json
            .get("candidates")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("content"))
            .and_then(|c| c.get("parts"))
            .and_then(|p| p.get(0))
            .and_then(|p| p.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        Ok(text)
    }

    fn convert_to_wav(audio_data: &[f32], sample_rate: u32) -> Result<Vec<u8>> {
        let mut wav_data = Vec::new();

        // WAV header
        wav_data.extend_from_slice(b"RIFF");
        wav_data.extend_from_slice(&(36 + audio_data.len() * 2).to_le_bytes()); // File size - 8
        wav_data.extend_from_slice(b"WAVE");

        // Format chunk
        wav_data.extend_from_slice(b"fmt ");
        wav_data.extend_from_slice(&16u32.to_le_bytes()); // Chunk size
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // Audio format (PCM)
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // Number of channels
        wav_data.extend_from_slice(&sample_rate.to_le_bytes()); // Sample rate
        wav_data.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // Byte rate
        wav_data.extend_from_slice(&2u16.to_le_bytes()); // Block align
        wav_data.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

        // Data chunk
        wav_data.extend_from_slice(b"data");
        wav_data.extend_from_slice(&(audio_data.len() * 2).to_le_bytes()); // Data size

        // Convert f32 to i16 PCM data
        for &sample in audio_data {
            let sample_i16 = (sample * i16::MAX as f32) as i16;
            wav_data.extend_from_slice(&sample_i16.to_le_bytes());
        }

        Ok(wav_data)
    }

    pub async fn analyze_content_structure(&self, text: &str) -> Result<Vec<Chapter>> {
        let url = format!("{}/models/{}:generateContent?key={}", self.base_url, self.model, self.api_key);

        let prompt = format!(
            "Analyze this transcription and break it into logical chapters with titles.
            Return a JSON array with this structure:
            [{{\"title\": \"Chapter Title\", \"content\": \"Chapter content...\", \"start_time\": 0.0}}]

            Transcription:
            {}",
            text
        );

        let request_body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await?;

        let response_json: Value = response.json().await?;

        let response_text = response_json
            .get("candidates")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("content"))
            .and_then(|c| c.get("parts"))
            .and_then(|p| p.get(0))
            .and_then(|p| p.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("[]");

        // Extract JSON from the response
        let json_start = response_text.find('[').unwrap_or(0);
        let json_end = response_text.rfind(']').unwrap_or(response_text.len());
        let json_str = &response_text[json_start..=json_end];

        let chapters_data: Value = serde_json::from_str(json_str)
            .unwrap_or_else(|_| json!([]));

        let mut chapters = Vec::new();
        if let Some(chapters_array) = chapters_data.as_array() {
            for (i, chapter_data) in chapters_array.iter().enumerate() {
                let chapter = Chapter {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: chapter_data.get("title")
                        .and_then(|t| t.as_str())
                        .unwrap_or(&format!("Chapter {}", i + 1))
                        .to_string(),
                    start_time: chapter_data.get("start_time")
                        .and_then(|t| t.as_f64())
                        .unwrap_or(0.0),
                    content: chapter_data.get("content")
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string(),
                    confidence: 0.9,
                    subsections: Vec::new(),
                };
                chapters.push(chapter);
            }
        }

        Ok(chapters)
    }
}

// Add base64 dependency to Cargo.toml
mod base64 {
    pub fn encode(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let mut result = String::new();
        let mut i = 0;

        while i < data.len() {
            let b1 = data[i];
            let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
            let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };

            let bitmap = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

            result.push(CHARS[((bitmap >> 18) & 0x3F) as usize] as char);
            result.push(CHARS[((bitmap >> 12) & 0x3F) as usize] as char);
            result.push(if i + 1 < data.len() { CHARS[((bitmap >> 6) & 0x3F) as usize] as char } else { '=' });
            result.push(if i + 2 < data.len() { CHARS[(bitmap & 0x3F) as usize] as char } else { '=' });

            i += 3;
        }

        result
    }
}
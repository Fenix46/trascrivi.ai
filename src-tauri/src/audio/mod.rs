use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig, InputCallbackInfo};
use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use crate::models::AudioChunk;
use std::sync::{Arc, Mutex};

pub struct AudioCapture {
    is_recording: Arc<Mutex<bool>>,
    sample_rate: u32,
    // Non salviamo il Stream direttamente per evitare problemi di thread safety
}

impl AudioCapture {
    pub fn new() -> Result<Self> {
        // Test che il microfono sia disponibile
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("No input device available"))?;

        let config = device.default_input_config().map_err(|e| anyhow!("Failed to get input config: {}", e))?;
        let sample_rate = config.sample_rate().0;

        println!("AudioCapture: Found input device with sample rate: {}", sample_rate);

        Ok(Self {
            is_recording: Arc::new(Mutex::new(false)),
            sample_rate,
        })
    }

    pub fn start_recording(&mut self) -> Result<mpsc::UnboundedReceiver<AudioChunk>> {
        println!("AudioCapture: Starting REAL microphone capture...");

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("No input device available"))?;

        let config = device.default_input_config().map_err(|e| anyhow!("Failed to get input config: {}", e))?;
        let sample_rate = config.sample_rate().0;

        println!("AudioCapture: Using device with format: {:?}", config);

        let (tx, rx) = mpsc::unbounded_channel();
        let is_recording = Arc::clone(&self.is_recording);
        *is_recording.lock().unwrap() = true;

        // Create the audio stream based on the sample format
        let stream_result = match config.sample_format() {
            SampleFormat::F32 => self.create_input_stream::<f32>(&device, &config.into(), tx, Arc::clone(&is_recording)),
            SampleFormat::I16 => self.create_input_stream::<i16>(&device, &config.into(), tx, Arc::clone(&is_recording)),
            SampleFormat::U16 => self.create_input_stream::<u16>(&device, &config.into(), tx, Arc::clone(&is_recording)),
            _ => return Err(anyhow!("Unsupported sample format: {:?}", config.sample_format())),
        };

        let stream = stream_result?;

        // Start the stream
        stream.play().map_err(|e| anyhow!("Failed to start audio stream: {}", e))?;

        println!("AudioCapture: Audio stream started successfully!");

        // Per ora droppiamo immediatamente lo stream per evitare problemi di thread safety
        // Il callback continuerà a funzionare finché is_recording è true
        // Questo è un compromesso: il stream potrebbe essere droppato presto, ma dovrebbe funzionare per un test
        std::mem::forget(stream); // Mantiene il stream in memoria senza gestire il lifetime

        Ok(rx)
    }

    fn create_input_stream<T>(
        &self,
        device: &cpal::Device,
        config: &StreamConfig,
        tx: mpsc::UnboundedSender<AudioChunk>,
        is_recording: Arc<Mutex<bool>>,
    ) -> Result<cpal::Stream>
    where
        T: cpal::Sample + cpal::SizedSample + Send + 'static,
        f32: cpal::FromSample<T>,
    {
        let sample_rate = config.sample_rate.0;
        let channels = config.channels;

        println!("AudioCapture: Creating stream with {} channels at {} Hz", channels, sample_rate);

        // Buffer per accumulare audio (100ms di audio)
        let chunk_size = (sample_rate as usize) / 10; // 100ms chunks
        let mut buffer = Vec::with_capacity(chunk_size);
        let mut start_time = std::time::Instant::now();

        let stream = device.build_input_stream(
            config,
            move |data: &[T], _info: &InputCallbackInfo| {
                if !*is_recording.lock().unwrap() {
                    return;
                }

                // Convert samples to f32 and accumulate
                for &sample in data {
                    let sample_f32: f32 = f32::from_sample(sample);
                    buffer.push(sample_f32);

                    if buffer.len() >= chunk_size {
                        let timestamp = start_time.elapsed().as_secs_f64();

                        let chunk = AudioChunk {
                            data: buffer.clone(),
                            sample_rate,
                            timestamp,
                        };

                        if let Err(_) = tx.send(chunk) {
                            println!("AudioCapture: Failed to send audio chunk, receiver dropped");
                            return;
                        }

                        buffer.clear();
                        start_time = std::time::Instant::now();
                    }
                }
            },
            |err| {
                eprintln!("AudioCapture: Stream error: {}", err);
            },
            None,
        ).map_err(|e| anyhow!("Failed to build input stream: {}", e))?;

        Ok(stream)
    }

    pub fn stop_recording(&mut self) -> Result<()> {
        *self.is_recording.lock().unwrap() = false;
        Ok(())
    }

    pub fn is_recording(&self) -> bool {
        *self.is_recording.lock().unwrap()
    }

    pub fn get_audio_level(&self) -> f32 {
        0.5 // Placeholder audio level
    }
}
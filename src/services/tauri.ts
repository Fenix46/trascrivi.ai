import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { Transcription, RecordingState, ExportFormat, TranscriptionChunk } from '../types';

export class TauriService {
  static async startRecording(): Promise<string> {
    return await invoke('start_recording');
  }

  static async stopRecording(): Promise<Transcription> {
    return await invoke('stop_recording');
  }

  static async getTranscriptions(): Promise<Transcription[]> {
    return await invoke('get_transcriptions');
  }

  static async getTranscription(id: string): Promise<Transcription> {
    return await invoke('get_transcription', { id });
  }

  static async deleteTranscription(id: string): Promise<void> {
    return await invoke('delete_transcription', { id });
  }

  static async exportTranscription(id: string, format: ExportFormat): Promise<string> {
    return await invoke('export_transcription', { id, format });
  }

  static async setApiKey(apiKey: string): Promise<void> {
    return await invoke('set_api_key', { apiKey });
  }

  static async getRecordingState(): Promise<RecordingState | null> {
    return await invoke('get_recording_state');
  }

  static async analyzeTranscriptionStructure(id: string): Promise<Transcription> {
    return await invoke('analyze_transcription_structure', { id });
  }

  static listenToTranscriptionChunks(callback: (chunk: TranscriptionChunk) => void) {
    return listen<TranscriptionChunk>('transcription-chunk', (event) => {
      callback(event.payload);
    });
  }

  static async openExportedFile(filePath: string): Promise<void> {
    const { shell } = await import('@tauri-apps/api');
    return shell.open(filePath);
  }
}
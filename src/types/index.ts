export interface Transcription {
  id: string;
  title: string;
  created_at: string;
  duration: number;
  chapters: Chapter[];
  raw_text: string;
  status: TranscriptionStatus;
}

export interface Chapter {
  id: string;
  title: string;
  start_time: number;
  content: string;
  confidence: number;
  subsections: Subsection[];
}

export interface Subsection {
  id: string;
  content: string;
  start_time: number;
  end_time: number;
  confidence: number;
}

export type TranscriptionStatus =
  | "Recording"
  | "Processing"
  | "Completed"
  | { Error: string };

export interface RecordingState {
  is_recording: boolean;
  current_text: string;
  duration: number;
  audio_level: number;
  transcription_id?: string;
}

export interface ExportFormat {
  format_type: ExportType;
  include_timestamps: boolean;
  include_chapters: boolean;
  custom_template?: string;
}

export type ExportType = "Pdf" | "Docx" | "Txt" | "Markdown";

export interface TranscriptionChunk {
  text: string;
  confidence: number;
  start_time: number;
  end_time: number;
  is_final: boolean;
}

export interface GeminiModel {
  id: string;
  name: string;
  description: string;
  supports_audio: boolean;
  context_window: string;
}
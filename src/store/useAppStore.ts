import { create } from 'zustand';
import { Transcription, RecordingState, TranscriptionChunk } from '../types';

interface AppStore {
  // State
  transcriptions: Transcription[];
  selectedTranscription: Transcription | null;
  recordingState: RecordingState | null;
  isLoading: boolean;
  error: string | null;
  apiKey: string | null;
  sidebarCollapsed: boolean;

  // Actions
  setTranscriptions: (transcriptions: Transcription[]) => void;
  addTranscription: (transcription: Transcription) => void;
  updateTranscription: (transcription: Transcription) => void;
  deleteTranscription: (id: string) => void;
  setSelectedTranscription: (transcription: Transcription | null) => void;
  setRecordingState: (state: RecordingState | null) => void;
  updateRecordingText: (chunk: TranscriptionChunk) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setApiKey: (key: string | null) => void;
  toggleSidebar: () => void;
  resetStore: () => void;
}

export const useAppStore = create<AppStore>((set, get) => ({
  // Initial state
  transcriptions: [],
  selectedTranscription: null,
  recordingState: null,
  isLoading: false,
  error: null,
  apiKey: null,
  sidebarCollapsed: false,

  // Actions
  setTranscriptions: (transcriptions) => set({ transcriptions }),

  addTranscription: (transcription) =>
    set((state) => ({
      transcriptions: [transcription, ...state.transcriptions],
    })),

  updateTranscription: (transcription) =>
    set((state) => ({
      transcriptions: state.transcriptions.map((t) =>
        t.id === transcription.id ? transcription : t
      ),
      selectedTranscription:
        state.selectedTranscription?.id === transcription.id
          ? transcription
          : state.selectedTranscription,
    })),

  deleteTranscription: (id) =>
    set((state) => ({
      transcriptions: state.transcriptions.filter((t) => t.id !== id),
      selectedTranscription:
        state.selectedTranscription?.id === id
          ? null
          : state.selectedTranscription,
    })),

  setSelectedTranscription: (transcription) =>
    set({ selectedTranscription: transcription }),

  setRecordingState: (state) => set({ recordingState: state }),

  updateRecordingText: (chunk) =>
    set((state) => {
      if (!state.recordingState) return state;

      return {
        recordingState: {
          ...state.recordingState,
          current_text: state.recordingState.current_text + ' ' + chunk.text,
          duration: chunk.end_time,
        },
      };
    }),

  setLoading: (loading) => set({ isLoading: loading }),

  setError: (error) => set({ error }),

  setApiKey: (key) => set({ apiKey: key }),

  toggleSidebar: () =>
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),

  resetStore: () =>
    set({
      transcriptions: [],
      selectedTranscription: null,
      recordingState: null,
      isLoading: false,
      error: null,
      sidebarCollapsed: false,
    }),
}));
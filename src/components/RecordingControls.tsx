import React, { useState } from 'react';
import { AudioVisualizer } from './AudioVisualizer';
import { useAppStore } from '../store/useAppStore';
import { TauriService } from '../services/tauri';

export const RecordingControls: React.FC = () => {
  const {
    recordingState,
    setRecordingState,
    addTranscription,
    setError,
    setLoading,
    apiKey,
  } = useAppStore();

  const [isStarting, setIsStarting] = useState(false);
  const [isStopping, setIsStopping] = useState(false);

  const formatDuration = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const handleStartRecording = async () => {
    if (!apiKey) {
      setError('Please set your Gemini API key first');
      return;
    }

    setIsStarting(true);
    setError(null);

    try {
      const transcriptionId = await TauriService.startRecording();
      setRecordingState({
        is_recording: true,
        current_text: '',
        duration: 0,
        audio_level: 0,
        transcription_id: transcriptionId,
      });
    } catch (error) {
      setError(`Failed to start recording: ${error}`);
    } finally {
      setIsStarting(false);
    }
  };

  const handleStopRecording = async () => {
    if (!recordingState?.is_recording) return;

    setIsStopping(true);
    setLoading(true);

    try {
      const transcription = await TauriService.stopRecording();
      addTranscription(transcription);
      setRecordingState(null);
    } catch (error) {
      setError(`Failed to stop recording: ${error}`);
    } finally {
      setIsStopping(false);
      setLoading(false);
    }
  };

  const isRecording = recordingState?.is_recording || false;

  return (
    <div className="flex items-center gap-4 p-4 bg-gray-800 border-b border-gray-700">
      <div className="flex items-center gap-3">
        <button
          onClick={isRecording ? handleStopRecording : handleStartRecording}
          disabled={isStarting || isStopping}
          className={`
            relative w-12 h-12 rounded-full transition-all duration-200 flex items-center justify-center
            ${isRecording
              ? 'bg-red-600 hover:bg-red-700 recording-pulse'
              : 'bg-primary-600 hover:bg-primary-700'
            }
            ${(isStarting || isStopping) ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}
          `}
        >
          {isStarting || isStopping ? (
            <div className="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
          ) : isRecording ? (
            <div className="w-4 h-4 bg-white rounded-sm" />
          ) : (
            <div className="w-0 h-0 border-l-[8px] border-l-white border-y-[6px] border-y-transparent ml-1" />
          )}
        </button>

        <AudioVisualizer
          isRecording={isRecording}
          audioLevel={recordingState?.audio_level || 0.5}
        />
      </div>

      {isRecording && (
        <div className="flex items-center gap-4 text-sm text-gray-300">
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-red-500 rounded-full animate-pulse" />
            <span>Recording</span>
          </div>
          <div className="font-mono">
            {formatDuration(recordingState?.duration || 0)}
          </div>
        </div>
      )}

      {!apiKey && !isRecording && (
        <div className="text-sm text-yellow-400">
          ⚠️ Set API key to enable recording
        </div>
      )}
    </div>
  );
};
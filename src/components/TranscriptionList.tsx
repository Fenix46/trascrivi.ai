import React from 'react';
import { Transcription } from '../types';
import { useAppStore } from '../store/useAppStore';
import { TauriService } from '../services/tauri';

interface TranscriptionListProps {
  transcriptions: Transcription[];
  onSelect: (transcription: Transcription) => void;
  isCollapsed: boolean;
}

export const TranscriptionList: React.FC<TranscriptionListProps> = ({
  transcriptions,
  onSelect,
  isCollapsed,
}) => {
  const { selectedTranscription, deleteTranscription, setError } = useAppStore();

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const formatDuration = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const handleDelete = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();

    if (confirm('Are you sure you want to delete this transcription?')) {
      try {
        await TauriService.deleteTranscription(id);
        deleteTranscription(id);
      } catch (error) {
        setError(`Failed to delete transcription: ${error}`);
      }
    }
  };

  const getPreviewText = (transcription: Transcription) => {
    const text = transcription.raw_text || '';
    return text.length > 100 ? text.substring(0, 100) + '...' : text;
  };

  if (isCollapsed) {
    return (
      <div className="w-16 bg-gray-900 border-r border-gray-700 flex flex-col items-center py-4 gap-2">
        {transcriptions.slice(0, 5).map((transcription) => (
          <button
            key={transcription.id}
            onClick={() => onSelect(transcription)}
            className={`
              w-10 h-10 rounded-lg flex items-center justify-center text-xs font-medium
              transition-colors duration-200
              ${selectedTranscription?.id === transcription.id
                ? 'bg-primary-600 text-white'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
              }
            `}
            title={transcription.title}
          >
            {transcription.title.substring(0, 2).toUpperCase()}
          </button>
        ))}
      </div>
    );
  }

  return (
    <div className="w-80 bg-gray-900 border-r border-gray-700 flex flex-col">
      <div className="p-4 border-b border-gray-700">
        <h2 className="text-lg font-semibold text-white">Transcriptions</h2>
        <p className="text-sm text-gray-400">{transcriptions.length} recordings</p>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide">
        {transcriptions.length === 0 ? (
          <div className="p-4 text-center text-gray-500">
            <div className="mb-4">
              <svg className="w-12 h-12 mx-auto text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
              </svg>
            </div>
            <p className="text-sm">No recordings yet</p>
            <p className="text-xs text-gray-600 mt-1">Start recording to create your first transcription</p>
          </div>
        ) : (
          transcriptions.map((transcription) => (
            <div
              key={transcription.id}
              onClick={() => onSelect(transcription)}
              className={`
                p-4 border-b border-gray-800 cursor-pointer transition-colors duration-200
                ${selectedTranscription?.id === transcription.id
                  ? 'bg-primary-900/30 border-primary-700'
                  : 'hover:bg-gray-800'
                }
              `}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <h3 className="font-medium text-white truncate">
                    {transcription.title}
                  </h3>
                  <p className="text-xs text-gray-400 mt-1">
                    {formatDate(transcription.created_at)} • {formatDuration(transcription.duration)}
                  </p>
                  {transcription.chapters.length > 0 && (
                    <p className="text-xs text-blue-400 mt-1">
                      {transcription.chapters.length} chapters
                    </p>
                  )}
                  <p className="text-sm text-gray-500 mt-2 line-clamp-2">
                    {getPreviewText(transcription)}
                  </p>
                </div>
                <button
                  onClick={(e) => handleDelete(e, transcription.id)}
                  className="ml-2 p-1 text-gray-500 hover:text-red-400 transition-colors"
                  title="Delete transcription"
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};
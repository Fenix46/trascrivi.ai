import React, { useState } from 'react';
import { Transcription, ExportFormat, ExportType } from '../types';
import { useAppStore } from '../store/useAppStore';
import { TauriService } from '../services/tauri';

interface TranscriptionViewProps {
  transcription: Transcription | null;
  isLive: boolean;
}

export const TranscriptionView: React.FC<TranscriptionViewProps> = ({
  transcription,
  isLive,
}) => {
  const { recordingState, updateTranscription, setError } = useAppStore();
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [isExporting, setIsExporting] = useState(false);
  const [exportFormat, setExportFormat] = useState<ExportType>('Pdf');
  const [includeTimestamps, setIncludeTimestamps] = useState(true);
  const [includeChapters, setIncludeChapters] = useState(true);

  const formatDuration = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const handleAnalyzeStructure = async () => {
    if (!transcription) return;

    setIsAnalyzing(true);
    try {
      const analyzedTranscription = await TauriService.analyzeTranscriptionStructure(transcription.id);
      updateTranscription(analyzedTranscription);
    } catch (error) {
      setError(`Failed to analyze structure: ${error}`);
    } finally {
      setIsAnalyzing(false);
    }
  };

  const handleExport = async () => {
    if (!transcription) return;

    setIsExporting(true);
    try {
      const format: ExportFormat = {
        format_type: exportFormat,
        include_timestamps: includeTimestamps,
        include_chapters: includeChapters,
      };

      const filePath = await TauriService.exportTranscription(transcription.id, format);
      await TauriService.openExportedFile(filePath);
    } catch (error) {
      setError(`Failed to export: ${error}`);
    } finally {
      setIsExporting(false);
    }
  };

  const currentText = isLive ? recordingState?.current_text || '' : transcription?.raw_text || '';
  const currentDuration = isLive ? recordingState?.duration || 0 : transcription?.duration || 0;

  if (!transcription && !isLive) {
    return (
      <div className="flex-1 flex items-center justify-center bg-gray-800">
        <div className="text-center text-gray-500">
          <svg className="w-16 h-16 mx-auto mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <h3 className="text-lg font-medium mb-2">No transcription selected</h3>
          <p className="text-sm">Select a transcription from the sidebar or start recording</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 flex flex-col bg-gray-800">
      {/* Header */}
      <div className="p-6 border-b border-gray-700">
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold text-white mb-2">
              {isLive ? 'Live Recording' : transcription?.title}
            </h1>
            <div className="flex items-center gap-4 text-sm text-gray-400">
              {transcription && (
                <span>{new Date(transcription.created_at).toLocaleDateString()}</span>
              )}
              <span>Duration: {formatDuration(currentDuration)}</span>
              {transcription?.chapters.length && (
                <span>{transcription.chapters.length} chapters</span>
              )}
            </div>
          </div>

          {transcription && !isLive && (
            <div className="flex items-center gap-2">
              <button
                onClick={handleAnalyzeStructure}
                disabled={isAnalyzing}
                className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors disabled:opacity-50"
              >
                {isAnalyzing ? 'Analyzing...' : 'Analyze Structure'}
              </button>

              <div className="relative">
                <button
                  onClick={() => setIsExporting(!isExporting)}
                  className="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors"
                >
                  Export
                </button>

                {isExporting && (
                  <div className="absolute right-0 top-12 bg-gray-700 border border-gray-600 rounded-lg p-4 w-64 z-10">
                    <div className="mb-3">
                      <label className="block text-sm font-medium text-gray-300 mb-2">Format</label>
                      <select
                        value={exportFormat}
                        onChange={(e) => setExportFormat(e.target.value as ExportType)}
                        className="w-full bg-gray-600 border border-gray-500 rounded px-3 py-2 text-white"
                      >
                        <option value="Pdf">PDF</option>
                        <option value="Docx">Word Document</option>
                        <option value="Txt">Text File</option>
                        <option value="Markdown">Markdown</option>
                      </select>
                    </div>

                    <div className="space-y-2 mb-4">
                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={includeTimestamps}
                          onChange={(e) => setIncludeTimestamps(e.target.checked)}
                          className="mr-2"
                        />
                        <span className="text-sm text-gray-300">Include timestamps</span>
                      </label>
                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={includeChapters}
                          onChange={(e) => setIncludeChapters(e.target.checked)}
                          className="mr-2"
                        />
                        <span className="text-sm text-gray-300">Include chapters</span>
                      </label>
                    </div>

                    <div className="flex gap-2">
                      <button
                        onClick={handleExport}
                        className="flex-1 px-3 py-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors"
                      >
                        Export
                      </button>
                      <button
                        onClick={() => setIsExporting(false)}
                        className="px-3 py-2 bg-gray-600 hover:bg-gray-500 text-white rounded transition-colors"
                      >
                        Cancel
                      </button>
                    </div>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-6">
        {transcription?.chapters.length ? (
          <div className="space-y-6">
            {transcription.chapters.map((chapter) => (
              <div key={chapter.id} className="chapter-marker">
                <h3 className="text-lg font-semibold text-white mb-2">
                  {chapter.title}
                </h3>
                <p className="text-xs text-gray-400 mb-3">
                  Start: {formatDuration(chapter.start_time)} • Confidence: {Math.round(chapter.confidence * 100)}%
                </p>
                <p className="transcript-text text-gray-300 leading-relaxed">
                  {chapter.content}
                </p>
              </div>
            ))}
          </div>
        ) : (
          <div className="space-y-4">
            {isLive && (
              <div className="flex items-center gap-2 text-sm text-blue-400 mb-4">
                <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse" />
                <span>Transcribing in real-time...</span>
              </div>
            )}
            <p className="transcript-text text-gray-300 leading-relaxed">
              {currentText || 'Start speaking to see transcription...'}
            </p>
          </div>
        )}
      </div>
    </div>
  );
};
import React, { useEffect, useState } from 'react';
import { RecordingControls } from './components/RecordingControls';
import { TranscriptionList } from './components/TranscriptionList';
import { TranscriptionView } from './components/TranscriptionView';
import { Settings } from './components/Settings';
import { useAppStore } from './store/useAppStore';
import { TauriService } from './services/tauri';

function App() {
  const {
    transcriptions,
    selectedTranscription,
    recordingState,
    isLoading,
    error,
    sidebarCollapsed,
    setTranscriptions,
    setSelectedTranscription,
    setRecordingState,
    updateRecordingText,
    setError,
    toggleSidebar,
  } = useAppStore();

  const [showSettings, setShowSettings] = useState(false);

  useEffect(() => {
    // Load initial data
    const loadInitialData = async () => {
      try {
        const transcriptions = await TauriService.getTranscriptions();
        setTranscriptions(transcriptions);

        const recordingState = await TauriService.getRecordingState();
        setRecordingState(recordingState);
      } catch (error) {
        setError(`Failed to load data: ${error}`);
      }
    };

    loadInitialData();

    // Listen for real-time transcription updates
    const unsubscribe = TauriService.listenToTranscriptionChunks((chunk) => {
      updateRecordingText(chunk);
    });

    return () => {
      unsubscribe.then(unlisten => unlisten());
    };
  }, [setTranscriptions, setRecordingState, updateRecordingText, setError]);

  const handleSelectTranscription = (transcription: any) => {
    setSelectedTranscription(transcription);
  };

  const isLiveView = recordingState?.is_recording && !selectedTranscription;

  return (
    <div className="h-screen flex flex-col bg-gray-900">
      {/* Header */}
      <div className="flex items-center justify-between p-4 bg-gray-900 border-b border-gray-700">
        <div className="flex items-center gap-4">
          <button
            onClick={toggleSidebar}
            className="p-2 text-gray-400 hover:text-white transition-colors"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          <h1 className="text-xl font-bold text-white">Trascrivi AI</h1>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => setShowSettings(true)}
            className="p-2 text-gray-400 hover:text-white transition-colors"
            title="Settings"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </button>
        </div>
      </div>

      {/* Recording Controls */}
      <RecordingControls />

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        <TranscriptionList
          transcriptions={transcriptions}
          onSelect={handleSelectTranscription}
          isCollapsed={sidebarCollapsed}
        />

        <TranscriptionView
          transcription={isLiveView ? null : selectedTranscription}
          isLive={isLiveView}
        />
      </div>

      {/* Error Display */}
      {error && (
        <div className="bg-red-600 text-white p-3 flex items-center justify-between">
          <span className="text-sm">{error}</span>
          <button
            onClick={() => setError(null)}
            className="text-red-200 hover:text-white transition-colors"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      )}

      {/* Loading Overlay */}
      {isLoading && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-40">
          <div className="bg-gray-800 rounded-lg p-6 flex items-center gap-3">
            <div className="w-6 h-6 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
            <span className="text-white">Processing...</span>
          </div>
        </div>
      )}

      {/* Settings Modal */}
      <Settings isOpen={showSettings} onClose={() => setShowSettings(false)} />
    </div>
  );
}

export default App;
import React, { useState, useEffect } from 'react';
import { useAppStore } from '../store/useAppStore';
import { TauriService } from '../services/tauri';

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
}

export const Settings: React.FC<SettingsProps> = ({ isOpen, onClose }) => {
  const {
    apiKey,
    setApiKey,
    availableModels,
    selectedModel,
    setAvailableModels,
    setSelectedModel,
    setError
  } = useAppStore();

  const [tempApiKey, setTempApiKey] = useState(apiKey || '');
  const [tempModel, setTempModel] = useState(selectedModel);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (isOpen) {
      // Load available models when settings open
      const loadModels = async () => {
        try {
          const models = await TauriService.getAvailableModels();
          setAvailableModels(models);

          const currentModel = await TauriService.getSelectedModel();
          setTempModel(currentModel);
        } catch (error) {
          setError(`Failed to load models: ${error}`);
        }
      };

      loadModels();
      setTempApiKey(apiKey || '');
    }
  }, [isOpen, apiKey, setAvailableModels, setError]);

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await TauriService.setApiKey(tempApiKey);
      await TauriService.setSelectedModel(tempModel);

      setApiKey(tempApiKey);
      setSelectedModel(tempModel);
      onClose();
    } catch (error) {
      setError(`Failed to save settings: ${error}`);
    } finally {
      setIsSaving(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-full max-w-md">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-semibold text-white">Settings</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Gemini API Key
            </label>
            <input
              type="password"
              value={tempApiKey}
              onChange={(e) => setTempApiKey(e.target.value)}
              placeholder="Enter your Gemini API key"
              className="w-full bg-gray-700 border border-gray-600 rounded-lg px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <p className="text-xs text-gray-500 mt-1">
              Get your API key from{' '}
              <a
                href="https://aistudio.google.com/app/apikey"
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-400 hover:text-blue-300"
              >
                Google AI Studio
              </a>
            </p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Gemini Model
            </label>
            <select
              value={tempModel}
              onChange={(e) => setTempModel(e.target.value)}
              className="w-full bg-gray-700 border border-gray-600 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {availableModels.map((model) => (
                <option key={model.id} value={model.id}>
                  {model.name} - {model.context_window}
                </option>
              ))}
            </select>
            {availableModels.find(m => m.id === tempModel) && (
              <div className="mt-2 p-3 bg-gray-800 rounded-lg">
                <p className="text-xs text-gray-400">
                  {availableModels.find(m => m.id === tempModel)?.description}
                </p>
                <div className="flex items-center gap-2 mt-1">
                  <div className={`w-2 h-2 rounded-full ${
                    availableModels.find(m => m.id === tempModel)?.supports_audio
                      ? 'bg-green-500'
                      : 'bg-red-500'
                  }`} />
                  <span className="text-xs text-gray-500">
                    Audio Support: {availableModels.find(m => m.id === tempModel)?.supports_audio ? 'Yes' : 'No'}
                  </span>
                </div>
              </div>
            )}
          </div>
        </div>

        <div className="flex justify-end gap-3 mt-6">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-400 hover:text-white transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving || !tempApiKey.trim()}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSaving ? 'Saving...' : 'Save Settings'}
          </button>
        </div>
      </div>
    </div>
  );
};
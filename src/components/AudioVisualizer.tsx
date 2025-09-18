import React from 'react';

interface AudioVisualizerProps {
  isRecording: boolean;
  audioLevel?: number;
}

export const AudioVisualizer: React.FC<AudioVisualizerProps> = ({
  isRecording,
  audioLevel = 0.5
}) => {
  if (!isRecording) return null;

  return (
    <div className="audio-visualizer">
      {[...Array(5)].map((_, i) => (
        <div
          key={i}
          className="audio-bar"
          style={{
            height: `${Math.max(4, audioLevel * 16 + Math.random() * 8)}px`,
          }}
        />
      ))}
    </div>
  );
};
# Trascrivi AI

A cross-platform desktop application for intelligent audio transcription using Tauri + Rust backend + React TypeScript frontend with Gemini AI integration.

## Features

### Core Functionality
- **Real-time Audio Capture**: Capture audio directly from system microphone using Rust backend
- **Streaming Transcription**: Real-time audio processing with Gemini API integration
- **Intelligent Content Analysis**: Automatic identification of titles, sections, and hierarchical structure
- **Session Management**: Multiple transcription sessions with local persistence
- **Export Options**: PDF, DOCX, TXT, and Markdown export formats

### Interface
- **Responsive React Frontend**: Modern UI with collapsible sidebar
- **Real-time Display**: Live transcription streaming during recording
- **Session Browser**: Sidebar with transcription history and preview
- **Export Panel**: Configurable export options with format selection

## Technology Stack

### Backend (Rust)
- **Tauri**: Cross-platform desktop framework
- **cpal**: Cross-platform audio capture
- **tokio**: Async runtime for real-time processing
- **reqwest**: HTTP client for Gemini API communication
- **serde**: Serialization/deserialization
- **printpdf**: Native PDF generation
- **anyhow**: Error handling

### Frontend (React TypeScript)
- **React 18**: Modern React with hooks
- **TypeScript**: Type-safe development
- **Tailwind CSS**: Utility-first styling
- **Zustand**: Lightweight state management
- **Tauri API**: Rust backend communication

## Project Structure

```
trascrivi-ai/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── audio/       # Audio capture system
│   │   ├── transcription/ # Gemini API integration
│   │   ├── storage/     # Data persistence
│   │   ├── export/      # Export functionality
│   │   ├── models.rs    # Data structures
│   │   └── main.rs      # Main application & Tauri commands
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── src/                # React frontend
│   ├── components/     # React components
│   ├── store/          # State management
│   ├── services/       # API services
│   ├── types/          # TypeScript types
│   └── App.tsx         # Main application component
├── package.json        # Node.js dependencies
└── vite.config.ts      # Vite configuration
```

## Setup & Installation

### Prerequisites
- **Rust**: Latest stable version
- **Node.js**: 18+ with npm
- **Gemini API Key**: Get from [Google AI Studio](https://aistudio.google.com/app/apikey)

### Development Setup

1. **Clone and navigate to project**:
   ```bash
   cd trascrivi-ai
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Development mode**:
   ```bash
   npm run tauri:dev
   ```

4. **Build for production**:
   ```bash
   npm run tauri:build
   ```

### API Configuration
1. Launch the application
2. Click the settings icon (⚙️) in the top-right
3. Enter your Gemini API key
4. Save and start recording

## Usage

### Recording
1. Set your Gemini API key in settings
2. Click the record button (▶️) to start recording
3. Speak naturally - transcription appears in real-time
4. Click stop (⏹️) to end recording and save

### Content Analysis
1. Select a transcription from the sidebar
2. Click "Analyze Structure" to automatically detect chapters
3. View organized content with titles and sections

### Export
1. Select a transcription
2. Click "Export" and choose format (PDF, DOCX, TXT, Markdown)
3. Configure options (timestamps, chapters)
4. Export opens automatically when complete

## Architecture Details

### Audio Processing Pipeline
1. **Capture**: cpal captures raw audio samples from microphone
2. **Preprocessing**: Noise reduction and normalization in Rust
3. **Chunking**: Audio divided into optimal segments for API
4. **Streaming**: Continuous sending to Gemini Speech-to-Text
5. **Processing**: Content analysis for intelligent structuring
6. **UI Updates**: Real-time events sent to React frontend

### Real-time Communication
- **Tauri Events**: WebSocket-like communication between Rust and React
- **State Synchronization**: Zustand store manages frontend state
- **Error Handling**: Comprehensive error recovery throughout pipeline

### Data Persistence
- **Local Storage**: JSON files in app config directory
- **Session Management**: Automatic saving of transcriptions
- **Cross-platform Paths**: Platform-specific data directories

## Platform Support

- **Windows**: Native Windows executable
- **macOS**: Native macOS app bundle
- **Linux**: AppImage (with additional audio dependencies)

## Performance Optimizations

- **Efficient Buffering**: Minimal latency audio processing
- **Batch API Calls**: Intelligent chunking reduces API usage
- **Local Caching**: Avoid re-processing of existing content
- **Memory Management**: Optimized for long recording sessions

## Development Notes

### Adding New Features
1. Define data models in `src-tauri/src/models.rs`
2. Implement Rust backend logic in appropriate module
3. Add Tauri commands in `main.rs`
4. Create React components and update store
5. Add TypeScript types in `src/types/`

### Testing
- Rust: `cargo test` in `src-tauri/`
- Frontend: `npm test`
- Integration: `npm run tauri:dev` for full testing

### Building for Distribution
- Development: `npm run tauri:dev`
- Production build: `npm run tauri:build`
- Cross-compilation: Configure in `src-tauri/tauri.conf.json`

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]
# LaTeX Editor - Project Summary

## Overview
**LaTeX Editor** is a modern desktop application that provides a real-time LaTeX editing and compilation environment. Built with Tauri, React, and Rust, it combines a powerful web-based editor with native system performance to deliver an integrated LaTeX development experience.

## Core Functionality
The application features a split-pane interface:
- **Left Panel**: A Monaco-based code editor with custom LaTeX syntax highlighting and language support
- **Right Panel**: Real-time PDF preview of the compiled document

Users can write LaTeX code in the editor and compile it to PDF with a single button click. The compilation happens instantly, and the generated PDF is displayed in the preview pane without leaving the application.

## Technical Architecture

### Frontend Stack
- **Framework**: React 19.1.0 with Vite as the build tool
- **Editor**: Monaco Editor (@monaco-editor/react ^4.7.0) - the same editor engine that powers VS Code
- **UI**: Custom-built responsive layout with a split-view design
- **Language Support**: Custom LaTeX language configuration including:
  - Syntax highlighting for LaTeX commands, comments, delimiters, and brackets
  - Auto-closing pairs for brackets and dollar signs
  - Bracket matching and surrounding pairs
  - Line comment support with `%` character

### Backend Stack
- **Runtime**: Tauri 2 - provides a lightweight Rust-based native app wrapper
- **LaTeX Engine**: Tectonic - a modern, self-contained LaTeX/XeTeX engine
- **Compilation Pipeline**:
  1. Frontend sends LaTeX code to Rust backend via Tauri's IPC mechanism
  2. Backend writes code to a temporary `.tex` file in the system temp directory (`/tmp/tauri_latex_build/`)
  3. Tectonic CLI is invoked to compile the `.tex` file
  4. Generated PDF is read as binary data and returned to the frontend
  5. Frontend converts the binary data to a Blob URL and displays it in an iframe

### Build System
- **Frontend**: Vite with React plugin and Monaco Editor plugin
- **Backend**: Cargo (Rust package manager)
- **Integration**: Tauri CLI orchestrates both build processes
- **Development Mode**: Hot-reload enabled for both frontend and backend

## Key Features

### 1. Custom LaTeX Language Support
The application implements a complete Monaco language configuration for LaTeX:
- Tokenization for keywords (commands starting with `\`), comments, delimiters, and brackets
- Auto-closing pairs for `{}`, `[]`, `()`, and `$` (math mode)
- Proper bracket matching and surrounding pair support

### 2. Integrated Compilation
- One-click compilation with visual feedback (loading state, status messages)
- Error handling with detailed error messages from Tectonic
- Compilation logs displayed in the UI toolbar
- PDF size reporting for successful compilations

### 3. Default Template
Ships with a pre-configured LaTeX template that includes:
- Article document class
- AMS Math package
- Title, author, and sections
- Mathematical formula example (Einstein's E=mc²)

### 4. Cross-Platform Desktop App
Built with Tauri 2, the app:
- Has a small bundle size compared to Electron-based editors
- Uses native system APIs for file operations
- Runs on macOS, Windows, and Linux (targets "all" platforms)
- Has a native window with an 800x600 default size

## Project Structure

```
latex-editor/
├── src/                      # React frontend source
│   ├── App.jsx              # Main application component
│   ├── App.css              # Application styles
│   └── main.jsx             # React entry point
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Tauri app with compile_latex command
│   │   └── lib.rs           # Library entry point
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
├── public/                  # Static assets
├── index.html               # HTML entry point
├── package.json             # Node dependencies and scripts
└── vite.config.js           # Vite configuration
```

## Dependencies

### Frontend Dependencies
- `react` & `react-dom` (19.1.0): UI framework
- `@monaco-editor/react` (4.7.0): Code editor component
- `@tauri-apps/api` (2.9.1): Tauri frontend API
- `@tauri-apps/plugin-opener` (2): Native file/URL opener

### Backend Dependencies
- `tauri` (2.x): Core Tauri framework
- `tauri-plugin-opener` (2.x): File opener plugin
- `serde` & `serde_json` (1.x): JSON serialization
- `tauri-build` (2.x): Build-time dependency

### External Dependencies
- **Tectonic**: Must be installed separately via Homebrew (`brew install tectonic`)

## Use Cases

1. **Quick LaTeX Prototyping**: Test LaTeX snippets and formulas without a full LaTeX distribution
2. **Educational Tool**: Learn LaTeX with immediate visual feedback
3. **Document Authoring**: Write simple LaTeX documents with live preview
4. **Formula Testing**: Experiment with mathematical equations and see results instantly

## Development Workflow

### Available Commands
- `npm run dev`: Start Vite dev server (frontend only)
- `npm run build`: Build frontend for production
- `npm run preview`: Preview production build
- `npm run tauri`: Access Tauri CLI commands
- `npm run tauri dev`: Run full app in development mode (recommended)
- `npm run tauri build`: Create production bundle

### Current State
The project is in **version 0.1.0** (early development). The core functionality is implemented and working:
- ✅ LaTeX editor with syntax highlighting
- ✅ PDF compilation via Tectonic
- ✅ Real-time preview
- ✅ Error handling and user feedback

## Technical Highlights

1. **Efficient IPC**: Uses Tauri's efficient Rust-JavaScript bridge for communication
2. **Memory Efficient**: Binary PDF data is transferred directly without intermediate encodings
3. **Isolated Build Environment**: Uses temporary directories to avoid conflicts
4. **Modern Tooling**: Leverages cutting-edge tools (Vite, React 19, Tauri 2)
5. **Type Safety**: Rust backend provides memory safety and type safety
6. **Custom Language**: Full Monaco language configuration for LaTeX

## Future Potential

While not currently implemented, the architecture supports:
- File system operations (open/save LaTeX documents)
- Custom Tectonic configurations
- Package management integration
- Multi-file LaTeX projects
- Export options (different PDF settings)
- Syntax error highlighting
- Auto-completion for LaTeX commands
- Document templates library

## Conclusion

This LaTeX Editor represents a modern approach to LaTeX document creation, combining the power of a native desktop application with the flexibility of web technologies. It provides a streamlined, lightweight alternative to heavier LaTeX editors while maintaining the essential features needed for document creation and preview.

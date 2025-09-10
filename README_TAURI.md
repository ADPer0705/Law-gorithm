# Law-gorithm - Tauri Version

## Overview
This is the Tauri-based version of Law-gorithm, a privacy-first legal AI application.

## Project Structure
```
law-gorithm/
├── src-tauri/              # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── lib.rs          # Library exports
│   │   ├── config/         # Configuration management
│   │   ├── services/       # Core services (anonymization, RAG, AI)
│   │   ├── processors/     # Document processing (PDF to MD, chunking)
│   │   ├── storage/        # Vector database management
│   │   └── utils/          # Utility functions
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── src/                    # Frontend (HTML/CSS/JS)
│   ├── index.html          # Main HTML file
│   ├── styles.css          # Styles
│   ├── app.js              # Frontend JavaScript
│   └── components/         # Frontend components
├── python_modules/         # Python code for RAG operations
│   ├── rag_processor.py    # Python RAG implementation
│   ├── requirements.txt    # Python dependencies
│   └── pyproject.toml      # Python package config
├── public/                 # Static assets
└── docs/                   # Documentation
```

## Setup Instructions

1. Install Rust and Tauri CLI
2. Install Node.js and npm
3. Install Python 3.8+
4. Run `npm install` in the root directory
5. Run `pip install -r python_modules/requirements.txt`
6. Run `npm run tauri dev` to start development

## Features
- PDF to Markdown conversion using Tesseract
- Text anonymization with NER
- RAG-based document processing
- Vector database for embeddings
- Google AI Studio integration
- Privacy-first design

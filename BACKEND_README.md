# Backend Development Guide - RAG & Document Processing

## 🎯 **Backend Team Responsibilities**

This guide is specifically for the **Backend Team** focusing on:
- **RAG (Retrieval Augmented Generation)** system implementation
- **Document Processing** pipeline (PDF, OCR, text extraction)
- **Vector Database** operations and similarity search
- **Privacy & Anonymization** services
- **API Integration** with AI/LLM services

## 🏗️ **Architecture Overview**

The backend uses a **hybrid Rust + Python architecture**:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Rust Backend  │    │  Python RAG     │
│   (Web UI)      │───▶│   (Tauri App)   │───▶│   (Processing)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │   SQLite + VSS  │
                       │  (Vector Store) │
                       └─────────────────┘
```

### **Component Breakdown**
- **Rust Services**: High-performance file I/O, OCR, database operations
- **Python RAG**: Machine learning, embeddings, similarity search
- **SQLite + VSS**: Local vector database with full-text search
- **Tauri Commands**: Bridge between frontend and backend services

## 📂 **Backend File Structure**

### **Rust Backend (`src-tauri/src/`)**
```
src-tauri/src/
├── main.rs                    # Tauri app entry point & command definitions
├── lib.rs                     # Library exports
│
├── config/                    # Configuration management
│   ├── mod.rs                 # Config module exports
│   ├── ai_config.rs           # AI/LLM settings
│   ├── db_config.rs           # Database configuration
│   ├── ocr_config.rs          # OCR settings
│   ├── processing_config.rs   # Processing parameters
│   ├── python_config.rs       # Python integration config
│   └── *.toml                 # TOML configuration files
│
├── services/                  # Core business logic
│   ├── ai.rs                  # AI/LLM service integration
│   ├── api.rs                 # External API handling
│   ├── app.rs                 # Application state management
│   ├── ocr.rs                 # 🔥 OCR processing (bundled Tesseract)
│   ├── rag.rs                 # RAG coordination service
│   ├── anonymization.rs       # 🔥 PII anonymization
│   ├── embedding.rs           # Vector embedding generation
│   ├── file_storage.rs        # File management
│   ├── ingestion.rs           # Document ingestion pipeline
│   └── retrieval.rs           # Information retrieval
│
├── processors/                # Document processing
│   ├── chunking.rs            # 🔥 General text chunking
│   ├── chunking_legal.rs      # Legal document chunking
│   ├── chunking_contract.rs   # Contract-specific chunking
│   └── pdf_to_md.rs           # PDF to Markdown conversion
│
├── storage/                   # Database layer
│   ├── database.rs            # 🔥 SQLite management
│   └── vector_db.rs           # 🔥 Vector operations (VSS)
│
└── utils/                     # Utility functions
    ├── mod.rs                 # Utility exports
    └── file_utils.rs          # File system operations
```

### **Python RAG (`python_modules/`)**
```
python_modules/
├── rag_processor.py           # 🔥 Main RAG implementation
├── requirements.txt           # Python dependencies
└── pyproject.toml             # Python project configuration
```

## 🔥 **Key Backend Components (Priority)**

### **1. OCR Service (`src-tauri/src/services/ocr.rs`)**
- **Status**: ✅ Working - Uses bundled Tesseract binaries
- **Purpose**: Extract text from PDF images and scanned documents
- **Key Features**:
  - Bundled Tesseract (`vendor/tesseract-linux/`)
  - Multi-language support
  - Markdown formatting output
  - Error handling and logging

```rust
// Example usage
let ocr_service = OcrService::new(ocr_config);
let markdown_text = ocr_service.extract_text_from_image(&image_path)?;
```

### **2. Vector Database (`src-tauri/src/storage/vector_db.rs`)**
- **Status**: ✅ Working - SQLite with VSS extension
- **Purpose**: Store and search document embeddings
- **Key Features**:
  - Vector similarity search
  - Embedding storage and retrieval
  - Cosine similarity ranking

```rust
// Example usage
let embedding = vec![0.1, 0.2, 0.3]; // From Python RAG
let similar_docs = db.search_similar(&embedding, 5)?;
```

### **3. RAG Processor (`python_modules/rag_processor.py`)**
- **Status**: ✅ Working - ChromaDB + SentenceTransformers
- **Purpose**: Generate embeddings and perform similarity search
- **Key Features**:
  - Sentence-BERT embeddings (`all-MiniLM-L6-v2`)
  - ChromaDB vector store
  - Batch processing support

```python
# Example usage
rag = RagProcessor()
rag.add_chunks(text_chunks)
results = rag.search("contract terms", n_results=5)
```

### **4. Document Processing (`src-tauri/src/processors/`)**
- **Status**: ✅ Implemented - Multiple chunking strategies
- **Purpose**: Break documents into searchable chunks
- **Key Features**:
  - Legal-specific chunking
  - Contract clause detection
  - Configurable chunk sizes and overlap

### **5. Anonymization Service (`src-tauri/src/services/anonymization.rs`)**
- **Status**: ⚠️ Stub implementation - Needs development
- **Purpose**: Remove PII before processing
- **Required Features**:
  - Named Entity Recognition (NER)
  - PII detection and replacement
  - Pseudonym mapping management

## 🚀 **Development Workflow**

### **Setting Up Backend Development**
```bash
# 1. Install Rust dependencies
cd src-tauri
cargo build

# 2. Install Python dependencies
cd ../python_modules
pip install -r requirements.txt

# 3. Run tests
cd ../src-tauri
cargo test

# 4. Start development server
cargo run  # or npm run tauri dev from root
```

### **Working with the RAG System**
```bash
# Test Python RAG independently
cd python_modules
python3 -c "
from rag_processor import RagProcessor
rag = RagProcessor()
rag.add_chunks(['Sample legal text', 'Contract clause example'])
print(rag.search('legal'))
"
```

### **Database Management**
```bash
# Check SQLite database (after running app)
sqlite3 ./data/law-gorithm.db
.tables  # Show tables
.schema documents  # Show document schema
```

## 🔧 **Backend API Commands**

The Rust backend exposes these Tauri commands for frontend integration:

### **File Operations**
```javascript
// Upload and process document
await invoke('upload_document', { 
    filePath: '/path/to/document.pdf',
    fileName: 'contract.pdf',
    fileSize: 1024000 
});

// Get document list
const documents = await invoke('get_documents');
```

### **RAG Operations**
```javascript
// Search documents
const results = await invoke('search_documents', {
    query: 'contract terms',
    limit: 5
});

// Get document chunks
const chunks = await invoke('get_document_chunks', {
    documentId: 'doc123'
});
```

### **AI Integration**
```javascript
// Generate summary
const summary = await invoke('generate_summary', {
    documentId: 'doc123'
});

// Ask question about document
const answer = await invoke('ask_question', {
    documentId: 'doc123',
    query: 'What are the payment terms?'
});
```

## ⚠️ **Current Known Issues**

### **LibSoup Conflict**
- **Issue**: Development mode crashes due to libsoup2/libsoup3 conflict
- **Status**: Compilation works, runtime issue on some Linux systems
- **Workaround**: Use `cargo build --release` for production builds

### **Missing Features for MVP**
1. **Anonymization Service**: Stub implementation needs NER integration
2. **Error Handling**: Improve error propagation from Python to Rust
3. **Configuration**: Runtime configuration without rebuilding
4. **Logging**: Comprehensive logging system for debugging

## 🔄 **Integration Points**

### **Rust ↔ Python Communication**
```rust
// In Rust service
use pyo3::prelude::*;

pub fn call_python_rag(&self, chunks: Vec<String>) -> PyResult<Vec<String>> {
    Python::with_gil(|py| {
        let rag_module = py.import("rag_processor")?;
        let rag_class = rag_module.getattr("RagProcessor")?;
        let rag_instance = rag_class.call0()?;
        rag_instance.call_method1("add_chunks", (chunks,))
    })
}
```

### **Database ↔ Vector Store Sync**
- SQLite stores document metadata and text
- Python RAG manages vector embeddings
- Coordination through document IDs

## 📋 **Backend Development Checklist**

### **Immediate Tasks (High Priority)**
- [ ] Complete anonymization service with NER
- [ ] Improve Python-Rust error handling
- [ ] Add comprehensive logging
- [ ] Create backend API tests
- [ ] Document configuration options

### **Medium Priority**
- [ ] Optimize chunking strategies for legal documents
- [ ] Add support for more file formats (DOCX, TXT)
- [ ] Implement caching for frequently accessed embeddings
- [ ] Add batch processing capabilities

### **Future Enhancements**
- [ ] Support for multiple embedding models
- [ ] Advanced legal document parsing
- [ ] Real-time document processing updates
- [ ] Backup and recovery mechanisms

## 🐛 **Debugging & Testing**

### **Rust Backend Testing**
```bash
cd src-tauri

# Run all tests
cargo test

# Test specific module
cargo test storage::database

# Run with output
cargo test -- --nocapture
```

### **Python RAG Testing**
```bash
cd python_modules

# Install test dependencies
pip install pytest

# Run tests (create tests first)
pytest test_rag_processor.py
```

### **Integration Testing**
```bash
# Full application test
npm run tauri dev  # Start app and test manually
```

## 📞 **Support & Communication**

- **Backend Issues**: Focus on Rust compilation, Python integration, database operations
- **RAG Problems**: Vector search accuracy, embedding generation, similarity matching
- **OCR Issues**: Text extraction quality, bundled Tesseract configuration
- **Performance**: Memory usage, processing speed, file handling efficiency

---

**🔥 Priority Focus**: OCR service, vector database, RAG processor, and anonymization service are the core components that need the most attention for a working MVP.

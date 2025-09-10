# Law-gorithm Data Flow & Component Interaction

## System Overview

Law-gorithm operates as a **unified single-container system** with clear separation between **Frontend**, **Backend**, and **RAG Engine** components, while integrating with **Google AI Studio API** for LLM processing.

## Component Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                            LAW-GORITHM CONTAINER                                │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  🌐 FRONTEND (Web UI)                    🔧 BACKEND (API Server)               │
│  ┌─────────────────────────────┐         ┌─────────────────────────────────┐   │
│  │ • src/ui/app.py             │ ◄────── │ • src/main.py (FastAPI)         │   │
│  │ • static/ (CSS, JS, Assets) │         │ • src/api/routes.py             │   │
│  │ • templates/ (HTML)         │         │ • Session Management            │   │
│  │ • Responsive Design         │         │ • Request Orchestration         │   │
│  │ • File Upload Interface     │         │ • Privacy Validation            │   │
│  │ • Query Interface           │         │ • Audit Logging                 │   │
│  └─────────────────────────────┘         └─────────────────────────────────┘   │
│                                                      │                         │
│                                                      ▼                         │
│  🧠 RAG ENGINE (Embedded Services)                                             │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                        PROCESSING PIPELINE                              │   │
│  │                                                                         │   │
│  │  📄 Document Service          🔒 Anonymization Service                  │   │
│  │  • src/services/document_     • src/services/anonymization_service.py  │   │
│  │    service.py                 • NER Models (spaCy)                     │   │
│  │  • OCR Processing             • Entity Detection & Classification       │   │
│  │  • PDF → Markdown             • Pseudonym Generation                   │   │
│  │  • Image Processing           • Local Dictionary Management             │   │
│  │                                                                         │   │
│  │  🗄️ RAG Service               🤖 AI Service                             │   │
│  │  • src/services/rag_service.py • src/services/ai_service.py            │   │
│  │  • Vector Database            • Google AI Studio Integration           │   │
│  │  • ChromaDB/FAISS            • Gemini API Client                       │   │
│  │  • Semantic Search           • Request/Response Handling               │   │
│  │  • Context Retrieval         • Rate Limiting & Error Handling          │   │
│  │                                                                         │   │
│  │  🔓 De-anonymization Service                                           │   │
│  │  • src/services/deanonymization_service.py                            │   │
│  │  • Pseudonym Dictionary Lookup                                         │   │
│  │  • Response Translation                                                 │   │
│  │  • Result Mapping                                                       │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼ (Anonymized data only)
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         🌐 EXTERNAL SERVICES                                    │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ Google AI Studio API (Gemini)                                           │   │
│  │ • Receives ONLY anonymized chunks                                       │   │
│  │ • Processes: "PARTY_1 signed AGREEMENT_TYPE_A on CONTRACT_DATE"         │   │
│  │ • Never sees real names, dates, or identifying information              │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Data Flow Scenarios

### 1. Document Ingestion Workflow

```
📱 USER UPLOADS PDF/IMAGE
│
▼ HTTP POST /api/v1/process-documents
┌─────────────────────────────────────────────────┐
│ 🌐 Frontend (src/ui/app.py)                     │
│ • File validation                               │
│ • Session creation                              │
│ • Upload progress tracking                      │
└─────────────────────────────────────────────────┘
│
▼ Internal API Call
┌─────────────────────────────────────────────────┐
│ 🔧 Backend (src/api/routes.py)                  │
│ • Route: POST /process-documents                │
│ • Authentication & validation                   │
│ • Session management                            │
│ • Orchestrates service calls                    │
└─────────────────────────────────────────────────┘
│
▼ Service Call
┌─────────────────────────────────────────────────┐
│ 📄 Document Service                             │
│ • OCR: PDF/Image → Text                         │
│ • Format conversion: Text → Markdown            │
│ • Document structure analysis                   │
│ • CONTAINS: Real PII at this stage              │
└─────────────────────────────────────────────────┘
│
▼ Service Call
┌─────────────────────────────────────────────────┐
│ 🔒 Anonymization Service                        │
│ • NER: Extract entities (PERSON, ORG, DATE...)  │
│ • Generate contextual pseudonyms                │
│ • Create local dictionary mapping               │
│ • Replace PII: "John Smith" → "PARTY_1"         │
│ • RESULT: Anonymized markdown                   │
└─────────────────────────────────────────────────┘
│
▼ Service Call  
┌─────────────────────────────────────────────────┐
│ 🧠 RAG Service                                   │
│ • Chunk anonymized text                         │
│ • Generate embeddings                           │
│ • Store in vector database                      │
│ • Index for semantic search                     │
│ • NO PII in chunks                              │
└─────────────────────────────────────────────────┘
│
▼ Response Chain
📱 USER RECEIVES: "Document processed successfully with privacy protection"
```

### 2. Query Processing Workflow

```
📱 USER ASKS: "What are my obligations under this contract?"
│
▼ HTTP POST /api/v1/query
┌─────────────────────────────────────────────────┐
│ 🌐 Frontend (src/ui/app.py)                     │
│ • Query validation                              │
│ • Loading indicators                            │
│ • Response formatting                           │
└─────────────────────────────────────────────────┘
│
▼ Internal API Call
┌─────────────────────────────────────────────────┐
│ 🔧 Backend (src/api/routes.py)                  │
│ • Route: POST /query                            │
│ • Session validation                            │
│ • Query preprocessing                           │
│ • Orchestrates RAG → AI → De-anonymization      │
└─────────────────────────────────────────────────┘
│
▼ Service Call
┌─────────────────────────────────────────────────┐
│ 🧠 RAG Service                                   │
│ • Semantic search on vector database            │
│ • Retrieve relevant anonymized chunks           │
│ • Context ranking and selection                 │
│ • RETURNS: Anonymized context chunks            │
└─────────────────────────────────────────────────┘
│
▼ Service Call (PRIVACY BOUNDARY - External API)
┌─────────────────────────────────────────────────┐
│ 🤖 AI Service → Google AI Studio                │
│ • Send ONLY anonymized chunks                   │
│ • Prompt: "Based on this PARTY_1 agreement..."  │
│ • Receive: Anonymized response                  │
│ • NO real PII transmitted                       │
└─────────────────────────────────────────────────┘
│
▼ Service Call
┌─────────────────────────────────────────────────┐
│ 🔓 De-anonymization Service                     │
│ • Look up pseudonyms in local dictionary        │
│ • Replace: "PARTY_1" → "John Smith"             │
│ • Restore context and meaning                   │
│ • Generate user-friendly response               │
└─────────────────────────────────────────────────┘
│
▼ Response Chain
📱 USER RECEIVES: "Based on your contract with ABC Corp, your key obligations are..."
```

## Component Responsibilities

### Frontend (src/ui/app.py)
- **Purpose**: User interface for non-technical users
- **Target**: Mobile-first, touch-friendly design
- **Responsibilities**:
  - File upload (drag-and-drop, camera integration)
  - Query input and display
  - Progress tracking and loading states
  - Result presentation in plain language

### Backend (src/main.py, src/api/routes.py)
- **Purpose**: Request orchestration and service coordination
- **Responsibilities**:
  - HTTP request handling (REST API)
  - Session management and user isolation
  - Service workflow orchestration
  - Privacy validation and audit logging
  - Error handling and user feedback

### RAG Engine (Embedded Services)
- **Document Service**: OCR, PDF processing, format conversion
- **Anonymization Service**: NER-based PII detection and pseudonymization
- **RAG Service**: Vector database, semantic search, context retrieval
- **AI Service**: Google AI Studio API integration (anonymized data only)
- **De-anonymization Service**: Result translation using local dictionary

### External LLM (Google AI Studio)
- **Input**: Only anonymized text chunks
- **Processing**: Question answering, summarization, analysis
- **Output**: Anonymized responses
- **Privacy**: Never sees real PII

## Privacy Protection Layers

### Layer 1: Document Processing (Local Only)
- Raw documents with PII
- OCR and text extraction
- **NEVER leaves the container**

### Layer 2: Anonymization (Local Only)
- NER entity detection
- Pseudonym generation
- Dictionary creation and storage
- **NEVER transmitted externally**

### Layer 3: Vector Storage (Local Only)
- Anonymized text chunks only
- Semantic embeddings
- Search indices
- **No PII in any stored data**

### Layer 4: External API (Anonymized Only)
- Google AI Studio receives anonymized chunks
- Processes questions about pseudonymized entities
- Returns anonymized responses
- **Complete PII isolation**

### Layer 5: Response Translation (Local Only)
- Map pseudonyms back to real entities
- Restore context for user
- Generate final user-friendly response
- **Local dictionary never exposed**

## Resource Optimization

### Configuration Modes
```python
# src/config/settings.py
RESOURCE_MODES = {
    "LIGHT": {
        "ner_model": "en_core_web_sm",          # Small model
        "vector_store": "memory",                # In-memory only
        "chunk_size": 256,                       # Smaller chunks
        "max_concurrent": 1,                     # Single thread
        "cache_size": 50                         # Limited cache
    },
    "STANDARD": {
        "ner_model": "en_core_web_trf",          # Large model
        "vector_store": "disk",                  # Persistent storage
        "chunk_size": 512,                       # Standard chunks
        "max_concurrent": 2,                     # Multi-thread
        "cache_size": 200                        # Standard cache
    }
}
```

This architecture ensures:
1. **Clear component separation** with defined responsibilities
2. **Efficient design** optimized for various resource constraints
3. **Complete privacy protection** with clear boundaries
4. **Scalable resource usage** from basic to powerful systems
5. **Simple deployment** in a single container

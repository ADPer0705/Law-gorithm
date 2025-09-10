# Privacy-First Architecture Documentation

## Overview

Law-gorithm v3.0 introduces a revolutionary privacy-first architecture that ensures complete PII protection through advanced NER-based pseudonymization. This Tauri-based desktop solution processes legal documents entirely within the user's environment, guaranteeing that no sensitive information ever leaves the local processing boundary.

## Core Privacy Principles

### 1. Zero PII Exposure
- **Principle**: No personally identifiable information ever leaves the user environment
- **Implementation**: All PII is replaced with contextual pseudonyms before any processing
- **Validation**: Audit logs verify no real entities are processed or transmitted

### 2. Complete Data Isolation
- **Principle**: All processing happens within the desktop application
- **Implementation**: No external API calls for sensitive data processing
- **Validation**: Network monitoring confirms only anonymized data interactions

### 3. Reversible Anonymization
- **Principle**: Results must be meaningful while maintaining privacy
- **Implementation**: Local pseudonym dictionary enables result interpretation
- **Validation**: Users can map results back to real entities without exposing PII

## Tauri Desktop Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    LAW-GORITHM TAURI APPLICATION                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  🌐 FRONTEND (Web UI)                                              │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ • HTML/CSS/JS Interface (src/index.html)                   │   │
│  │ • File Upload (PDF, Images)                                │   │
│  │ • Query Interface                                           │   │
│  │ • Native Desktop Integration                               │   │
│  │ • Responsive Design                                        │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↕ Tauri IPC                              │
│  🦀 RUST BACKEND ORCHESTRATION                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ • Tauri Commands (src-tauri/src/main.rs)                    │   │
│  │ • Request Routing & Session Management                     │   │
│  │ • Service Coordination                                      │   │
│  │ • Privacy Validation & Audit Logging                       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↓                                         │
│  📄 DOCUMENT PROCESSING PIPELINE                                   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 1️⃣ DOCUMENT INGESTION (src-tauri/src/processors/pdf_to_md.rs) │   │
│  │    ├── PDF Processing (pdf-extract crate)                   │   │
│  │    ├── OCR Engine (Tesseract crate)                         │   │
│  │    ├── Image Processing (image crate)                       │   │
│  │    └── Markdown Conversion                                  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↓                                         │
│  🔒 ANONYMIZATION ENGINE                                           │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 2️⃣ NER-BASED ANONYMIZATION (src-tauri/src/services/anonymization.rs) │
│  │    ├── NER Processing (spaCy via PyO3)                      │   │
│  │    ├── Entity Classification                               │   │
│  │    │   ├── PERSON → PARTY_1, PARTY_2, ...                 │   │
│  │    │   ├── ORG → COMPANY_A, COMPANY_B, ...                │   │
│  │    │   ├── GPE → GOVERNING_BODY_1, LOCATION_X, ...        │   │
│  │    │   ├── DATE → CONTRACT_DATE, EFFECTIVE_MONTH_YEAR      │   │
│  │    │   ├── MONEY → AMOUNT_1, FINANCIAL_FIGURE_A           │   │
│  │    │   └── CUSTOM_LEGAL → LAW_REFERENCE_1, CLAUSE_TYPE_B  │   │
│  │    ├── Pseudonym Generation & Local Dictionary            │   │
│  │    └── Privacy Validation                                  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↓                                         │
│  🧠 RAG ENGINE                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 3️⃣ VECTOR PROCESSING (src-tauri/src/services/rag.rs)       │   │
│  │    ├── Text Chunking (Anonymized Content Only)             │   │
│  │    ├── Vector Embeddings (ChromaDB via PyO3)               │   │
│  │    ├── Semantic Indexing                                    │   │
│  │    ├── Query Processing                                     │   │
│  │    └── Context Retrieval                                    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                    ↓ (Anonymized Chunks Only)                      │
│  🤖 LLM INTERFACE                                                  │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 4️⃣ GOOGLE AI STUDIO API (src-tauri/src/services/ai.rs)     │   │
│  │    ├── Gemini API Integration                              │   │
│  │    ├── Query Processing (Anonymized Data Only)             │   │
│  │    ├── Response Generation                                  │   │
│  │    └── API Rate Limiting & Error Handling                  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                    ↓ (Anonymized Response)                         │
│  🔓 DE-ANONYMIZATION ENGINE                                        │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 5️⃣ RESULT MAPPING (src-tauri/src/services/deanonymization.rs)│   │
│  │    ├── Pseudonym Dictionary Lookup                         │   │
│  │    ├── Response Translation                                 │   │
│  │    ├── Context Restoration                                  │   │
│  │    └── User-Friendly Output Generation                     │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘

📊 WORKFLOW FOR DOCUMENT INGESTION:
Frontend → Backend → Document Processing → Anonymization → RAG Chunking → Vector Store

📊 WORKFLOW FOR USER QUERIES:
Frontend Query → Backend → RAG Retrieval → Google AI (anonymized) → De-anonymization → User Response

🔒 PRIVACY BOUNDARIES:
• Everything above LLM Interface: Contains real PII (stays local)
• LLM Interface and below: Only anonymized data (can leave device)
• Local Dictionary: Never transmitted, enables result translation
│  │    └── Local Dictionary Storage (SQLite/JSON)             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↓                                         │
│  PROCESSING STACK                                                   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 🧠 RAG PROCESSING ENGINE                                    │   │
│  │    ├── Text Chunking (Anonymized Content Only)            │   │
│  │    ├── Vector Embeddings (sentence-transformers)          │   │
│  │    ├── Vector Storage (ChromaDB, FAISS)                   │   │
│  │    ├── Semantic Indexing                                   │   │
│  │    └── Retrieval System                                    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↓                                         │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 🤖 AI INTERFACE LAYER                                       │   │
│  │    ├── Query Processing (Anonymized)                       │   │
│  │    ├── Context Preparation                                 │   │
│  │    ├── Google Gemini API Integration                       │   │
│  │    ├── Response Generation                                  │   │
│  │    └── Quality Validation                                   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↓                                         │
│  OUTPUT PROCESSING STACK                                            │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 🔄 DE-ANONYMIZATION ENGINE                                  │   │
│  │    ├── Response Parsing                                     │   │
│  │    ├── Pseudonym Resolution                                │   │
│  │    ├── User-Friendly Formatting                            │   │
│  │    └── Audit Trail Generation                              │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           ↓                                         │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 📊 USER INTERFACE                                           │   │
│  │    ├── Web Interface (FastAPI + React)                     │   │
│  │    ├── API Endpoints                                       │   │
│  │    ├── Real-time Processing Status                         │   │
│  │    └── Results Display                                      │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  SUPPORTING SYSTEMS                                                 │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 🛡️ SECURITY & MONITORING                                   │   │
│  │    ├── Audit Logging                                       │   │
│  │    ├── Privacy Validation                                  │   │
│  │    ├── Performance Monitoring                              │   │
│  │    └── Error Handling                                      │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Privacy Processing Flow

### Phase 1: Document Ingestion & OCR
1. **Document Upload**: User uploads PDF/image files to local container
2. **OCR Processing**: Extract text content using local OCR engines
3. **Markdown Conversion**: Convert extracted text to structured markdown
4. **Initial Parsing**: Identify document structure and sections

### Phase 2: NER-Based Anonymization
1. **Entity Recognition**: Identify all named entities using advanced NER models
2. **Classification**: Categorize entities into privacy-sensitive types
3. **Pseudonym Generation**: Create contextual pseudonyms for each entity
4. **Dictionary Creation**: Store mappings in local, encrypted dictionary
5. **Content Replacement**: Replace all real entities with pseudonyms

### Phase 3: RAG Processing
1. **Chunk Creation**: Split anonymized content into semantic chunks
2. **Embedding Generation**: Create vector embeddings for anonymous chunks
3. **Vector Storage**: Store embeddings in local vector database
4. **Indexing**: Build searchable indexes on anonymized content

### Phase 4: Query Processing
1. **Query Anonymization**: Replace any entities in user queries with pseudonyms
2. **Semantic Retrieval**: Find relevant chunks using vector similarity
3. **Context Assembly**: Prepare anonymized context for AI processing
4. **AI Generation**: Generate responses using external AI APIs (Google Gemini)

### Phase 5: Response De-anonymization
1. **Response Parsing**: Extract entities from AI-generated responses
2. **Pseudonym Resolution**: Map pseudonyms back to real entities using local dictionary
3. **User Formatting**: Present results with real entity names
4. **Audit Recording**: Log all operations for compliance

## Entity Anonymization Strategy

### Named Entity Categories

#### 1. Person Entities
- **Detection**: Names, titles, signatures
- **Anonymization**: PARTY_1, PARTY_2, LEGAL_REPRESENTATIVE_A
- **Context Preservation**: Maintain relationships (e.g., PARTY_1's attorney)

#### 2. Organization Entities
- **Detection**: Companies, institutions, law firms
- **Anonymization**: COMPANY_A, COMPANY_B, LAW_FIRM_1
- **Context Preservation**: Maintain business relationships

#### 3. Geopolitical Entities
- **Detection**: Governments, states, cities
- **Anonymization**: GOVERNING_BODY_1, JURISDICTION_A, LOCATION_X
- **Context Preservation**: Maintain jurisdictional relationships

#### 4. Temporal Entities
- **Detection**: Dates, time periods, deadlines
- **Anonymization**: 
  - Full dates → MONTH_YEAR (e.g., January 2024)
  - Key dates → CONTRACT_DATE, EFFECTIVE_DATE, EXPIRATION_DATE
- **Context Preservation**: Maintain temporal relationships

#### 5. Financial Entities
- **Detection**: Amounts, percentages, financial terms
- **Anonymization**: AMOUNT_1, PERCENTAGE_A, FINANCIAL_FIGURE_X
- **Context Preservation**: Maintain financial relationships

#### 6. Legal Entities
- **Detection**: Laws, regulations, court cases, legal references
- **Anonymization**: LAW_REFERENCE_1, REGULATION_A, CASE_CITATION_X
- **Context Preservation**: Maintain legal precedent relationships

#### 7. Identifier Entities
- **Detection**: SSN, tax ID, account numbers, addresses
- **Anonymization**: IDENTIFIER_1, REFERENCE_NUMBER_A, ADDRESS_LOCATION_X
- **Complete Obfuscation**: No traceable patterns

## Privacy Validation Framework

### 1. Pre-Processing Validation
- Scan input documents for PII exposure risks
- Validate OCR accuracy to ensure complete text extraction
- Confirm proper document structure recognition

### 2. Anonymization Validation
- Verify all entities are properly detected and replaced
- Confirm no real names or identifiers remain in processed content
- Validate pseudonym consistency across document

### 3. Processing Validation
- Monitor all external API calls for PII leakage
- Validate that only anonymized content is transmitted
- Confirm response generation doesn't expose real entities

### 4. Output Validation
- Verify proper de-anonymization of results
- Confirm user receives meaningful, accurate information
- Validate audit trail completeness

## Security Measures

### 1. Container Isolation
- **Network Isolation**: Restricted external connections
- **Filesystem Isolation**: Sandboxed file access
- **Process Isolation**: Containerized execution environment
- **Resource Limits**: Memory and CPU constraints

### 2. Data Encryption
- **Dictionary Encryption**: Local pseudonym storage encrypted at rest
- **Transport Security**: TLS for any external communications
- **Memory Protection**: Secure memory handling for sensitive operations

### 3. Audit and Compliance
- **Complete Audit Logs**: Every operation logged with timestamps
- **Privacy Compliance**: GDPR, HIPAA, SOX compliance by design
- **Data Residency**: Complete control over data location
- **Regulatory Reporting**: Generate compliance reports

## Performance Considerations

### Resource Requirements
- **CPU**: 4+ cores for parallel NER processing
- **Memory**: 8GB+ RAM for embedding generation
- **Storage**: 20GB+ for models and temporary processing
- **Network**: Minimal bandwidth (only for AI API calls with anonymous data)

### Optimization Strategies
- **Model Caching**: Pre-load NER and embedding models
- **Batch Processing**: Process multiple documents efficiently
- **Memory Management**: Efficient cleanup of temporary data
- **Parallel Processing**: Multi-threaded anonymization and embedding

## Deployment Considerations

### Single Container Benefits
- **Simplified Deployment**: One container, one command
- **Resource Efficiency**: Optimized resource usage
- **Privacy Guarantee**: Complete data isolation
- **Portability**: Run anywhere with Docker support

### Configuration Requirements
- **Google AI API Keys**: For anonymized content processing only
- **Model Downloads**: NER and embedding models
- **Resource Allocation**: CPU and memory configuration
- **Volume Mounts**: For persistent pseudonym dictionaries

This architecture ensures that Law-gorithm provides powerful legal document analysis while maintaining absolute privacy protection, making it suitable for the most sensitive legal document processing needs.

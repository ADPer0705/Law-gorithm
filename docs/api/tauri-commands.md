# Tauri Commands API Reference - Law-gorithm

## 📋 **API Status: Updated & Working (December 2024)**

This document provides a comprehensive reference for all Tauri commands available in the Law-gorithm application. These commands bridge the JavaScript frontend with the Rust backend services.

## 🔧 **Command Categories**

### **📄 Document Management**
### **🔍 Search & Retrieval** 
### **🤖 AI & RAG Operations**
### **⚙️ Application Control**
### **📊 Status & Monitoring**

---

## 📄 **Document Management Commands**

### **`upload_document`**
Upload and process a legal document for analysis.

**JavaScript Usage:**
```javascript
const result = await window.__TAURI__.invoke('upload_document', {
    filePath: '/path/to/document.pdf',
    fileName: 'contract.pdf', 
    fileSize: 1024000
});
```

**Parameters:**
- `filePath` (string): Absolute path to the document file
- `fileName` (string): Original filename for display
- `fileSize` (number): File size in bytes

**Returns:**
```json
{
  "success": true,
  "documentId": "doc_1234567890", 
  "message": "Document uploaded successfully",
  "processingStatus": "queued"
}
```

**Rust Implementation:**
```rust
#[tauri::command]
async fn upload_document(
    state: tauri::State<'_, AppState>,
    file_path: String,
    file_name: String, 
    file_size: u64
) -> Result<serde_json::Value, String> {
    // Implementation in src/main.rs
}
```

### Anonymization Commands

#### `anonymize_text`
Applies NER-based anonymization to text content.

**Parameters:**
- `text: String` - Text to anonymize
- `config: AnonymizationConfig` - Anonymization settings

**Returns:**
- `Result<AnonymizedText, Error>` - Anonymized text with mapping dictionary

#### `deanonymize_text`
Restores original entities from anonymized text.

**Parameters:**
- `anonymized_text: String` - Text with pseudonyms
- `mapping: HashMap<String, String>` - Pseudonym to original mapping

**Returns:**
- `Result<String, Error>` - De-anonymized text

### RAG Processing

#### `process_query`
Processes a user query against the document knowledge base.

**Parameters:**
- `query: String` - User's question
- `document_id: String` - Document identifier
- `context_options: RagOptions` - Retrieval configuration

**Returns:**
- `Result<QueryResult, Error>` - Processed answer with citations

#### `chunk_document`
Splits document into semantically meaningful chunks.

**Parameters:**
- `text: String` - Document text to chunk
- `chunk_config: ChunkConfig` - Chunking parameters

**Returns:**
- `Result<Vec<String>, Error>` - Document chunks

### AI Integration

#### `generate_embedding`
Creates vector embeddings for text chunks.

**Parameters:**
- `text: String` - Text to embed
- `model_config: EmbeddingConfig` - Embedding model settings

**Returns:**
- `Result<Vec<f32>, Error>` - Vector embedding

#### `summarize_content`
Generates AI-powered summaries of document content.

**Parameters:**
- `content: String` - Content to summarize
- `summary_options: SummaryOptions` - Summarization settings

**Returns:**
- `Result<String, Error>` - Generated summary

### Configuration Commands

#### `update_settings`
Updates application configuration settings.

**Parameters:**
- `settings: AppSettings` - New configuration values

**Returns:**
- `Result<(), Error>` - Success confirmation

#### `validate_api_key`
Validates Google AI API key configuration.

**Parameters:**
- `api_key: String` - API key to validate

**Returns:**
- `Result<bool, Error>` - Validation result

## Error Handling

All commands return `Result<T, Error>` where `Error` includes:
- **Error Type**: Categorization (Validation, Processing, Network, etc.)
- **Message**: Human-readable error description
- **Details**: Technical details for debugging
- **Recovery Suggestions**: Recommended actions for resolution

## Privacy Validation

Every command includes privacy validation:
- **Input Sanitization**: PII detection in command parameters
- **Processing Audit**: Complete logging of operations
- **Output Validation**: Verification that no PII leaks in responses
- **Compliance Logging**: GDPR/HIPAA compliance tracking

## Performance Considerations

### Asynchronous Processing
All commands are async by default, preventing UI blocking:
```javascript
const result = await invoke('process_document', { filePath });
```

### Streaming Responses
Large document processing uses streaming for better UX:
```javascript
const stream = await invoke('process_large_document', { filePath });
for await (const chunk of stream) {
    updateProgress(chunk.progress);
}
```

### Caching Strategy
Intelligent caching reduces redundant processing:
- Document chunks cached locally
- Embeddings stored in vector database
- API responses cached with TTL

## Security Model

### Command Authorization
- **Permission Checks**: Runtime validation of command access
- **Context Validation**: User context verification
- **Rate Limiting**: Protection against abuse

### Data Protection
- **Encryption**: Sensitive data encrypted at rest
- **Secure IPC**: All frontend-backend communication secured
- **Audit Trail**: Complete operation logging

## Usage Examples

### Basic Document Processing
```javascript
// Upload and process document
const filePath = await openFileDialog();
const metadata = await invoke('process_document', { filePath });

// Query the processed document
const answer = await invoke('process_query', {
    query: "What are my rights?",
    documentId: metadata.id
});
```

### Advanced Configuration
```javascript
// Configure anonymization settings
await invoke('update_settings', {
    anonymization: {
        strictMode: true,
        customEntities: ['PROPRIETARY_TERM']
    }
});
```

This API reference provides the foundation for building privacy-first legal document processing applications with complete audit trails and compliance validation.

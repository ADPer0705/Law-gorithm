#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Tauri v2 uses different imports for menu
use std::sync::Mutex;
use crate::services::app::App;

mod config;
mod services;
mod storage;
mod processors;
mod utils;

struct AppState(Mutex<App>);

fn main() {
    let context = tauri::generate_context!();

    // Initialize the app
    let app = App::new().expect("Failed to initialize app");

    tauri::Builder::default()
        .manage(AppState(Mutex::new(app)))
        .invoke_handler(tauri::generate_handler![
            // File operations
            upload_file,
            list_documents,
            get_document_content,
            delete_document,
            get_storage_stats,

            // AI operations
            process_query,
            generate_summary,

            // Data operations
            get_legal_vocabulary,
            save_settings,

            // Legacy commands (keeping for compatibility)
            list_files,
            delete_file,
            delete_files,
            process_document_ocr
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
async fn upload_file(
    state: tauri::State<'_, AppState>,
    filePath: String,
    fileName: String,
    fileSize: u64
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual file upload from filePath
    // For now, return dummy success response
    println!("📁 Upload requested: {} ({})", fileName, fileSize);

    Ok(serde_json::json!({
        "success": true,
        "message": format!("File '{}' uploaded successfully", fileName),
        "file_id": format!("file_{}", fileName.replace(' ', "_")),
        "name": fileName,
        "size": fileSize
    }))
}

#[tauri::command]
async fn list_documents(
    state: tauri::State<'_, AppState>
) -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implement actual document listing from database
    // For now, return dummy documents
    println!("📋 Listing documents...");

    let dummy_documents = vec![
        serde_json::json!({
            "id": "doc_1",
            "name": "Sample Contract.pdf",
            "size": 245760,
            "uploaded_at": "2025-09-08T10:30:00Z",
            "type": "pdf",
            "processed": true
        }),
        serde_json::json!({
            "id": "doc_2",
            "name": "Legal Agreement.docx",
            "size": 189440,
            "uploaded_at": "2025-09-08T09:15:00Z",
            "type": "docx",
            "processed": false
        })
    ];

    Ok(dummy_documents)
}

#[tauri::command]
async fn get_document_content(
    state: tauri::State<'_, AppState>,
    documentId: String
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual document content retrieval
    // For now, return dummy content
    println!("📖 Getting content for document: {}", documentId);

    let dummy_content = match documentId.as_str() {
        "doc_1" => "This is a sample contract document content...",
        "doc_2" => "This is a legal agreement document content...",
        _ => "Document content not found"
    };

    Ok(serde_json::json!({
        "success": true,
        "content": dummy_content,
        "document_id": documentId
    }))
}

#[tauri::command]
async fn delete_document(
    state: tauri::State<'_, AppState>,
    documentId: String
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual document deletion
    // For now, return dummy success
    println!("🗑️ Deleting document: {}", documentId);

    Ok(serde_json::json!({
        "success": true,
        "message": format!("Document '{}' deleted successfully", documentId)
    }))
}

#[tauri::command]
async fn list_files(
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual file listing from database
    // For now, return dummy files
    println!("📋 Listing files (legacy command)...");

    let dummy_files = vec![
        serde_json::json!({
            "id": "file_1",
            "original_name": "Sample Contract.pdf",
            "size_bytes": 245760,
            "mime_type": "application/pdf",
            "uploaded_at": "2025-09-08T10:30:00Z",
            "processed": true
        })
    ];

    Ok(serde_json::json!({
        "success": true,
        "files": dummy_files
    }))
}

#[tauri::command]
async fn delete_file(
    state: tauri::State<'_, AppState>,
    file_id: String
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual file deletion
    // For now, return dummy success
    println!("🗑️ Deleting file (legacy command): {}", file_id);

    Ok(serde_json::json!({
        "success": true,
        "deleted": true
    }))
}

#[tauri::command]
async fn delete_files(
    state: tauri::State<'_, AppState>,
    file_ids: Vec<String>
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual bulk file deletion
    // For now, return dummy success
    println!("🗑️ Deleting files (legacy command): {:?}", file_ids);

    Ok(serde_json::json!({
        "success": true,
        "deleted_count": file_ids.len()
    }))
}

#[tauri::command]
async fn get_storage_stats(
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual storage statistics
    // For now, return dummy stats
    println!("📊 Getting storage stats...");

    Ok(serde_json::json!({
        "success": true,
        "stats": {
            "total_files": 2,
            "total_size_bytes": 435200,
            "last_upload": "2025-09-08T10:30:00Z"
        }
    }))
}

#[tauri::command]
async fn process_document_ocr(
    state: tauri::State<'_, AppState>,
    file_id: String
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual OCR processing
    // For now, return dummy success
    println!("🔍 Processing OCR for file: {}", file_id);

    Ok(serde_json::json!({
        "success": true,
        "message": "Document processed successfully (dummy)",
        "markdown_path": format!("/tmp/{}.md", file_id),
        "original_file": "sample.pdf"
    }))
}

#[tauri::command]
async fn process_query(
    state: tauri::State<'_, AppState>,
    query: String,
    context: Vec<serde_json::Value>
) -> Result<String, String> {
    // TODO: Implement actual AI query processing with RAG
    // For now, return dummy AI response
    println!("🤖 Processing query: {}", query);
    println!("📄 Context documents: {}", context.len());

    // Simulate processing delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let dummy_responses: Vec<&str> = vec![
        "Sure — let's walk through that. Could you share which document this relates to?",
        "Here’s a concise take: based on your question, the key issues are consent, scope, and termination.",
        "I can help analyze clauses and highlight risks. Upload the file or reference an existing document.",
    ];

    let response = if dummy_responses.is_empty() {
        "I’m ready to help. Upload a document or ask a question about your legal matter."
    } else {
        let idx = query.len() % dummy_responses.len();
        dummy_responses[idx]
    };

    Ok(response.to_string())
}
#[tauri::command]
async fn generate_summary(
    state: tauri::State<'_, AppState>,
    documentId: String
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual document summarization with AI
    // For now, return dummy summary
    println!("📝 Generating summary for document: {}", documentId);

    // Simulate processing delay
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    let dummy_summary = match documentId.as_str() {
        "doc_1" => serde_json::json!({
            "content": "This contract establishes a service agreement between two parties. Key provisions include payment terms, service delivery requirements, and termination conditions. The document contains standard liability limitations and dispute resolution clauses typical of professional service contracts.",
            "summary": "Standard service contract with payment terms and liability provisions",
            "topics": ["Contract Terms", "Payment", "Liability", "Termination"],
            "risk_level": "Medium",
            "generated_at": "2025-09-08T12:00:00Z"
        }),
        "doc_2" => serde_json::json!({
            "content": "This legal agreement outlines the terms for a business partnership. It covers profit sharing, decision-making authority, and exit strategies. The document includes non-compete clauses and confidentiality requirements.",
            "summary": "Business partnership agreement with profit sharing and exit terms",
            "topics": ["Partnership", "Profit Sharing", "Non-compete", "Confidentiality"],
            "risk_level": "Low",
            "generated_at": "2025-09-08T12:05:00Z"
        }),
        _ => serde_json::json!({
            "content": "Document summary not available. Please ensure the document has been properly processed.",
            "summary": "Summary unavailable",
            "topics": [],
            "risk_level": "Unknown",
            "generated_at": "2025-09-08T12:00:00Z"
        })
    };

    Ok(dummy_summary)
}

#[tauri::command]
async fn get_legal_vocabulary(
    state: tauri::State<'_, AppState>
) -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implement actual vocabulary loading from database/config
    // For now, return dummy legal vocabulary
    println!("📚 Loading legal vocabulary...");

    let dummy_vocabulary = vec![
        serde_json::json!({
            "term": "Breach of Contract",
            "definition": "Failure to perform any term of a contract, written or oral, without a legitimate legal excuse.",
            "category": "contracts"
        }),
        serde_json::json!({
            "term": "Due Diligence",
            "definition": "The process of investigating and evaluating a potential investment or business deal.",
            "category": "corporate"
        }),
        serde_json::json!({
            "term": "Intellectual Property",
            "definition": "Creations of the mind such as inventions, designs, software code, and artistic works.",
            "category": "property"
        }),
        serde_json::json!({
            "term": "Liability",
            "definition": "Legal responsibility for one's actions or omissions that cause harm to another party.",
            "category": "litigation"
        }),
        serde_json::json!({
            "term": "Force Majeure",
            "definition": "A clause that frees both parties from liability in the event of extraordinary circumstances.",
            "category": "contracts"
        }),
        serde_json::json!({
            "term": "Indemnification",
            "definition": "A contractual provision where one party agrees to compensate the other for certain losses.",
            "category": "contracts"
        }),
        serde_json::json!({
            "term": "Consideration",
            "definition": "Something of value exchanged between parties to a contract that induces them to enter into the agreement.",
            "category": "contracts"
        }),
        serde_json::json!({
            "term": "Fiduciary Duty",
            "definition": "A legal obligation to act in the best interests of another party, typically involving trust and confidence.",
            "category": "corporate"
        })
    ];

    Ok(dummy_vocabulary)
}

#[tauri::command]
async fn save_settings(
    state: tauri::State<'_, AppState>,
    settings: serde_json::Value
) -> Result<serde_json::Value, String> {
    // TODO: Implement actual settings persistence
    // For now, just log and return success
    println!("⚙️ Saving settings: {}", settings);

    Ok(serde_json::json!({
        "success": true,
        "message": "Settings saved successfully",
        "settings": settings
    }))
}



#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod services;
mod utils;

fn main() {
    if let Err(e) = crate::config::get_app_directories() {
        eprintln!("Warning: Failed to initialize application directories: {}", e);
    }
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_files,
            update_files,
            query,
            delete_all_user_data,
        ])
        .run(context)
        .expect("error while running tauri application");
}

// ---------- FILE MANAGEMENT ----------//

// List all files linked in the file_links dir
#[tauri::command]
async fn list_files() -> Result<Vec<serde_json::Value>, String> {
    let file_links = crate::services::file_mgmt::list_file_links()
        .map_err(|e| format!("Failed to list file links: {}", e))?;

    let files: Vec<serde_json::Value> = file_links.into_iter().map(|file_link| {
        serde_json::json!({
            "id": file_link.id,
            "name": file_link.original_name,
            "original_path": file_link.original_path
        })
    }).collect();

    Ok(files)
}

// Update the files used as context for the system
#[tauri::command]
async fn update_files(files: Vec<serde_json::Value>) -> Result<serde_json::Value, String> {
    let existing_links = crate::services::file_mgmt::list_file_links()
        .map_err(|e| e.to_string())?;
    
    // Extract file IDs from the input
    let file_ids: Vec<String> = files.iter()
        .filter_map(|file| file.get("id").and_then(|id| id.as_str()).map(String::from))
        .collect();
    
    // Extract file paths from the input
    let file_paths: Vec<(String, String)> = files.iter()
        .filter_map(|file| {
            let id = file.get("id").and_then(|id| id.as_str())?;
            let path = file.get("path").and_then(|path| path.as_str())?;
            Some((id.to_string(), path.to_string()))
        })
        .collect();

    // --- Remove unnecessary files ---
    for link in &existing_links {
        if !file_ids.contains(&link.id) {
            // Remove link from storage
            crate::services::file_mgmt::remove_file_link(&link.id)
                .map_err(|e| e.to_string())?;

            // Remove associated markdown
            crate::services::file_mgmt::remove_markdown_for_file(&link.id)
                .map_err(|e| e.to_string())?;
        }
    }

    // --- Add missing files ---
    for (file_id, file_path) in &file_paths {
        if !existing_links.iter().any(|link| &link.id == file_id) {
            // Convert to markdown through OCR
            crate::services::file_mgmt::generate_markdown_for_file(file_id, file_path)
                .map_err(|e| e.to_string())?;

            // Register link in storage
            crate::services::file_mgmt::add_file_link(file_id, file_path)
                .map_err(|e| e.to_string())?;
        }
    }

    // --- Update embeddings after sync ---
    crate::services::embeddings_db::generate_embeddings(&file_ids)
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({ "success": true }))
}


// ---------- QUERY ----------//

// Query: user query through chat interface
#[tauri::command]
async fn query(query: String) -> Result<serde_json::Value, String> {
    let query_embedding = crate::services::retrieval::embed_query(&query).map_err(|e| e.to_string())?;
    let chunks = crate::services::retrieval::retrieve_chunks_with_embedding(&query_embedding, None).map_err(|e| e.to_string())?;
    let llm_response = crate::services::llm::call_llm_api(&query, &chunks).map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "success": true,
        "response": llm_response,
        "relevant_sources": chunks.iter().filter_map(|c| c.source.clone()).collect::<Vec<_>>(),
    }))
}

// ---------- USER DATA SANITATION ----------//

// Delete all user data
#[tauri::command]
async fn delete_all_user_data() -> Result<serde_json::Value, String> {
    crate::services::user_data_sanitization::delete_all_user_data()
        .map_err(|e| format!("Failed to delete all user data: {}", e))?;
    Ok(serde_json::json!({ "success": true }))
}

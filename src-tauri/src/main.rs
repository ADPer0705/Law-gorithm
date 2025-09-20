#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


// ---------- MODULE IMPORTS ----------//

mod config;    // Application Directory Config
mod services;  // Core services: file management, embeddings database management, queries, user data sanitization and anonymization
mod utils;     // Utility functions: ocr, ner, embeddings, chunking, directory paths

// ---------- METHODS IMPORTS ----------//
use crate::services::file_mgmt::{
    list_file_links,                                                // listing all files
    remove_file_link,                                               // removing a file link
    add_file_link,                                                  // adding a file link
    generate_markdown_for_file,                                     // generating markdown via OCR
    remove_markdown_for_file,                                       // removing markdown for a file
};
use crate::services::embeddings_db::{
    populate_database_from_markdown,                                // regenerating embeddings database
};
use crate::services::query::query;                                  // handling user queries
use crate::services::user_data_sanitization::clear_user_data;       // clearing all user data




// ---------- MAIN ENTRY POINT ----------//

fn main() {
    // Initialize application directories at startup
    if let Err(e) = crate::config::get_app_directories() {
        eprintln!("Warning: Failed to initialize application directories: {}", e);
    }

    // Generate Tauri context from configuration
    let context = tauri::generate_context!();

    // Build and run Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_files,             // List all files added to the app
            update_files,           // Sync/Update files for context
            query,                  // Query system through chat interface
            delete_all_user_data,   // Purge all user data
        ])
        .run(context)
        .expect("error while running tauri application");
}




// ========================= FILE MANAGEMENT ========================= //

// List all files currently registered in the file_links directory
#[tauri::command]
async fn list_files() -> Result<Vec<serde_json::Value>, String> {
    let file_links = list_file_links();
        .map_err(|e| format!("Failed to list file links: {}", e))?;

    // Transform into JSON representation for frontend
    let files: Vec<serde_json::Value> = file_links.into_iter().map(|file_link| {
        serde_json::json!({
            "id": file_link.id,
            "name": file_link.original_name,
            "original_path": file_link.original_path
        })
    }).collect();

    Ok(files)
}

// Update files used by the system (add/remove) and regenerate embeddings
#[tauri::command]
async fn update_files(files: Vec<serde_json::Value>) -> Result<serde_json::Value, String> {
    // Retrieve existing links
    let existing_links = list_file_links()
        .map_err(|e| e.to_string())?;
    
    // Extract IDs and paths from the incoming request
    let file_ids: Vec<String> = files.iter()
        .filter_map(|file| file.get("id").and_then(|id| id.as_str()).map(String::from))
        .collect();

    let file_paths: Vec<(String, String)> = files.iter()
        .filter_map(|file| {
            let id = file.get("id").and_then(|id| id.as_str())?;
            let path = file.get("path").and_then(|path| path.as_str())?;
            Some((id.to_string(), path.to_string()))
        })
        .collect();

    // Remove files no longer needed
    for link in &existing_links {
        if !file_ids.contains(&link.id) {
            remove_file_link(&link.id)
                .map_err(|e| e.to_string())?;
            remove_markdown_for_file(&link.id)
                .map_err(|e| e.to_string())?;
        }
    }

    // Add new files and generate Markdown via OCR
    for (file_id, file_path) in &file_paths {
        if !existing_links.iter().any(|link| &link.id == file_id) {
            generate_markdown_for_file(file_id, file_path)
                .map_err(|e| e.to_string())?;
            add_file_link(file_id, file_path)
                .map_err(|e| e.to_string())?;
        }
    }

    // Update embeddings database asynchronously
    populate_database_from_markdown().await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({ "success": true }))
}




// ========================= QUERY HANDLER ========================= //

// Handle user queries through chat interface
#[tauri::command]
async fn query(query: String) -> Result<serde_json::Value, String> {
    match query(query).await {
        Ok(result) => Ok(serde_json::json!({
            "success": true,
            "response": result.get("response").unwrap_or(&serde_json::Value::String("No response".to_string())),
            "relevant_sources": result.get("sources").unwrap_or(&serde_json::Value::Array(vec![]))
        })),
        Err(e) => {
            eprintln!("Query error: {}", e);
            Err(format!("Failed to process query: {}", e))
        }
    }
}




// ========================= USER DATA SANITATION ========================= //

// Delete all user data (files, markdowns, database)
#[tauri::command]
async fn delete_all_user_data() -> Result<serde_json::Value, String> {
    clear_user_data()
        .map_err(|e| format!("Failed to delete all user data: {}", e))?;
    Ok(serde_json::json!({ "success": true }))
}

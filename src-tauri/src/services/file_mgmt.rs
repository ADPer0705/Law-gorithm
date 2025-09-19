// Simple file storage service for Law-gorithm
use crate::utils::dirs::{get_file_links_dir, get_markdowns_dir};
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use std::error::Error;

// --- Structs --- //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLink {
    pub id: String,             // unique ID
    pub original_name: String,  // e.g., myfile.pdf
    pub original_path: PathBuf, // full original location
}

pub struct FileStorageService;
pub struct MarkdownService;

// --- File Storage Implementation --- //

impl FileStorageService {
    pub fn new() -> Self {
        Self
    }

    /// Store a link to an uploaded file
    pub fn add_file_link(&self, file_id: &str, file_path: &str) -> Result<FileLink, Box<dyn Error>> {
        let path = PathBuf::from(file_path);
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("Invalid filename")?;

        let file_links_dir = get_file_links_dir()?;
        let link_file_path = file_links_dir.join(format!("{}.json", file_id));

        let file_link = FileLink {
            id: file_id.to_string(),
            original_name: filename.to_string(),
            original_path: path.clone(),
        };

        // Store the file link as JSON
        let json_content = serde_json::to_string_pretty(&file_link)?;
        fs::write(&link_file_path, json_content)?;

        println!("Stored file link: {} -> {}", filename, link_file_path.display());
        Ok(file_link)
    }

    /// List all file links
    pub fn list_file_links(&self) -> Result<Vec<FileLink>, Box<dyn Error>> {
        let file_links_dir = get_file_links_dir()?;
        let mut file_links = Vec::new();

        if file_links_dir.exists() {
            for entry in fs::read_dir(&file_links_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(file_link) = serde_json::from_str::<FileLink>(&content) {
                            file_links.push(file_link);
                        }
                    }
                }
            }
        }

        Ok(file_links)
    }

    /// Remove a file link (and its markdown)
    pub fn remove_file_link(&self, file_id: &str) -> Result<bool, Box<dyn Error>> {
        let file_links_dir = get_file_links_dir()?;
        let link_file_path = file_links_dir.join(format!("{}.json", file_id));

        if link_file_path.exists() {
            fs::remove_file(&link_file_path)?;
            
            // Also try to remove associated OCR markdown
            let ocr_markdowns_dir = get_markdowns_dir()?;
            let markdown_path = ocr_markdowns_dir.join(format!("{}.md", file_id));
            if markdown_path.exists() {
                if let Err(e) = fs::remove_file(&markdown_path) {
                    println!("Failed to remove markdown for {}: {}", file_id, e);
                }
            }

            println!("Removed file link and markdown for {}", file_id);
            Ok(true)
        } else {
            println!("No file link found for {}", file_id);
            Ok(false)
        }
    }
}

// --- Markdown Service Implementation --- //

impl MarkdownService {
    /// Generate a markdown for a single PDF and store it in the OCR markdowns directory
    pub fn generate_markdown_for_file(file_id: &str, file_path: &str) -> Result<PathBuf, Box<dyn Error>> {
        let pdf_path = PathBuf::from(file_path);
        let ocr_markdowns_dir = get_markdowns_dir()?;
        let markdown_path = ocr_markdowns_dir.join(format!("{}.md", file_id));

        crate::utils::ocr::process_pdf_to_markdown(&pdf_path, &markdown_path)?;

        println!("Markdown generated: {:?}", markdown_path);
        Ok(markdown_path)
    }

    /// Remove the corresponding markdown file for a given file_id
    pub fn remove_markdown_for_file(file_id: &str) -> Result<(), Box<dyn Error>> {
        let ocr_markdowns_dir = get_markdowns_dir()?;
        let markdown_path = ocr_markdowns_dir.join(format!("{}.md", file_id));

        if markdown_path.exists() {
            fs::remove_file(&markdown_path)?;
            println!("Removed markdown: {:?}", markdown_path);
        } else {
            println!("No markdown found for {}", file_id);
        }

        Ok(())
    }
}

// --- Quick Functions --- //

/// File Management
pub fn add_file_link(file_id: &str, file_path: &str) -> Result<FileLink, Box<dyn Error>> {
    FileStorageService::new().add_file_link(file_id, file_path)
}

pub fn list_file_links() -> Result<Vec<FileLink>, Box<dyn Error>> {
    FileStorageService::new().list_file_links()
}

pub fn remove_file_link(file_id: &str) -> Result<bool, Box<dyn Error>> {
    FileStorageService::new().remove_file_link(file_id)
}

/// Markdown Management
pub fn generate_markdown_for_file(file_id: &str, file_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    MarkdownService::generate_markdown_for_file(file_id, file_path)
}

pub fn remove_markdown_for_file(file_id: &str) -> Result<(), Box<dyn Error>> {
    MarkdownService::remove_markdown_for_file(file_id)
}

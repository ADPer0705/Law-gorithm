// File storage service
use crate::config::FileStorageConfig;
use crate::utils::file_utils::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rusqlite::Result as SqlResult;

#[derive(Debug, Clone)]
pub struct StoredFile {
    pub id: String,
    pub original_name: String,
    pub stored_path: PathBuf,
    pub size_bytes: u64,
    pub mime_type: String,
    pub uploaded_at: DateTime<Utc>,
    pub processed: bool,
}

pub struct FileStorageService {
    config: FileStorageConfig,
    base_path: PathBuf,
    temp_path: PathBuf,
}

impl FileStorageService {
    pub fn new(config: FileStorageConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let base_path = resolve_storage_path(&config.storage_path)?;
        let temp_path = resolve_storage_path(&config.temp_dir)?;

        // Create directories if they don't exist
        fs::create_dir_all(&base_path)?;
        fs::create_dir_all(&temp_path)?;

        Ok(Self {
            config,
            base_path,
            temp_path,
        })
    }

    pub fn store_file(&self, filename: &str, content: &[u8]) -> Result<StoredFile, Box<dyn std::error::Error>> {
        // Generate unique ID and sanitized filename
        let file_id = uuid::Uuid::new_v4().to_string();
        let sanitized_name = sanitize_filename(filename);
        let extension = get_file_extension(Path::new(&sanitized_name))
            .ok_or("File must have an extension")?;

        let final_filename = format!("{}_{}.{}", file_id, sanitized_name, extension);

        // Determine storage path
        let stored_path = if self.config.create_date_folders {
            create_date_based_path(&self.base_path, &final_filename)
        } else {
            self.base_path.join(final_filename)
        };

        // Write file
        fs::write(&stored_path, content)?;

        let stored_file = StoredFile {
            id: file_id,
            original_name: filename.to_string(),
            stored_path: stored_path.clone(),
            size_bytes: content.len() as u64,
            mime_type: self.guess_mime_type(&extension),
            uploaded_at: Utc::now(),
            processed: false,
        };

        Ok(stored_file)
    }

    pub fn get_file(&self, file_id: &str) -> Result<Option<StoredFile>, Box<dyn std::error::Error>> {
        // This would typically query a database, but for now we'll scan the directory
        // In a real implementation, you'd store file metadata in the database
        self.find_file_by_id(file_id)
    }

    pub fn delete_file(&self, file_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some(stored_file) = self.find_file_by_id(file_id)? {
            fs::remove_file(&stored_file.stored_path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn delete_files(&self, file_ids: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
        let mut deleted_count = 0;
        for file_id in file_ids {
            if self.delete_file(&file_id)? {
                deleted_count += 1;
            }
        }
        Ok(deleted_count)
    }

    pub fn list_files(&self) -> Result<Vec<StoredFile>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        self.scan_directory(&self.base_path, &mut files)?;
        Ok(files)
    }

    pub fn get_storage_stats(&self) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
        let mut stats = HashMap::new();
        let files = self.list_files()?;

        stats.insert("total_files".to_string(), files.len() as u64);
        stats.insert("total_size_bytes".to_string(), files.iter().map(|f| f.size_bytes).sum());

        let mut extension_counts: HashMap<String, u64> = HashMap::new();
        for file in &files {
            if let Some(ext) = get_file_extension(&file.stored_path) {
                *extension_counts.entry(ext).or_insert(0) += 1;
            }
        }

        for (ext, count) in extension_counts {
            stats.insert(format!("{}_files", ext), count);
        }

        Ok(stats)
    }

    // Private helper methods
    fn find_file_by_id(&self, file_id: &str) -> Result<Option<StoredFile>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        self.scan_directory(&self.base_path, &mut files)?;

        for file in files {
            if file.id == file_id {
                return Ok(Some(file));
            }
        }

        Ok(None)
    }

    fn scan_directory(&self, dir: &Path, files: &mut Vec<StoredFile>) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.scan_directory(&path, files)?;
            } else if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if let Some(file_id) = self.extract_file_id_from_filename(filename) {
                    let metadata = fs::metadata(&path)?;
                    let stored_file = StoredFile {
                        id: file_id,
                        original_name: self.extract_original_name_from_filename(filename),
                        stored_path: path.clone(),
                        size_bytes: metadata.len(),
                        mime_type: self.guess_mime_type(&get_file_extension(&path).unwrap_or_default()),
                        uploaded_at: metadata.modified()?.into(), // This is approximate
                        processed: false, // Would be stored in DB
                    };
                    files.push(stored_file);
                }
            }
        }

        Ok(())
    }

    fn extract_file_id_from_filename(&self, filename: &str) -> Option<String> {
        // Assuming format: {uuid}_{original_name}.{ext}
        filename.split('_').next().map(|s| s.to_string())
    }

    fn extract_original_name_from_filename(&self, filename: &str) -> String {
        // Extract original name from format: {uuid}_{original_name}.{ext}
        if let Some(underscore_pos) = filename.find('_') {
            if let Some(dot_pos) = filename.rfind('.') {
                return filename[underscore_pos + 1..dot_pos].to_string();
            }
        }
        filename.to_string()
    }

    fn guess_mime_type(&self, extension: &str) -> String {
        match extension.to_lowercase().as_str() {
            "pdf" => "application/pdf",
            "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "doc" => "application/msword",
            "md" => "text/markdown",
            "txt" => "text/plain",
            "rtf" => "application/rtf",
            _ => "application/octet-stream",
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_file_storage_basic() {
        let temp_dir = tempdir().unwrap();
        let storage_path = temp_dir.path().join("test_storage");

        // Create a basic config
        let config = FileStorageConfig {
            storage_path: storage_path.to_string_lossy().to_string(),
            create_date_folders: false,
            temp_dir: temp_dir.path().join("temp").to_string_lossy().to_string(),
        };

        let service = FileStorageService::new(config).unwrap();

        // Test file storage
        let test_content = b"Hello, World!";
        let stored_file = service.store_file("test.txt", test_content).unwrap();

        assert_eq!(stored_file.original_name, "test.txt");
        assert_eq!(stored_file.size_bytes, test_content.len() as u64);
        assert!(stored_file.stored_path.exists());

        // Test file retrieval
        let retrieved = service.get_file(&stored_file.id).unwrap().unwrap();
        assert_eq!(retrieved.id, stored_file.id);

        // Test file listing
        let files = service.list_files().unwrap();
        assert_eq!(files.len(), 1);

        // Test file deletion
        let deleted = service.delete_file(&stored_file.id).unwrap();
        assert!(deleted);

        // Verify file is gone
        let files_after = service.list_files().unwrap();
        assert_eq!(files_after.len(), 0);
    }

    #[test]
    fn test_bulk_file_operations() {
        let temp_dir = tempdir().unwrap();
        let config = FileStorageConfig {
            storage_path: temp_dir.path().join("storage").to_string_lossy().to_string(),
            create_date_folders: false,
            temp_dir: temp_dir.path().join("temp").to_string_lossy().to_string(),
        };

        let service = FileStorageService::new(config).unwrap();

        // Store multiple files
        let mut file_ids = Vec::new();
        for i in 0..3 {
            let content = format!("Content {}", i).into_bytes();
            let filename = format!("file{}.txt", i);
            let stored_file = service.store_file(&filename, &content).unwrap();
            file_ids.push(stored_file.id);
        }

        // Verify all files are stored
        let files = service.list_files().unwrap();
        assert_eq!(files.len(), 3);

        // Test bulk deletion
        let deleted_count = service.delete_files(file_ids).unwrap();
        assert_eq!(deleted_count, 3);

        // Verify all files are gone
        let files_after = service.list_files().unwrap();
        assert_eq!(files_after.len(), 0);
    }

    #[test]
    fn test_markdown_formatting() {
        let temp_dir = tempdir().unwrap();
        let config = FileStorageConfig {
            storage_path: temp_dir.path().join("storage").to_string_lossy().to_string(),
            create_date_folders: false,
            temp_dir: temp_dir.path().join("temp").to_string_lossy().to_string(),
        };

        let service = FileStorageService::new(config).unwrap();

        // Test markdown formatting (this would be part of OCR service in real implementation)
        let test_text = "This is a heading\n\nThis is a paragraph.\n\n- List item 1\n- List item 2\n\n1. Numbered item";
        let formatted = format_test_markdown(test_text);

        assert!(formatted.contains("# This is a heading"));
        assert!(formatted.contains("- List item"));
        assert!(formatted.contains("1. Numbered item"));
    }
}

// Helper function for testing markdown formatting
fn format_test_markdown(text: &str) -> String {
    let mut markdown = String::new();
    let paragraphs: Vec<&str> = text.split("\n\n").collect();

    for paragraph in paragraphs {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Simple heading detection
        if trimmed.split_whitespace().count() <= 10 && !trimmed.ends_with('.') {
            markdown.push_str(&format!("# {}\n\n", trimmed));
        }
        // List detection
        else if trimmed.starts_with("- ") || trimmed.starts_with("1. ") {
            markdown.push_str(&format!("{}\n", trimmed));
        }
        else {
            markdown.push_str(&format!("{}\n\n", trimmed));
        }
    }

    markdown
}

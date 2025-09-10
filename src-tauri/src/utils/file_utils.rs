// File utilities
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use chrono::{DateTime, Utc};
use dirs;

pub fn read_file_to_string(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_string_to_file(path: &Path, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn copy_file(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, dst)?;
    Ok(())
}

pub fn get_file_metadata(path: &Path) -> Result<fs::Metadata, Box<dyn std::error::Error>> {
    Ok(fs::metadata(path)?)
}

pub fn resolve_storage_path(config_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if config_path.starts_with('/') || config_path.contains('\\') {
        // Absolute path
        Ok(PathBuf::from(config_path))
    } else {
        // Relative path - resolve to app documents directory
        let documents_dir = dirs::document_dir()
            .ok_or_else(|| "Could not find documents directory".to_string())?
            .join("law-gorithm");

        Ok(documents_dir.join(config_path))
    }
}

pub fn create_date_based_path(base_path: &Path, filename: &str) -> PathBuf {
    let now = Utc::now();
    let date_folder = now.format("%Y-%m-%d").to_string();
    base_path.join(date_folder).join(filename)
}

pub fn sanitize_filename(filename: &str) -> String {
    // Remove or replace unsafe characters
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()?.to_str()?.to_lowercase().into()
}
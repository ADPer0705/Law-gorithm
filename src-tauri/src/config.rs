// Simple application configuration
use std::fs;
use std::io::{self, ErrorKind};
use dirs;

#[derive(Debug, Clone)]
/// Holds the main directories for the application
pub struct AppDirectories {
    pub base_dir: std::path::PathBuf,
    pub file_links: std::path::PathBuf,
    pub markdowns: std::path::PathBuf,
    pub database: std::path::PathBuf,
}

/// Get application directories for the current OS.
/// If they do not exist, they will be created automatically.
pub fn get_app_directories() -> io::Result<AppDirectories> {
    // Base directory: e.g., ~/.local/share/law-gorithm (Linux),
    // %APPDATA%\law-gorithm (Windows), ~/Library/Application Support/law-gorithm (macOS)
    let base_dir = dirs::data_dir()
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not find data directory"))?
        .join("law-gorithm");

    // Subdirectories for different data types
    let file_links = base_dir.join("file_links");
    let markdowns = base_dir.join("markdowns");
    let database = base_dir.join("database");

    // Ensure they exist (create if missing, no-op if already present)
    for dir in [&base_dir, &file_links, &markdowns, &database] {
        fs::create_dir_all(dir)?;
    }

    Ok(AppDirectories {
        base_dir,
        file_links,
        markdowns,
        database,
    })
}
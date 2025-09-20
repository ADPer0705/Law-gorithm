// Simple application configuration

use std::fs;
use std::io::{self, ErrorKind};
use dirs;

/// Struct representing the main directories used by the application.
#[derive(Debug, Clone)]
/// `AppDirectories` holds paths for:
/// - `base_dir`: The root data directory for the app.
/// - `file_links`: A subdirectory to store symbolic links or references to original files.
/// - `markdowns`: A subdirectory for storing Markdown-converted content from documents.
/// - `database`: A subdirectory for storing internal database files or persistent data.
pub struct AppDirectories {
    pub base_dir: std::path::PathBuf,
    pub file_links: std::path::PathBuf,
    pub markdowns: std::path::PathBuf,
    pub database: std::path::PathBuf,
}

/// Returns the application directories for the current operating system.
///
/// Automatically creates the directories if they do not already exist.
///
/// # Returns
/// `io::Result<AppDirectories>`:
/// - `Ok(AppDirectories)` containing paths to the base and subdirectories.
/// - `Err(io::Error)` if the system data directory cannot be determined or directories cannot be created.
///
/// # Directory Layout Examples
/// - **Linux**: `~/.local/share/law-gorithm/`
///     - `file_links/`
///     - `markdowns/`
///     - `database/`
/// - **Windows**: `%APPDATA%\law-gorithm\`
/// - **macOS**: `~/Library/Application Support/law-gorithm/`
///
/// # Usage
/// ```rust
/// let dirs = config::get_app_directories()?;
/// println!("Base directory: {:?}", dirs.base_dir);
/// ```
pub fn get_app_directories() -> io::Result<AppDirectories> {
    // Determine base directory based on OS
    let base_dir = dirs::data_dir()
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not find data directory"))?
        .join("law-gorithm");

    // Define subdirectories
    let file_links = base_dir.join("file_links");
    let markdowns = base_dir.join("markdowns");
    let database = base_dir.join("database");

    // Create directories if they don't exist
    for dir in [&base_dir, &file_links, &markdowns, &database] {
        fs::create_dir_all(dir)?;
    }

    // Return struct containing all directory paths
    Ok(AppDirectories {
        base_dir,
        file_links,
        markdowns,
        database,
    })
}

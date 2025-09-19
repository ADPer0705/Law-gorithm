use std::io;
use std::path::PathBuf;
use crate::config::get_app_directories;

// Return the base application directory path
pub fn get_base_dir() -> io::Result<PathBuf> {
    Ok(get_app_directories()?.base_dir)
}

// Return the file_links directory path
pub fn get_file_links_dir() -> io::Result<PathBuf> {
    Ok(get_app_directories()?.file_links)
}

// Return the markdowns directory path
pub fn get_markdowns_dir() -> io::Result<PathBuf> {
    Ok(get_app_directories()?.markdowns)
}

// Return the database directory path
pub fn get_database_dir() -> io::Result<PathBuf> {
    Ok(get_app_directories()?.database)
}

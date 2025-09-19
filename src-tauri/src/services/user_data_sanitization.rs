use std::error::Error;
use std::fs;
use std::path::Path;

/// Service for cleaning up user data (files, dirs, databases).
pub struct UserDataSanitizationService;

impl UserDataSanitizationService {
    /// Remove all contents of a directory, but not the directory itself.
    fn remove_directory_contents(&self, dir: &Path) -> Result<(), Box<dyn Error>> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    fs::remove_file(&path)?;
                } else if path.is_dir() {
                    fs::remove_dir_all(&path)?;
                }
            }
        }
        Ok(())
    }

    /// Delete all user data across directories and databases.
    pub fn delete_all_user_data(&self) -> Result<(), Box<dyn Error>> {
        println!("Deleting all files from all directories...");

        // Get all directories
        let base_dir = crate::utils::dirs::get_base_dir()?;
        let file_links_dir = crate::utils::dirs::get_file_links_dir()?;
        let markdowns_dir = crate::utils::dirs::get_markdowns_dir()?;
        let database_dir = crate::utils::dirs::get_database_dir()?;

        // Remove contents from each directory
        if file_links_dir.exists() {
            println!("Removing all files from: {}", file_links_dir.display());
            self.remove_directory_contents(&file_links_dir)?;
        }

        if markdowns_dir.exists() {
            println!("Removing all files from: {}", markdowns_dir.display());
            self.remove_directory_contents(&markdowns_dir)?;
        }

        if database_dir.exists() {
            println!("Removing all files from: {}", database_dir.display());

            // Call embeddings_db cleanup if available
            #[cfg(feature = "embeddings")]
            {
                crate::services::embeddings_db::wipe_user_data(&database_dir)?;
            }

            self.remove_directory_contents(&database_dir)?;
        }

        // Remove pseudonyms.sqlite from base directory
        if base_dir.exists() {
            let pseudonyms_file = base_dir.join("pseudonyms.sqlite");
            if pseudonyms_file.exists() {
                println!("Removing pseudonyms from {}", pseudonyms_file.display());
                fs::remove_file(&pseudonyms_file)?;
            }
        }

        println!("All files deleted successfully");
        Ok(())
    }
}

// Expose a free function to match main.rs usage.
pub fn delete_all_user_data() -> Result<(), Box<dyn std::error::Error>> {
    let service = UserDataSanitizationService;
    service.delete_all_user_data()
}

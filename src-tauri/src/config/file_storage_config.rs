use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FileStorageConfig {
    // General
    pub storage_path: String,

    // Organization
    pub create_date_folders: bool,

    // Preprocessing (OCR is always enabled)
    pub temp_dir: String,
}

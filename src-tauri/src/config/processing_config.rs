use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessingConfig {
    // Input limits
    pub max_file_size_mb: usize,
    pub supported_formats: Vec<String>,

    // Chunking
    pub chunk_size: usize,
    pub chunk_overlap: usize,

    // Privacy
    pub anonymization: bool,
}

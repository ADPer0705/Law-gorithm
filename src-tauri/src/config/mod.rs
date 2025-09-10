// Configuration management
pub mod ai_config;
pub mod db_config;
pub mod file_storage_config;
pub mod ocr_config;
pub mod processing_config;
pub mod python_config;
pub mod reader;

pub use ai_config::AiConfig;
pub use db_config::DbConfig;
pub use file_storage_config::FileStorageConfig;
pub use ocr_config::OcrConfig;
pub use processing_config::ProcessingConfig;
pub use python_config::PythonConfig;

pub struct Config {
    pub ai: AiConfig,
    pub db: DbConfig,
    pub file_storage: FileStorageConfig,
    pub ocr: OcrConfig,
    pub processing: ProcessingConfig,
    pub python: PythonConfig,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            ai: reader::load_config("ai_config.toml")?,
            db: reader::load_config("db_config.toml")?,
            file_storage: reader::load_config("file_storage_config.toml")?,
            ocr: reader::load_config("ocr_config.toml")?,
            processing: reader::load_config("processing_config.toml")?,
            python: reader::load_config("python_config.toml")?,
        })
    }
}

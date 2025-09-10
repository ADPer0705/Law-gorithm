// Config file reader
use std::fs;
use toml;
use super::{AiConfig, DbConfig, FileStorageConfig, OcrConfig, ProcessingConfig, PythonConfig};

pub fn load_config<T: serde::de::DeserializeOwned>(filename: &str) -> Result<T, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(format!("src-tauri/src/config/{}", filename))?;
    let config: T = toml::from_str(&content)?;
    Ok(config)
}

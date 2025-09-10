// src-tauri/src/config/ocr_config.rs

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct OcrConfig {
    /// Path to the Tesseract executable (bundled with the application)
    pub tesseract_cmd: String,

    /// Language to use (keep it minimal for performance, e.g., "eng")
    pub language: String,

    /// Page segmentation mode (PSM)
    /// 3 = Fully automatic page segmentation
    /// 6 = Assume a single uniform block of text
    /// 11 = Sparse text
    pub psm: u8,

    /// OCR Engine mode (OEM)
    /// 1 = Neural nets LSTM only
    /// 2 = Legacy engine only
    /// 3 = Default, based on available models
    pub oem: u8,

    /// Whether to preserve interword spacing
    pub preserve_interword_spaces: bool,
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            tesseract_cmd: Self::get_bundled_tesseract_path(),
            language: "eng".to_string(),
            psm: 6,
            oem: 1,
            preserve_interword_spaces: true,
        }
    }
}

impl OcrConfig {
    /// Get the path to the bundled Tesseract binary based on the current platform
    fn get_bundled_tesseract_path() -> String {
        // First try to get path from environment variable (set by build.rs in development)
        if let Ok(tesseract_path) = std::env::var("LAW_GORITHM_TESSERACT_PATH") {
            return tesseract_path;
        }
        
        // Fallback logic for production builds
        let mut path = std::env::current_exe()
            .unwrap_or_else(|_| std::env::current_dir().unwrap().join("target/debug/app"))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();

        // Navigate to vendor directory
        path.push("vendor");

        // Add platform-specific directory
        #[cfg(target_os = "linux")]
        path.push("tesseract-linux");

        #[cfg(target_os = "windows")]
        path.push("tesseract-windows");

        #[cfg(target_os = "macos")]
        path.push("tesseract-macos");

        // Add the tesseract executable
        #[cfg(target_os = "windows")]
        path.push("tesseract.exe");
        #[cfg(not(target_os = "windows"))]
        path.push("tesseract");

        path.to_string_lossy().to_string()
    }
}

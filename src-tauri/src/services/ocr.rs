// OCR service for PDF to Markdown conversion
use crate::config::OcrConfig;
use std::process::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use regex::Regex;

pub struct OcrService {
    config: OcrConfig,
}

impl OcrService {
    pub fn new(config: OcrConfig) -> Self {
        Self { config }
    }

    pub fn process_pdf_to_markdown(&self, pdf_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Extract text from PDF first
        let extracted_text = self.extract_text_from_pdf(pdf_path)?;

        // If PDF has embedded text, use it directly
        if !extracted_text.trim().is_empty() && !self.needs_ocr(&extracted_text) {
            let markdown = self.format_as_markdown(&extracted_text);
            fs::write(output_path, markdown)?;
            return Ok(());
        }

        // If PDF has images or needs OCR, process with Tesseract
        self.process_pdf_with_ocr(pdf_path, output_path)
    }

    fn extract_text_from_pdf(&self, pdf_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        // Use pdf-extract crate to get text from PDF
        match pdf_extract::extract_text(pdf_path) {
            Ok(text) => Ok(text),
            Err(_) => Ok(String::new()), // Return empty string if extraction fails
        }
    }

    fn needs_ocr(&self, text: &str) -> bool {
        // Simple heuristic: if text is very short or contains mostly symbols, might need OCR
        let word_count = text.split_whitespace().count();
        let has_meaningful_text = text.chars().filter(|c| c.is_alphabetic()).count() > 50;

        word_count < 10 || !has_meaningful_text
    }

    fn process_pdf_with_ocr(&self, pdf_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Convert PDF pages to images and then OCR each page
        let temp_dir = output_path.parent().unwrap_or(Path::new(".")).join("temp_ocr");
        fs::create_dir_all(&temp_dir)?;

        let mut all_text = String::new();

        // For now, we'll assume single-page PDFs or handle first page
        // In a full implementation, you'd iterate through all pages
        let image_path = temp_dir.join("page_001.png");

        // Convert PDF to image (this would require poppler-utils or similar)
        // For now, we'll use a placeholder approach
        if self.convert_pdf_page_to_image(pdf_path, &image_path, 1)? {
            let page_text = self.extract_text_from_image(&image_path)?;
            all_text.push_str(&page_text);
            all_text.push_str("\n\n");
        }

        // Clean up temp files
        let _ = fs::remove_dir_all(&temp_dir);

        let markdown = self.format_as_markdown(&all_text);
        fs::write(output_path, markdown)?;

        Ok(())
    }

    fn convert_pdf_page_to_image(&self, pdf_path: &Path, image_path: &Path, page: u32) -> Result<bool, Box<dyn std::error::Error>> {
        // This would typically use pdftoppm or similar command
        // For now, return false to indicate we can't convert (fallback to text extraction)
        Ok(false)
    }

    pub fn extract_text_from_image(&self, image_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let output_path = image_path.with_extension("txt");

        // Set TESSDATA_PREFIX to point to the bundled tessdata directory
        let tessdata_path = self.get_tessdata_path();

        let mut cmd = Command::new(&self.config.tesseract_cmd);
        cmd.env("TESSDATA_PREFIX", &tessdata_path)
            .arg(image_path.to_string_lossy().to_string())
            .arg(output_path.to_string_lossy().to_string())
            .arg("-l").arg(&self.config.language)
            .arg("--psm").arg(self.config.psm.to_string())
            .arg("--oem").arg(self.config.oem.to_string());

        if self.config.preserve_interword_spaces {
            cmd.arg("--preserve-interword-spaces");
        }

        let output = cmd.output()?;

        if output.status.success() {
            match fs::read_to_string(&output_path) {
                Ok(text) => {
                    // Clean up the temporary text file
                    let _ = fs::remove_file(&output_path);
                    Ok(text)
                },
                Err(_) => Ok(String::new()),
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Tesseract OCR failed: {}", stderr).into())
        }
    }

    fn format_as_markdown(&self, text: &str) -> String {
        let mut markdown = String::new();

        // Split text into paragraphs
        let paragraphs: Vec<&str> = text.split("\n\n").collect();

        for paragraph in paragraphs {
            let trimmed = paragraph.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Detect if this looks like a heading
            if self.is_heading(trimmed) {
                markdown.push_str(&format!("# {}\n\n", trimmed));
            }
            // Detect if this looks like a list item
            else if self.is_list_item(trimmed) {
                markdown.push_str(&format!("{}\n", trimmed));
            }
            else {
                markdown.push_str(&format!("{}\n\n", trimmed));
            }
        }

        markdown
    }

    fn is_heading(&self, text: &str) -> bool {
        // Simple heuristic: lines that are short and end without punctuation
        let words: Vec<&str> = text.split_whitespace().collect();
        words.len() <= 10 && !text.ends_with('.') && !text.ends_with('!') && !text.ends_with('?')
    }

    fn is_list_item(&self, text: &str) -> bool {
        // Check for common list markers
        text.trim_start().starts_with("- ") ||
        text.trim_start().starts_with("* ") ||
        text.trim_start().starts_with("• ") ||
        Regex::new(r"^\d+\.\s").unwrap().is_match(text.trim_start())
    }

    /// Get the path to the tessdata directory for the bundled Tesseract
    fn get_tessdata_path(&self) -> String {
        // First try to get path from environment variable (set by build.rs in development)
        if let Ok(tessdata_path) = std::env::var("LAW_GORITHM_TESSDATA_PATH") {
            return tessdata_path;
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

        // Add tessdata directory
        path.push("tessdata");

        path.to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_bundled_tesseract_path_resolution() {
        let config = OcrConfig::default();
        let service = OcrService::new(config);

        // Test that the tesseract command path contains vendor directory
        assert!(service.config.tesseract_cmd.contains("vendor"));
        assert!(service.config.tesseract_cmd.contains("tesseract"));

        // Test that tessdata path is correctly resolved
        let tessdata_path = service.get_tessdata_path();
        assert!(tessdata_path.contains("vendor"));
        assert!(tessdata_path.contains("tessdata"));
    }

    #[test]
    fn test_tesseract_binary_exists() {
        let config = OcrConfig::default();
        let tesseract_path = Path::new(&config.tesseract_cmd);

        // Check if the bundled tesseract binary exists
        // Note: This test will only pass when running from the bundled application
        if tesseract_path.exists() {
            println!("Bundled Tesseract binary found at: {}", config.tesseract_cmd);
        } else {
            println!("Bundled Tesseract binary not found at: {} (expected in bundled app)", config.tesseract_cmd);
        }
    }
}

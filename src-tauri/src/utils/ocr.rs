use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::error::Error;
use pyo3::prelude::*;
use pyo3::types::PyModule;

/// OCR processor for converting a PDF to markdown
pub struct PdfOcrProcessor {
    tesseract_path: PathBuf,
    ocr_markdowns_dir: PathBuf,
}

impl PdfOcrProcessor {
    /// Create a new OCR processor
    pub fn new(ocr_markdowns_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let tesseract_path = std::env::current_exe()?
            .parent()
            .ok_or("Cannot find executable directory")?
            .join("vendor")
            .join("tesseract");

        fs::create_dir_all(&ocr_markdowns_dir)?;

        Ok(Self {
            tesseract_path,
            ocr_markdowns_dir,
        })
    }

    /// Convert a single PDF into markdown
    pub fn process_pdf(
        &self,
        pdf_path: &Path,
        markdown_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !pdf_path.exists() {
            return Err(format!("PDF not found: {}", pdf_path.display()).into());
        }

        // Convert PDF → images
        let images = self.pdf_to_images(pdf_path)?;

        // Run OCR on images and build markdown
        let mut markdown = format!(
            "# OCR Result for {}\n\n",
            pdf_path.file_name().unwrap().to_string_lossy()
        );

        for (i, image_path) in images.iter().enumerate() {
            markdown.push_str(&format!("## Page {}\n\n", i + 1));
            markdown.push_str(&self.ocr_image(image_path)?);
            markdown.push_str("\n\n---\n\n");
            let _ = fs::remove_file(image_path); // cleanup temp image
        }

        fs::write(markdown_path, markdown)?;
        Ok(())
    }

    fn pdf_to_images(&self, pdf_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        Python::with_gil(|py| {
            let pdf2image = PyModule::import_bound(py, "pdf2image")?;
            let images = pdf2image
                .getattr("convert_from_path")?
                .call1((pdf_path.to_string_lossy(),))?;

            let mut paths = Vec::new();
            for (i, img) in images.iter()?.enumerate() {
                let img = img?;
                let temp = std::env::temp_dir().join(format!(
                    "ocr_temp_{}_{}.png",
                    pdf_path.file_stem().unwrap().to_string_lossy(),
                    i
                ));
                img.call_method1("save", (temp.to_string_lossy(),))?;
                paths.push(temp);
            }
            Ok(paths)
        })
    }

    fn ocr_image(&self, image_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new(&self.tesseract_path)
            .args(&[image_path.to_string_lossy().as_ref(), "stdout", "-l", "eng", "--psm", "6"])
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Tesseract failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(Self::clean_text(String::from_utf8(output.stdout)?))
    }

    fn clean_text(text: String) -> String {
        text.lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Convenience function to process a single PDF into a given markdown file
pub fn process_pdf_to_markdown(
    pdf_path: &Path,
    markdown_path: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    let processor = PdfOcrProcessor::new(
        markdown_path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf(),
    )?;
    processor.process_pdf(pdf_path, markdown_path)?;
    Ok(markdown_path.to_path_buf())
}
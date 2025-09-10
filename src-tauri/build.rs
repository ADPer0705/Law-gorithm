fn main() {
    // Set environment variable to help find the tesseract binary in development
    if std::env::var("CARGO_CFG_DEBUG_ASSERTIONS").is_ok() {
        // Development mode
        let project_root = std::env::var("CARGO_MANIFEST_DIR")
            .map(|dir| std::path::Path::new(&dir).parent().unwrap().to_path_buf())
            .unwrap_or_else(|_| std::env::current_dir().unwrap());
        
        let tesseract_path = project_root.join("vendor/tesseract-linux/tesseract");
        let tessdata_path = project_root.join("vendor/tesseract-linux/tessdata");
        
        println!("cargo:rustc-env=LAW_GORITHM_TESSERACT_PATH={}", tesseract_path.display());
        println!("cargo:rustc-env=LAW_GORITHM_TESSDATA_PATH={}", tessdata_path.display());
    }
    
    tauri_build::build()
}

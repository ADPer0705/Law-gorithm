fn main() {
    // Link against system libtesseract and libleptonica
    println!("cargo:rustc-link-lib=dylib=tesseract");
    println!("cargo:rustc-link-lib=dylib=leptonica");
    
    // Tell pkg-config to find tesseract
    if let Err(_) = pkg_config::Config::new().probe("tesseract") {
        // Fallback: try common system library paths
        println!("cargo:rustc-link-search=native=/usr/lib");
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    }
    
    tauri_build::build()
}

pub mod dirs;
pub mod embeddings_simple;
pub mod ocr_simple;

// Re-export simplified versions for CPU-only build
pub use embeddings_simple as embeddings;
pub use ocr_simple as ocr;
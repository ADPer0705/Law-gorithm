use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig {
    // General
    pub provider: String,          // "sqlite" | "sqlite-vss" | "qdrant" | "faiss"
    pub path: String,              // DB file path for sqlite, URL for qdrant
    pub use_ann: bool,             // Whether to use ANN or brute force

    // Performance tuning
    pub journal_mode: String,      // e.g. "WAL"
    pub synchronous: String,       // "NORMAL" | "FULL" | "OFF"
    pub cache_size: i32,           // KB; negative = memory pages
    pub temp_store: String,        // "MEMORY" or "FILE"

    // Schema consistency
    pub foreign_keys: bool,        // Enforce foreign keys in sqlite
    pub embedding_dimension: usize,// Must match embedding model
    pub metric: String,            // "cosine" | "l2" | "ip"
}

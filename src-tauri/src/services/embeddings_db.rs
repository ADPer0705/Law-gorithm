// src-tauri/src/services/embeddings_db_structs.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rusqlite::{Connection, params};
use std::path::PathBuf;

use crate::utils::embeddings::embed_chunks;


// ---------- DEFINING CHUNKS ---------- //

/// Source information for a chunk (keeps provenance lightweight).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub filename: String,
    pub section: Option<String>,
    pub sub_section: Option<String>,
}

/// Core chunk record (what lives in memory before being written out).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub text: String,
    pub source: Source,
    pub metadata: HashMap<String, String>,
}

impl Chunk {
    /// Create a new chunk with given source and text.
    pub fn new(source: Source, text: String) -> Self {
        Chunk {
            text,
            source,
            metadata: HashMap::new(),
        }
    }

    /// Attach or update metadata for the chunk.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}


/// Chunk along with its embedding vector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkWithEmbedding {
    pub chunk: Chunk,
    pub embedding: Vec<f32>,
}

/// Handle for the embeddings database (SQLite + VSS).
#[derive(Debug, Clone)]
pub enum SimilarityMetric {
    Cosine,
    L2,
    InnerProduct,
}

impl ToString for SimilarityMetric {
    fn to_string(&self) -> String {
        match self {
            SimilarityMetric::Cosine => "cosine".into(),
            SimilarityMetric::L2 => "l2".into(),
            SimilarityMetric::InnerProduct => "ip".into(),
        }
    }
}

pub struct EmbeddingsDatabase {
    pub db_path: PathBuf,
    pub embedding_dimensions: usize,
    pub metric: SimilarityMetric,
}

impl EmbeddingsDatabase {
    pub fn new(db_dir: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(db_dir)?;
        let db_path = db_dir.join("embeddings.db");

        Ok(EmbeddingsDatabase {
            db_path,
            embedding_dimensions: 384, // default for your embedding model
            metric: SimilarityMetric::Cosine,
        })
    }

    pub fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;

        // Load VSS extension
        conn.execute("SELECT load_extension('vss0')", [])?;

        // Create embeddings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                filename TEXT NOT NULL,
                section TEXT,
                sub_section TEXT,
                metadata TEXT NOT NULL,
                embedding BLOB
            )",
            [],
        )?;

        // Create VSS virtual table for similarity search
        conn.execute(
            &format!(
                "CREATE VIRTUAL TABLE IF NOT EXISTS vss_embeddings USING vss0(
                    embedding({})
                )",
                self.embedding_dimensions
            ),
            [],
        )?;

        Ok(())
    }

    pub fn wipe_database(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DROP TABLE IF EXISTS vss_embeddings", [])?;
        conn.execute("DROP TABLE IF EXISTS embeddings", [])?;
        self.initialize()?;
        Ok(())
    }

    pub fn insert_chunks(
        &self,
        chunks_with_embeddings: Vec<ChunkWithEmbedding>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = Connection::open(&self.db_path)?;
        let tx = conn.transaction()?;

        {
            let mut stmt = tx.prepare(
                "INSERT INTO embeddings (text, filename, section, sub_section, metadata, embedding)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )?;

            let mut vss_stmt = tx.prepare(
                "INSERT INTO vss_embeddings (rowid, embedding) VALUES (?1, ?2)"
            )?;

            for cwe in chunks_with_embeddings {
                let metadata_json = serde_json::to_string(&cwe.chunk.metadata)?;
                let embedding_blob = Self::vec_to_blob(&cwe.embedding);

                stmt.execute(params![
                    cwe.chunk.text,
                    cwe.chunk.source.filename,
                    cwe.chunk.source.section,
                    cwe.chunk.source.sub_section,
                    metadata_json,
                    embedding_blob
                ])?;

                let rowid = tx.last_insert_rowid();
                vss_stmt.execute(params![rowid, embedding_blob])?;
            }
        } // Statements are dropped here when this scope ends

        tx.commit()?;
        Ok(())
    }

    pub fn search_similar(
        &self,
        query_embedding: &[f32],
        limit: usize
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        let embedding_blob = Self::vec_to_blob(query_embedding);

        let mut stmt = conn.prepare(
            "SELECT e.id, e.text, e.filename, e.section, e.sub_section, e.metadata, v.distance
             FROM vss_embeddings v
             JOIN embeddings e ON e.id = v.rowid
             WHERE vss_search(v.embedding, ?1)
             ORDER BY v.distance
             LIMIT ?2"
        )?;

        let rows = stmt.query_map(params![embedding_blob, limit], |row| {
            let metadata_json: String = row.get(5)?;
            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_json)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e)))?;

            Ok(SearchResult {
                id: row.get(0)?,
                chunk: Chunk {
                    text: row.get(1)?,
                    source: Source {
                        filename: row.get(2)?,
                        section: row.get(3)?,
                        sub_section: row.get(4)?,
                    },
                    metadata,
                },
                distance: row.get(6)?,
            })
        })?;

        Ok(rows.filter_map(Result::ok).collect())
    }

    fn vec_to_blob(vec: &[f32]) -> Vec<u8> {
        vec.iter().flat_map(|&f| f.to_le_bytes().to_vec()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: i64,
    pub chunk: Chunk,
    pub distance: f64,
}

// Convenience functions
pub fn create_embeddings_db(db_dir: &PathBuf) -> Result<EmbeddingsDatabase, Box<dyn std::error::Error>> {
    let db = EmbeddingsDatabase::new(db_dir)?;
    db.initialize()?;
    Ok(db)
}

pub fn wipe_user_data(db_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let db = EmbeddingsDatabase::new(db_dir)?;
    db.wipe_database()
}

// ---------- DATABASE WORKFLOWS ---------- //
// Populating the Database with the markdown embeddings
pub async fn populate_database_from_markdown() -> Result<(), Box<dyn std::error::Error>> {
    // Get the markdown directory
    let markdown_dir = crate::utils::dirs::get_markdowns_dir()?;
    println!("Starting to process markdown files from: {:?}", markdown_dir);
    
    // Create or get the database (create_embeddings_db handles checking if it exists)
    let db_dir = crate::utils::dirs::get_database_dir()?;
    std::fs::create_dir_all(&db_dir)?; // Ensure directory exists
    let db = create_embeddings_db(&db_dir)?;
    
    // Step 1: Chunk markdown files
    let markdown_dir_str = markdown_dir.to_str().ok_or("Invalid UTF-8 in path")?;
    let chunks = crate::services::chunking::chunk_markdown_dir(markdown_dir_str)?;
    println!("Created {} chunks from markdown files", chunks.len());
    
    // Step 2: Anonymize the chunks (now async)
    let anonymized_chunks = crate::services::anonymization::anonymize_chunks(chunks).await?;
    println!("Anonymized {} chunks", anonymized_chunks.len());
    
    // Step 3: Embed the chunks
    let chunks_with_embeddings = crate::services::embeddings_db::embed_chunks(anonymized_chunks)?;
    println!("Embedded {} chunks", chunks_with_embeddings.len());
    
    // Step 4: Update the database
    db.insert_chunks(chunks_with_embeddings)?;
    println!("Successfully inserted chunks into the database");
    
    Ok(())
}


// Retrieval of relevant chunks based on user query
/// Search for chunks similar to the given query text.
/// 
/// # Arguments
/// 
/// * `query` - The user's query text to search for
/// * `limit` - Maximum number of results to return
/// 
/// # Returns
/// 
/// A vector of `SearchResult` containing the most similar chunks to the query
pub fn search_query(query: &str, limit: usize) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    // Get the database directory
    let db_dir = crate::utils::dirs::get_database_dir()?;
    
    // Open the database
    let db = EmbeddingsDatabase::new(&db_dir)?;
    
    // Embed the query
    let query_embedding = crate::utils::embeddings::embed_text(query)?;
    
    // Search for similar chunks
    let results = db.search_similar(&query_embedding, limit)?;
    
    Ok(results)
}

// Ingestion service
use crate::storage::Database;
use rusqlite::Result;

pub struct IngestionService {
    db: Database,
}

impl IngestionService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn ingest_document(&self, chunks: Vec<String>, embeddings: Vec<Vec<f32>>) -> Result<()> {
        for (chunk, embedding) in chunks.into_iter().zip(embeddings) {
            self.db.store_embedding(&chunk, &embedding)?;
        }
        Ok(())
    }
}

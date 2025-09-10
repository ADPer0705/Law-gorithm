// Retrieval service
use crate::storage::Database;
use rusqlite::Result;

pub struct RetrievalService {
    db: Database,
}

impl RetrievalService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn retrieve_chunks(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<String>> {
        let results = self.db.search_similar(query_embedding, limit)?;
        Ok(results.into_iter().map(|(content, _)| content).collect())
    }
}

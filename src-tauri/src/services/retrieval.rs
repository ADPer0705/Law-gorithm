use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Configuration for retrieval parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalConfig {
    pub top_k: usize,
    pub similarity_threshold: f32,
    pub max_tokens: Option<usize>,
    pub embedding_model: String,
}

impl Default for RetrievalConfig {
    fn default() -> Self {
        Self {
            top_k: 5,
            similarity_threshold: 0.7,
            max_tokens: Some(4000),
            embedding_model: "text-embedding-ada-002".to_string(),
        }
    }
}

// Result structure for retrieved chunks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedChunk {
    pub id: String,
    pub content: String,
    pub score: f32,
    pub metadata: HashMap<String, String>,
    pub source: Option<String>,
}

// Query file_mgmt result
#[derive(Debug, Clone)]
pub struct ProcessedQuery {
    pub original: String,
    pub embedding: Vec<f32>,
    pub keywords: Vec<String>,
}

// Main retrieval trait for pluggability
pub trait RetrievalEngine {
    fn retrieve(&self, query: &str) -> Result<Vec<RetrievedChunk>, Box<dyn std::error::Error>>;
    fn configure(&mut self, config: RetrievalConfig);
}

// Core retriever implementation
pub struct Retriever {
    config: RetrievalConfig,
    embedding_client: Option<EmbeddingClient>,
    database_client: Option<DatabaseClient>,
}

impl Retriever {
    pub fn new() -> Self {
        Self {
            config: RetrievalConfig::default(),
            embedding_client: None,
            database_client: None,
        }
    }

    pub fn with_config(mut self, config: RetrievalConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_embedding_client(mut self, client: EmbeddingClient) -> Self {
        self.embedding_client = Some(client);
        self
    }

    pub fn with_database_client(mut self, client: DatabaseClient) -> Self {
        self.database_client = Some(client);
        self
    }

    // Process user query into embeddings and keywords
    fn process_query(&self, query: &str) -> Result<ProcessedQuery, Box<dyn std::error::Error>> {
        let cleaned_query = self.preprocess_query(query);
        let keywords = self.extract_keywords(&cleaned_query);

        let embedding = match &self.embedding_client {
            Some(client) => client.generate_embedding(&cleaned_query)?,
            None => return Err("Embedding client not configured".into()),
        };

        Ok(ProcessedQuery {
            original: query.to_string(),
            embedding,
            keywords,
        })
    }

    // Simple query prefile_mgmt
    fn preprocess_query(&self, query: &str) -> String {
        query
            .trim()
            .to_lowercase()
            .replace(&['\n', '\t', '\r'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    // Extract keywords for hybrid search
    fn extract_keywords(&self, query: &str) -> Vec<String> {
        query
            .split_whitespace()
            .filter(|word| word.len() > 2)
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|word| !word.is_empty())
            .map(String::from)
            .collect()
    }

    // Perform semantic search
    fn semantic_search(&self, processed_query: &ProcessedQuery) -> Result<Vec<RetrievedChunk>, Box<dyn std::error::Error>> {
        match &self.database_client {
            Some(client) => {
                client.similarity_search(
                    &processed_query.embedding,
                    self.config.top_k,
                    self.config.similarity_threshold,
                )
            }
            None => Err("Database client not configured".into()),
        }
    }

    // Filter and rank results
    fn post_process_results(&self, mut chunks: Vec<RetrievedChunk>) -> Vec<RetrievedChunk> {
        // Sort by score (highest first)
        chunks.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        // Apply token limit if configured
        if let Some(max_tokens) = self.config.max_tokens {
            let mut total_tokens = 0;
            chunks.retain(|chunk| {
                let chunk_tokens = chunk.content.split_whitespace().count();
                if total_tokens + chunk_tokens <= max_tokens {
                    total_tokens += chunk_tokens;
                    true
                } else {
                    false
                }
            });
        }

        chunks
    }
}

impl RetrievalEngine for Retriever {
    fn retrieve(&self, query: &str) -> Result<Vec<RetrievedChunk>, Box<dyn std::error::Error>> {
        if query.trim().is_empty() {
            return Err("Query cannot be empty".into());
        }

        // Process the query
        let processed_query = self.process_query(query)?;

        // Perform semantic search
        let raw_results = self.semantic_search(&processed_query)?;

        // Post-process and return results
        Ok(self.post_process_results(raw_results))
    }

    fn configure(&mut self, config: RetrievalConfig) {
        self.config = config;
    }
}

// Mock embedding client for demonstration
pub struct EmbeddingClient {
    model: String,
}

impl EmbeddingClient {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
        }
    }

    pub fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        // Mock implementation - replace with actual embedding service
        let hash = std::collections::hash_map::DefaultHasher::new();
    use std::hash::{Hash, Hasher};

        let mut hasher = hash;
        text.hash(&mut hasher);
        let seed = hasher.finish();

        // Generate deterministic "embedding" for testing
        let mut embedding = Vec::with_capacity(384);
        for i in 0..384 {
            embedding.push(((seed.wrapping_add(i as u64) % 1000) as f32 - 500.0) / 500.0);
        }

        Ok(embedding)
    }
}

// Mock database client for demonstration
pub struct DatabaseClient {
    // Mock data store
    chunks: Vec<RetrievedChunk>,
}

impl DatabaseClient {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
        }
    }

    pub fn with_mock_data(mut self) -> Self {
        self.chunks = vec![
            RetrievedChunk {
                id: "chunk_1".to_string(),
                content: "This is a sample legal document about contracts.".to_string(),
                score: 0.0,
                metadata: HashMap::from([("type".to_string(), "contract".to_string())]),
                source: Some("doc1.pdf".to_string()),
            },
            RetrievedChunk {
                id: "chunk_2".to_string(),
                content: "Legal precedent regarding employment law.".to_string(),
                score: 0.0,
                metadata: HashMap::from([("type".to_string(), "precedent".to_string())]),
                source: Some("doc2.pdf".to_string()),
            },
        ];
        self
    }

    pub fn similarity_search(
        &self,
        query_embedding: &[f32],
        top_k: usize,
        threshold: f32,
    ) -> Result<Vec<RetrievedChunk>, Box<dyn std::error::Error>> {
        let mut results = self.chunks.clone();

        // Mock similarity calculation
        for chunk in &mut results {
            chunk.score = self.calculate_similarity(query_embedding, &chunk.content);
        }

        // Filter by threshold and take top_k
        results.retain(|chunk| chunk.score >= threshold);
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(top_k);

        Ok(results)
    }

    fn calculate_similarity(&self, _embedding: &[f32], content: &str) -> f32 {
        // Mock similarity - replace with actual cosine similarity
        (content.len() as f32 % 100.0) / 100.0 + 0.5
    }
}

// Convenience function for quick retrieval
pub fn retrieve_chunks(query: &str, config: Option<RetrievalConfig>) -> Result<Vec<RetrievedChunk>, Box<dyn std::error::Error>> {
    let config = config.unwrap_or_default();

    let retriever = Retriever::new()
        .with_config(config.clone())
        .with_embedding_client(EmbeddingClient::new(&config.embedding_model))
        .with_database_client(DatabaseClient::new().with_mock_data());

    retriever.retrieve(query)
}
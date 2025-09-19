use std::collections::HashMap;
use std::fs;

/// Represents a chunk of text with metadata
#[derive(Debug, Clone)]
pub struct Chunk {
    pub text: String,
    pub source: Source,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub filename: String,
    pub section: String,
    pub sub_section: String,
}

/// Basic chunking options
pub struct ChunkingOptions {
    pub max_tokens: usize,   // max tokens or words per chunk
    pub overlap: usize,      // overlap between chunks
}

pub struct Chunker {
    options: ChunkingOptions,
}

impl Chunker {
    pub fn new(max_tokens: usize, overlap: usize) -> Self {
        Self {
            options: ChunkingOptions {
                max_tokens,
                overlap,
            },
        }
    }

    /// Split raw text into chunks
    pub fn chunk_text(
        &self,
        text: &str,
        source: Source,
    ) -> Vec<Chunk> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut chunks = Vec::new();
        let mut start = 0;

        while start < words.len() {
            let end = (start + self.options.max_tokens).min(words.len());
            let chunk_text = words[start..end].join(" ");

            let chunk = Chunk {
                text: chunk_text,
                source: source.clone(),
                metadata: HashMap::new(),
            };
            chunks.push(chunk);

            if end == words.len() {
                break;
            }
            start += self.options.max_tokens - self.options.overlap;
        }

        chunks
    }
}

// ---------- CHUNKING WORKFLOWS ---------- //

/// Walk through the markdown directory and chunk all files
pub fn chunk_markdown_dir(markdown_dir: &str) -> std::io::Result<Vec<Chunk>> {
    let mut all_chunks = Vec::new();
    // Create a chunker with sensible defaults
    let chunker = Chunker::new(512, 50);  // 512 max tokens with 50 token overlap

    for entry in fs::read_dir(markdown_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|ext| ext == "md").unwrap_or(false) {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let contents = fs::read_to_string(&path)?;

            // TODO: For now, treat entire file as one "section"
            let source = Source {
                filename: filename.clone(),
                section: "full".to_string(),
                sub_section: "".to_string(),
            };

            let chunks = chunker.chunk_text(&contents, source);
            all_chunks.extend(chunks);
        }
    }

    Ok(all_chunks)
}

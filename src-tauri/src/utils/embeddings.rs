use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use std::sync::Mutex;
use thiserror::Error;

use crate::chunking::Chunk;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("Python error: {0}")]
    PythonError(#[from] PyErr),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, EmbeddingError>;

// ---------- STATIC MODEL STORAGE ---------- //
static MODEL: OnceCell<Mutex<Py<PyAny>>> = OnceCell::new();

/// Initialize the embedding model (only once)
pub fn init_embeddings() -> Result<()> {
    Python::with_gil(|py| {
        MODEL.get_or_try_init(|| {
            let code = r#"
from sentence_transformers import SentenceTransformer

_model = None

def _load():
    global _model
    if _model is None:
        _model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")
    return _model

def embed(text: str):
    model = _load()
    emb = model.encode([text], convert_to_numpy=True, normalize_embeddings=True)[0]
    return emb.astype(float).tolist()

def embed_batch(texts):
    model = _load()
    embs = model.encode(list(texts), convert_to_numpy=True, normalize_embeddings=True)
    return embs.astype(float).tolist()
"#;

            let module = PyModule::from_code_bound(py, code, "st_embed.py", "st_embed")?;
            // Store the module itself
            Ok(Mutex::new(module.into()))
        })?;
        Ok(())
    })
}

/// Embed a single string into a vector
pub fn embed_text(text: &str) -> Result<Vec<f32>> {
    if text.trim().is_empty() {
        return Err(EmbeddingError::InvalidInput("Empty text".into()));
    }

    Python::with_gil(|py| {
        let model = MODEL
            .get()
            .expect("Embeddings not initialized. Call init_embeddings first.")
            .lock()
            .unwrap();

        let module = model.as_ref(py);
        let func = module.getattr("embed")?;
        let result = func.call1((text,))?;
        let v: Vec<f32> = result.extract()?;
        Ok(v)
    })
}

/// Embed multiple strings at once
pub fn embed_batch(texts: &[String]) -> Result<Vec<Vec<f32>>> {
    if texts.is_empty() {
        return Err(EmbeddingError::InvalidInput("Empty batch".into()));
    }

    Python::with_gil(|py| {
        let model = MODEL
            .get()
            .expect("Embeddings not initialized. Call init_embeddings first.")
            .lock()
            .unwrap();

        let module = model.as_ref(py);
        let func = module.getattr("embed_batch")?;
        let result = func.call1((texts.to_vec(),))?;
        let v: Vec<Vec<f32>> = result.extract()?;
        Ok(v)
    })
}

/// Helper: embed a list of Chunks
pub fn embed_chunks(chunks: &[Chunk]) -> Result<Vec<(Chunk, Vec<f32>)>> {
    let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
    let embeddings = embed_batch(&texts)?;
    Ok(chunks.to_vec().into_iter().zip(embeddings).collect())
}

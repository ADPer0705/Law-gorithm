use dotenv::dotenv;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use thiserror::Error;

use crate::services::embeddings_db::{search_query, SearchResult};
use crate::services::anonymization::deanonymize_text;

/// ------------------- Error Definitions -------------------
#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Embeddings DB error: {0}")]
    DbError(String),
    #[error("Google AI error: {0}")]
    GoogleAIError(String),
    #[error("API key not found in environment")]
    MissingApiKey,
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// ------------------- Static HTTP Client -------------------
static CLIENT: OnceCell<Client> = OnceCell::new();

fn get_client() -> &'static Client {
    CLIENT.get_or_init(Client::new)
}

/// ------------------- Gemini API Response Structs -------------------
#[derive(Debug, serde::Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, serde::Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Debug, serde::Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, serde::Deserialize)]
struct Part {
    text: Option<String>,
}

/// ------------------- Public Query Workflow -------------------
pub async fn query(query: String) -> Result<Value, QueryError> {
    // Search DB for top 5 relevant chunks
    let search_results = search_query(&query, 5)
        .map_err(|e| QueryError::DbError(e.to_string()))?;

    // Collect chunk texts and sources
    let chunks_text = search_results
        .iter()
        .map(|r| r.chunk.text.clone())
        .collect::<Vec<_>>()
        .join("\n\n");

    let sources = search_results
        .iter()
        .map(|r| r.chunk.source.clone())
        .collect::<Vec<_>>();

    // Summarize using Google AI Studio
    let summary = summarize_with_google_ai(&query, &chunks_text, &sources).await?;

    // De-anonymize the response if needed
    let deanon_response = if let Some(answer) = summary.get("answer").and_then(|v| v.as_str()) {
        deanonymize_text(answer).unwrap_or_else(|_| answer.to_string())
    } else {
        "".to_string()
    };

    // Return structured JSON with only the deanonymized response
    Ok(json!({
        "query": query,
        "response": deanon_response,
        "sources": sources,
    }))
}

/// ------------------- Google AI Summarization -------------------
async fn summarize_with_google_ai(
    query: &str,
    context: &str,
    sources: &[String],
) -> Result<Value, QueryError> {
    dotenv().ok();
    let api_key = env::var("GOOGLE_AI_API_KEY").map_err(|_| QueryError::MissingApiKey)?;

    let prompt = format!(
        r#"
You are a summarization assistant.
Given the following query and context, return a JSON object with fields:
- "answer": concise answer
- "citations": list of relevant source filenames (must choose from provided sources)

Query: {}
Context: {}
Available sources: {}
"#,
        query,
        context,
        sources.join(", ")
    );

    let payload = json!({
        "contents": [
            { "role": "user", "parts": [{ "text": prompt }] }
        ],
        "generationConfig": {
            "temperature": 0.2,
            "topP": 0.8,
            "topK": 40,
            "maxOutputTokens": 1024
        }
    });

    let client = get_client();
    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent")
        .query(&[("key", api_key)])
        .json(&payload)
        .send()
        .await?;

    let response_json: GeminiResponse = response.json().await?;

    let generated_text = response_json
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .and_then(|p| p.text.clone())
        .unwrap_or_else(|| "{}".to_string());

    let parsed: Value = serde_json::from_str(&generated_text)
        .unwrap_or_else(|_| json!({ "answer": generated_text, "citations": [] }));

    Ok(parsed)
}

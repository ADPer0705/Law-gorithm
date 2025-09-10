use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Google,
    Openai,
    Local,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AiConfig {
    // Provider
    pub provider: Provider,
    pub api_key: Option<String>,

    // Core Models
    pub embedding_model: String,
    pub llm_model: String,

    // Parameters
    pub temperature: f32,
    pub max_tokens: usize,
}

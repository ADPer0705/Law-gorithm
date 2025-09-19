use std::error::Error;
use std::path::PathBuf;
use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;
use hf_hub::api::tokio::Api;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;
use crate::services::anonymization::{DetectedEntity, EntityType};

static NER_MODEL: OnceCell<Mutex<Option<NERModel>>> = OnceCell::new();

pub struct NERModel {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
    config: Config,
}

pub struct RustNER {
    // This will be a lightweight wrapper that uses the global model
}

impl RustNER {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Initialize the global model if not already done
        tokio::spawn(async {
            let _ = Self::ensure_model_loaded().await;
        });
        
        Ok(Self {})
    }

    async fn ensure_model_loaded() -> Result<()> {
        let model_cell = NER_MODEL.get_or_init(|| Mutex::new(None));
        let mut model_guard = model_cell.lock().await;
        
        if model_guard.is_none() {
            *model_guard = Some(Self::load_ner_model().await?);
        }
        
        Ok(())
    }

    async fn load_ner_model() -> Result<NERModel> {
        // Use a pre-trained NER model from Hugging Face
        let api = Api::new()?;
        let repo = api.model("dbmdz/bert-large-cased-finetuned-conll03-english".to_string());
        
        // Download model files
        let model_file = repo.get("pytorch_model.bin").await?;
        let config_file = repo.get("config.json").await?;
        let tokenizer_file = repo.get("tokenizer.json").await?;
        
        // Load tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_file)?;
        
        // Load config
        let config_content = std::fs::read_to_string(config_file)?;
        let config: Config = serde_json::from_str(&config_content)?;
        
        // Initialize device (CPU for maximum compatibility)
        let device = Device::Cpu;
        
        // Load model weights
        let weights = candle_core::safetensors::load(model_file, &device)?;
        let var_builder = VarBuilder::from_tensors(weights, candle_core::DType::F32, &device);
        let model = BertModel::load(&var_builder, &config)?;
        
        Ok(NERModel {
            model,
            tokenizer,
            device,
            config,
        })
    }

    // Synchronous wrapper for the async detect method
    pub fn detect(&self, text: &str) -> Result<Vec<DetectedEntity>, Box<dyn Error>> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.detect_async(text))
    }
    
    async fn detect_async(&self, text: &str) -> Result<Vec<DetectedEntity>, Box<dyn Error>> {
        Self::ensure_model_loaded().await?;
        
        let model_cell = NER_MODEL.get().unwrap();
        let model_guard = model_cell.lock().await;
        let model = model_guard.as_ref().unwrap();
        
        // Tokenize input
        let encoding = model.tokenizer.encode(text, true)
            .map_err(|e| format!("Tokenization error: {}", e))?;
        
        let tokens = encoding.get_tokens();
        let token_ids = encoding.get_ids();
        
        // Convert to tensor
        let input_ids = Tensor::new(token_ids, &model.device)?
            .unsqueeze(0)?; // Add batch dimension
        
        // Run inference
        let outputs = model.model.forward(&input_ids)?;
        let logits = outputs; // Assuming the model outputs logits directly
        
        // Apply softmax to get probabilities
        let probabilities = candle_nn::ops::softmax(&logits, 2)?;
        
        // Convert probabilities to predictions
        let predictions = probabilities.argmax(2)?;
        let predictions_vec: Vec<u32> = predictions.to_vec1()?;
        
        // Map predictions to entities
        let mut entities = Vec::new();
        let mut current_entity: Option<(String, EntityType, usize, usize)> = None;
        
        for (i, (&token_id, token, &prediction)) in token_ids.iter()
            .zip(tokens.iter())
            .zip(predictions_vec.iter())
            .enumerate() {
            
            let label = Self::id_to_label(prediction);
            
            match label.as_str() {
                "B-PER" => {
                    if let Some((text, entity_type, start, end)) = current_entity.take() {
                        entities.push(self.create_entity(text, entity_type, start, end, &text)?);
                    }
                    current_entity = Some((token.clone(), EntityType::Person, i, i + 1));
                },
                "I-PER" => {
                    if let Some((ref mut entity_text, EntityType::Person, start, ref mut end)) = current_entity {
                        if !token.starts_with("##") {
                            entity_text.push(' ');
                        }
                        entity_text.push_str(&token.replace("##", ""));
                        *end = i + 1;
                    }
                },
                "B-ORG" => {
                    if let Some((text, entity_type, start, end)) = current_entity.take() {
                        entities.push(self.create_entity(text, entity_type, start, end, &text)?);
                    }
                    current_entity = Some((token.clone(), EntityType::Organization, i, i + 1));
                },
                "I-ORG" => {
                    if let Some((ref mut entity_text, EntityType::Organization, start, ref mut end)) = current_entity {
                        if !token.starts_with("##") {
                            entity_text.push(' ');
                        }
                        entity_text.push_str(&token.replace("##", ""));
                        *end = i + 1;
                    }
                },
                "B-LOC" => {
                    if let Some((text, entity_type, start, end)) = current_entity.take() {
                        entities.push(self.create_entity(text, entity_type, start, end, &text)?);
                    }
                    current_entity = Some((token.clone(), EntityType::Location, i, i + 1));
                },
                "I-LOC" => {
                    if let Some((ref mut entity_text, EntityType::Location, start, ref mut end)) = current_entity {
                        if !token.starts_with("##") {
                            entity_text.push(' ');
                        }
                        entity_text.push_str(&token.replace("##", ""));
                        *end = i + 1;
                    }
                },
                "B-MISC" => {
                    if let Some((text, entity_type, start, end)) = current_entity.take() {
                        entities.push(self.create_entity(text, entity_type, start, end, &text)?);
                    }
                    current_entity = Some((token.clone(), EntityType::LegalReference, i, i + 1));
                },
                "I-MISC" => {
                    if let Some((ref mut entity_text, EntityType::LegalReference, start, ref mut end)) = current_entity {
                        if !token.starts_with("##") {
                            entity_text.push(' ');
                        }
                        entity_text.push_str(&token.replace("##", ""));
                        *end = i + 1;
                    }
                },
                _ => {
                    // O (outside) tag or unknown
                    if let Some((text, entity_type, start, end)) = current_entity.take() {
                        entities.push(self.create_entity(text, entity_type, start, end, &text)?);
                    }
                }
            }
        }
        
        // Handle final entity
        if let Some((text, entity_type, start, end)) = current_entity {
            entities.push(self.create_entity(text, entity_type, start, end, &text)?);
        }
        
        Ok(entities)
    }
}
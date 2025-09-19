// src/services/anonymization.rs
use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::Mutex,
};

use once_cell::sync::OnceCell;
use regex::Regex;
use rusqlite::{Connection, OptionalExtension, Result as SqliteResult};
use serde::{Deserialize, Serialize};

use crate::services::simple_ner::{self};
use crate::services::chunking::Chunk;

// ------------------- Entity Definitions -------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Money,
    Date,
    Phone,
    Email,
    CaseNumber,
    LegalReference,
    SocialSecurity,
}

impl EntityType {
    fn to_prefix(&self) -> &str {
        match self {
            EntityType::Person => "PERSON",
            EntityType::Organization => "ORG",
            EntityType::Location => "LOC",
            EntityType::Money => "AMOUNT",
            EntityType::Date => "DATE",
            EntityType::Phone => "PHONE",
            EntityType::Email => "EMAIL",
            EntityType::CaseNumber => "CASE",
            EntityType::LegalReference => "REF",
            EntityType::SocialSecurity => "SSN",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DetectedEntity {
    pub text: String,
    pub entity_type: EntityType,
    pub start: usize, // byte offsets (Rust-safe)
    pub end: usize,
}

// ------------------- Regex Detector -------------------

pub struct RegexDetector {
    patterns: HashMap<EntityType, Vec<Regex>>,
}

impl RegexDetector {
    pub fn new() -> Result<Self, regex::Error> {
        use EntityType::*;
        let mut patterns = HashMap::new();

        patterns.insert(CaseNumber, vec![
            Regex::new(r"\b\d{4}-\d{4,6}\b")?,
            Regex::new(r"\bCase No\.?\s*\d+")?,
        ]);
        patterns.insert(Money, vec![
            Regex::new(r"\$[\d,]+(?:\.\d{2})?")?,
            Regex::new(r"\b\d+(?:,\d{3})*(?:\.\d{2})?\s*dollars?\b")?,
        ]);
        patterns.insert(Phone, vec![
            Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b")?,
            Regex::new(r"\(\d{3}\)\s*\d{3}[-.]?\d{4}")?,
        ]);
        patterns.insert(Email, vec![
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b")?,
        ]);
        patterns.insert(Date, vec![
            Regex::new(r"\b\d{1,2}/\d{1,2}/\d{4}\b")?,
            Regex::new(r"\b\d{4}-\d{2}-\d{2}\b")?,
            Regex::new(r"\b(?:January|February|March|April|May|June|July|August|September|October|November|December)\s+\d{1,2},?\s+\d{4}\b")?,
        ]);

        Ok(Self { patterns })
    }

    pub fn detect(&self, text: &str) -> Vec<DetectedEntity> {
        let mut entities = Vec::new();

        for (entity_type, regexes) in &self.patterns {
            for regex in regexes {
                for mat in regex.find_iter(text) {
                    entities.push(DetectedEntity {
                        text: mat.as_str().to_string(),
                        entity_type: entity_type.clone(),
                        start: mat.start(),
                        end: mat.end(),
                    });
                }
            }
        }
        entities
    }
}

// ------------------- CPU-Optimized NER (No Python/GPU Required) -------------------

/// CPU-optimized NER detector - works on any computer, no GPU needed
pub struct CpuNerDetector;

impl CpuNerDetector {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self)
    }

    pub async fn detect(&self, text: &str) -> Result<Vec<DetectedEntity>, Box<dyn Error>> {
        // Use our CPU-optimized NER
        let detected = simple_ner::detect_entities_async(text).await?;
        
        // Convert to our internal format
        let mut entities = Vec::new();
        for entity in detected {
            let entity_type = match entity.entity_type.as_str() {
                "PERSON" => EntityType::Person,
                "ORGANIZATION" => EntityType::Organization,
                "LOCATION" => EntityType::Location,
                "MONEY" => EntityType::Money,
                "DATE" => EntityType::Date,
                "PHONE" => EntityType::Phone,
                "SSN" => EntityType::SocialSecurity,
                "LEGAL_CASE" => EntityType::CaseNumber,
                "LEGAL_DOCUMENT" => EntityType::LegalReference,
                _ => continue,
            };
            
            entities.push(DetectedEntity {
                text: entity.text,
                entity_type,
                start: entity.start,
                end: entity.end,
            });
        }
        
        Ok(entities)
    }
}

// ------------------- Pseudonym Database -------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PseudonymMapping {
    pub original: String,
    pub pseudonym: String,
    pub entity_type: EntityType,
}

pub struct PseudonymDatabase {
    db_path: PathBuf,
}

impl PseudonymDatabase {
    pub fn new(data_dir: &PathBuf) -> SqliteResult<Self> {
        let db_path = data_dir.join("pseudonyms.sqlite");
        let db = Self { db_path };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pseudonyms (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original TEXT NOT NULL UNIQUE,
                pseudonym TEXT NOT NULL,
                entity_type TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_pseudonym(&self, original: &str) -> SqliteResult<Option<String>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT pseudonym FROM pseudonyms WHERE original = ?1")?;
        stmt.query_row([original], |row| row.get::<_, String>(0))
            .optional()
    }

    pub fn store_pseudonym(&self, mapping: &PseudonymMapping) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT OR REPLACE INTO pseudonyms (original, pseudonym, entity_type)
             VALUES (?1, ?2, ?3)",
            [&mapping.original, &mapping.pseudonym, mapping.entity_type.to_prefix()],
        )?;
        Ok(())
    }

    pub fn get_next_counter(&self, entity_type: &EntityType) -> SqliteResult<u32> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM pseudonyms WHERE entity_type = ?1")?;
        let count: u32 = stmt.query_row([entity_type.to_prefix()], |row| row.get(0))?;
        Ok(count + 1)
    }
}

// ------------------- Pseudonym Generator -------------------

pub struct PseudonymGenerator {
    database: PseudonymDatabase,
}

impl PseudonymGenerator {
    pub fn new(data_dir: &PathBuf) -> SqliteResult<Self> {
        Ok(Self {
            database: PseudonymDatabase::new(data_dir)?,
        })
    }

    pub fn get_or_create_pseudonym(
        &self,
        original: &str,
        entity_type: EntityType,
    ) -> SqliteResult<String> {
        if let Some(existing) = self.database.get_pseudonym(original)? {
            return Ok(existing);
        }
        let counter = self.database.get_next_counter(&entity_type)?;
        let pseudonym = format!("{}_{}", entity_type.to_prefix(), counter);

        let mapping = PseudonymMapping {
            original: original.to_string(),
            pseudonym: pseudonym.clone(),
            entity_type,
        };
        self.database.store_pseudonym(&mapping)?;
        Ok(pseudonym)
    }
}

// ------------------- Workflow -------------------

pub struct AnonymizationWorkflow {
    regex_detector: RegexDetector,
    ner_detector: CpuNerDetector,
    pseudonym_generator: PseudonymGenerator,
}

impl AnonymizationWorkflow {
    pub fn new(data_dir: &PathBuf) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            regex_detector: RegexDetector::new()?,
            ner_detector: CpuNerDetector::new()?,
            pseudonym_generator: PseudonymGenerator::new(data_dir)?,
        })
    }

    pub async fn anonymize_text(&self, text: &str) -> Result<String, Box<dyn Error>> {
        let mut entities = self.regex_detector.detect(text);
        entities.append(&mut self.ner_detector.detect(text).await?);

        entities.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)).then(a.text.cmp(&b.text)));
        entities.dedup_by(|a, b| a.start == b.start && a.end == b.end && a.text == b.text);

        entities.sort_by(|a, b| b.start.cmp(&a.start));

        let mut anonymized = text.to_string();
        for ent in entities {
            if ent.start >= ent.end || ent.end > anonymized.len() {
                continue;
            }
            let pseudonym = self.pseudonym_generator.get_or_create_pseudonym(&ent.text, ent.entity_type)?;
            anonymized.replace_range(ent.start..ent.end, &pseudonym);
        }
        Ok(anonymized)
    }

    pub fn deanonymize_text(&self, text: &str) -> Result<String, Box<dyn Error>> {
        let conn = Connection::open(&self.pseudonym_generator.database.db_path)?;
        let mut stmt = conn.prepare("SELECT pseudonym, original FROM pseudonyms")?;
        let mappings: Vec<(String, String)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?.collect::<Result<_, _>>()?;

        let mut deanonymized = text.to_string();
        for (pseudonym, original) in {
            let mut sorted = mappings;
            sorted.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
            sorted
        } {
            deanonymized = deanonymized.replace(&pseudonym, &original);
        }
        Ok(deanonymized)
    }

    pub async fn anonymize_chunks(&self, chunks: Vec<Chunk>) -> Result<Vec<Chunk>, Box<dyn Error>> {
        let mut result = Vec::new();
        for mut chunk in chunks {
            chunk.text = self.anonymize_text(&chunk.text).await?;
            result.push(chunk);
        }
        Ok(result)
    }
}

// ---------- STATIC WORKFLOW STORAGE ---------- //

static WORKFLOW: OnceCell<Mutex<AnonymizationWorkflow>> = OnceCell::new();

pub fn init_anonymization(data_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    WORKFLOW.get_or_try_init(|| {
        let workflow = AnonymizationWorkflow::new(data_dir)?;
        Ok::<_, Box<dyn Error>>(Mutex::new(workflow))
    })?;
    Ok(())
}

pub async fn anonymize_chunks(chunks: Vec<Chunk>) -> Result<Vec<Chunk>, Box<dyn Error>> {
    // For hackathon: create a simple temporary workflow to avoid Send issues
    let regex_detector = RegexDetector::new()?;
    let ner_detector = CpuNerDetector::new()?;
    
    let mut result = Vec::new();
    for mut chunk in chunks {
        // Use regex patterns for anonymization (fast and reliable)
        let mut entities = regex_detector.detect(&chunk.text);
        entities.append(&mut ner_detector.detect(&chunk.text).await?);
        
        // Simple replacement for hackathon
        let mut anonymized_text = chunk.text.clone();
        entities.sort_by(|a, b| b.start.cmp(&a.start)); // Reverse order to preserve indices
        
        for entity in entities {
            let replacement = format!("[{}]", entity.entity_type.to_prefix());
            anonymized_text.replace_range(entity.start..entity.end, &replacement);
        }
        
        chunk.text = anonymized_text;
        result.push(chunk);
    }
    
    Ok(result)
}

pub fn deanonymize_response(text: &str) -> Result<String, Box<dyn Error>> {
    let workflow = WORKFLOW
        .get()
        .expect("AnonymizationWorkflow not initialized. Call init_anonymization first.")
        .lock()
        .unwrap();
    workflow.deanonymize_text(text)
}

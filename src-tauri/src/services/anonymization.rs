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

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule};
use pyo3::Bound; // <- added

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

// ------------------- HuggingFace NER (via PyO3) -------------------

const HF_NER_PY: &str = r#"
from transformers import AutoTokenizer, AutoModelForTokenClassification, pipeline

class NERDetector:
    def __init__(self):
        model_name = "dbmdz/bert-large-cased-finetuned-conll03-english"
        self.tokenizer = AutoTokenizer.from_pretrained(model_name)
        self.model = AutoModelForTokenClassification.from_pretrained(model_name)
        self.ner_pipeline = pipeline("ner", model=self.model, tokenizer=self.tokenizer, aggregation_strategy="simple")

    def detect_entities(self, text):
        return self.ner_pipeline(text)

detector = NERDetector()
"#;

pub struct HuggingFaceNER {
    detector: Py<PyAny>,
}

impl HuggingFaceNER {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Python::with_gil(|py| {
            // Use the stable API (Bound)
            let module = PyModule::from_code_bound(py, HF_NER_PY, "hf_ner.py", "hf_ner")?;
            let detector = module.getattr("detector")?.unbind(); // Bound<'py, PyAny> -> Py<PyAny>
            Ok(Self { detector })
        })
        .map_err(|e: pyo3::PyErr| Box::new(e) as Box<dyn Error>)
    }

    pub fn detect(&self, text: &str) -> Result<Vec<DetectedEntity>, Box<dyn Error>> {
        Python::with_gil(|py| {
            let detector_obj = self.detector.bind(py); // Py<PyAny> -> Bound<'py, PyAny>
            let res = detector_obj.call_method1("detect_entities", (text,))?;
            let py_list = res.downcast::<PyList>()?; // Bound<'py, PyAny> -> Bound<'py, PyList>

            let mut entities = Vec::new();

            // helper to extract string from dict for multiple possible keys
            fn get_string_from_dict(dict: &Bound<'_, PyDict>, keys: &[&str]) -> Option<String> {
                for &k in keys {
                    if let Ok(Some(v)) = dict.get_item(k) {
                        if let Ok(s) = v.extract::<String>() {
                            return Some(s);
                        }
                    }
                }
                None
            }

            // helper to extract usize from dict for a key
            fn get_usize_from_dict(dict: &Bound<'_, PyDict>, key: &str) -> Option<usize> {
                if let Ok(Some(v)) = dict.get_item(key) {
                    if let Ok(n) = v.extract::<usize>() {
                        return Some(n);
                    }
                }
                None
            }

            for item in py_list.iter() {
                let dict = match item.downcast::<PyDict>() {
                    Ok(d) => d,
                    Err(_) => continue, // skip unexpected items
                };

                let word = get_string_from_dict(&dict, &["word", "entity", "text"]).unwrap_or_default();
                let label = get_string_from_dict(&dict, &["entity_group", "label"]).unwrap_or_default();

                let start_char = get_usize_from_dict(&dict, "start").unwrap_or(0);
                let end_char = get_usize_from_dict(&dict, "end")
                    .unwrap_or_else(|| start_char + word.chars().count());

                let start_byte = char_idx_to_byte_idx(text, start_char).unwrap_or(0);
                let end_byte = char_idx_to_byte_idx(text, end_char).unwrap_or(start_byte + word.len());

                let entity_type = match label.as_str() {
                    "PER" | "PERSON" => EntityType::Person,
                    "ORG" | "ORGANIZATION" => EntityType::Organization,
                    "LOC" | "LOCATION" => EntityType::Location,
                    "MISC" => EntityType::LegalReference,
                    _ => continue,
                };

                if start_byte < end_byte && end_byte <= text.len() {
                    let final_text = if word.is_empty() {
                        text[start_byte..end_byte].to_string()
                    } else {
                        word
                    };

                    entities.push(DetectedEntity {
                        text: final_text,
                        entity_type,
                        start: start_byte,
                        end: end_byte,
                    });
                }
            }

            Ok(entities)
        })
        .map_err(|e: pyo3::PyErr| Box::new(e) as Box<dyn Error>)
    }
}

fn char_idx_to_byte_idx(s: &str, char_idx: usize) -> Option<usize> {
    // char_idx is an index in terms of chars (0-based). We return the byte index of that char.
    // If char_idx == number of chars, return len() (end of string).
    if char_idx == 0 {
        return Some(0);
    }
    let total_chars = s.chars().count();
    if char_idx >= total_chars {
        return Some(s.len());
    }
    s.char_indices().nth(char_idx).map(|(byte_idx, _)| byte_idx)
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
    ner_detector: HuggingFaceNER,
    pseudonym_generator: PseudonymGenerator,
}

impl AnonymizationWorkflow {
    pub fn new(data_dir: &PathBuf) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            regex_detector: RegexDetector::new()?,
            ner_detector: HuggingFaceNER::new()?,
            pseudonym_generator: PseudonymGenerator::new(data_dir)?,
        })
    }

    pub fn anonymize_text(&self, text: &str) -> Result<String, Box<dyn Error>> {
        let mut entities = self.regex_detector.detect(text);
        entities.append(&mut self.ner_detector.detect(text)?);

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

    pub fn anonymize_chunks(&self, chunks: Vec<Chunk>) -> Result<Vec<Chunk>, Box<dyn Error>> {
        chunks.into_iter()
            .map(|mut chunk| {
                chunk.text = self.anonymize_text(&chunk.text)?;
                Ok(chunk)
            })
            .collect()
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

pub fn anonymize_chunks(chunks: Vec<Chunk>) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let workflow = WORKFLOW
        .get()
        .expect("AnonymizationWorkflow not initialized. Call init_anonymization first.")
        .lock()
        .unwrap();
    workflow.anonymize_chunks(chunks)
}

pub fn deanonymize_response(text: &str) -> Result<String, Box<dyn Error>> {
    let workflow = WORKFLOW
        .get()
        .expect("AnonymizationWorkflow not initialized. Call init_anonymization first.")
        .lock()
        .unwrap();
    workflow.deanonymize_text(text)
}

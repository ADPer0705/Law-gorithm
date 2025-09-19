// src/services/anonymization.rs
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::OnceCell;
use regex::Regex;
use rusqlite::{Connection, OptionalExtension, Result as SqliteResult};
use serde::{Deserialize, Serialize};

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule};

use crate::chunking::Chunk;

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
    /// byte offsets in the original string (safe for Rust slicing)
    pub start: usize,
    pub end: usize,
}

// ------------------- Regex Detector -------------------

pub struct RegexDetector {
    patterns: HashMap<EntityType, Vec<Regex>>,
}

impl RegexDetector {
    pub fn new() -> Result<Self, regex::Error> {
        let mut patterns = HashMap::new();

        patterns.insert(EntityType::CaseNumber, vec![
            Regex::new(r"\b\d{4}-\d{4,6}\b")?,
            Regex::new(r"\bCase No\.?\s*\d+")?,
        ]);

        patterns.insert(EntityType::Money, vec![
            Regex::new(r"\$[\d,]+(?:\.\d{2})?")?,
            Regex::new(r"\b\d+(?:,\d{3})*(?:\.\d{2})?\s*dollars?\b")?,
        ]);

        patterns.insert(EntityType::Phone, vec![
            Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b")?,
            Regex::new(r"\(\d{3}\)\s*\d{3}[-.]?\d{4}")?,
        ]);

        patterns.insert(EntityType::Email, vec![
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b")?,
        ]);

        patterns.insert(EntityType::Date, vec![
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
//
// This initializes a small Python module (pipeline) and keeps the Python detector as a Py<PyAny>
// so subsequent calls reuse the loaded model (avoid reloading every call).
//
// NOTE: Python environment must have `transformers` and `torch` installed and internet access
// (or a local cache of the model). First initialization will download the model and can take time.

const HF_NER_PY: &str = r#"
from transformers import AutoTokenizer, AutoModelForTokenClassification, pipeline

class NERDetector:
    def __init__(self):
        model_name = "dbmdz/bert-large-cased-finetuned-conll03-english"
        # these will download model/tokenizer on first run if not cached
        self.tokenizer = AutoTokenizer.from_pretrained(model_name)
        self.model = AutoModelForTokenClassification.from_pretrained(model_name)
        self.ner_pipeline = pipeline("ner", model=self.model, tokenizer=self.tokenizer, aggregation_strategy="simple")

    def detect_entities(self, text):
        # Returns list of dicts with keys like: word, entity_group, score, start, end
        return self.ner_pipeline(text)

detector = NERDetector()
"#;

pub struct HuggingFaceNER {
    detector: Py<PyAny>,
}

impl HuggingFaceNER {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Python::with_gil(|py| {
            // create a module from the Python code and take the `detector` object
            let module = PyModule::from_code(py, HF_NER_PY, "hf_ner.py")?;
            let detector = module.getattr("detector")?.into_py(py);
            Ok(HuggingFaceNER { detector })
        }).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    /// Detect entities using the HF pipeline and convert start/end (character indices)
    /// to byte offsets for Rust strings.
    pub fn detect(&self, text: &str) -> Result<Vec<DetectedEntity>, Box<dyn Error>> {
        Python::with_gil(|py| {
            let detector_obj = self.detector.as_ref(py);
            let res = detector_obj.call_method1("detect_entities", (text,))?;
            let py_list = res.downcast::<PyList>()?;

            let mut entities = Vec::new();
            for item in py_list.iter() {
                let dict = item.downcast::<PyDict>()?;

                // try a few keys providers may use
                let word: String = dict.get_item("word")
                    .or_else(|| dict.get_item("word").or_else(|| dict.get_item("entity")))
                    .or_else(|| dict.get_item("text"))
                    .and_then(|v| v.extract().ok())
                    .unwrap_or_else(|| "".to_string());

                let label: String = dict.get_item("entity_group")
                    .or_else(|| dict.get_item("label"))
                    .and_then(|v| v.extract().ok())
                    .unwrap_or_else(|| "".to_string());

                // start/end are usually character offsets from the HF pipeline
                let start_char: usize = dict.get_item("start")
                    .and_then(|v| v.extract().ok())
                    .unwrap_or(0usize);
                let end_char: usize = dict.get_item("end")
                    .and_then(|v| v.extract().ok())
                    .unwrap_or(start_char + word.chars().count());

                // convert char-based indices to byte-based indices for Rust slicing
                let start_byte = char_idx_to_byte_idx(text, start_char).unwrap_or(0);
                let end_byte = char_idx_to_byte_idx(text, end_char).unwrap_or(start_byte + word.len());

                // map label -> EntityType (extend as needed)
                let entity_type = match label.as_str() {
                    "PER" | "PERSON" => EntityType::Person,
                    "ORG" | "ORGANIZATION" => EntityType::Organization,
                    "LOC" | "LOCATION" => EntityType::Location,
                    "MISC" => EntityType::LegalReference,
                    _ => {
                        // If HF returns unknown label, skip — regex detector may catch other types
                        continue;
                    }
                };

                // Skip empty matches
                if start_byte >= end_byte || end_byte > text.len() {
                    continue;
                }

                let text_span = text.get(start_byte..end_byte).unwrap_or("").to_string();

                entities.push(DetectedEntity {
                    text: if word.is_empty() { text_span.clone() } else { word },
                    entity_type,
                    start: start_byte,
                    end: end_byte,
                });
            }

            Ok(entities)
        }).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

/// Helper: convert a character index (0..n chars) into a byte index in Rust UTF-8 string.
fn char_idx_to_byte_idx(s: &str, char_idx: usize) -> Option<usize> {
    if char_idx == 0 { return Some(0); }
    let mut chars = s.char_indices();
    let mut i = 0usize;
    while let Some((byte_pos, _ch)) = chars.next() {
        i += 1;
        if i == char_idx {
            return Some(byte_pos + s[byte_pos..].chars().next().unwrap().len_utf8());
        }
    }
    // If char_idx is past last char, return len()
    if char_idx >= s.chars().count() {
        return Some(s.len());
    }
    None
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
        stmt.query_row([original], |row| row.get::<_, String>(0)).optional()
    }

    pub fn store_pseudonym(&self, mapping: &PseudonymMapping) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT OR REPLACE INTO pseudonyms (original, pseudonym, entity_type)
             VALUES (?1, ?2, ?3)",
            [&mapping.original, &mapping.pseudonym, &mapping.entity_type.to_prefix()],
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
    /// data_dir is where pseudonyms.sqlite will be created / stored
    pub fn new(data_dir: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize NER (this will load model — can be slow first time)
        let ner = HuggingFaceNER::new()?;
        let regex = RegexDetector::new()?;
        let pg = PseudonymGenerator::new(data_dir)?;

        Ok(Self {
            regex_detector: regex,
            ner_detector: ner,
            pseudonym_generator: pg,
        })
    }

    /// Anonymize a single text: run regex detector and HF NER (both),
    /// merge detections, dedupe, then replace in reverse order so indices stay valid.
    pub fn anonymize_text(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Collect regex entities (byte indices)
        let mut entities = self.regex_detector.detect(text);

        // Collect NER entities (converted to byte indices inside detect)
        let mut ner_entities = self.ner_detector.detect(text)?;
        entities.append(&mut ner_entities);

        // normalize: remove duplicates based on start,end,text
        entities.sort_by(|a, b| {
            a.start.cmp(&b.start).then(a.end.cmp(&b.end)).then(a.text.cmp(&b.text))
        });
        entities.dedup_by(|a, b| a.start == b.start && a.end == b.end && a.text == b.text);

        // Replace in reverse order of start so earlier replacements don't shift later indices
        entities.sort_by(|a, b| b.start.cmp(&a.start));

        let mut anonymized = text.to_string();

        for ent in entities {
            // guard indices
            if ent.start >= ent.end || ent.end > anonymized.len() {
                // If original indices are out-of-bounds (unlikely), skip
                continue;
            }

            // Retrieve/create pseudonym
            let pseudonym = self.pseudonym_generator.get_or_create_pseudonym(&ent.text, ent.entity_type.clone())?;
            // Replace range in anonymized string (we're processing from end -> start)
            anonymized.replace_range(ent.start..ent.end, &pseudonym);
        }

        Ok(anonymized)
    }

    /// Public API: pass in chunks, get anonymized chunks back.
    pub fn anonymize_chunks(&self, chunks: Vec<Chunk>) -> Result<Vec<Chunk>, Box<dyn std::error::Error>> {
        let mut out = Vec::with_capacity(chunks.len());
        for mut chunk in chunks.into_iter() {
            chunk.text = self.anonymize_text(&chunk.text)?;
            out.push(chunk);
        }
        Ok(out)
    }
}


// ---------- STATIC WORKFLOW STORAGE ---------- //
static WORKFLOW: OnceCell<Mutex<AnonymizationWorkflow>> = OnceCell::new();

/// Initialize the anonymization workflow (only once)
pub fn init_anonymization(data_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    WORKFLOW.get_or_try_init(|| {
        let workflow = AnonymizationWorkflow::new(data_dir)?;
        Ok(Mutex::new(workflow))
    })?;
    Ok(())
}

/// Public function: anonymize chunks using global workflow
pub fn anonymize_chunks(chunks: Vec<Chunk>) -> Result<Vec<Chunk>, Box<dyn std::error::Error>> {
    let workflow_mutex = WORKFLOW
        .get()
        .expect("AnonymizationWorkflow not initialized. Call init_anonymization first.");
    let workflow = workflow_mutex.lock().unwrap();
    workflow.anonymize_chunks(chunks)
}
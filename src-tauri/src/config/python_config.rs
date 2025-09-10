use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PythonConfig {
    // Path to Python binary (can be system, venv, or embedded)
    pub python_path: String, // e.g. "/usr/bin/python3"

    // Path to the python_modules/ directory
    pub module_path: String, // e.g. "./python_modules"

    // Main entry script for RAG operations
    pub entry_script: String, // e.g. "rag_processor.py"

    // Toggle integration on/off (some installs may not need Python at all)
    pub enabled: bool,

    // Resource constraints
    pub max_memory_mb: Option<usize>, // Limit Python side usage
    pub timeout_secs: Option<u64>,    // Safety net for long calls
}

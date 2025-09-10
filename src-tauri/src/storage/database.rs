// Database operations
use rusqlite::{Connection, Result};
use crate::config::DbConfig;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use dirs;

#[derive(Clone)]
pub struct Database {
    pub conn: Arc<Mutex<Connection>>,
    pub config: DbConfig,
}

impl Database {
    pub fn new(config: DbConfig) -> Result<Self> {
        let db_path = Self::resolve_db_path(&config.path)?;
        let conn = Arc::new(Mutex::new(Connection::open(&db_path)?));

        // Apply performance tuning from config
        {
            let conn_guard = conn.lock().unwrap();
            conn_guard.execute(&format!("PRAGMA journal_mode = {};", config.journal_mode), [])?;
            conn_guard.execute(&format!("PRAGMA synchronous = {};", config.synchronous), [])?;
            conn_guard.execute(&format!("PRAGMA cache_size = {};", config.cache_size), [])?;
            conn_guard.execute(&format!("PRAGMA temp_store = {};", config.temp_store), [])?;
            if config.foreign_keys {
                conn_guard.execute("PRAGMA foreign_keys = ON;", [])?;
            }
        }

        let mut db = Self { conn, config };

        if db.config.provider == "sqlite-vss" {
            db.init_vss()?;
        }

        Ok(db)
    }

    fn resolve_db_path(config_path: &str) -> Result<PathBuf> {
        if config_path.starts_with('/') || config_path.contains('\\') {
            // Absolute path
            Ok(PathBuf::from(config_path))
        } else {
            // Relative path - resolve to app data directory
            let app_data_dir = dirs::data_dir()
                .ok_or_else(|| rusqlite::Error::InvalidPath(config_path.into()))?
                .join("law-gorithm");
            
            std::fs::create_dir_all(&app_data_dir)
                .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?;
            Ok(app_data_dir.join(config_path))
        }
    }

    fn init_vss(&mut self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // For sqlite-vss, we need the vss0 extension
        // In development, try to load from common locations
        // In production, bundle the extension with the app
        let extension_paths = [
            "vss0",  // If in system path
            "./vss0.so",  // Relative to current dir
            "/usr/local/lib/vss0.so",  // Common system location
        ];

        let mut loaded = false;
        for path in &extension_paths {
            if std::path::Path::new(path).exists() || path == &"vss0" {
                match unsafe { conn.load_extension(path, None) } {
                    Ok(_) => {
                        loaded = true;
                        break;
                    }
                    Err(_) => continue,
                }
            }
        }

        if !loaded {
            eprintln!("Warning: sqlite-vss extension not found. Vector search will not be available.");
            return Ok(());
        }

        // Create vector table if not exists
        conn.execute(
            &format!("CREATE VIRTUAL TABLE IF NOT EXISTS chunks_vss USING vss0(
                embedding({})  -- dimension from config
            );", self.config.embedding_dimension),
            [],
        )?;

        // Create regular table for chunks with foreign key to vss table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                rowid INTEGER,
                FOREIGN KEY (rowid) REFERENCES chunks_vss(rowid)
            );",
            [],
        )?;

        Ok(())
    }

    pub fn update_with_chunks(&self, chunks: Vec<String>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // For now, store chunks without embeddings
        // In real implementation, generate embeddings for chunks
        for chunk in chunks {
            conn.execute(
                "INSERT INTO chunks (content) VALUES (?);",
                [&chunk],
            )?;
        }
        Ok(())
    }

    pub fn query_chunks(&self, embeddings: Vec<Vec<f32>>) -> Result<Vec<String>> {
        // For now, use the first embedding for search
        // In real implementation, handle multiple embeddings
        if let Some(query_embedding) = embeddings.first() {
            let results = self.search_similar(query_embedding, 10)?;
            Ok(results.into_iter().map(|(content, _)| content).collect())
        } else {
            Ok(vec![])
        }
    }
}

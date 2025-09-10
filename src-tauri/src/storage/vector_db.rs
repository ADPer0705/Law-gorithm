// Vector database operations integrated with sqlite-vss
use rusqlite::Result;
use crate::storage::Database;

impl Database {
    pub fn store_embedding(&self, content: &str, embedding: &[f32]) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        
        if self.config.provider != "sqlite-vss" {
            return Err(rusqlite::Error::InvalidQuery);
        }

        // Insert into regular table first
        conn.execute(
            "INSERT INTO chunks (content) VALUES (?);",
            [content],
        )?;
        let rowid = conn.last_insert_rowid();

        // Insert embedding into vss table
        let embedding_str = format!("[{}]", embedding.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(","));

        conn.execute(
            "INSERT INTO chunks_vss (rowid, embedding) VALUES (?, ?);",
            [&rowid.to_string(), &embedding_str],
        )?;

        Ok(rowid)
    }

    pub fn search_similar(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<(String, f32)>> {
        let conn = self.conn.lock().unwrap();
        
        if self.config.provider != "sqlite-vss" {
            return Ok(vec![]);
        }

        let embedding_str = format!("[{}]", query_embedding.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(","));

        let sql = format!(
            "SELECT c.content, v.distance
             FROM chunks_vss v
             JOIN chunks c ON v.rowid = c.rowid
             WHERE vss_search(v.embedding, ?) = 1
             ORDER BY v.distance
             LIMIT {}",
            limit
        );

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([embedding_str], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f32>(1)?))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }
}

use crate::database::DbState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub id: String,
    pub word: String,
    pub language_code: String,
    pub source: String,
    pub created_at: String,
}

pub fn add_word(
    app_handle: &AppHandle,
    word: &str,
    language_code: &str,
    source: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let word = word.trim();
    if word.is_empty() {
        return Err("Word cannot be empty".into());
    }

    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let id = format!("word_{}", uuid_v4_fallback());
    conn.execute(
        "INSERT OR IGNORE INTO dictionary_entries (id, word, language_code, source, created_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        params![id, word, language_code, source],
    )?;

    // Emit event to update search/suggestions
    let _ = app_handle.emit("dictionary:updated", word);

    Ok(id)
}

pub fn remove_word(
    app_handle: &AppHandle,
    id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    conn.execute(
        "DELETE FROM dictionary_entries WHERE id = ?1",
        params![id],
    )?;

    let _ = app_handle.emit("dictionary:updated", "");
    Ok(())
}

pub fn list_words(
    app_handle: &AppHandle,
    language_code: &str,
) -> Result<Vec<DictionaryEntry>, Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = conn.prepare(
        "SELECT id, word, language_code, source, created_at 
         FROM dictionary_entries 
         WHERE language_code = ?1 
         ORDER BY word ASC",
    )?;

    let rows = stmt.query_map(params![language_code], |row| {
        Ok(DictionaryEntry {
            id: row.get(0)?,
            word: row.get(1)?,
            language_code: row.get(2)?,
            source: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

#[derive(Serialize, Deserialize)]
struct ImportWord {
    word: String,
    language_code: String,
}

pub fn import_dict(
    app_handle: &AppHandle,
    file_path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err("File does not exist".into());
    }

    let data = fs::read_to_string(path)?;
    let words: Vec<ImportWord> = serde_json::from_str(&data)?;

    let db = app_handle.state::<DbState>();
    let mut conn = db.lock().unwrap();
    let tx = conn.transaction()?;

    let mut count = 0;
    {
        let mut stmt = tx.prepare(
            "INSERT OR IGNORE INTO dictionary_entries (id, word, language_code, source, created_at)
             VALUES (?1, ?2, ?3, 'import', datetime('now'))",
        )?;

        for item in words {
            let id = format!("word_{}", uuid_v4_fallback());
            let word = item.word.trim();
            if !word.is_empty() {
                stmt.execute(params![id, word, item.language_code])?;
                count += 1;
            }
        }
    }
    tx.commit()?;

    let _ = app_handle.emit("dictionary:updated", "");
    Ok(count)
}

pub fn export_dict(
    app_handle: &AppHandle,
    format_type: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = conn.prepare(
        "SELECT word, language_code FROM dictionary_entries ORDER BY word ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(ImportWord {
            word: row.get(0)?,
            language_code: row.get(1)?,
        })
    })?;

    let words: Vec<ImportWord> = rows.filter_map(|r| r.ok()).collect();

    if format_type.to_lowercase() == "json" {
        let json_str = serde_json::to_string_pretty(&words)?;
        
        let app_dir = app_handle.path().app_data_dir()?;
        let export_path = app_dir.join("boarddy_dictionary_export.json");
        fs::write(&export_path, json_str)?;
        
        Ok(export_path.to_string_lossy().to_string())
    } else {
        Err("Unsupported export format".into())
    }
}

fn uuid_v4_fallback() -> String {
    let rand_num = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", rand_num)
}

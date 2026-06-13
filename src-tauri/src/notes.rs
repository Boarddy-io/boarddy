use crate::database::DbState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub source_clipboard_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_note(
    app_handle: &AppHandle,
    title: Option<&str>,
    content: &str,
    source_clipboard_id: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let content = content.trim();
    if content.is_empty() {
        return Err("Note content cannot be empty".into());
    }

    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let id = format!("note_{}", uuid_v4_fallback());
    let title_val = title.map(|t| t.trim().to_string());

    conn.execute(
        "INSERT INTO notes (id, title, content, source_clipboard_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'), datetime('now'))",
        params![id, title_val, content, source_clipboard_id],
    )?;

    let _ = app_handle.emit("note:created", id.clone());

    Ok(id)
}

pub fn create_note_from_clipboard(
    app_handle: &AppHandle,
    clipboard_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();

    // Fetch clipboard content in a scoped block to release the mutex lock
    let (content, source_app, created_at): (String, Option<String>, String) = {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT content, source_app, created_at FROM clipboard_entries WHERE id = ?1",
        )?;
        stmt.query_row(params![clipboard_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?
    };

    let title_suffix = source_app.unwrap_or_else(|| "Clipboard".to_string());
    let title = format!("Saved from {} ({})", title_suffix, &created_at[11..16]);

    create_note(app_handle, Some(&title), &content, Some(clipboard_id))
}

pub fn update_note(
    app_handle: &AppHandle,
    id: &str,
    title: Option<&str>,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = content.trim();
    if content.is_empty() {
        return Err("Note content cannot be empty".into());
    }

    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let title_val = title.map(|t| t.trim().to_string());

    conn.execute(
        "UPDATE notes 
         SET title = ?1, content = ?2, updated_at = datetime('now') 
         WHERE id = ?3",
        params![title_val, content, id],
    )?;

    Ok(())
}

pub fn delete_note(
    app_handle: &AppHandle,
    id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;

    Ok(())
}

pub fn list_notes(
    app_handle: &AppHandle,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = conn.prepare(
        "SELECT id, title, content, source_clipboard_id, created_at, updated_at 
         FROM notes 
         ORDER BY created_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            source_clipboard_id: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

fn uuid_v4_fallback() -> String {
    let rand_num = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", rand_num)
}

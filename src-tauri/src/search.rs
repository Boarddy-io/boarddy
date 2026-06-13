use crate::clipboard::ClipboardEntry;
use crate::database::DbState;
use crate::notes::Note;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize)]
pub struct GlobalSearchResult {
    pub clipboard: Vec<ClipboardEntry>,
    pub notes: Vec<Note>,
}

pub fn search_clipboard(
    app_handle: &AppHandle,
    query: &str,
) -> Result<Vec<ClipboardEntry>, Box<dyn std::error::Error>> {
    let query = query.trim();
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    if query.is_empty() {
        // Fallback: return recent items
        let mut stmt = conn.prepare(
            "SELECT id, content, content_type, source_app, source_window, language_code, is_favorite, is_pinned, created_at, updated_at 
             FROM clipboard_entries 
             ORDER BY created_at DESC LIMIT 50",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ClipboardEntry {
                id: row.get(0)?,
                content: row.get(1)?,
                content_type: row.get(2)?,
                source_app: row.get(3)?,
                source_window: row.get(4)?,
                language_code: row.get(5)?,
                is_favorite: row.get::<_, i32>(6)? == 1,
                is_pinned: row.get::<_, i32>(7)? == 1,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        return Ok(list);
    }

    // Clean query to avoid FTS syntax errors, escape double quotes
    let clean_query = query.replace('"', "\"\"");
    let fts_query = format!("\"{}\"*", clean_query);

    let mut stmt = conn.prepare(
        "SELECT ce.id, ce.content, ce.content_type, ce.source_app, ce.source_window, ce.language_code, ce.is_favorite, ce.is_pinned, ce.created_at, ce.updated_at
         FROM clipboard_entries ce
         JOIN clipboard_fts fts ON ce.rowid = fts.rowid
         WHERE clipboard_fts MATCH ?1
         ORDER BY rank LIMIT 100",
    )?;

    let rows = stmt.query_map(params![fts_query], |row| {
        Ok(ClipboardEntry {
            id: row.get(0)?,
            content: row.get(1)?,
            content_type: row.get(2)?,
            source_app: row.get(3)?,
            source_window: row.get(4)?,
            language_code: row.get(5)?,
            is_favorite: row.get::<_, i32>(6)? == 1,
            is_pinned: row.get::<_, i32>(7)? == 1,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

pub fn search_notes(
    app_handle: &AppHandle,
    query: &str,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let query = query.trim();
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    if query.is_empty() {
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
        return Ok(list);
    }

    let clean_query = query.replace('"', "\"\"");
    let fts_query = format!("\"{}\"*", clean_query);

    let mut stmt = conn.prepare(
        "SELECT n.id, n.title, n.content, n.source_clipboard_id, n.created_at, n.updated_at
         FROM notes n
         JOIN notes_fts fts ON n.rowid = fts.rowid
         WHERE notes_fts MATCH ?1
         ORDER BY rank LIMIT 100",
    )?;

    let rows = stmt.query_map(params![fts_query], |row| {
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

pub fn global_search(
    app_handle: &AppHandle,
    query: &str,
) -> Result<GlobalSearchResult, Box<dyn std::error::Error>> {
    let clipboard = search_clipboard(app_handle, query)?;
    let notes = search_notes(app_handle, query)?;
    Ok(GlobalSearchResult { clipboard, notes })
}

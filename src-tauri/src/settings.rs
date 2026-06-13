use crate::database::DbState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Emitter};

#[derive(Serialize, Deserialize)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub is_enabled: bool,
}

pub fn get_settings(
    app_handle: &AppHandle,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut map = HashMap::new();
    for r in rows {
        let (k, v) = r?;
        map.insert(k, v);
    }
    Ok(map)
}

pub fn update_setting(
    app_handle: &AppHandle,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) 
         VALUES (?1, ?2, datetime('now'))",
        params![key, value],
    )?;

    // Emit event
    let _ = app_handle.emit("settings:changed", key);

    Ok(())
}

pub fn get_languages(
    app_handle: &AppHandle,
) -> Result<Vec<Language>, Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = conn.prepare(
        "SELECT language_code, language_name, is_enabled FROM languages ORDER BY language_name ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Language {
            code: row.get(0)?,
            name: row.get(1)?,
            is_enabled: row.get::<_, i32>(2)? == 1,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

pub fn enable_language(
    app_handle: &AppHandle,
    code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    conn.execute(
        "UPDATE languages SET is_enabled = 1 WHERE language_code = ?1",
        params![code],
    )?;

    let _ = app_handle.emit("language:changed", code);
    Ok(())
}

pub fn disable_language(
    app_handle: &AppHandle,
    code: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    conn.execute(
        "UPDATE languages SET is_enabled = 0 WHERE language_code = ?1",
        params![code],
    )?;

    let _ = app_handle.emit("language:changed", code);
    Ok(())
}

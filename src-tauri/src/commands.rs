use crate::clipboard::{self, ClipboardEntry, ClipboardListResult};
use crate::database::DbState;
use crate::dictionary::{self, DictionaryEntry};
use crate::notes::{self, Note};
use crate::search::{self, GlobalSearchResult};
use crate::settings::{self, Language};
use crate::win_util;
use rusqlite::params;
use serde::Serialize;
use std::collections::HashMap;
use tauri::{AppHandle, Manager};

#[derive(Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: &str, message: &str) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.to_string(),
                message: message.to_string(),
            }),
        }
    }
}

// Helper to convert Result to ApiResponse
fn to_response<T, E: std::fmt::Display>(r: Result<T, E>, err_code: &str) -> ApiResponse<T> {
    match r {
        Ok(data) => ApiResponse::success(data),
        Err(e) => ApiResponse::error(err_code, &e.to_string()),
    }
}

// --- Clipboard Commands ---

#[tauri::command]
pub fn get_clipboard_entries(
    app: AppHandle,
    page: u32,
    limit: u32,
) -> ApiResponse<ClipboardListResult> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let offset = (page.saturating_sub(1)) * limit;

    let mut stmt = match conn.prepare(
        "SELECT id, content, content_type, source_app, source_window, language_code, is_favorite, is_pinned, created_at, updated_at 
         FROM clipboard_entries 
         ORDER BY is_pinned DESC, created_at DESC LIMIT ?1 OFFSET ?2",
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::error("DB_ERROR", &e.to_string()),
    };

    let rows = stmt.query_map(params![limit, offset], |row| {
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
    });

    let items = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return ApiResponse::error("QUERY_ERROR", &e.to_string()),
    };

    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM clipboard_entries", [], |row| row.get(0))
        .unwrap_or(0);

    ApiResponse::success(ClipboardListResult { items, total })
}

#[tauri::command]
pub fn get_clipboard_entry(app: AppHandle, id: String) -> ApiResponse<ClipboardEntry> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let entry = conn.query_row(
        "SELECT id, content, content_type, source_app, source_window, language_code, is_favorite, is_pinned, created_at, updated_at 
         FROM clipboard_entries WHERE id = ?1",
        params![id],
        |row| {
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
        },
    );

    to_response(entry, "ENTRY_NOT_FOUND")
}

#[tauri::command]
pub fn search_clipboard(app: AppHandle, query: String) -> ApiResponse<Vec<ClipboardEntry>> {
    to_response(search::search_clipboard(&app, &query), "SEARCH_ERROR")
}

#[tauri::command]
pub fn delete_clipboard_entry(app: AppHandle, id: String) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let r = conn.execute("DELETE FROM clipboard_entries WHERE id = ?1", params![id]);
    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("DELETE_FAILED", &e.to_string()),
    }
}

#[tauri::command]
pub fn clear_clipboard_history(app: AppHandle) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    // Do not delete pinned or favorited entries
    let r = conn.execute(
        "DELETE FROM clipboard_entries WHERE is_pinned = 0 AND is_favorite = 0",
        [],
    );
    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("CLEAR_FAILED", &e.to_string()),
    }
}

#[tauri::command]
pub fn pin_clipboard_entry(app: AppHandle, id: String) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let r = conn.execute(
        "UPDATE clipboard_entries SET is_pinned = 1 WHERE id = ?1",
        params![id],
    );
    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("PIN_FAILED", &e.to_string()),
    }
}

#[tauri::command]
pub fn unpin_clipboard_entry(app: AppHandle, id: String) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let r = conn.execute(
        "UPDATE clipboard_entries SET is_pinned = 0 WHERE id = ?1",
        params![id],
    );
    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("UNPIN_FAILED", &e.to_string()),
    }
}

#[tauri::command]
pub fn favorite_clipboard_entry(app: AppHandle, id: String) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let r = conn.execute(
        "UPDATE clipboard_entries SET is_favorite = 1 WHERE id = ?1",
        params![id],
    );
    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("FAVORITE_FAILED", &e.to_string()),
    }
}

#[tauri::command]
pub fn unfavorite_clipboard_entry(app: AppHandle, id: String) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let r = conn.execute(
        "UPDATE clipboard_entries SET is_favorite = 0 WHERE id = ?1",
        params![id],
    );
    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("UNFAVORITE_FAILED", &e.to_string()),
    }
}

// --- Notes Commands ---

#[tauri::command]
pub fn create_note(app: AppHandle, title: Option<String>, content: String) -> ApiResponse<String> {
    to_response(
        notes::create_note(&app, title.as_deref(), &content, None),
        "CREATE_NOTE_FAILED",
    )
}

#[tauri::command]
pub fn create_note_from_clipboard(app: AppHandle, clipboard_id: String) -> ApiResponse<String> {
    to_response(
        notes::create_note_from_clipboard(&app, &clipboard_id),
        "CONVERT_CLIPBOARD_FAILED",
    )
}

#[tauri::command]
pub fn update_note(app: AppHandle, id: String, title: Option<String>, content: String) -> ApiResponse<()> {
    to_response(
        notes::update_note(&app, &id, title.as_deref(), &content),
        "UPDATE_NOTE_FAILED",
    )
}

#[tauri::command]
pub fn delete_note(app: AppHandle, id: String) -> ApiResponse<()> {
    to_response(notes::delete_note(&app, &id), "DELETE_NOTE_FAILED")
}

#[tauri::command]
pub fn list_notes(app: AppHandle) -> ApiResponse<Vec<Note>> {
    to_response(notes::list_notes(&app), "LIST_NOTES_FAILED")
}

#[tauri::command]
pub fn search_notes(app: AppHandle, query: String) -> ApiResponse<Vec<Note>> {
    to_response(search::search_notes(&app, &query), "SEARCH_NOTES_FAILED")
}

// --- Dictionary Commands ---

#[tauri::command]
pub fn add_dictionary_word(app: AppHandle, word: String, language: String) -> ApiResponse<String> {
    to_response(
        dictionary::add_word(&app, &word, &language, "user"),
        "ADD_WORD_FAILED",
    )
}

#[tauri::command]
pub fn remove_dictionary_word(app: AppHandle, id: String) -> ApiResponse<()> {
    to_response(dictionary::remove_word(&app, &id), "REMOVE_WORD_FAILED")
}

#[tauri::command]
pub fn list_dictionary_words(app: AppHandle, language: String) -> ApiResponse<Vec<DictionaryEntry>> {
    to_response(
        dictionary::list_words(&app, &language),
        "LIST_WORDS_FAILED",
    )
}

#[tauri::command]
pub fn import_dictionary(app: AppHandle, file_path: String) -> ApiResponse<usize> {
    to_response(
        dictionary::import_dict(&app, &file_path),
        "IMPORT_FAILED",
    )
}

#[tauri::command]
pub fn export_dictionary(app: AppHandle, format: String) -> ApiResponse<String> {
    to_response(
        dictionary::export_dict(&app, &format),
        "EXPORT_FAILED",
    )
}

// --- Autocorrect Commands ---

#[tauri::command]
pub fn get_correction_suggestions(
    app: AppHandle,
    word: String,
    language: String,
) -> ApiResponse<Vec<String>> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = match conn.prepare(
        "SELECT replacement_word FROM autocorrect_rules WHERE LOWER(trigger_word) = LOWER(?1) AND language_code = ?2",
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::error("QUERY_ERROR", &e.to_string()),
    };

    let suggestion: Option<String> = stmt
        .query_row(params![word, language], |row| row.get(0))
        .ok();

    if let Some(s) = suggestion {
        ApiResponse::success(vec![s])
    } else {
        ApiResponse::success(vec![])
    }
}

#[tauri::command]
pub fn add_correction_rule(app: AppHandle, trigger: String, replacement: String) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let id = format!("rule_{}", trigger.to_lowercase());
    let r = conn.execute(
        "INSERT OR REPLACE INTO autocorrect_rules (id, trigger_word, replacement_word, language_code, created_at)
         VALUES (?1, ?2, ?3, 'en', datetime('now'))",
        params![id, trigger.trim(), replacement.trim()],
    );

    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("ADD_RULE_FAILED", &e.to_string()),
    }
}

#[tauri::command]
pub fn remove_correction_rule(app: AppHandle, rule_id: String) -> ApiResponse<()> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let r = conn.execute(
        "DELETE FROM autocorrect_rules WHERE id = ?1",
        params![rule_id],
    );

    match r {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error("REMOVE_RULE_FAILED", &e.to_string()),
    }
}

#[derive(Serialize)]
pub struct AutocorrectRule {
    pub id: String,
    pub trigger_word: String,
    pub replacement_word: String,
    pub language_code: Option<String>,
    pub created_at: String,
}

#[tauri::command]
pub fn get_autocorrect_rules(app: AppHandle) -> ApiResponse<Vec<AutocorrectRule>> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = match conn.prepare(
        "SELECT id, trigger_word, replacement_word, language_code, created_at FROM autocorrect_rules ORDER BY trigger_word ASC",
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::error("QUERY_ERROR", &e.to_string()),
    };

    let rows = stmt.query_map([], |row| {
        Ok(AutocorrectRule {
            id: row.get(0)?,
            trigger_word: row.get(1)?,
            replacement_word: row.get(2)?,
            language_code: row.get(3)?,
            created_at: row.get(4)?,
        })
    });

    let items = match rows {
        Ok(r) => r.filter_map(|x| x.ok()).collect(),
        Err(e) => return ApiResponse::error("QUERY_ERROR", &e.to_string()),
    };

    ApiResponse::success(items)
}


// --- Autocomplete Commands ---

#[tauri::command]
pub fn get_autocomplete_suggestions(app: AppHandle, prefix: String) -> ApiResponse<Vec<String>> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let query = format!("{}%", prefix.trim());
    let mut stmt = match conn.prepare(
        "SELECT word FROM dictionary_entries WHERE word LIKE ?1
         UNION
         SELECT word FROM autocomplete_cache WHERE word LIKE ?1
         ORDER BY word LIMIT 5",
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::error("QUERY_ERROR", &e.to_string()),
    };

    let list = stmt
        .query_map(params![query], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    ApiResponse::success(list)
}

// --- Language Commands ---

#[tauri::command]
pub fn get_languages(app: AppHandle) -> ApiResponse<Vec<Language>> {
    to_response(settings::get_languages(&app), "GET_LANGUAGES_FAILED")
}

#[tauri::command]
pub fn enable_language(app: AppHandle, code: String) -> ApiResponse<()> {
    to_response(settings::enable_language(&app, &code), "ENABLE_LANG_FAILED")
}

#[tauri::command]
pub fn disable_language(app: AppHandle, code: String) -> ApiResponse<()> {
    to_response(settings::disable_language(&app, &code), "DISABLE_LANG_FAILED")
}

// --- Global Search ---

#[tauri::command]
pub fn global_search(app: AppHandle, query: String) -> ApiResponse<GlobalSearchResult> {
    to_response(search::global_search(&app, &query), "GLOBAL_SEARCH_FAILED")
}

// --- Settings Commands ---

#[tauri::command]
pub fn get_settings(app: AppHandle) -> ApiResponse<HashMap<String, String>> {
    to_response(settings::get_settings(&app), "GET_SETTINGS_FAILED")
}

#[tauri::command]
pub fn update_setting(app: AppHandle, key: String, value: String) -> ApiResponse<()> {
    let res = settings::update_setting(&app, &key, &value);
    if res.is_ok() {
        crate::typing::reload_config(&app);
    }
    to_response(res, "UPDATE_SETTING_FAILED")
}

// --- Shortcuts Commands ---

#[tauri::command]
pub fn get_shortcuts(app: AppHandle) -> ApiResponse<HashMap<String, String>> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = match conn.prepare(
        "SELECT key, value FROM settings WHERE key LIKE '%shortcut%' OR key LIKE 'gesture_%'",
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::error("QUERY_ERROR", &e.to_string()),
    };

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    });

    let mut map = HashMap::new();
    if let Ok(r) = rows {
        for item in r {
            if let Ok((k, v)) = item {
                map.insert(k, v);
            }
        }
    }

    ApiResponse::success(map)
}

#[tauri::command]
pub fn update_shortcut(app: AppHandle, action: String, shortcut: String) -> ApiResponse<()> {
    // Save to settings table using setting key for shortcut
    let key = if action.starts_with("gesture_") {
        action.clone()
    } else {
        format!("{}_shortcut", action)
    };
    let res = settings::update_setting(&app, &key, &shortcut);
    if res.is_ok() {
        crate::typing::reload_config(&app);
    }
    to_response(
        res,
        "UPDATE_SHORTCUT_FAILED",
    )
}

#[tauri::command]
pub fn check_shortcut_conflict(app: AppHandle, action: String, shortcut: String) -> ApiResponse<Option<String>> {
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();
    
    let key_to_ignore = if action.starts_with("gesture_") {
        action.clone()
    } else {
        format!("{}_shortcut", action)
    };

    let mut stmt = match conn.prepare(
        "SELECT key, value FROM settings WHERE (key LIKE '%shortcut%' OR key LIKE 'gesture_%') AND key != ?1",
    ) {
        Ok(s) => s,
        Err(e) => return ApiResponse::error("DB_ERROR", &e.to_string()),
    };

    let rows = match stmt.query_map(params![key_to_ignore], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }) {
        Ok(r) => r,
        Err(e) => return ApiResponse::error("QUERY_ERROR", &e.to_string()),
    };

    for r in rows {
        if let Ok((k, v)) = r {
            if v.to_lowercase().replace(" ", "") == shortcut.to_lowercase().replace(" ", "") {
                let friendly_name = k
                    .replace("gesture_", "")
                    .replace("_shortcut", "")
                    .replace("_", " ");
                return ApiResponse::success(Some(friendly_name));
            }
        }
    }
    ApiResponse::success(None)
}

// --- Window Manipulation Commands ---

#[tauri::command]
pub fn show_main_window(app: AppHandle) -> ApiResponse<()> {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
    }
    ApiResponse::success(())
}

#[tauri::command]
pub fn hide_main_window(app: AppHandle) -> ApiResponse<()> {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
    ApiResponse::success(())
}

#[tauri::command]
pub fn show_quick_paste(app: AppHandle) -> ApiResponse<()> {
    if let Some(w) = app.get_webview_window("quick_paste") {
        let _ = w.show();
        let _ = w.set_focus();
    }
    ApiResponse::success(())
}

#[tauri::command]
pub fn hide_quick_paste(app: AppHandle) -> ApiResponse<()> {
    if let Some(w) = app.get_webview_window("quick_paste") {
        let _ = w.hide();
    }
    ApiResponse::success(())
}

// Instantly pastes selected text into active window
#[tauri::command]
pub fn paste_text(app: AppHandle, text: String) -> ApiResponse<()> {
    // Increment paste count in the database (local learning engine)
    let db = app.state::<DbState>();
    let conn = db.lock().unwrap();
    let _ = conn.execute(
        "UPDATE clipboard_entries SET paste_count = paste_count + 1 WHERE content = ?1",
        params![text],
    );
    drop(conn);

    #[cfg(target_os = "windows")]
    {
        // 1. Store currently copied text
        let old_text = clipboard::get_native_clipboard_text();
        
        // 2. Set clipboard to new selected text
        clipboard::set_native_clipboard_text(&text);
        
        // 3. Trigger Ctrl + V paste action
        win_util::send_paste_shortcut();
        
        // 4. Restore old clipboard after a brief delay
        let app_clone = app.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(150));
            if let Some(t) = old_text {
                clipboard::set_native_clipboard_text(&t);
            }
        });
    }
    ApiResponse::success(())
}

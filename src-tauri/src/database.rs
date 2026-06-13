use rusqlite::{params, Connection};
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

pub type DbState = Arc<Mutex<Connection>>;

pub fn init_db(app_handle: &AppHandle) -> Result<DbState, Box<dyn std::error::Error>> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }

    let db_path = app_dir.join("boarddy.db");
    let conn = Connection::open(db_path)?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    // Optimize SQLite for desktop responsiveness
    conn.execute("PRAGMA journal_mode = WAL;", [])?;
    conn.execute("PRAGMA synchronous = NORMAL;", [])?;

    let db = Arc::new(Mutex::new(conn));
    setup_schema(&db)?;

    Ok(db)
}

fn setup_schema(db: &DbState) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.lock().unwrap();

    // 1. Settings Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT
        );",
        [],
    )?;

    // 2. Clipboard Entries Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_entries (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            content_type TEXT NOT NULL,
            source_app TEXT,
            source_window TEXT,
            language_code TEXT,
            is_favorite INTEGER DEFAULT 0,
            is_pinned INTEGER DEFAULT 0,
            created_at TEXT,
            updated_at TEXT
        );",
        [],
    )?;

    // 3. Clipboard Metadata Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_metadata (
            id TEXT PRIMARY KEY,
            clipboard_id TEXT NOT NULL,
            key TEXT NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (clipboard_id) REFERENCES clipboard_entries(id) ON DELETE CASCADE
        );",
        [],
    )?;

    // 4. Clipboard Tags Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_tags (
            id TEXT PRIMARY KEY,
            clipboard_id TEXT NOT NULL,
            tag TEXT NOT NULL,
            created_at TEXT,
            FOREIGN KEY (clipboard_id) REFERENCES clipboard_entries(id) ON DELETE CASCADE
        );",
        [],
    )?;

    // 5. Notes Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT,
            content TEXT NOT NULL,
            source_clipboard_id TEXT,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY (source_clipboard_id) REFERENCES clipboard_entries(id) ON DELETE SET NULL
        );",
        [],
    )?;

    // 6. Note Tags Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note_tags (
            id TEXT PRIMARY KEY,
            note_id TEXT NOT NULL,
            tag TEXT NOT NULL,
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
        );",
        [],
    )?;

    // 7. Dictionary Entries Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS dictionary_entries (
            id TEXT PRIMARY KEY,
            word TEXT NOT NULL UNIQUE,
            language_code TEXT NOT NULL,
            source TEXT,
            created_at TEXT
        );",
        [],
    )?;

    // 8. Languages Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS languages (
            id TEXT PRIMARY KEY,
            language_code TEXT UNIQUE,
            language_name TEXT,
            is_enabled INTEGER DEFAULT 1,
            created_at TEXT
        );",
        [],
    )?;

    // 9. Autocorrect Rules Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS autocorrect_rules (
            id TEXT PRIMARY KEY,
            trigger_word TEXT NOT NULL UNIQUE,
            replacement_word TEXT NOT NULL,
            language_code TEXT,
            created_at TEXT
        );",
        [],
    )?;

    // 10. Autocomplete Cache Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS autocomplete_cache (
            id TEXT PRIMARY KEY,
            word TEXT NOT NULL UNIQUE,
            language_code TEXT,
            frequency INTEGER DEFAULT 0,
            last_used_at TEXT
        );",
        [],
    )?;

    // 10b. Autocomplete Ranking Table (Adaptive Ranking)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS autocomplete_ranking (
            prefix TEXT,
            word TEXT,
            frequency INTEGER DEFAULT 1,
            last_selected_at TEXT,
            PRIMARY KEY (prefix, word)
        );",
        [],
    )?;

    // Add paste_count column if it doesn't exist (ignore error if it already exists)
    let _ = conn.execute("ALTER TABLE clipboard_entries ADD COLUMN paste_count INTEGER DEFAULT 0;", []);

    // 11. App Usage Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_usage (
            id TEXT PRIMARY KEY,
            app_name TEXT UNIQUE,
            usage_count INTEGER DEFAULT 0,
            last_used_at TEXT
        );",
        [],
    )?;

    // 12. Search History Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS search_history (
            id TEXT PRIMARY KEY,
            query TEXT,
            created_at TEXT
        );",
        [],
    )?;

    // Recommended Indexes
    conn.execute("CREATE INDEX IF NOT EXISTS idx_clipboard_created ON clipboard_entries(created_at);", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_clipboard_type ON clipboard_entries(content_type);", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_dictionary_word ON dictionary_entries(word);", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_created ON notes(created_at);", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_search_history ON search_history(created_at);", [])?;

    // --- FTS5 SETUP ---
    // FTS5 Virtual Tables (External Content linked to baseline tables)
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS clipboard_fts USING fts5(
            content,
            content='clipboard_entries',
            content_id='id'
        );",
        [],
    )?;

    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
            title,
            content,
            content='notes',
            content_id='id'
        );",
        [],
    )?;

    // Triggers to keep FTS5 indexes in sync with base tables
    // Clipboard Entries Triggers
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS clipboard_entries_ai AFTER INSERT ON clipboard_entries BEGIN
            INSERT INTO clipboard_fts(rowid, content) VALUES (new.rowid, new.content);
        END;",
        [],
    )?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS clipboard_entries_ad AFTER DELETE ON clipboard_entries BEGIN
            INSERT INTO clipboard_fts(clipboard_fts, rowid, content) VALUES('delete', old.rowid, old.content);
        END;",
        [],
    )?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS clipboard_entries_au AFTER UPDATE ON clipboard_entries BEGIN
            INSERT INTO clipboard_fts(clipboard_fts, rowid, content) VALUES('delete', old.rowid, old.content);
            INSERT INTO clipboard_fts(rowid, content) VALUES (new.rowid, new.content);
        END;",
        [],
    )?;

    // Notes Triggers
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS notes_ai AFTER INSERT ON notes BEGIN
            INSERT INTO notes_fts(rowid, title, content) VALUES (new.rowid, new.title, new.content);
        END;",
        [],
    )?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS notes_ad AFTER DELETE ON notes BEGIN
            INSERT INTO notes_fts(notes_fts, rowid, title, content) VALUES('delete', old.rowid, old.title, old.content);
        END;",
        [],
    )?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS notes_au AFTER UPDATE ON notes BEGIN
            INSERT INTO notes_fts(notes_fts, rowid, title, content) VALUES('delete', old.rowid, old.title, old.content);
            INSERT INTO notes_fts(rowid, title, content) VALUES (new.rowid, new.title, new.content);
        END;",
        [],
    )?;

    // Seed default settings if not exists
    seed_defaults(&conn)?;

    Ok(())
}

fn seed_defaults(conn: &Connection) -> Result<(), rusqlite::Error> {
    let defaults = [
        ("theme", "dark"),
        ("language", "en"),
        ("clipboard_limit", "100"),
        ("launch_on_startup", "false"),
        ("quick_paste_shortcut", "Ctrl+Shift+V"),
        ("autocorrect_enabled", "true"),
        ("autocomplete_enabled", "true"),
        ("number_selection", "true"),
        ("letter_selection", "false"),
        ("right_arrow_acceptance", "true"),
        ("adaptive_ranking", "true"),
        ("keyboard_gestures_enabled", "true"),
        ("double_shift_clipboard", "false"),
        ("gesture_conflict_detection", "true"),
        ("gesture_word_delete_backspace", "Backspace+Left"),
        ("gesture_word_delete_delete", "Delete+Right"),
        ("gesture_line_delete_up", "Alt+Backspace+Up"),
        ("gesture_line_delete_down", "Alt+Delete+Down"),
        ("gesture_cursor_move_char", "Space+Left/Right"),
        ("gesture_cursor_move_word", "Space+Up/Down"),
        ("gesture_select_word", "Space+W"),
        ("gesture_select_line", "Space+L"),
        ("gesture_select_paragraph", "Space+P"),
    ];

    for (k, v) in defaults.iter() {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'));",
            [k, v],
        )?;
    }

    // Seed default autocorrect rules if not exists
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM autocorrect_rules")?;
    let ar_count: i64 = stmt.query_row([], |r| r.get(0))?;
    if ar_count == 0 {
        let default_rules = [
            ("teh", "the"),
            ("reciept", "receipt"),
            ("recieve", "receive"),
            ("dont", "don't"),
            ("wont", "won't"),
            ("cant", "can't"),
        ];
        for (trig, rep) in default_rules.iter() {
            let id = format!("rule_{}", trig);
            conn.execute(
                "INSERT OR IGNORE INTO autocorrect_rules (id, trigger_word, replacement_word, language_code, created_at)
                 VALUES (?1, ?2, ?3, 'en', datetime('now'));",
                params![&id, trig, rep],
            )?;
        }
    }

    // Seed default personal dictionary words
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM dictionary_entries")?;
    let dict_count: i64 = stmt.query_row([], |r| r.get(0))?;
    if dict_count == 0 {
        let default_words = ["HunaPay", "HunaRemit", "EcoSynergy", "Cloakey", "Boarddy"];
        for w in default_words.iter() {
            let id = format!("word_{}", w.to_lowercase());
            conn.execute(
                "INSERT OR IGNORE INTO dictionary_entries (id, word, language_code, source, created_at)
                 VALUES (?1, ?2, 'en', 'system', datetime('now'));",
                params![&id, w, "en"],
            )?;
        }
    }

    // Seed default languages
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM languages")?;
    let lang_count: i64 = stmt.query_row([], |r| r.get(0))?;
    if lang_count == 0 {
        let default_langs = [
            ("en", "English"),
            ("yo", "Yoruba"),
            ("ha", "Hausa"),
            ("ig", "Igbo"),
            ("fr", "French"),
            ("es", "Spanish"),
        ];
        for (code, name) in default_langs.iter() {
            let id = format!("lang_{}", code);
            conn.execute(
                "INSERT OR IGNORE INTO languages (id, language_code, language_name, is_enabled, created_at)
                 VALUES (?1, ?2, ?3, 1, datetime('now'));",
                params![&id, code, name],
            )?;
        }
    }

    Ok(())
}

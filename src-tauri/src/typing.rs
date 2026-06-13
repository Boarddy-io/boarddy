#![cfg_attr(not(target_os = "windows"), allow(dead_code, unused_variables))]

use crate::database::DbState;
use crate::win_util;
use rusqlite::params;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, Emitter};

// State for global keyboard hooks
static APP_HANDLE: Mutex<Option<AppHandle>> = Mutex::new(None);
static TYPED_BUFFER: Mutex<String> = Mutex::new(String::new());
static SIMULATING_INPUT: AtomicBool = AtomicBool::new(false);

// Stores information to undo the last correction
struct UndoInfo {
    correction_len: usize,
    original: String,
    boundary: String,
}
static LAST_CORRECTION: Mutex<Option<UndoInfo>> = Mutex::new(None);

#[derive(serde::Serialize, Clone)]
pub struct Suggestion {
    pub word: String,
    pub shortcut: String,
    pub confidence: u32,
    pub rank: usize,
}

pub struct TypingConfig {
    pub autocorrect_enabled: bool,
    pub autocomplete_enabled: bool,
    pub pks_enabled: bool,
    pub right_arrow_acceptance: bool,
    pub number_selection: bool,
    pub letter_selection: bool,
    pub adaptive_ranking: bool,
    pub keyboard_gestures_enabled: bool,
    pub double_shift_clipboard: bool,
}

static TYPING_CONFIG: Mutex<TypingConfig> = Mutex::new(TypingConfig {
    autocorrect_enabled: true,
    autocomplete_enabled: true,
    pks_enabled: true,
    right_arrow_acceptance: true,
    number_selection: true,
    letter_selection: false,
    adaptive_ranking: true,
    keyboard_gestures_enabled: true,
    double_shift_clipboard: false,
});

// Autocomplete suggestions state
static SUGGESTIONS: Mutex<Vec<Suggestion>> = Mutex::new(Vec::new());
static SUGGESTION_WINDOW_VISIBLE: AtomicBool = AtomicBool::new(false);
static LAST_SHIFT_TIME: Mutex<Option<std::time::Instant>> = Mutex::new(None);
static SPACE_ERASED: AtomicBool = AtomicBool::new(false);

pub fn init_typing_service(app_handle: AppHandle) {
    *APP_HANDLE.lock().unwrap() = Some(app_handle.clone());
    reload_config(&app_handle);
    #[cfg(target_os = "windows")]
    start_keyboard_hook();
}

pub fn reload_config(app_handle: &AppHandle) {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let mut stmt = match conn.prepare("SELECT key, value FROM settings") {
        Ok(s) => s,
        Err(_) => return,
    };

    let rows = match stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }) {
        Ok(r) => r,
        Err(_) => return,
    };

    let mut map = std::collections::HashMap::new();
    for r in rows {
        if let Ok((k, v)) = r {
            map.insert(k, v);
        }
    }

    let mut config = TYPING_CONFIG.lock().unwrap();
    config.autocorrect_enabled = map.get("autocorrect_enabled").map(|v| v == "true").unwrap_or(true);
    config.autocomplete_enabled = map.get("autocomplete_enabled").map(|v| v == "true").unwrap_or(true);
    config.pks_enabled = map.get("pks_enabled").map(|v| v == "true").unwrap_or(true);
    config.right_arrow_acceptance = map.get("right_arrow_acceptance").map(|v| v == "true").unwrap_or(true);
    config.number_selection = map.get("number_selection").map(|v| v == "true").unwrap_or(true);
    config.letter_selection = map.get("letter_selection").map(|v| v == "true").unwrap_or(false);
    config.adaptive_ranking = map.get("adaptive_ranking").map(|v| v == "true").unwrap_or(true);
    config.keyboard_gestures_enabled = map.get("keyboard_gestures_enabled").map(|v| v == "true").unwrap_or(true);
    config.double_shift_clipboard = map.get("double_shift_clipboard").map(|v| v == "true").unwrap_or(false);
}

// Letter Mode Unique Activation Key Generator
fn assign_activation_letters(prefix: &str, suggestions: &[String]) -> Option<Vec<char>> {
    let mut assigned = std::collections::HashSet::new();
    let prefix_lower = prefix.to_lowercase();
    let mut result = Vec::new();

    for word in suggestions {
        let word_lower = word.to_lowercase();
        let mut found = None;

        // 1. Try to find an unused letter that is in the word and not in the prefix
        for c in word_lower.chars() {
            if c.is_ascii_alphabetic() && !prefix_lower.contains(c) && !assigned.contains(&c) {
                found = Some(c);
                break;
            }
        }

        // 2. Fallback: try to find any unused letter in the word (even if in prefix)
        if found.is_none() {
            for c in word_lower.chars() {
                if c.is_ascii_alphabetic() && !assigned.contains(&c) {
                    found = Some(c);
                    break;
                }
            }
        }

        if let Some(c) = found {
            assigned.insert(c);
            result.push(c);
        } else {
            return None; // Fall back to numbers if uniqueness is impossible
        }
    }
    Some(result)
}

fn record_typed_word(app_handle: &AppHandle, word: &str) {
    let config = TYPING_CONFIG.lock().unwrap();
    if !config.adaptive_ranking {
        return;
    }
    drop(config);

    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let uuid = uuid_v4_fallback();
    let _ = conn.execute(
        "INSERT INTO autocomplete_cache (id, word, language_code, frequency, last_used_at)
         VALUES (?1, ?2, 'en', 1, datetime('now'))
         ON CONFLICT(word) DO UPDATE SET
             frequency = frequency + 1,
             last_used_at = datetime('now')",
        params![format!("cache_{}", uuid), word],
    );
}

fn record_suggestion_selection(app_handle: &AppHandle, prefix: &str, word: &str) {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let uuid = uuid_v4_fallback();
    // 1. Update general cache
    let _ = conn.execute(
        "INSERT INTO autocomplete_cache (id, word, language_code, frequency, last_used_at)
         VALUES (?1, ?2, 'en', 1, datetime('now'))
         ON CONFLICT(word) DO UPDATE SET
             frequency = frequency + 1,
             last_used_at = datetime('now')",
        params![format!("cache_{}", uuid), word],
    );

    // 2. Update adaptive ranking (prefix-suggestion history)
    let _ = conn.execute(
        "INSERT INTO autocomplete_ranking (prefix, word, frequency, last_selected_at)
         VALUES (?1, ?2, 1, datetime('now'))
         ON CONFLICT(prefix, word) DO UPDATE SET
             frequency = frequency + 1,
             last_selected_at = datetime('now')",
        params![prefix, word],
    );
}

fn uuid_v4_fallback() -> String {
    let rand_num = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", rand_num)
}

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyboardLayout, GetKeyboardState, GetKeyState, ToUnicodeEx, VIRTUAL_KEY, VK_BACK,
    VK_CAPITAL, VK_CONTROL, VK_ESCAPE, VK_LSHIFT, VK_RSHIFT, VK_SHIFT, VK_TAB, VK_V, VK_Z,
    VK_RIGHT,
};
#[cfg(target_os = "windows")]
use windows::Win32::UI::TextServices::HKL;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetForegroundWindow, GetGUIThreadInfo, GetWindowThreadProcessId,
    GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG,
    WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};

#[cfg(target_os = "windows")]
static mut KEYBOARD_HOOK: Option<HHOOK> = None;

#[cfg(target_os = "windows")]
fn start_keyboard_hook() {
    std::thread::spawn(|| unsafe {
        let h_instance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None).unwrap_or_default();
        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            h_instance,
            0,
        );
        if let Ok(h) = hook {
            KEYBOARD_HOOK = Some(h);
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {
                // Keep the message loop running
            }
            let _ = UnhookWindowsHookEx(h);
        }
    });
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn keyboard_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 && !SIMULATING_INPUT.load(Ordering::Relaxed) {
        let hook_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
        let vk = hook_struct.vkCode;
        let is_keydown = wparam.0 == WM_KEYDOWN as usize || wparam.0 == WM_SYSKEYDOWN as usize;

        if is_keydown {
            if handle_key_down(vk, hook_struct.scanCode) {
                return LRESULT(1); // Block propagation
            }
        }
    }
    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

#[cfg(target_os = "windows")]
fn handle_key_down(vk: u32, scan_code: u32) -> bool {
    let app_handle_opt = APP_HANDLE.lock().unwrap();
    let app_handle = match &*app_handle_opt {
        Some(h) => h,
        None => return false,
    };

    let is_ctrl = (unsafe { GetKeyState(VK_CONTROL.0 as i32) } as u16 & 0x8000) != 0;
    let is_shift = (unsafe { GetKeyState(VK_SHIFT.0 as i32) } as u16 & 0x8000) != 0;

    // Reset spacebar gesture state if Space itself is pressed
    if vk == 0x20 {
        SPACE_ERASED.store(false, Ordering::Relaxed);
    }

    // Retrieve settings parameters
    let (keyboard_gestures_enabled, pks_enabled, right_arrow_acceptance, double_shift_clipboard) = {
        let config = TYPING_CONFIG.lock().unwrap();
        (
            config.keyboard_gestures_enabled,
            config.pks_enabled,
            config.right_arrow_acceptance,
            config.double_shift_clipboard,
        )
    };

    // Handle double shift clipboard trigger (if enabled)
    let is_shift_key = vk == VK_SHIFT.0 as u32 || vk == VK_LSHIFT.0 as u32 || vk == VK_RSHIFT.0 as u32;
    if is_shift_key {
        if double_shift_clipboard {
            let now = std::time::Instant::now();
            let mut last_shift = LAST_SHIFT_TIME.lock().unwrap();
            if let Some(last_time) = *last_shift {
                let elapsed = now.duration_since(last_time);
                if elapsed.as_millis() < 300 {
                    *last_shift = None;
                    drop(last_shift);
                    // Open Quick Paste overlay
                    if let Some(window) = app_handle.get_webview_window("quick_paste") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    return true; // Block key event
                }
            }
            *last_shift = Some(now);
        }
    } else {
        // Reset double shift tracker on any other key press
        *LAST_SHIFT_TIME.lock().unwrap() = None;
    }

    // --- KEYBOARD GESTURES ENGINE ---
    if keyboard_gestures_enabled {
        let is_backspace_down = (unsafe { GetKeyState(VK_BACK.0 as i32) } as u16 & 0x8000) != 0;
        let is_delete_down = (unsafe { GetKeyState(0x2E) } as u16 & 0x8000) != 0; // 0x2E is VK_DELETE
        let is_space_down = (unsafe { GetKeyState(0x20) } as u16 & 0x8000) != 0;
        let is_alt_down = (unsafe { GetKeyState(0x12) } as u16 & 0x8000) != 0; // 0x12 is Alt

        // 1. Alt + Backspace + Up: Delete previous line
        if vk == 0x26 && is_backspace_down && is_alt_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            win_util::send_key_combo(0x24, false, false, false); // Home
            win_util::send_key_combo(0x08, false, false, false); // Backspace
            win_util::send_key_combo(0x24, false, true, false);  // Shift + Home
            win_util::send_key_combo(0x08, false, false, false); // Backspace
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 2. Alt + Delete + Down: Delete next line
        if vk == 0x28 && is_delete_down && is_alt_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            win_util::send_key_combo(0x23, false, false, false); // End
            win_util::send_key_combo(0x2E, false, false, false); // Delete
            win_util::send_key_combo(0x23, false, true, false);  // Shift + End
            win_util::send_key_combo(0x2E, false, false, false); // Delete
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 3. Backspace + Left: Delete previous word
        if vk == 0x25 && is_backspace_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            win_util::send_key_combo(0x25, true, true, false); // Ctrl + Shift + Left
            win_util::send_key_combo(0x08, false, false, false); // Backspace
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 4. Delete + Right: Delete next word
        if vk == 0x27 && is_delete_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            win_util::send_key_combo(0x27, true, true, false); // Ctrl + Shift + Right
            win_util::send_key_combo(0x2E, false, false, false); // Delete
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 5. Space + Left/Right: Move cursor character-by-character
        if (vk == 0x25 || vk == 0x27) && is_space_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            if !SPACE_ERASED.swap(true, Ordering::Relaxed) {
                win_util::send_key_combo(0x08, false, false, false); // Backspace space character
            }
            win_util::send_key_combo(vk, false, false, false); // Move cursor
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 6. Space + Up/Down: Move cursor word-by-word
        if (vk == 0x26 || vk == 0x28) && is_space_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            if !SPACE_ERASED.swap(true, Ordering::Relaxed) {
                win_util::send_key_combo(0x08, false, false, false); // Backspace space character
            }
            let target_vk = if vk == 0x26 { 0x25 } else { 0x27 }; // Up -> Left, Down -> Right
            win_util::send_key_combo(target_vk, true, false, false); // Ctrl + Left/Right
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 7. Space + W: Select previous word
        if vk == b'W' as u32 && is_space_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            if !SPACE_ERASED.swap(true, Ordering::Relaxed) {
                win_util::send_key_combo(0x08, false, false, false); // Backspace space character
            }
            win_util::send_key_combo(0x25, true, true, false); // Ctrl + Shift + Left
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 8. Space + L: Select current line
        if vk == b'L' as u32 && is_space_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            if !SPACE_ERASED.swap(true, Ordering::Relaxed) {
                win_util::send_key_combo(0x08, false, false, false); // Backspace space character
            }
            win_util::send_key_combo(0x24, false, false, false); // Home
            win_util::send_key_combo(0x23, false, true, false);  // Shift + End
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }

        // 9. Space + P: Select current paragraph
        if vk == b'P' as u32 && is_space_down {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            if !SPACE_ERASED.swap(true, Ordering::Relaxed) {
                win_util::send_key_combo(0x08, false, false, false); // Backspace space character
            }
            win_util::send_key_combo(0x26, true, true, false); // Ctrl + Shift + Up
            win_util::send_key_combo(0x28, true, true, false); // Ctrl + Shift + Down
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }
    }

    // Intercept Ctrl + Shift + V for Quick Paste
    if is_ctrl && is_shift && vk == VK_V.0 as u32 {
        if let Some(window) = app_handle.get_webview_window("quick_paste") {
            let _ = window.show();
            let _ = window.set_focus();
        }
        return true;
    }

    // Intercept Ctrl + Z for autocorrect undo
    if is_ctrl && vk == VK_Z.0 as u32 {
        let mut last_corr = LAST_CORRECTION.lock().unwrap();
        if let Some(undo) = last_corr.take() {
            SIMULATING_INPUT.store(true, Ordering::Relaxed);
            win_util::send_backspaces(undo.correction_len);
            win_util::send_string(&undo.original);
            win_util::send_string(&undo.boundary);
            let mut buf = TYPED_BUFFER.lock().unwrap();
            *buf = undo.original.clone();
            SIMULATING_INPUT.store(false, Ordering::Relaxed);
            return true;
        }
    }

    // Intercept Tab / Right Arrow for autocomplete selection
    if SUGGESTION_WINDOW_VISIBLE.load(Ordering::Relaxed) {
        if vk == VK_TAB.0 as u32 && pks_enabled {
            select_suggestion(app_handle, 0);
            return true;
        }
        if vk == VK_RIGHT.0 as u32 && pks_enabled && right_arrow_acceptance {
            select_suggestion(app_handle, 0);
            return true;
        }
    }

    // Intercept Escape to hide autocomplete popup
    if vk == VK_ESCAPE.0 as u32 && SUGGESTION_WINDOW_VISIBLE.load(Ordering::Relaxed) {
        hide_autocomplete_popup(app_handle);
        return false;
    }

    // Translate virtual key to unicode character
    let hwnd = unsafe { GetForegroundWindow() };
    let thread_id = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut 0)) };
    let layout = unsafe { GetKeyboardLayout(thread_id) };

    let mut keystate = [0u8; 256];
    let _ = unsafe { GetKeyboardState(&mut keystate) };
    if is_shift {
        keystate[VK_SHIFT.0 as usize] = 0x80;
    }
    let is_caps = (unsafe { GetKeyState(VK_CAPITAL.0 as i32) } as u16 & 0x0001) != 0;
    if is_caps {
        keystate[VK_CAPITAL.0 as usize] = 0x01;
    }

    let mut buf = [0u16; 16];
    let len = unsafe {
        ToUnicodeEx(
            vk,
            scan_code,
            &keystate,
            &mut buf,
            0,
            layout,
        )
    };

    if len > 0 {
        let text = String::from_utf16_lossy(&buf[..len as usize]);
        let ch = text.chars().next().unwrap();

        // 1. Intercept Suggestion Shortcuts (if visible)
        if SUGGESTION_WINDOW_VISIBLE.load(Ordering::Relaxed) {
            let ch_lower = ch.to_lowercase().to_string();
            let suggestions = SUGGESTIONS.lock().unwrap();
            let found_idx = suggestions.iter().position(|s| s.shortcut.to_lowercase() == ch_lower);
            if let Some(idx) = found_idx {
                drop(suggestions);
                select_suggestion(app_handle, idx);
                return true;
            }
            drop(suggestions);
        }

        if ch.is_alphanumeric() || ch == '\'' || ch == '-' {
            TYPED_BUFFER.lock().unwrap().push(ch);
            check_autocomplete(app_handle);
        } else if is_boundary_char(ch) {
            let word = TYPED_BUFFER.lock().unwrap().clone();
            TYPED_BUFFER.lock().unwrap().clear();
            hide_autocomplete_popup(app_handle);

            if !word.is_empty() {
                if let Some(replacement) = check_autocorrect(app_handle, &word) {
                    SIMULATING_INPUT.store(true, Ordering::Relaxed);
                    win_util::send_backspaces(word.len());
                    win_util::send_string(&replacement);
                    
                    *LAST_CORRECTION.lock().unwrap() = Some(UndoInfo {
                        correction_len: replacement.len() + text.len(),
                        original: word.clone(),
                        boundary: text.clone(),
                    });

                    SIMULATING_INPUT.store(false, Ordering::Relaxed);
                    
                    // Learning Engine log
                    record_typed_word(app_handle, &replacement);
                    return false;
                } else {
                    // Learning Engine log
                    record_typed_word(app_handle, &word);
                }
            }
            *LAST_CORRECTION.lock().unwrap() = None;
        } else {
            TYPED_BUFFER.lock().unwrap().clear();
            hide_autocomplete_popup(app_handle);
        }
    } else {
        if vk == VK_BACK.0 as u32 {
            let mut buf = TYPED_BUFFER.lock().unwrap();
            if !buf.is_empty() {
                buf.pop();
                drop(buf);
                check_autocomplete(app_handle);
            } else {
                hide_autocomplete_popup(app_handle);
            }
        } else if vk != VK_SHIFT.0 as u32 && vk != VK_LSHIFT.0 as u32 && vk != VK_RSHIFT.0 as u32 {
            TYPED_BUFFER.lock().unwrap().clear();
            hide_autocomplete_popup(app_handle);
        }
    }

    false
}

fn is_boundary_char(ch: char) -> bool {
    ch.is_whitespace() || ch == '.' || ch == ',' || ch == '!' || ch == '?' || ch == ';' || ch == ':' || ch == '"' || ch == '(' || ch == ')'
}

// Queries the database to check if the typed word should be autocorrected
fn check_autocorrect(app_handle: &AppHandle, word: &str) -> Option<String> {
    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    // 1. Check if word is in personal dictionary (if yes, NEVER autocorrect)
    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM dictionary_entries WHERE LOWER(word) = LOWER(?1)")
        .unwrap();
    let count: i64 = stmt.query_row(params![word], |row| row.get(0)).unwrap_or(0);
    if count > 0 {
        return None;
    }

    // 2. Check autocorrect rule
    let mut stmt = conn
        .prepare("SELECT replacement_word FROM autocorrect_rules WHERE LOWER(trigger_word) = LOWER(?1)")
        .unwrap();
    let replacement: Option<String> = stmt
        .query_row(params![word], |row| row.get(0))
        .ok();

    replacement
}

// Queries the database for autocomplete suggestions
fn check_autocomplete(app_handle: &AppHandle) {
    let prefix = TYPED_BUFFER.lock().unwrap().clone();
    if prefix.len() < 2 {
        hide_autocomplete_popup(app_handle);
        return;
    }

    let config = TYPING_CONFIG.lock().unwrap();
    if !config.autocomplete_enabled {
        drop(config);
        hide_autocomplete_popup(app_handle);
        return;
    }
    let letter_selection = config.letter_selection;
    drop(config);

    let db = app_handle.state::<DbState>();
    let conn = db.lock().unwrap();

    let query_str = format!("{}%", prefix);
    
    // Adaptive Ranking Query
    let mut stmt = conn
        .prepare(
            "SELECT w.word,
                    COALESCE((SELECT frequency FROM autocomplete_ranking WHERE prefix = ?1 AND word = w.word), 0) as pref_freq,
                    COALESCE((SELECT frequency FROM autocomplete_cache WHERE word = w.word), 0) as gen_freq
             FROM (
                 SELECT word FROM dictionary_entries WHERE word LIKE ?2
                 UNION
                 SELECT word FROM autocomplete_cache WHERE word LIKE ?2
             ) w
             ORDER BY pref_freq DESC, gen_freq DESC, w.word ASC
             LIMIT 9",
        )
        .unwrap();

    let rows = match stmt.query_map(params![prefix, query_str], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    }) {
        Ok(r) => r,
        Err(_) => {
            hide_autocomplete_popup(app_handle);
            return;
        }
    };

    let mut raw_matches = Vec::new();
    let mut total_pref_freq = 0;
    let mut total_gen_freq = 0;

    for r in rows {
        if let Ok((word, pref_freq, gen_freq)) = r {
            total_pref_freq += pref_freq;
            total_gen_freq += gen_freq;
            raw_matches.push((word, pref_freq, gen_freq));
        }
    }

    if !raw_matches.is_empty() {
        let words: Vec<String> = raw_matches.iter().map(|(w, _, _)| w.clone()).collect();
        
        let shortcuts = if letter_selection {
            if let Some(letters) = assign_activation_letters(&prefix, &words) {
                letters.iter().map(|c| c.to_string()).collect::<Vec<String>>()
            } else {
                (1..=raw_matches.len()).map(|i| i.to_string()).collect::<Vec<String>>()
            }
        } else {
            (1..=raw_matches.len()).map(|i| i.to_string()).collect::<Vec<String>>()
        };

        let mut suggestions = Vec::new();
        for (idx, (word, pref_freq, gen_freq)) in raw_matches.iter().enumerate() {
            let confidence = if total_pref_freq > 0 {
                ((*pref_freq as f64 / total_pref_freq as f64) * 100.0) as u32
            } else if total_gen_freq > 0 {
                ((*gen_freq as f64 / total_gen_freq as f64) * 100.0) as u32
            } else {
                (100 / raw_matches.len()) as u32
            };

            suggestions.push(Suggestion {
                word: word.clone(),
                shortcut: shortcuts.get(idx).cloned().unwrap_or_else(|| (idx + 1).to_string()),
                confidence,
                rank: idx + 1,
            });
        }

        *SUGGESTIONS.lock().unwrap() = suggestions.clone();
        SUGGESTION_WINDOW_VISIBLE.store(true, Ordering::Relaxed);

        let (x, y) = win_util::get_caret_position();

        if let Some(window) = app_handle.get_webview_window("autocomplete") {
            let _ = window.set_position(tauri::PhysicalPosition::new(x, y + 25));
            let _ = window.show();
            let _ = window.emit("suggestions:update", suggestions);
        }
    } else {
        hide_autocomplete_popup(app_handle);
    }
}

fn select_suggestion(app_handle: &AppHandle, idx: usize) {
    let suggestions_lock = SUGGESTIONS.lock().unwrap();
    if idx >= suggestions_lock.len() {
        return;
    }
    let suggestion = suggestions_lock[idx].clone();
    drop(suggestions_lock);

    let prefix = TYPED_BUFFER.lock().unwrap().clone();

    SIMULATING_INPUT.store(true, Ordering::Relaxed);

    let suffix = if suggestion.word.to_lowercase().starts_with(&prefix.to_lowercase()) {
        &suggestion.word[prefix.len()..]
    } else {
        &suggestion.word
    };

    win_util::send_string(suffix);
    win_util::send_string(" ");

    SIMULATING_INPUT.store(false, Ordering::Relaxed);

    record_suggestion_selection(app_handle, &prefix, &suggestion.word);

    hide_autocomplete_popup(app_handle);
    TYPED_BUFFER.lock().unwrap().clear();
}

fn hide_autocomplete_popup(app_handle: &AppHandle) {
    SUGGESTION_WINDOW_VISIBLE.store(false, Ordering::Relaxed);
    SUGGESTIONS.lock().unwrap().clear();
    if let Some(window) = app_handle.get_webview_window("autocomplete") {
        let _ = window.hide();
    }
}

// Global API mocks for non-windows platforms
#[cfg(not(target_os = "windows"))]
fn start_keyboard_hook() {}

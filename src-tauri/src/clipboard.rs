use crate::database::DbState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub id: String,
    pub content: String,
    pub content_type: String,
    pub source_app: Option<String>,
    pub source_window: Option<String>,
    pub language_code: Option<String>,
    pub is_favorite: bool,
    pub is_pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardListResult {
    pub items: Vec<ClipboardEntry>,
    pub total: i64,
}

// Global callback for the native clipboard listener
static CLIPBOARD_CALLBACK: Mutex<Option<Box<dyn Fn() + Send + 'static>>> = Mutex::new(None);

pub fn set_clipboard_callback<F>(f: F)
where
    F: Fn() + Send + 'static,
{
    *CLIPBOARD_CALLBACK.lock().unwrap() = Some(Box::new(f));
}

// Win32 imports for Windows platform
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, HANDLE, HGLOBAL};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, WM_CLIPBOARDUPDATE};

// Windows native clipboard listener
#[cfg(target_os = "windows")]
pub fn start_clipboard_listener() {
    use std::ptr;
    use windows::core::PCWSTR;
    use windows::Win32::System::DataExchange::AddClipboardFormatListener;
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, GetMessageW, RegisterClassW, WNDCLASSW, HWND_MESSAGE, MSG,
    };

    std::thread::spawn(|| unsafe {
        let class_name = "BoarddyClipboardListenerClass\0"
            .encode_utf16()
            .collect::<Vec<u16>>();
        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(clipboard_wnd_proc),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            ..Default::default()
        };

        RegisterClassW(&wnd_class);

        let hwnd = CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(ptr::null()),
            windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE(0),
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            None,
            None,
            None,
        );

        if hwnd.0 == 0 {
            return;
        }

        let _ = AddClipboardFormatListener(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {
            let _ = windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
            let _ = windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(&msg);
        }
    });
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn clipboard_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_CLIPBOARDUPDATE {
        if let Some(ref cb) = *CLIPBOARD_CALLBACK.lock().unwrap() {
            cb();
        }
        LRESULT(0)
    } else {
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

// Non-Windows clipboard listener fallback (polling loop)
#[cfg(not(target_os = "windows"))]
pub fn start_clipboard_listener() {
    std::thread::spawn(|| {
        let mut last_text = String::new();
        loop {
            if let Some(text) = get_native_clipboard_text() {
                if !text.is_empty() && text != last_text {
                    last_text = text.clone();
                    if let Some(ref cb) = *CLIPBOARD_CALLBACK.lock().unwrap() {
                        cb();
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });
}

// Get/Set Clipboard helpers using Win32 API directly
#[cfg(target_os = "windows")]
const CF_UNICODETEXT: u32 = 13;

#[cfg(target_os = "windows")]
pub fn get_native_clipboard_text() -> Option<String> {
    use windows::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
    use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};

    unsafe {
        if OpenClipboard(HWND(0)).is_err() {
            return None;
        }
        let handle = match GetClipboardData(CF_UNICODETEXT) {
            Ok(h) => h,
            Err(_) => {
                let _ = CloseClipboard();
                return None;
            }
        };
        if handle.0 == 0 {
            let _ = CloseClipboard();
            return None;
        }
        
        let hglobal = HGLOBAL(handle.0 as *mut std::ffi::c_void);
        let ptr = GlobalLock(hglobal);
        if ptr.is_null() {
            let _ = CloseClipboard();
            return None;
        }
        let mut len = 0;
        let u16_ptr = ptr as *const u16;
        while *u16_ptr.offset(len) != 0 {
            len += 1;
        }
        let slice = std::slice::from_raw_parts(u16_ptr, len as usize);
        let text = String::from_utf16_lossy(slice);
        let _ = GlobalUnlock(hglobal);
        let _ = CloseClipboard();
        Some(text)
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_native_clipboard_text() -> Option<String> {
    // Return empty on non-supported OS for V1 skeleton
    None
}

#[cfg(target_os = "windows")]
pub fn set_native_clipboard_text(text: &str) -> bool {
    use windows::Win32::System::DataExchange::{CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData};
    use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GHND};

    unsafe {
        if OpenClipboard(HWND(0)).is_err() {
            return false;
        }
        let _ = EmptyClipboard();
        let utf16: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let size = utf16.len() * 2;
        let handle = match GlobalAlloc(GHND, size) {
            Ok(h) => h,
            Err(_) => {
                let _ = CloseClipboard();
                return false;
            }
        };
        if handle.0.is_null() {
            let _ = CloseClipboard();
            return false;
        }
        let ptr = GlobalLock(handle);
        if ptr.is_null() {
            let _ = CloseClipboard();
            return false;
        }
        std::ptr::copy_nonoverlapping(utf16.as_ptr(), ptr as *mut u16, utf16.len());
        let _ = GlobalUnlock(handle);
        
        let handle_to_set = HANDLE(handle.0 as isize);
        if SetClipboardData(CF_UNICODETEXT, handle_to_set).is_err() {
            let _ = CloseClipboard();
            return false;
        }
        let _ = CloseClipboard();
        true
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_native_clipboard_text(_text: &str) -> bool {
    false
}

// Classifier/Normalizer
pub fn detect_content_type(content: &str) -> &str {
    let content = content.trim();
    if content.is_empty() {
        return "text";
    }
    
    // Check Email
    if let Ok(email_re) = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
        if email_re.is_match(content) {
            return "email";
        }
    }
    
    // Check URL
    if let Ok(url_re) = regex::Regex::new(r"^(https?://|www\.)[^\s/$.?#].[^\s]*$") {
        if url_re.is_match(content) {
            return "url";
        }
    }
    
    // Check Phone
    if content.chars().filter(|c| c.is_ascii_digit()).count() >= 7
        && content.chars().all(|c| c.is_ascii_digit() || c == '+' || c == '-' || c == ' ' || c == '(' || c == ')')
    {
        return "phone";
    }
    
    // Check Code Heuristics
    let code_keywords = [
        "fn ", "def ", "function ", "const ", "let ", "var ", "import ", "export ", "class ", "public ", "private ", "void ",
        "struct ", "impl ", "#include", "using namespace", "package ", "import java", "<html>", "</div>", "select * from",
    ];
    let has_braces = content.contains('{') && content.contains('}');
    let has_keyword = code_keywords.iter().any(|&kw| content.to_lowercase().contains(kw));
    let has_semicolon_lines = content.lines().filter(|line| line.trim().ends_with(';')).count() >= 1;
    if (has_braces && (has_keyword || has_semicolon_lines))
        || (has_keyword && has_semicolon_lines)
        || (content.lines().count() > 3 && has_braces)
        || (content.to_lowercase().contains("def ") && content.to_lowercase().contains("return"))
    {
        return "code";
    }
    
    "text"
}

// Repository / Operations
pub fn handle_clipboard_change(app_handle: &AppHandle) {
    if let Some(text) = get_native_clipboard_text() {
        let text = text.trim().to_string();
        if text.is_empty() {
            return;
        }

        let db = app_handle.state::<DbState>();
        let conn = db.lock().unwrap();

        // 1. Check if same content exists
        let mut stmt = conn
            .prepare("SELECT id, is_favorite, is_pinned FROM clipboard_entries WHERE content = ?1")
            .unwrap();
        let existing = stmt
            .query_row(params![text], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i32>(1)? == 1,
                    row.get::<_, i32>(2)? == 1,
                ))
            })
            .ok();

        let id = if let Some((existing_id, _fav, _pinned)) = existing {
            // Update timestamp to now
            conn.execute(
                "UPDATE clipboard_entries SET created_at = datetime('now'), updated_at = datetime('now') WHERE id = ?1",
                params![existing_id],
            )
            .unwrap();
            existing_id
        } else {
            // Create new entry
            let new_id = format!("clip_{}", uuid_v4_fallback());
            let content_type = detect_content_type(&text).to_string();
            
            // Resolve active window details
            let (app_name, win_title) = if cfg!(target_os = "windows") {
                let details = crate::win_util::get_active_window_details();
                (Some(details.process_name), Some(details.title))
            } else {
                (None, None)
            };

            conn.execute(
                "INSERT INTO clipboard_entries (id, content, content_type, source_app, source_window, is_favorite, is_pinned, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, 0, 0, datetime('now'), datetime('now'))",
                params![new_id, text, content_type, app_name, win_title],
            )
            .unwrap();

            new_id
        };

        // Enforce clipboard size limit from settings
        let limit: i64 = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'clipboard_limit'",
                [],
                |row| {
                    let val: String = row.get(0)?;
                    Ok(val.parse::<i64>().unwrap_or(100))
                },
            )
            .unwrap_or(100);

        // Delete oldest entries exceeding limit (ignoring favorites/pinned items)
        conn.execute(
            "DELETE FROM clipboard_entries 
             WHERE is_pinned = 0 AND is_favorite = 0 AND id NOT IN (
                 SELECT id FROM clipboard_entries ORDER BY created_at DESC LIMIT ?1
             )",
            params![limit],
        )
        .unwrap();

        // Emit change event
        let payload = serde_json::json!({ "id": id });
        let _ = app_handle.emit("clipboard:changed", payload);
    }
}

// Simple fallback UUID generator without extra external crates
fn uuid_v4_fallback() -> String {
    let rand_num = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", rand_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_content_type() {
        // Test Email
        assert_eq!(detect_content_type("test@example.com"), "email");
        assert_eq!(detect_content_type("user.name+label@sub.domain.co.uk"), "email");

        // Test URL
        assert_eq!(detect_content_type("https://google.com"), "url");
        assert_eq!(detect_content_type("http://sub.domain.org/path?query=123"), "url");
        assert_eq!(detect_content_type("www.boarddy.app"), "url");

        // Test Phone
        assert_eq!(detect_content_type("+1 (555) 123-4567"), "phone");
        assert_eq!(detect_content_type("08031234567"), "phone");

        // Test Code Heuristics
        assert_eq!(detect_content_type("fn main() {\n    println!(\"Hello!\");\n}"), "code");
        assert_eq!(detect_content_type("def hello():\n    return 'world'"), "code");
        assert_eq!(detect_content_type("const x = 5;\nlet y = 10;"), "code");

        // Test Plain Text
        assert_eq!(detect_content_type("Hello world, this is a plain text clipboard entry."), "text");
    }
}


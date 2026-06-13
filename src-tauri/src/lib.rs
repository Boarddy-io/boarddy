mod database;
mod win_util;
mod clipboard;
mod typing;
mod dictionary;
mod notes;
mod search;
mod settings;
mod commands;

use std::sync::Arc;
use tauri::{AppHandle, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 1. Initialize SQLite Database
            let db = database::init_db(app.app_handle())
                .expect("Failed to initialize SQLite database");
            app.manage(db);

            // 2. Start Keyboard hook typing service
            typing::init_typing_service(app.app_handle().clone());

            // 3. Start Clipboard listener service
            let handle = app.app_handle().clone();
            clipboard::set_clipboard_callback(move || {
                clipboard::handle_clipboard_change(&handle);
            });
            clipboard::start_clipboard_listener();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_entries,
            commands::get_clipboard_entry,
            commands::search_clipboard,
            commands::delete_clipboard_entry,
            commands::clear_clipboard_history,
            commands::pin_clipboard_entry,
            commands::unpin_clipboard_entry,
            commands::favorite_clipboard_entry,
            commands::unfavorite_clipboard_entry,
            commands::create_note,
            commands::create_note_from_clipboard,
            commands::update_note,
            commands::delete_note,
            commands::list_notes,
            commands::search_notes,
            commands::add_dictionary_word,
            commands::remove_dictionary_word,
            commands::list_dictionary_words,
            commands::import_dictionary,
            commands::export_dictionary,
            commands::get_correction_suggestions,
            commands::add_correction_rule,
            commands::remove_correction_rule,
            commands::get_autocorrect_rules,
            commands::get_autocomplete_suggestions,
            commands::get_languages,
            commands::enable_language,
            commands::disable_language,
            commands::global_search,
            commands::get_settings,
            commands::update_setting,
            commands::get_shortcuts,
            commands::update_shortcut,
            commands::show_main_window,
            commands::hide_main_window,
            commands::show_quick_paste,
            commands::hide_quick_paste,
            commands::paste_text,
            commands::check_shortcut_conflict,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

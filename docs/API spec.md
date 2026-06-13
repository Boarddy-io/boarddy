# API Specification.md

Version: 1.0

Product: Boarddy

Architecture: Tauri + Rust + React

API Style: Command-Based + Event-Driven

Status: V1

---

# API Overview

Boarddy uses three API layers.

## Layer 1

Frontend → Backend

Technology:

Tauri Commands

Purpose:

Allow React UI to communicate with Rust services.

---

## Layer 2

Service → Service

Technology:

Internal Rust Interfaces

Purpose:

Communication between modules.

---

## Layer 3

Event Bus

Technology:

Tauri Events

Purpose:

Real-time updates.

---

# Standard Response Format

Success

```json
{
  "success": true,
  "data": {}
}
```

---

Failure

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error"
  }
}
```

---

# Clipboard API

# Get Clipboard Entries

Command

```rust
get_clipboard_entries
```

Parameters

```json
{
  "page": 1,
  "limit": 50
}
```

Response

```json
{
  "success": true,
  "data": {
    "items": [],
    "total": 500
  }
}
```

---

# Get Clipboard Entry

Command

```rust
get_clipboard_entry
```

Parameters

```json
{
  "id": "clipboard_id"
}
```

---

# Search Clipboard

Command

```rust
search_clipboard
```

Parameters

```json
{
  "query": "invoice"
}
```

---

# Delete Clipboard Entry

Command

```rust
delete_clipboard_entry
```

Parameters

```json
{
  "id": "clipboard_id"
}
```

---

# Clear Clipboard History

Command

```rust
clear_clipboard_history
```

Parameters

```json
{}
```

---

# Pin Clipboard Entry

Command

```rust
pin_clipboard_entry
```

Parameters

```json
{
  "id": "clipboard_id"
}
```

---

# Unpin Clipboard Entry

Command

```rust
unpin_clipboard_entry
```

Parameters

```json
{
  "id": "clipboard_id"
}
```

---

# Favorite Clipboard Entry

Command

```rust
favorite_clipboard_entry
```

Parameters

```json
{
  "id": "clipboard_id"
}
```

---

# Notes API

# Create Note

Command

```rust
create_note
```

Parameters

```json
{
  "title": "Research",
  "content": "Boarddy ideas"
}
```

---

# Create Note From Clipboard

Command

```rust
create_note_from_clipboard
```

Parameters

```json
{
  "clipboard_id": "123"
}
```

---

# Update Note

Command

```rust
update_note
```

Parameters

```json
{
  "id": "note_id",
  "title": "Updated",
  "content": "New Content"
}
```

---

# Delete Note

Command

```rust
delete_note
```

Parameters

```json
{
  "id": "note_id"
}
```

---

# Search Notes

Command

```rust
search_notes
```

Parameters

```json
{
  "query": "research"
}
```

---

# Dictionary API

# Add Dictionary Entry

Command

```rust
add_dictionary_word
```

Parameters

```json
{
  "word": "HunaPay",
  "language": "en"
}
```

---

# Remove Dictionary Entry

Command

```rust
remove_dictionary_word
```

Parameters

```json
{
  "id": "word_id"
}
```

---

# List Dictionary Entries

Command

```rust
list_dictionary_words
```

Parameters

```json
{
  "language": "en"
}
```

---

# Import Dictionary

Command

```rust
import_dictionary
```

Parameters

```json
{
  "file_path": "/path/file.json"
}
```

---

# Export Dictionary

Command

```rust
export_dictionary
```

Parameters

```json
{
  "format": "json"
}
```

---

# Autocorrect API

# Get Suggestions

Command

```rust
get_correction_suggestions
```

Parameters

```json
{
  "word": "reciept",
  "language": "en"
}
```

Response

```json
{
  "success": true,
  "data": [
    "receipt"
  ]
}
```

---

# Add Correction Rule

Command

```rust
add_correction_rule
```

Parameters

```json
{
  "trigger": "teh",
  "replacement": "the"
}
```

---

# Remove Correction Rule

Command

```rust
remove_correction_rule
```

Parameters

```json
{
  "rule_id": "rule_id"
}
```

---

# Autocomplete API

# Get Suggestions

Command

```rust
get_autocomplete_suggestions
```

Parameters

```json
{
  "prefix": "than"
}
```

Response

```json
{
  "success": true,
  "data": [
    "thank",
    "thanks"
  ]
}
```

---

# Language API

# List Languages

Command

```rust
get_languages
```

Response

```json
{
  "success": true,
  "data": [
    {
      "code": "en",
      "name": "English"
    }
  ]
}
```

---

# Enable Language

Command

```rust
enable_language
```

Parameters

```json
{
  "code": "yo"
}
```

---

# Disable Language

Command

```rust
disable_language
```

Parameters

```json
{
  "code": "yo"
}
```

---

# Search API

# Global Search

Command

```rust
global_search
```

Parameters

```json
{
  "query": "invoice"
}
```

Response

```json
{
  "success": true,
  "data": {
    "clipboard": [],
    "notes": []
  }
}
```

---

# Settings API

# Get Settings

Command

```rust
get_settings
```

---

# Update Setting

Command

```rust
update_setting
```

Parameters

```json
{
  "key": "theme",
  "value": "dark"
}
```

---

# Shortcut API

# Get Shortcuts

Command

```rust
get_shortcuts
```

---

# Update Shortcut

Command

```rust
update_shortcut
```

Parameters

```json
{
  "action": "quick_paste",
  "shortcut": "Ctrl+Shift+V"
}
```

---

# System Tray API

# Show Main Window

Command

```rust
show_main_window
```

---

# Hide Main Window

Command

```rust
hide_main_window
```

---

# Show Quick Paste

Command

```rust
show_quick_paste
```

---

# Event Bus Specification

Boarddy uses event-driven communication.

---

# Clipboard Changed

Event

```rust
clipboard:changed
```

Payload

```json
{
  "id": "clipboard_id"
}
```

Subscribers

* Clipboard UI
* Search Indexer
* Metadata Service

---

# Note Created

Event

```rust
note:created
```

Payload

```json
{
  "id": "note_id"
}
```

---

# Dictionary Updated

Event

```rust
dictionary:updated
```

Payload

```json
{
  "word": "HunaPay"
}
```

---

# Language Changed

Event

```rust
language:changed
```

Payload

```json
{
  "language": "yo"
}
```

---

# Settings Changed

Event

```rust
settings:changed
```

Payload

```json
{
  "key": "theme"
}
```

---

# Internal Service Interfaces

# Clipboard Service

```rust
trait ClipboardService {
    fn save();
    fn get();
    fn search();
    fn delete();
    fn clear();
}
```

---

# Notes Service

```rust
trait NotesService {
    fn create();
    fn update();
    fn delete();
    fn search();
}
```

---

# Dictionary Service

```rust
trait DictionaryService {
    fn add_word();
    fn remove_word();
    fn import();
    fn export();
}
```

---

# Search Service

```rust
trait SearchService {
    fn global_search();
}
```

---

# Future V2 APIs

Screenshot APIs

```rust
create_screenshot
search_screenshots
delete_screenshot
```

---

OCR APIs

```rust
extract_text
index_ocr
```

---

Voice APIs

```rust
start_dictation
stop_dictation
```

---

# Future V3 APIs

Snippet APIs

```rust
create_snippet
update_snippet
delete_snippet
expand_snippet
```

---

Mobile Companion APIs

```rust
pair_device
unpair_device
sync_clipboard
```

---

AI APIs

```rust
ai_search
ai_recall
ai_completion
```

---

# API Design Principles

Every API must be:

* Fast
* Offline First
* Versioned
* Testable
* Privacy Respecting

No API should require internet access for core functionality.

All V1 functionality must operate entirely on the user's device.

Boarddy is a local-first product, and the API architecture must reflect that principle.

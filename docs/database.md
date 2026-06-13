# Database.md

Version: 1.0

Product: Boarddy

Database Engine: SQLite

Search Engine: SQLite FTS5

Architecture: Local First

---

# Database Overview

Boarddy stores:

* Clipboard History
* Notes
* Dictionary Entries
* Languages
* Metadata
* Settings
* Search Indexes

Future Support:

* Screenshots
* OCR
* Voice Notes
* Snippets
* AI Memory
* Device Sync

---

# Entity Relationship Overview

```text
Users
 │
 ├── Settings
 │
 ├── Clipboard Entries
 │      │
 │      ├── Metadata
 │      ├── Tags
 │      └── Notes
 │
 ├── Dictionary Entries
 │
 ├── Languages
 │
 ├── Snippets (V3)
 │
 ├── Screenshots (V2)
 │
 └── AI Memories (V3)
```

---

# users

Reserved for future sync support.

Current Usage:

Single local user.

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    name TEXT,
    email TEXT,
    created_at DATETIME,
    updated_at DATETIME
);
```

---

# settings

Stores application preferences.

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME
);
```

Examples:

```text
theme
language
clipboard_limit
launch_on_startup
quick_paste_shortcut
```

---

# clipboard_entries

Core table.

Stores copied content.

```sql
CREATE TABLE clipboard_entries (
    id TEXT PRIMARY KEY,

    content TEXT NOT NULL,

    content_type TEXT NOT NULL,

    source_app TEXT,

    source_window TEXT,

    language_code TEXT,

    is_favorite BOOLEAN DEFAULT 0,

    is_pinned BOOLEAN DEFAULT 0,

    created_at DATETIME,

    updated_at DATETIME
);
```

---

# Supported Content Types

```text
text
url
email
phone
code
image
screenshot
file
```

---

# clipboard_metadata

Stores searchable metadata.

```sql
CREATE TABLE clipboard_metadata (
    id TEXT PRIMARY KEY,

    clipboard_id TEXT NOT NULL,

    key TEXT NOT NULL,

    value TEXT NOT NULL,

    FOREIGN KEY (clipboard_id)
    REFERENCES clipboard_entries(id)
);
```

Examples:

```text
browser=chrome

website=hunapay.com

language=en

window=dashboard
```

---

# clipboard_tags

Tag system.

```sql
CREATE TABLE clipboard_tags (
    id TEXT PRIMARY KEY,

    clipboard_id TEXT NOT NULL,

    tag TEXT NOT NULL,

    created_at DATETIME,

    FOREIGN KEY (clipboard_id)
    REFERENCES clipboard_entries(id)
);
```

Examples:

```text
invoice

client

research

important
```

---

# notes

Permanent saved notes.

```sql
CREATE TABLE notes (
    id TEXT PRIMARY KEY,

    title TEXT,

    content TEXT NOT NULL,

    source_clipboard_id TEXT,

    created_at DATETIME,

    updated_at DATETIME,

    FOREIGN KEY (source_clipboard_id)
    REFERENCES clipboard_entries(id)
);
```

---

# note_tags

Tags for notes.

```sql
CREATE TABLE note_tags (
    id TEXT PRIMARY KEY,

    note_id TEXT NOT NULL,

    tag TEXT NOT NULL,

    FOREIGN KEY (note_id)
    REFERENCES notes(id)
);
```

---

# dictionary_entries

Personal dictionary.

```sql
CREATE TABLE dictionary_entries (
    id TEXT PRIMARY KEY,

    word TEXT NOT NULL,

    language_code TEXT NOT NULL,

    source TEXT,

    created_at DATETIME
);
```

---

# Sources

```text
user

import

system

organization
```

---

# languages

Installed language tracking.

```sql
CREATE TABLE languages (
    id TEXT PRIMARY KEY,

    language_code TEXT UNIQUE,

    language_name TEXT,

    is_enabled BOOLEAN DEFAULT 1,

    created_at DATETIME
);
```

Examples:

```text
en
yo
ha
ig
fr
ar
```

---

# autocorrect_rules

Custom correction rules.

```sql
CREATE TABLE autocorrect_rules (
    id TEXT PRIMARY KEY,

    trigger_word TEXT NOT NULL,

    replacement_word TEXT NOT NULL,

    language_code TEXT,

    created_at DATETIME
);
```

Examples:

```text
teh → the

reciept → receipt
```

---

# autocomplete_cache

Frequently suggested words.

```sql
CREATE TABLE autocomplete_cache (
    id TEXT PRIMARY KEY,

    word TEXT NOT NULL,

    language_code TEXT,

    frequency INTEGER DEFAULT 0,

    last_used_at DATETIME
);
```

---

# app_usage

Used for personalization.

```sql
CREATE TABLE app_usage (
    id TEXT PRIMARY KEY,

    app_name TEXT,

    usage_count INTEGER,

    last_used_at DATETIME
);
```

---

# search_history

Stores user searches.

```sql
CREATE TABLE search_history (
    id TEXT PRIMARY KEY,

    query TEXT,

    created_at DATETIME
);
```

---

# Full Text Search Tables

# clipboard_fts

```sql
CREATE VIRTUAL TABLE clipboard_fts
USING fts5(
    content,
    content='clipboard_entries'
);
```

---

# notes_fts

```sql
CREATE VIRTUAL TABLE notes_fts
USING fts5(
    title,
    content,
    content='notes'
);
```

---

# V2 Future Tables

# screenshots

```sql
CREATE TABLE screenshots (
    id TEXT PRIMARY KEY,

    file_path TEXT,

    thumbnail_path TEXT,

    source_app TEXT,

    source_window TEXT,

    created_at DATETIME
);
```

---

# screenshot_ocr

```sql
CREATE TABLE screenshot_ocr (
    id TEXT PRIMARY KEY,

    screenshot_id TEXT,

    extracted_text TEXT,

    created_at DATETIME,

    FOREIGN KEY (screenshot_id)
    REFERENCES screenshots(id)
);
```

---

# voice_transcripts

```sql
CREATE TABLE voice_transcripts (
    id TEXT PRIMARY KEY,

    transcript TEXT,

    language_code TEXT,

    created_at DATETIME
);
```

---

# V3 Future Tables

# snippets

```sql
CREATE TABLE snippets (
    id TEXT PRIMARY KEY,

    trigger TEXT UNIQUE,

    content TEXT NOT NULL,

    created_at DATETIME,

    updated_at DATETIME
);
```

Example:

```text
;email

support@company.com
```

---

# signatures

```sql
CREATE TABLE signatures (
    id TEXT PRIMARY KEY,

    name TEXT,

    image_path TEXT,

    created_at DATETIME
);
```

---

# devices

Future sync support.

```sql
CREATE TABLE devices (
    id TEXT PRIMARY KEY,

    device_name TEXT,

    device_type TEXT,

    created_at DATETIME
);
```

---

# ai_memories

Future AI memory layer.

```sql
CREATE TABLE ai_memories (
    id TEXT PRIMARY KEY,

    source_type TEXT,

    source_id TEXT,

    embedding_id TEXT,

    summary TEXT,

    created_at DATETIME
);
```

---

# Recommended Indexes

```sql
CREATE INDEX idx_clipboard_created
ON clipboard_entries(created_at);

CREATE INDEX idx_clipboard_type
ON clipboard_entries(content_type);

CREATE INDEX idx_dictionary_word
ON dictionary_entries(word);

CREATE INDEX idx_notes_created
ON notes(created_at);

CREATE INDEX idx_search_history
ON search_history(created_at);
```

---

# Data Retention Strategy

Default:

Unlimited History

User Configurable:

* 30 Days
* 90 Days
* 180 Days
* 1 Year
* Forever

---

# Export Formats

Supported:

JSON

CSV

Markdown

Future:

Encrypted Backup Files

---

# Database Principles

Every piece of information should be:

1. Captured
2. Searchable
3. Taggable
4. Recoverable
5. Exportable

Boarddy's database is not merely storage.

It is the foundation of a personal memory system capable of evolving from clipboard history into a complete digital recall platform.

# CLI Spec.md

Version: 1.0

Product: Boarddy

CLI Name: boarddy

Platform:

* Windows
* macOS
* Linux

Status: V1

---

# Overview

Boarddy CLI provides command-line access to Boarddy functionality.

The CLI enables:

* Clipboard management
* Notes management
* Dictionary management
* Search
* Automation
* AI agent integration
* Future scripting workflows

The CLI interacts with the local Boarddy instance and database.

---

# Design Principles

The CLI must be:

* Fast
* Human-friendly
* Scriptable
* Automation-ready
* Consistent

Every command should return predictable output.

---

# Installation

Windows

```bash
boarddy.exe
```

macOS

```bash
boarddy
```

Linux

```bash
boarddy
```

---

# Global Commands

## Version

```bash
boarddy version
```

Output

```bash
Boarddy v1.0.0
```

---

## Health

```bash
boarddy health
```

Output

```json
{
  "status": "healthy",
  "database": "connected",
  "clipboard": "active"
}
```

---

## Status

```bash
boarddy status
```

Output

```bash
Clipboard Entries: 142
Notes: 38
Dictionary Words: 420
Languages: 3
```

---

# Clipboard Commands

# Copy

Add item directly to clipboard history.

```bash
boarddy clipboard add "Hello World"
```

---

# List Clipboard

```bash
boarddy clipboard list
```

Output

```bash
1. Hello World
2. support@boarddy.io
3. Meeting Notes
```

---

# Search Clipboard

```bash
boarddy clipboard search invoice
```

Output

```bash
Invoice Number
Invoice Address
Invoice Email
```

---

# Get Clipboard Item

```bash
boarddy clipboard get 123
```

Output

```bash
Invoice #2026-001
```

---

# Delete Clipboard Item

```bash
boarddy clipboard delete 123
```

---

# Clear Clipboard

```bash
boarddy clipboard clear
```

---

# Favorite Clipboard Item

```bash
boarddy clipboard favorite 123
```

---

# Pin Clipboard Item

```bash
boarddy clipboard pin 123
```

---

# Export Clipboard

```bash
boarddy clipboard export
```

---

Formats

```bash
boarddy clipboard export --json
boarddy clipboard export --csv
boarddy clipboard export --md
```

---

# Notes Commands

# Create Note

```bash
boarddy notes create
```

Interactive

```bash
Title:
Content:
```

---

# Create Note Directly

```bash
boarddy notes create \
--title "Research" \
--content "Boarddy ideas"
```

---

# List Notes

```bash
boarddy notes list
```

---

# Search Notes

```bash
boarddy notes search research
```

---

# Get Note

```bash
boarddy notes get 15
```

---

# Delete Note

```bash
boarddy notes delete 15
```

---

# Export Notes

```bash
boarddy notes export
```

Formats:

```bash
--json
--csv
--md
```

---

# Dictionary Commands

# Add Word

```bash
boarddy dictionary add HunaPay
```

---

# Remove Word

```bash
boarddy dictionary remove HunaPay
```

---

# List Dictionary

```bash
boarddy dictionary list
```

---

# Import Dictionary

```bash
boarddy dictionary import words.json
```

---

# Export Dictionary

```bash
boarddy dictionary export words.json
```

---

# Search Commands

# Global Search

```bash
boarddy search "invoice"
```

Searches:

* Clipboard
* Notes

---

# Search Clipboard Only

```bash
boarddy search clipboard invoice
```

---

# Search Notes Only

```bash
boarddy search notes invoice
```

---

# Language Commands

# List Languages

```bash
boarddy languages list
```

Output

```bash
English
Yoruba
French
```

---

# Enable Language

```bash
boarddy languages enable yo
```

---

# Disable Language

```bash
boarddy languages disable yo
```

---

# Settings Commands

# Show Settings

```bash
boarddy settings list
```

---

# Get Setting

```bash
boarddy settings get theme
```

---

# Set Setting

```bash
boarddy settings set theme dark
```

---

# Quick Paste Commands

# Open Quick Paste

```bash
boarddy quickpaste
```

Launches overlay.

---

# Paste Recent

```bash
boarddy paste latest
```

---

# Paste Specific

```bash
boarddy paste 123
```

---

# Backup Commands

# Create Backup

```bash
boarddy backup create
```

Output

```bash
backup-2026-06-13.boarddy
```

---

# Restore Backup

```bash
boarddy backup restore backup.boarddy
```

---

# Export Commands

# Export Everything

```bash
boarddy export all
```

---

Formats

```bash
--json
--csv
--md
```

---

# Automation Commands

# Watch Clipboard

Real-time stream.

```bash
boarddy clipboard watch
```

Output

```bash
[12:01] New Copy:
hello world
```

---

# Pipe Into Boarddy

```bash
echo "Research Note" | boarddy clipboard add
```

---

# Save Pipeline Output

```bash
cat notes.txt | boarddy notes create
```

---

# AI Agent Commands (Future)

# Query Memory

```bash
boarddy memory ask \
"Where is the invoice I copied last month?"
```

---

# Recall

```bash
boarddy memory recall invoice
```

---

# Sync Commands (Future)

# Pair Device

```bash
boarddy sync pair
```

---

# Start Sync

```bash
boarddy sync start
```

---

# Stop Sync

```bash
boarddy sync stop
```

---

# Exit Codes

Success

```bash
0
```

General Error

```bash
1
```

Not Found

```bash
2
```

Permission Error

```bash
3
```

Database Error

```bash
4
```

Invalid Input

```bash
5
```

---

# JSON Output Mode

All commands support:

```bash
--json
```

Example

```bash
boarddy clipboard list --json
```

Output

```json
[
  {
    "id": 1,
    "content": "Hello World"
  }
]
```

---

# AI Agent Compatibility

The CLI must be fully usable by:

* Claude Code
* OpenAI Codex
* Cursor Agents
* Windsurf
* Roo Code
* OpenHands
* Future Autonomous Agents

Every command should be:

* Predictable
* Structured
* Machine-readable

---

# Long-Term Vision

The Boarddy CLI is not merely a developer tool.

It is the automation interface for Boarddy.

Humans use the desktop app.

Scripts use the CLI.

AI agents use the CLI.

Future integrations use the CLI.

The CLI should expose the entire Boarddy memory system through a clean, stable, and scriptable interface.

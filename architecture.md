# Boarddy Architecture.md

Version: 1.0

Product: Boarddy

Platform: Windows First

Architecture Style: Modular Monolith

Backend: Rust

Frontend: React + TypeScript

Desktop Framework: Tauri

Database: SQLite

Search: SQLite FTS5

Storage Model: Local First

---

# Architecture Principles

## Lightweight First

Boarddy must consume minimal memory and CPU.

Target:

RAM < 200 MB

CPU Idle < 1%

Cold Start < 2 Seconds

---

## Local First

All user data remains local.

Internet connection is not required.

Core features must function offline.

---

## Modular Design

Each subsystem must operate independently.

Benefits:

* Easier maintenance
* Easier testing
* Faster feature development
* Reduced technical debt

---

## Privacy By Design

No keystrokes leave the machine.

No clipboard uploads.

No hidden telemetry.

User controls all synchronization.

---

# High-Level Architecture

```text
┌─────────────────────┐
│      React UI       │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│   Tauri Commands    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│   Core Services     │
└──────────┬──────────┘
           │
 ┌─────────┼──────────┐
 │         │          │
 ▼         ▼          ▼

Clipboard  Typing   Notes
Service    Service  Service

 ▼         ▼          ▼

Dictionary Search Metadata
Service    Service  Service

           ▼

       SQLite
```

---

# System Components

## Frontend Layer

Technology:

* React
* TypeScript
* Tailwind CSS

Responsibilities:

* User Interface
* Settings
* Search Screens
* Clipboard History
* Notes
* Dictionary Management

No business logic should exist here.

---

## Backend Layer

Technology:

Rust

Responsibilities:

* Clipboard Monitoring
* Keyboard Monitoring
* Search Processing
* Storage Operations
* OS Integrations

All business logic belongs here.

---

# Core Services

# Clipboard Service

Purpose:

Monitor and manage clipboard content.

Responsibilities:

* Detect clipboard changes
* Save clipboard entries
* Maintain history
* Delete entries
* Pin entries
* Favorite entries

Future:

* Image support
* Screenshot support
* OCR

---

## Clipboard Pipeline

```text
User Copies Content
        │
        ▼
Clipboard Listener
        │
        ▼
Normalizer
        │
        ▼
Metadata Generator
        │
        ▼
SQLite Storage
        │
        ▼
Search Index
```

---

# Typing Service

Purpose:

Provide typing assistance.

Responsibilities:

* Autocorrect
* Autocomplete
* Language detection
* Suggestion generation

Future:

* AI suggestions
* Predictive typing

---

## Typing Pipeline

```text
Keyboard Input
       │
       ▼
Language Detector
       │
       ▼
Dictionary Lookup
       │
       ▼
Correction Engine
       │
       ▼
Suggestion Engine
       │
       ▼
UI Popup
```

---

# Dictionary Service

Purpose:

Manage language and custom vocabulary.

Responsibilities:

* Personal Dictionary
* Imported Dictionaries
* Ignore Lists
* Language Dictionaries

Future:

* Team Dictionaries

---

# Notes Service

Purpose:

Store permanent user notes.

Responsibilities:

* Create Notes
* Edit Notes
* Delete Notes
* Tag Notes
* Categorize Notes

---

# Search Service

Purpose:

Provide fast retrieval.

Responsibilities:

* Clipboard Search
* Note Search
* Dictionary Search

Technology:

SQLite Full Text Search

---

## Search Pipeline

```text
User Query
     │
     ▼
Search Service
     │
     ▼
FTS Index
     │
     ▼
Results
```

---

# Metadata Service

Purpose:

Generate searchable metadata.

Responsibilities:

* Timestamps
* Source Application
* Source Window
* Language
* Device

Future:

* OCR Metadata
* Screenshot Metadata

---

# Settings Service

Purpose:

Manage application configuration.

Stores:

* Languages
* Shortcuts
* Theme
* Privacy Settings
* Clipboard Limits

---

# Database Layer

Technology:

SQLite

Benefits:

* Lightweight
* Fast
* Offline
* Reliable

---

## Database Access Pattern

```text
Service
   │
Repository
   │
SQLite
```

Services never communicate directly with SQLite.

Repositories handle persistence.

---

# OS Integration Layer

Purpose:

Interact with operating system APIs.

---

## Clipboard Hooks

Windows Clipboard API

Responsibilities:

* Monitor Clipboard
* Detect Changes
* Capture Content

---

## Keyboard Hooks

Windows Low-Level Keyboard Hook

Responsibilities:

* Detect Typing
* Generate Suggestions
* Trigger Shortcuts

---

## Language APIs

Windows Language Services

Responsibilities:

* Installed Languages
* Language Switching
* Input Detection

---

# UI Architecture

```text
App
│
├── Dashboard
├── Clipboard
├── Notes
├── Dictionary
├── Settings
└── Search
```

---

# Background Services

Boarddy runs background workers.

---

## Clipboard Worker

Monitors clipboard changes.

Runs continuously.

---

## Search Worker

Maintains search indexes.

Runs asynchronously.

---

## Dictionary Worker

Updates language dictionaries.

Runs on demand.

---

# Event System

Architecture Pattern:

Event Driven

Examples:

```text
Clipboard Changed
```

Triggers:

* Save Clipboard
* Generate Metadata
* Update Search Index

---

```text
Dictionary Updated
```

Triggers:

* Refresh Suggestions
* Refresh Corrections

---

# Security Architecture

Data Storage:

Local SQLite Database

Encryption:

Sensitive Settings Only

Future:

Database Encryption

---

# Sync Architecture (V3)

Current:

Disabled

Future:

```text
Device A
     │
Encrypted Sync
     │
Cloud Layer
     │
Encrypted Sync
     │
Device B
```

Boarddy servers never access plaintext user content.

---

# AI Architecture (V3)

Architecture:

Local First

Preferred:

On-device Models

Capabilities:

* Recall
* Search
* Suggestions
* Classification

No AI dependency in V1.

---

# Performance Targets

Startup:

< 2 Seconds

Clipboard Save:

< 100ms

Search:

< 200ms

Quick Paste Popup:

< 100ms

Dictionary Lookup:

< 50ms

Memory Usage:

< 200MB

---

# Logging Architecture

Default:

Minimal Logging

Log Types:

* Errors
* Warnings
* Crashes

No content logging.

No keystroke logging.

No clipboard content logging.

---

# Future Architecture Modules

V2

* Screenshot Service
* OCR Service
* Voice Service

V3

* Sync Service
* AI Service
* Mobile Bridge Service
* Team Workspace Service

---

# Final Architecture Principle

Boarddy should never feel like a heavy application.

Users should forget it is running.

The ideal experience is simple:

Type.

Copy.

Paste.

Search.

Remember.

Boarddy handles everything else silently.

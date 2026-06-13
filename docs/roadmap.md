# Boarddy Roadmap.md

Version: 1.0

Product: Boarddy

Execution Model: Phased Delivery

Strategy: Vertical Depth First

---

# Core Strategy

Boarddy is built in layers:

1. **Make typing faster (V1)**
2. **Make copying unforgettable (V2)**
3. **Make memory intelligent (V3)**

Each phase must be fully valuable on its own.

No phase depends on the next.

---

# Phase 1 — V1: Input Foundation (SHIP FIRST)

## Goal

Become a daily-use system-level typing companion.

If users install Boarddy and stop using it, V1 has failed.

If users install Boarddy and cannot type without it, V1 succeeds.

---

## Core Modules

### 1. Autocorrect Engine

* System-wide spell correction
* User-defined dictionary overrides
* Undo correction support

---

### 2. Autocomplete Engine

* Inline suggestions
* Context-based predictions
* Lightweight word suggestions

---

### 3. Clipboard History System

* Text clipboard capture
* Copy tracking
* Time-based history
* Basic search

---

### 4. Quick Paste Overlay

Shortcut:

```text id="v7h1qx"
Ctrl + Shift + V
```

Features:

* Search clipboard
* Select item
* Paste instantly

---

### 5. Personal Dictionary

* Add custom words
* Prevent unwanted autocorrect
* Language-specific entries

---

### 6. Multilingual Support

* OS language integration
* Multi-language typing
* Language switching support

---

## Success Criteria (V1)

* Users use Quick Paste at least 10x/day
* Clipboard becomes habit within 24 hours
* Autocorrect reduces visible typing errors
* App runs silently in background
* < 2 sec startup time maintained

---

## V1 Exit Condition

If users say:

> “I can’t go back to normal typing”

→ V1 is complete

---

# Phase 2 — V2: Smart Capture Layer

## Goal

Turn Boarddy into a **personal memory system**

Not just typing tool anymore.

---

## Core Modules

### 1. Screenshot Clipboard

* Auto capture screenshots
* Store with metadata
* Link to app/window

---

### 2. Metadata Engine

Every item stores:

* Time
* App
* Window
* Language
* Type

---

### 3. Search Engine Upgrade

* Full-text search across:

  * Clipboard
  * Notes
  * Screenshots
* Instant filtering

---

### 4. Voice Typing

* Speech-to-text input
* Multilingual support
* Lightweight mode

---

### 5. Glide Typing (Experimental)

* Touchpad swipe typing
* Optional feature
* Not core dependency

---

### 6. Bluetooth Keyboard Extension

* Phone as input device
* Shortcut integration
* Remote typing mode

---

## Success Criteria (V2)

* Users can retrieve anything copied in past 30 days instantly
* Screenshots become searchable
* Search becomes daily behavior
* Voice typing reduces manual typing in specific contexts

---

## V2 Exit Condition

If users say:

> “Boarddy remembers everything I did”

→ V2 is complete

---

# Phase 3 — V3: Boarddy Ecosystem (INTELLIGENCE LAYER)

## Goal

Transform Boarddy into a **personal input operating system**

---

## Core Modules

### 1. Cross-Device Sync

* Clipboard sync across devices
* Secure encrypted transfer
* Device pairing system

---

### 2. Snippets Engine

* Text expansion system
* Dynamic variables
* Personal shortcuts

Example:

```text id="0s7w6x"
;email → support@company.com
```

---

### 3. Signature System

* Mouse/trackpad signature capture
* Reusable signature storage

---

### 4. AI Recall Engine

* Natural language search
* Memory reconstruction
* Smart retrieval across all data types

Example:

> “Find invoice I copied last month”

---

### 5. Team Workspaces (Optional Layer)

* Shared dictionaries
* Shared snippets
* Shared templates

---

### 6. Mobile Companion App

* Phone keyboard
* Clipboard sync
* Remote input control

---

## Success Criteria (V3)

* Users rely on Boarddy as memory extension
* Search replaces manual browsing of files/messages
* Cross-device workflow feels seamless
* Boarddy becomes invisible infrastructure

---

## V3 Exit Condition

If users say:

> “Boarddy is my second brain for everything I type or copy”

→ V3 is complete

---

# Execution Principles

## 1. No Parallel Complexity

Never build V2 while V1 is incomplete.

---

## 2. Each Phase Must Stand Alone

Each version must be usable without future features.

---

## 3. Performance is Non-Negotiable

If latency increases:

* Feature is rejected
* Not optimized later
* Rebuilt if necessary

---

## 4. Invisible UX Rule

If user notices Boarddy too often:

→ It is failing

---

# Risk Strategy

## Risk 1: Feature Overload

Solution:

Strict phase isolation

---

## Risk 2: Clipboard Complexity

Solution:

Start simple (text only), expand later

---

## Risk 3: OS Hook Instability

Solution:

Rust + Tauri native bindings only

---

# Monetization Trigger Points

## V1

Free adoption driver

---

## V2

OCR + Search unlocks Pro tier

---

## V3

Sync + AI recall unlocks premium ecosystem

---

# Final Execution Philosophy

Boarddy is not built as a feature product.

It is built as an **evolution system**:

* V1 → Habit
* V2 → Dependency
* V3 → Intelligence layer

If any phase fails to create dependency, the next phase will fail automatically.

So the real goal is simple:

> Make copying and typing impossible to live without.

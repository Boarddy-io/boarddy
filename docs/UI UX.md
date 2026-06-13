# Boarddy UI/UX Design System.md

Version: 1.0

Product: Boarddy

Platform: Windows (Primary)

Design Philosophy: Invisible Productivity Layer

---

# Core Design Philosophy

Boarddy is not a dashboard product.

Boarddy is a **micro-interaction system** embedded into the OS.

Users should:

* Not think about opening Boarddy
* Not navigate complex menus
* Not manage workflows manually

Instead:

* It appears when needed
* It disappears when not needed
* It predicts intent without noise

---

# Design Principles

## 1. Speed over beauty

If it looks good but slows interaction → remove it.

If it is ugly but instant → keep it.

---

## 2. One action per surface

Every UI element should do ONE job.

No clutter.

No multi-purpose panels.

---

## 3. Zero cognitive load

Users should not “learn” Boarddy.

They should “discover” it through usage.

---

## 4. Keyboard-first, mouse-second

Boarddy is built for:

* Shortcuts
* Search
* Quick selection
* Minimal clicking

---

## 5. Invisible by default

Boarddy only appears when:

* User types
* User copies
* User presses shortcut

Otherwise it stays dormant.

---

# Core UI Components

# 1. Quick Paste Overlay

## Trigger

```text
Ctrl + Shift + V
```

---

## Design

Minimal floating panel:

```text
┌────────────────────────────┐
│ Search clipboard...        │
├────────────────────────────┤
│ Email - support@...        │
│ Invoice - GTB12345         │
│ Prompt - "Write email..."  │
│ Screenshot - 2 mins ago    │
└────────────────────────────┘
```

---

## Behavior

* Arrow navigation
* Enter to paste
* ESC to close
* Instant search (no lag)

---

## UX Rule

Open → select → paste in under 2 seconds

If slower → broken design

---

# 2. Clipboard Panel

## Layout

Split view:

```text
┌──────────────┬──────────────┐
│ History      │ Preview      │
├──────────────┼──────────────┤
│ Item list    │ Content view │
│              │ Metadata     │
└──────────────┴──────────────┘
```

---

## Features

* Search bar top
* Filters (Text / Image / Code / Links)
* Pin button
* Favorite star
* Convert to note

---

## UX rule

No more than 1 click to access any clipboard item.

---

# 3. Autocomplete Popup

## Design

Appears near cursor:

```text
Thank you for your [support | time | patience]
```

---

## Behavior

* Inline suggestions
* Light grey hint text
* Tab to accept
* Ignore if not needed

---

## UX rule

Never block typing flow.

Suggestions must feel like “ghost thoughts”.

---

# 4. Autocorrect System

## Behavior

* Silent correction
* Subtle underline for uncertainty
* Ctrl + Z reverses correction

---

## UX rule

Never aggressively correct sensitive words.

Boarddy should “suggest intelligence,” not “impose correctness.”

---

# 5. Notes Interface

## Layout

Simple editor:

```text
Title: ____________

Content:

[ Rich text area ]
```

---

## Features

* Tags at top
* Save automatically
* Link clipboard source
* Search notes instantly

---

## UX rule

Notes are not a “productivity system.”

They are a **dumping ground for captured thoughts.**

---

# 6. Settings UI

## Structure

Left navigation:

* General
* Clipboard
* Typing
* Dictionary
* Languages
* Shortcuts
* Privacy

---

## Rule

Settings should be:

* Rarely used
* Extremely simple
* Searchable

---

# 7. Search UI (Global)

## Trigger

Hotkey:

```text
Ctrl + Space (optional future)
```

---

## Layout

Single input:

```text
Search everything...
```

Results grouped:

* Clipboard
* Notes
* Dictionary
* Snippets (future)

---

## UX rule

Search must feel like:

> “Ask Boarddy where something is.”

Not:

> “Browse categories.”

---

# Interaction Design System

## Micro-interactions

Boarddy relies heavily on micro-feedback:

* Fade in overlays
* Soft slide transitions
* Instant highlight on selection
* No heavy animations

---

## Timing Rules

| Action                | Max Time |
| --------------------- | -------- |
| Clipboard capture     | < 100ms  |
| Quick paste open      | < 100ms  |
| Search results        | < 200ms  |
| Autocomplete response | < 50ms   |

---

If anything exceeds these:

👉 UX failure

---

# Typography System

## Font

System default font (OS-native)

Reason:

* No loading delay
* Native feel
* Better performance

---

## Hierarchy

* Headings: medium weight
* Content: regular weight
* Suggestions: light grey
* Warnings: subtle red underline

---

# Color System

## Light Mode

* White background
* Soft grey borders
* Blue accent

---

## Dark Mode

* Near black background
* Soft contrast layers
* Neon blue highlights

---

## Rule

No flashy UI.

Boarddy is not a design showcase.

It is a utility layer.

---

# Keyboard UX System

## Core principle

Everything must be controllable via keyboard.

---

## Key patterns

| Action      | Shortcut              |
| ----------- | --------------------- |
| Quick Paste | Ctrl + Shift + V      |
| Copy        | Default               |
| Search      | Ctrl + F (contextual) |
| Escape      | Close overlay         |
| Enter       | Select                |
| Arrow keys  | Navigate              |

---

# Clipboard UX Philosophy

Clipboard is the heart of Boarddy.

So UX rule:

> Clipboard should feel like “time travel for text.”

---

# Error Handling UX

## Principle

Never show technical errors first.

Instead:

* Retry silently
* Show simple message only if failure persists

Example:

❌ “Database write error”

✔ “Clipboard could not be saved. Retrying…”

---

# Empty States

## Clipboard empty

```text
No clipboard history yet.
Start copying to build your memory.
```

---

## Notes empty

```text
No notes yet.
Save a clipboard item to get started.
```

---

# Accessibility Design

* Full keyboard navigation
* Screen reader support
* High contrast mode
* Reduced motion mode

---

# UX Non-Goals

Boarddy will NOT:

* Be heavily animated
* Require onboarding tutorials
* Force account creation
* Push notifications aggressively
* Show ads or distractions

---

# Design Tone

Boarddy should feel:

* Quiet
* Fast
* Intelligent
* Invisible
* Helpful without noise

---

# Final UX Principle

If a user notices Boarddy too much,

👉 it is already too intrusive.

If a user forgets it exists but feels faster,

👉 it is working perfectly.

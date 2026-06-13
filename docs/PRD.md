# Boarddy PRD.md

# Product Requirements Document

Version: 1.0

Product: Boarddy

Stage: MVP (V1)

Platform: Windows Desktop

Status: Draft

Owner: Huna Inc.

---

# Executive Summary

Boarddy is a desktop productivity application that improves typing accuracy, speeds up text entry, and provides intelligent clipboard management.

The first version focuses on five core capabilities:

1. System-wide Autocorrect
2. System-wide Autocomplete
3. Clipboard History
4. Clipboard Notes
5. Personal Dictionary
6. Multilingual Support
7. Quick Paste

Boarddy aims to become an always-available productivity companion that works across all applications.

---

# Problem Statement

Desktop users repeatedly face the following problems:

### Problem 1

Typing mistakes slow users down.

Examples:

* teh
* recieve
* adress

Most desktop applications provide inconsistent correction experiences.

---

### Problem 2

Users lose copied content.

Typical workflow:

Copy Item A

Copy Item B

Item A is permanently lost.

---

### Problem 3

Users repeatedly type the same words and phrases.

Examples:

* Emails
* Addresses
* Business names
* Frequently used responses

---

### Problem 4

Professionals use custom terminology that traditional dictionaries incorrectly flag.

Examples:

* HunaPay
* EcoSynergy
* Cloakey
* HunaRemit

---

### Problem 5

Users operate across multiple languages and input systems.

Most desktop typing tools provide weak multilingual support.

---

# Goals

## Primary Goals

Improve typing speed.

Improve typing accuracy.

Reduce repeated typing.

Prevent clipboard data loss.

Provide a searchable clipboard history.

Support multilingual users.

---

## Secondary Goals

Create habit-forming daily usage.

Build foundation for future memory features.

Prepare infrastructure for AI-powered capabilities.

---

# Non Goals

The following are NOT part of V1.

* Screenshot OCR
* Voice Typing
* Glide Typing
* AI Writing Assistant
* Mobile Companion App
* Cloud Synchronization
* Team Workspaces
* Signature Capture
* Cross Device Clipboard

These belong to future versions.

---

# Target Audience

## Primary Users

Knowledge Workers

* Managers
* Consultants
* Founders
* Researchers

---

## Secondary Users

Developers

* Software Engineers
* Product Managers
* Designers

---

## Tertiary Users

Students

Writers

Content Creators

---

# User Stories

## Autocorrect

As a user,

I want common spelling mistakes corrected automatically,

So that I can type faster.

---

## Autocomplete

As a user,

I want word suggestions while typing,

So that I can reduce keystrokes.

---

## Clipboard History

As a user,

I want all copied content stored,

So that I never lose information.

---

## Clipboard Notes

As a user,

I want to save important clipboard entries as notes,

So that I can retrieve them later.

---

## Personal Dictionary

As a user,

I want to define custom words,

So Boarddy never treats them as mistakes.

---

## Multilingual

As a user,

I want Boarddy to work with multiple installed languages,

So I can switch languages seamlessly.

---

## Quick Paste

As a user,

I want access to previous clipboard items,

So I can paste more than the most recent item.

---

# Functional Requirements

# Module 1 - Autocorrect

## Requirements

Detect misspelled words.

Suggest corrections.

Automatically replace common errors.

Allow undo.

Allow disable per application.

Allow disable globally.

---

## Acceptance Criteria

Typing:

teh

Automatically becomes:

the

within 500ms.

---

# Module 2 - Autocomplete

## Requirements

Suggest words while typing.

Support multiple languages.

Learn user vocabulary.

Display suggestion popup.

Allow keyboard navigation.

---

## Acceptance Criteria

Typing:

Thank you for your

shows relevant suggestions.

---

# Module 3 - Clipboard History

## Requirements

Monitor clipboard changes.

Store clipboard entries.

Maintain chronological history.

Search clipboard content.

Pin important entries.

Delete entries.

Clear history.

---

## Clipboard Types

Text

URLs

Emails

Phone Numbers

Code Snippets

---

## Acceptance Criteria

Copied content appears in history within 1 second.

---

# Module 4 - Clipboard Notes

## Requirements

Convert clipboard entry into note.

Edit notes.

Delete notes.

Tag notes.

Search notes.

---

## Acceptance Criteria

User can save clipboard entry as note in one click.

---

# Module 5 - Personal Dictionary

## Requirements

Add custom words.

Delete custom words.

Import dictionary.

Export dictionary.

Ignore correction suggestions.

---

## Acceptance Criteria

Added word is never autocorrected.

---

# Module 6 - Multilingual Support

## Requirements

Detect installed languages.

Allow language switching.

Support mixed-language typing.

Maintain separate dictionaries.

---

## Acceptance Criteria

Boarddy functions correctly with multiple installed languages.

---

# Module 7 - Quick Paste

## Requirements

Shortcut:

Ctrl + Shift + V

Display clipboard history popup.

Search clipboard entries.

Select item.

Paste selected item.

---

## Acceptance Criteria

Selected item is pasted into active application.

---

# User Interface

# Main Window

Sections:

* Home
* Clipboard
* Notes
* Dictionary
* Settings

---

# Tray Menu

Options:

* Open Boarddy
* Clipboard History
* Quick Note
* Settings
* Exit

---

# Quick Paste Popup

Displays:

Recent clipboard entries

Search bar

Pinned items

Favorites

---

# Settings

General

Typing

Clipboard

Dictionary

Languages

Shortcuts

Privacy

---

# Performance Requirements

Startup Time

Less than 2 seconds.

---

Memory Usage

Less than 200MB RAM.

---

Clipboard Capture

Less than 1 second delay.

---

Search Results

Less than 200ms.

---

# Security Requirements

Store data locally.

Encrypt sensitive settings.

No keystroke transmission.

No user data sales.

No hidden telemetry.

User-controlled data deletion.

---

# Analytics

Track anonymously if enabled:

Daily Active Users

Clipboard Items Saved

Notes Created

Dictionary Entries Added

Quick Paste Usage

Feature Adoption

---

# Risks

## Technical Risk

System-wide keyboard interception complexity.

---

## UX Risk

Aggressive autocorrection may frustrate users.

Mitigation:

Easy undo.

---

## Adoption Risk

Users may not understand clipboard history value.

Mitigation:

Strong onboarding.

---

# Success Metrics

30-Day Retention

Clipboard Searches Per User

Quick Paste Usage

Average Clipboard Entries Stored

Average Notes Created

Average Dictionary Size

Weekly Active Users

---

# MVP Release Criteria

Autocorrect Functional

Autocomplete Functional

Clipboard History Functional

Clipboard Notes Functional

Personal Dictionary Functional

Multilingual Functional

Quick Paste Functional

Crash-Free Rate Above 99%

Memory Usage Below Target

Ready for Public Beta

export interface ClipboardEntry {
  id: string;
  content: string;
  content_type: string;
  source_app: string | null;
  source_window: string | null;
  language_code: string | null;
  is_favorite: boolean;
  is_pinned: boolean;
  created_at: string;
  updated_at: string;
}

export interface ClipboardListResult {
  items: ClipboardEntry[];
  total: number;
}

export interface Note {
  id: string;
  title: string | null;
  content: string;
  source_clipboard_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface DictionaryEntry {
  id: string;
  word: string;
  language_code: string;
  source: string;
  created_at: string;
}

export interface Language {
  code: string;
  name: string;
  is_enabled: boolean;
}

export interface AutocorrectRule {
  id: string;
  trigger_word: string;
  replacement_word: string;
  language_code: string | null;
  created_at: string;
}

export interface ApiError {
  code: string;
  message: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: ApiError | null;
}

export interface Suggestion {
  word: string;
  shortcut: string;
  confidence: number;
  rank: number;
}


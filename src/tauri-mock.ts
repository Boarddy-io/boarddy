import { ClipboardEntry, Note, DictionaryEntry, Language, AutocorrectRule, ApiResponse } from "./types";

// Local memory store for mocks to allow interactive additions/deletions on the web!
let mockClipboard: ClipboardEntry[] = [
  {
    id: "1",
    content: "winget install Boarddy",
    content_type: "text",
    source_app: "Terminal",
    source_window: "PowerShell",
    language_code: "en",
    is_favorite: true,
    is_pinned: true,
    created_at: new Date(Date.now() - 60000).toISOString(),
    updated_at: new Date(Date.now() - 60000).toISOString(),
  },
  {
    id: "2",
    content: "const fetchClipboard = async () => {\n  const res = await invoke('get_clipboard_entries');\n};",
    content_type: "code",
    source_app: "VS Code",
    source_window: "App.tsx - Boarddy",
    language_code: "typescript",
    is_favorite: false,
    is_pinned: false,
    created_at: new Date(Date.now() - 120000).toISOString(),
    updated_at: new Date(Date.now() - 120000).toISOString(),
  },
  {
    id: "3",
    content: "https://boarddy-io.github.io/boarddy/",
    content_type: "url",
    source_app: "Chrome",
    source_window: "GitHub Pages Documentation",
    language_code: null,
    is_favorite: true,
    is_pinned: false,
    created_at: new Date(Date.now() - 300000).toISOString(),
    updated_at: new Date(Date.now() - 300000).toISOString(),
  },
  {
    id: "4",
    content: "support@huna.io",
    content_type: "text",
    source_app: "Outlook",
    source_window: "Inbound Support",
    language_code: null,
    is_favorite: false,
    is_pinned: false,
    created_at: new Date(Date.now() - 600000).toISOString(),
    updated_at: new Date(Date.now() - 600000).toISOString(),
  }
];

let mockNotes: Note[] = [
  {
    id: "1",
    title: "Boarddy Project Roadmap",
    content: "- Implement local LLM integration\n- Build cross-platform sync layer\n- Submit casks to Homebrew and Chocolatey",
    source_clipboard_id: null,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  },
  {
    id: "2",
    title: "Vite Deploy Command",
    content: "VITE_WEB_BUILD=true npm run build",
    source_clipboard_id: "2",
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  }
];

let mockDictionary: DictionaryEntry[] = [
  { id: "1", word: "Tauri", language_code: "en", source: "user", created_at: new Date().toISOString() },
  { id: "2", word: "Huna", language_code: "en", source: "user", created_at: new Date().toISOString() },
  { id: "3", word: "Autocorrect", language_code: "en", source: "system", created_at: new Date().toISOString() }
];

let mockRules: AutocorrectRule[] = [
  { id: "1", trigger_word: "teh", replacement_word: "the", language_code: "en", created_at: new Date().toISOString() },
  { id: "2", trigger_word: "huna", replacement_word: "Huna Inc.", language_code: "en", created_at: new Date().toISOString() }
];

let mockSettings: Record<string, string> = {
  "theme": "dark",
  "clipboard_limit": "100",
  "enable_pks": "true",
  "enable_autocorrect": "true",
  "double_shift_paste": "true"
};

let mockLanguages: Language[] = [
  { code: "en", name: "English", is_enabled: true },
  { code: "es", name: "Spanish", is_enabled: false },
  { code: "fr", name: "French", is_enabled: false }
];

// Helper wrapper to format responses
function successResponse<T>(data: T): ApiResponse<T> {
  return { success: true, data, error: null };
}

// 1. Mock invoke function
export async function invoke(cmd: string, args?: any): Promise<any> {
  console.log(`[Tauri Mock] Invoke command: ${cmd}`, args);

  switch (cmd) {
    // Clipboard commands
    case "get_clipboard_entries":
      return successResponse({
        items: mockClipboard,
        total: mockClipboard.length
      });
    case "search_clipboard": {
      const q = (args?.query || "").toLowerCase();
      const filtered = mockClipboard.filter(item => 
        item.content.toLowerCase().includes(q) || 
        (item.source_app && item.source_app.toLowerCase().includes(q))
      );
      return successResponse(filtered);
    }
    case "toggle_favorite_clipboard": {
      mockClipboard = mockClipboard.map(item => 
        item.id === args?.id ? { ...item, is_favorite: !item.is_favorite } : item
      );
      return successResponse(null);
    }
    case "toggle_pin_clipboard": {
      mockClipboard = mockClipboard.map(item => 
        item.id === args?.id ? { ...item, is_pinned: !item.is_pinned } : item
      );
      return successResponse(null);
    }
    case "delete_clipboard_entry": {
      mockClipboard = mockClipboard.filter(item => item.id !== args?.id);
      return successResponse(null);
    }
    case "clear_clipboard_history": {
      mockClipboard = [];
      return successResponse(null);
    }
    case "create_note_from_clipboard": {
      const clip = mockClipboard.find(c => c.id === args?.id);
      if (clip) {
        const newNote: Note = {
          id: String(mockNotes.length + 1),
          title: "Note from Clipboard",
          content: clip.content,
          source_clipboard_id: clip.id,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        };
        mockNotes.push(newNote);
      }
      return successResponse("Note created successfully!");
    }

    // Notes commands
    case "search_notes": {
      const q = (args?.query || "").toLowerCase();
      const filtered = mockNotes.filter(n => 
        n.content.toLowerCase().includes(q) || 
        (n.title && n.title.toLowerCase().includes(q))
      );
      return successResponse(filtered);
    }
    case "update_note": {
      mockNotes = mockNotes.map(n => 
        n.id === args?.id ? { ...n, title: args.title, content: args.content, updated_at: new Date().toISOString() } : n
      );
      return successResponse(null);
    }
    case "create_note": {
      const newNote: Note = {
        id: String(mockNotes.length + 1),
        title: args.title || "Untitled Note",
        content: args.content || "",
        source_clipboard_id: null,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      };
      mockNotes.push(newNote);
      return successResponse(newNote.id);
    }
    case "delete_note": {
      mockNotes = mockNotes.filter(n => n.id !== args?.id);
      return successResponse(null);
    }

    // Dictionary commands
    case "list_dictionary_words":
      return successResponse(mockDictionary);
    case "add_dictionary_word": {
      const newWord: DictionaryEntry = {
        id: String(mockDictionary.length + 1),
        word: args.word,
        language_code: args.languageCode || "en",
        source: "user",
        created_at: new Date().toISOString()
      };
      mockDictionary.push(newWord);
      return successResponse("Word added!");
    }
    case "remove_dictionary_word": {
      mockDictionary = mockDictionary.filter(w => w.id !== args?.id);
      return successResponse(null);
    }

    // Autocorrect rules
    case "get_autocorrect_rules":
      return successResponse(mockRules);
    case "add_correction_rule": {
      const newRule: AutocorrectRule = {
        id: String(mockRules.length + 1),
        trigger_word: args.triggerWord,
        replacement_word: args.replacementWord,
        language_code: args.languageCode || "en",
        created_at: new Date().toISOString()
      };
      mockRules.push(newRule);
      return successResponse(null);
    }
    case "remove_correction_rule": {
      mockRules = mockRules.filter(r => r.id !== args?.ruleId);
      return successResponse(null);
    }

    // Settings commands
    case "get_settings":
      return successResponse(mockSettings);
    case "get_languages":
      return successResponse(mockLanguages);
    case "update_setting": {
      mockSettings[args.key] = args.value;
      return successResponse(null);
    }
    case "enable_language": {
      mockLanguages = mockLanguages.map(l => l.code === args.code ? { ...l, is_enabled: true } : l);
      return successResponse(null);
    }
    case "disable_language": {
      mockLanguages = mockLanguages.map(l => l.code === args.code ? { ...l, is_enabled: false } : l);
      return successResponse(null);
    }
    case "check_shortcut_conflict":
      return successResponse(null);
    case "update_shortcut":
      return successResponse(null);

    // Quick paste
    case "paste_text":
      alert(`[Mock Paste] Pasted: ${args.text}`);
      return successResponse(null);
    case "hide_quick_paste":
      console.log("[Mock Paste] Hiding window");
      return successResponse(null);

    default:
      console.warn(`[Tauri Mock] Unhandled command: ${cmd}`);
      return successResponse(null);
  }
}

// 2. Mock webviewWindow
export function getCurrentWebviewWindow() {
  return {
    label: "main",
    listen: (event: string, _callback: any) => {
      console.log(`[Tauri Mock] listen to event: ${event}`);
      return Promise.resolve(() => {});
    }
  };
}

// 3. Mock event
export function listen(event: string, _callback: any) {
  console.log(`[Tauri Mock] global listen to event: ${event}`);
  return Promise.resolve(() => {});
}

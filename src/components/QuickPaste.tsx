import React, { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { ClipboardEntry, ApiResponse } from "../types";
import { Globe, Mail, Phone, Code, FileText, Search, Star, Pin } from "lucide-react";

export default function QuickPaste() {
  const [query, setQuery] = useState("");
  const [items, setItems] = useState<ClipboardEntry[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const windowRef = useRef(getCurrentWebviewWindow());

  // Load items on mount and query updates
  useEffect(() => {
    fetchClips();
    // Focus search input
    if (searchInputRef.current) {
      searchInputRef.current.focus();
    }
  }, [query]);

  // Listen to window focus/show to reload clipboard entries
  useEffect(() => {
    const unlistenPromise = windowRef.current.onFocusChanged((event) => {
      if (event.payload) {
        fetchClips();
        setSelectedIndex(0);
        if (searchInputRef.current) {
          searchInputRef.current.focus();
        }
      }
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const fetchClips = async () => {
    try {
      const res: ApiResponse<ClipboardEntry[]> = await invoke("search_clipboard", { query });
      if (res.success && res.data) {
        setItems(res.data);
      }
    } catch (e) {
      console.error("Failed to fetch search clipboard: ", e);
    }
  };

  const handleKeyDown = async (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setSelectedIndex((prev) => (prev + 1) % Math.max(1, items.length));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setSelectedIndex((prev) => (prev - 1 + items.length) % Math.max(1, items.length));
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (items[selectedIndex]) {
        await handleSelect(items[selectedIndex]);
      }
    } else if (e.key === "Escape") {
      e.preventDefault();
      closeWindow();
    }
  };

  const handleSelect = async (item: ClipboardEntry) => {
    try {
      await invoke("paste_text", { text: item.content });
      closeWindow();
    } catch (e) {
      console.error("Failed to paste: ", e);
    }
  };

  const closeWindow = async () => {
    await invoke("hide_quick_paste");
  };

  const renderIcon = (type: string) => {
    const className = "w-4 h-4 text-sky-400";
    switch (type) {
      case "url":
        return <Globe className={className} />;
      case "email":
        return <Mail className={className} />;
      case "phone":
        return <Phone className={className} />;
      case "code":
        return <Code className={className} />;
      default:
        return <FileText className={className} />;
    }
  };

  const cleanAppLabel = (app: string | null) => {
    if (!app) return "System";
    const name = app.replace(/\.exe$/i, "");
    return name.charAt(0).toUpperCase() + name.slice(1);
  };

  return (
    <div
      className="glass w-screen h-screen rounded-xl flex flex-col p-3 shadow-2xl border border-white/10 select-none overflow-hidden"
      onKeyDown={handleKeyDown}
    >
      {/* Search Bar */}
      <div className="relative flex items-center bg-slate-900/80 border border-white/5 rounded-lg px-3 py-2 mb-2 focus-within:border-sky-500/50 transition duration-150">
        <Search className="w-4 h-4 text-slate-400 mr-2" />
        <input
          ref={searchInputRef}
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search clipboard..."
          className="bg-transparent border-none outline-none text-slate-100 placeholder-slate-500 text-sm w-full"
        />
      </div>

      {/* Clipboard List */}
      <div className="flex-1 overflow-y-auto space-y-1 pr-1">
        {items.length > 0 ? (
          items.map((item, index) => {
            const isSelected = index === selectedIndex;
            return (
              <div
                key={item.id}
                onClick={() => handleSelect(item)}
                onMouseEnter={() => setSelectedIndex(index)}
                className={`flex items-center justify-between px-3 py-2 rounded-lg cursor-pointer transition duration-150 ${
                  isSelected ? "bg-sky-600/35 border border-sky-500/30" : "hover:bg-slate-800/40 border border-transparent"
                }`}
              >
                <div className="flex items-center space-x-3 truncate">
                  {renderIcon(item.content_type)}
                  <div className="truncate flex flex-col">
                    <span className="text-slate-100 text-sm font-medium truncate">
                      {item.content.trim()}
                    </span>
                    <span className="text-slate-400 text-[10px] flex items-center space-x-2 mt-0.5">
                      <span>{cleanAppLabel(item.source_app)}</span>
                      {item.source_window && (
                        <span className="truncate max-w-[200px]">
                          • {item.source_window}
                        </span>
                      )}
                    </span>
                  </div>
                </div>

                <div className="flex items-center space-x-1 ml-2 flex-shrink-0">
                  {item.is_pinned && <Pin className="w-3.5 h-3.5 text-orange-400" />}
                  {item.is_favorite && <Star className="w-3.5 h-3.5 text-yellow-400 fill-yellow-400" />}
                </div>
              </div>
            );
          })
        ) : (
          <div className="text-center text-slate-500 text-xs py-8">
            No clipboard history matches.
          </div>
        )}
      </div>

      {/* Keyboard Shortcuts Hint */}
      <div className="text-[10px] text-slate-500 border-t border-white/5 pt-2 mt-2 flex justify-between">
        <span>↑↓ to navigate</span>
        <span>Enter to paste</span>
        <span>Esc to close</span>
      </div>
    </div>
  );
}

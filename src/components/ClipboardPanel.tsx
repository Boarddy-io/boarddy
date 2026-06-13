import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ClipboardEntry, ClipboardListResult, ApiResponse } from "../types";
import {
  Globe,
  Mail,
  Phone,
  Code,
  FileText,
  Search,
  Star,
  Pin,
  Trash2,
  FileDown,
  Copy,
  Clock,
  Sparkles,
} from "lucide-react";

export default function ClipboardPanel() {
  const [query, setQuery] = useState("");
  const [activeFilter, setActiveFilter] = useState("all");
  const [items, setItems] = useState<ClipboardEntry[]>([]);
  const [selectedItem, setSelectedItem] = useState<ClipboardEntry | null>(null);
  const [page, setPage] = useState(1);
  const [limit] = useState(50);
  const [notesMessage, setNotesMessage] = useState("");

  useEffect(() => {
    fetchClipboard();
  }, [query, activeFilter, page]);

  const fetchClipboard = async () => {
    try {
      if (!query && activeFilter === "all") {
        const res: ApiResponse<ClipboardListResult> = await invoke("get_clipboard_entries", {
          page,
          limit,
        });
        if (res.success && res.data) {
          setItems(res.data.items);
          if (res.data.items.length > 0 && !selectedItem) {
            setSelectedItem(res.data.items[0]);
          }
        }
      } else {
        // Search & filter
        const res: ApiResponse<ClipboardEntry[]> = await invoke("search_clipboard", { query });
        if (res.success && res.data) {
          let filtered = res.data;
          if (activeFilter !== "all") {
            filtered = res.data.filter((item) => item.content_type === activeFilter);
          }
          setItems(filtered);
          if (filtered.length > 0) {
            setSelectedItem(filtered[0]);
          } else {
            setSelectedItem(null);
          }
        }
      }
    } catch (e) {
      console.error("Failed to load clipboard entries: ", e);
    }
  };

  const handleTogglePin = async (item: ClipboardEntry) => {
    try {
      const command = item.is_pinned ? "unpin_clipboard_entry" : "pin_clipboard_entry";
      const res: ApiResponse<void> = await invoke(command, { id: item.id });
      if (res.success) {
        setSelectedItem((prev) => prev && prev.id === item.id ? { ...prev, is_pinned: !item.is_pinned } : prev);
        fetchClipboard();
      }
    } catch (e) {
      console.error("Pin toggle failed: ", e);
    }
  };

  const handleToggleFavorite = async (item: ClipboardEntry) => {
    try {
      const command = item.is_favorite ? "unfavorite_clipboard_entry" : "favorite_clipboard_entry";
      const res: ApiResponse<void> = await invoke(command, { id: item.id });
      if (res.success) {
        setSelectedItem((prev) => prev && prev.id === item.id ? { ...prev, is_favorite: !item.is_favorite } : prev);
        fetchClipboard();
      }
    } catch (e) {
      console.error("Favorite toggle failed: ", e);
    }
  };

  const handleDelete = async (item: ClipboardEntry) => {
    try {
      const res: ApiResponse<void> = await invoke("delete_clipboard_entry", { id: item.id });
      if (res.success) {
        if (selectedItem?.id === item.id) {
          setSelectedItem(null);
        }
        fetchClipboard();
      }
    } catch (e) {
      console.error("Deletion failed: ", e);
    }
  };

  const handleClearHistory = async () => {
    if (confirm("Are you sure you want to clear your clipboard history? (Pinned and favorited items will be kept)")) {
      try {
        const res: ApiResponse<void> = await invoke("clear_clipboard_history");
        if (res.success) {
          setSelectedItem(null);
          fetchClipboard();
        }
      } catch (e) {
        console.error("Clear history failed: ", e);
      }
    }
  };

  const handleConvertToNote = async (item: ClipboardEntry) => {
    try {
      const res: ApiResponse<string> = await invoke("create_note_from_clipboard", {
        clipboardId: item.id,
      });
      if (res.success) {
        setNotesMessage("Converted to note successfully!");
        setTimeout(() => setNotesMessage(""), 3000);
      } else {
        setNotesMessage("Conversion failed.");
        setTimeout(() => setNotesMessage(""), 3000);
      }
    } catch (e) {
      console.error("Convert to note failed: ", e);
    }
  };

  const handleCopyToClipboard = async (content: string) => {
    try {
      await navigator.clipboard.writeText(content);
      setNotesMessage("Copied to clipboard!");
      setTimeout(() => setNotesMessage(""), 2000);
    } catch (e) {
      console.error("Failed to copy: ", e);
    }
  };

  const renderIcon = (type: string, className = "w-4 h-4") => {
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

  const cleanAppName = (app: string | null) => {
    if (!app) return "System";
    const clean = app.replace(/\.exe$/i, "");
    return clean.charAt(0).toUpperCase() + clean.slice(1);
  };

  return (
    <div className="flex-1 flex overflow-hidden">
      {/* List Column */}
      <div className="w-1/2 flex flex-col border-r border-white/5 pr-4 overflow-hidden">
        {/* Search & Actions */}
        <div className="flex items-center space-x-2 mb-3">
          <div className="flex-1 relative flex items-center bg-slate-900/60 border border-white/5 rounded-lg px-3 py-1.5 focus-within:border-sky-500/50 transition">
            <Search className="w-4 h-4 text-slate-400 mr-2" />
            <input
              type="text"
              value={query}
              onChange={(e) => {
                setQuery(e.target.value);
                setPage(1);
              }}
              placeholder="Search history..."
              className="bg-transparent border-none outline-none text-slate-100 placeholder-slate-500 text-sm w-full"
            />
          </div>
          <button
            onClick={handleClearHistory}
            className="flex items-center space-x-1.5 px-3 py-1.5 bg-red-950/20 hover:bg-red-950/40 text-red-400 border border-red-900/30 rounded-lg text-xs font-semibold cursor-pointer transition"
          >
            <Trash2 className="w-3.5 h-3.5" />
            <span>Clear</span>
          </button>
        </div>

        {/* Content Type Filters */}
        <div className="flex space-x-1.5 mb-3 overflow-x-auto pb-1">
          {["all", "text", "code", "url", "email", "phone"].map((filter) => (
            <button
              key={filter}
              onClick={() => {
                setActiveFilter(filter);
                setPage(1);
              }}
              className={`px-3 py-1 rounded-full text-xs font-semibold capitalize cursor-pointer transition duration-150 ${
                activeFilter === filter
                  ? "bg-sky-500 text-white shadow-lg shadow-sky-500/20"
                  : "bg-slate-900/55 hover:bg-slate-800 text-slate-400"
              }`}
            >
              {filter}
            </button>
          ))}
        </div>

        {/* List items */}
        <div className="flex-1 overflow-y-auto space-y-1.5 pr-1">
          {items.length > 0 ? (
            items.map((item) => {
              const isSelected = selectedItem?.id === item.id;
              return (
                <div
                  key={item.id}
                  onClick={() => setSelectedItem(item)}
                  className={`flex flex-col p-3 rounded-lg border cursor-pointer transition duration-150 ${
                    isSelected
                      ? "bg-slate-900/70 border-sky-500/50"
                      : "bg-slate-900/30 hover:bg-slate-900/50 border-white/5"
                  }`}
                >
                  <div className="flex items-center justify-between mb-1.5">
                    <div className="flex items-center space-x-2">
                      {renderIcon(item.content_type, "w-3.5 h-3.5 text-sky-400")}
                      <span className="text-[10px] text-slate-500 font-semibold uppercase">
                        {item.content_type}
                      </span>
                    </div>
                    <div className="flex items-center space-x-1">
                      {item.is_pinned && <Pin className="w-3.5 h-3.5 text-orange-400" />}
                      {item.is_favorite && <Star className="w-3.5 h-3.5 text-yellow-400 fill-yellow-400" />}
                    </div>
                  </div>

                  <p className="text-slate-200 text-sm font-medium line-clamp-2 break-all mb-2">
                    {item.content.trim()}
                  </p>

                  <div className="flex items-center justify-between text-[10px] text-slate-500">
                    <span className="font-semibold">
                      {cleanAppName(item.source_app)}
                    </span>
                    <span className="flex items-center space-x-1">
                      <Clock className="w-3 h-3" />
                      <span>{item.created_at.slice(11, 16)}</span>
                    </span>
                  </div>
                </div>
              );
            })
          ) : (
            <div className="text-center text-slate-500 text-sm py-12">
              No clipboard items yet. Start copying text!
            </div>
          )}
        </div>
      </div>

      {/* Preview Column */}
      <div className="w-1/2 flex flex-col pl-4 overflow-hidden">
        {selectedItem ? (
          <div className="flex-1 flex flex-col overflow-hidden">
            {/* Header Details */}
            <div className="flex justify-between items-start mb-4 border-b border-white/5 pb-3 flex-shrink-0">
              <div>
                <h3 className="text-sm font-semibold text-slate-200 flex items-center space-x-2">
                  {renderIcon(selectedItem.content_type, "w-4 h-4 text-sky-400")}
                  <span>Clipboard Entry details</span>
                </h3>
                <p className="text-[11px] text-slate-500 mt-1">
                  Captured at {selectedItem.created_at} from{" "}
                  <span className="font-bold text-slate-400">
                    {cleanAppName(selectedItem.source_app)}
                  </span>
                </p>
                {selectedItem.source_window && (
                  <p className="text-[10px] text-slate-600 truncate max-w-[300px]">
                    Window: {selectedItem.source_window}
                  </p>
                )}
              </div>

              {/* Pin & Favorite Buttons */}
              <div className="flex space-x-1">
                <button
                  onClick={() => handleTogglePin(selectedItem)}
                  className={`p-1.5 rounded-lg border transition cursor-pointer ${
                    selectedItem.is_pinned
                      ? "bg-orange-500/10 border-orange-500/30 text-orange-400"
                      : "bg-slate-900 border-white/5 text-slate-400 hover:bg-slate-800"
                  }`}
                  title="Pin item"
                >
                  <Pin className="w-4 h-4" />
                </button>
                <button
                  onClick={() => handleToggleFavorite(selectedItem)}
                  className={`p-1.5 rounded-lg border transition cursor-pointer ${
                    selectedItem.is_favorite
                      ? "bg-yellow-500/10 border-yellow-500/30 text-yellow-400 fill-yellow-400"
                      : "bg-slate-900 border-white/5 text-slate-400 hover:bg-slate-800"
                  }`}
                  title="Star item"
                >
                  <Star className="w-4 h-4" />
                </button>
              </div>
            </div>

            {/* Main content body */}
            <div className="flex-1 overflow-y-auto bg-slate-950/70 border border-white/5 p-4 rounded-xl font-mono text-sm break-all whitespace-pre-wrap select-text selection:bg-sky-500/35 mb-4">
              {selectedItem.content}
            </div>

            {/* Quick Actions Footer */}
            <div className="flex items-center space-x-2 border-t border-white/5 pt-3 flex-shrink-0">
              <button
                onClick={() => handleCopyToClipboard(selectedItem.content)}
                className="flex items-center justify-center space-x-1.5 flex-1 py-2 bg-slate-900 hover:bg-slate-800 border border-white/10 rounded-lg text-xs font-semibold cursor-pointer transition text-slate-300"
              >
                <Copy className="w-4 h-4" />
                <span>Copy</span>
              </button>
              <button
                onClick={() => handleConvertToNote(selectedItem)}
                className="flex items-center justify-center space-x-1.5 flex-1 py-2 bg-sky-950/20 hover:bg-sky-950/40 border border-sky-900/30 rounded-lg text-xs font-semibold cursor-pointer transition text-sky-400"
              >
                <FileDown className="w-4 h-4" />
                <span>Convert to Note</span>
              </button>
              <button
                onClick={() => handleDelete(selectedItem)}
                className="flex items-center justify-center space-x-1.5 p-2 bg-red-950/20 hover:bg-red-950/45 border border-red-900/30 rounded-lg text-red-400 cursor-pointer transition"
                title="Delete Entry"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            </div>

            {notesMessage && (
              <div className="mt-2 text-center text-xs text-sky-400 bg-sky-950/30 border border-sky-500/20 py-1.5 rounded-lg flex items-center justify-center space-x-1">
                <Sparkles className="w-3.5 h-3.5" />
                <span>{notesMessage}</span>
              </div>
            )}
          </div>
        ) : (
          <div className="flex-1 flex flex-col items-center justify-center text-slate-500 text-sm border border-dashed border-white/5 rounded-xl">
            Select a clipboard entry to view details
          </div>
        )}
      </div>
    </div>
  );
}

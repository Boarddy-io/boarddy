import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Note, ApiResponse } from "../types";
import { Search, Plus, Trash2, Save, Clipboard, Sparkles } from "lucide-react";

export default function NotesPanel() {
  const [query, setQuery] = useState("");
  const [notes, setNotes] = useState<Note[]>([]);
  const [selectedNote, setSelectedNote] = useState<Note | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [message, setMessage] = useState("");

  useEffect(() => {
    fetchNotes();
  }, [query]);

  const fetchNotes = async () => {
    try {
      const res: ApiResponse<Note[]> = await invoke("search_notes", { query });
      if (res.success && res.data) {
        setNotes(res.data);
        if (res.data.length > 0 && !selectedNote) {
          // Select first note by default
          selectNote(res.data[0]);
        }
      }
    } catch (e) {
      console.error("Failed to load notes: ", e);
    }
  };

  const selectNote = (note: Note) => {
    setSelectedNote(note);
    setTitle(note.title || "");
    setContent(note.content);
    setIsEditing(false);
  };

  const handleCreateNewNote = () => {
    setSelectedNote(null);
    setTitle("");
    setContent("");
    setIsEditing(true);
  };

  const handleSaveNote = async () => {
    if (!content.trim()) {
      showMsg("Note content cannot be empty.");
      return;
    }

    try {
      if (selectedNote) {
        // Update
        const res: ApiResponse<void> = await invoke("update_note", {
          id: selectedNote.id,
          title: title.trim() ? title : null,
          content,
        });
        if (res.success) {
          showMsg("Note updated!");
          fetchNotes();
          setSelectedNote((prev) => prev ? { ...prev, title: title.trim() ? title : null, content } : null);
          setIsEditing(false);
        } else {
          showMsg(res.error?.message || "Failed to save note");
        }
      } else {
        // Create
        const res: ApiResponse<string> = await invoke("create_note", {
          title: title.trim() ? title : null,
          content,
        });
        if (res.success && res.data) {
          showMsg("Note created!");
          setIsEditing(false);
          const newNote: Note = {
            id: res.data,
            title: title.trim() ? title : null,
            content,
            source_clipboard_id: null,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
          };
          setSelectedNote(newNote);
          fetchNotes();
        } else {
          showMsg(res.error?.message || "Failed to create note");
        }
      }
    } catch (e) {
      console.error("Save note failed: ", e);
      showMsg("An error occurred while saving.");
    }
  };

  const handleDeleteNote = async (note: Note) => {
    if (confirm("Are you sure you want to delete this note?")) {
      try {
        const res: ApiResponse<void> = await invoke("delete_note", { id: note.id });
        if (res.success) {
          showMsg("Note deleted!");
          if (selectedNote?.id === note.id) {
            setSelectedNote(null);
            setTitle("");
            setContent("");
          }
          fetchNotes();
        }
      } catch (e) {
        console.error("Delete note failed: ", e);
      }
    }
  };

  const showMsg = (txt: string) => {
    setMessage(txt);
    setTimeout(() => setMessage(""), 3000);
  };

  return (
    <div className="flex-1 flex overflow-hidden">
      {/* Sidebar List */}
      <div className="w-5/12 flex flex-col border-r border-white/5 pr-4 overflow-hidden">
        {/* Search & New Note Actions */}
        <div className="flex items-center space-x-2 mb-3">
          <div className="flex-1 relative flex items-center bg-slate-900/60 border border-white/5 rounded-lg px-3 py-1.5 focus-within:border-sky-500/50 transition">
            <Search className="w-4 h-4 text-slate-400 mr-2" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search notes..."
              className="bg-transparent border-none outline-none text-slate-100 placeholder-slate-500 text-sm w-full"
            />
          </div>
          <button
            onClick={handleCreateNewNote}
            className="flex items-center justify-center p-2 bg-sky-600 hover:bg-sky-500 text-white rounded-lg cursor-pointer transition shadow-lg shadow-sky-500/10"
            title="Create New Note"
          >
            <Plus className="w-4 h-4" />
          </button>
        </div>

        {/* Notes List */}
        <div className="flex-1 overflow-y-auto space-y-1.5 pr-1">
          {notes.length > 0 ? (
            notes.map((note) => {
              const isSelected = selectedNote?.id === note.id;
              return (
                <div
                  key={note.id}
                  onClick={() => selectNote(note)}
                  className={`flex flex-col p-3 rounded-lg border cursor-pointer transition duration-150 ${
                    isSelected
                      ? "bg-slate-900/70 border-sky-500/50"
                      : "bg-slate-900/30 hover:bg-slate-900/50 border-white/5"
                  }`}
                >
                  <h4 className="text-slate-100 text-sm font-semibold truncate mb-1">
                    {note.title || "Untitled Note"}
                  </h4>
                  <p className="text-slate-400 text-xs line-clamp-2 mb-2">
                    {note.content}
                  </p>
                  <div className="flex items-center justify-between text-[10px] text-slate-500 font-medium">
                    <span>{note.created_at.slice(0, 10)}</span>
                    {note.source_clipboard_id && (
                      <span className="flex items-center space-x-1 text-sky-400/80">
                        <Clipboard className="w-2.5 h-2.5" />
                        <span>Clipboard Link</span>
                      </span>
                    )}
                  </div>
                </div>
              );
            })
          ) : (
            <div className="text-center text-slate-500 text-sm py-12">
              No notes saved. Click + to create a note or convert a clipboard item!
            </div>
          )}
        </div>
      </div>

      {/* Editor Panel */}
      <div className="w-7/12 flex flex-col pl-4 overflow-hidden">
        {selectedNote || isEditing ? (
          <div className="flex-1 flex flex-col overflow-hidden">
            {/* Title Input */}
            <input
              type="text"
              value={title}
              onChange={(e) => {
                setTitle(e.target.value);
                setIsEditing(true);
              }}
              placeholder="Title (Optional)"
              className="bg-transparent border-none outline-none text-slate-100 text-lg font-semibold py-2 mb-2 w-full focus:border-b focus:border-sky-500/20"
            />

            {/* Note Content Textarea */}
            <textarea
              value={content}
              onChange={(e) => {
                setContent(e.target.value);
                setIsEditing(true);
              }}
              placeholder="Start writing..."
              className="flex-1 bg-slate-950/70 border border-white/5 rounded-xl p-4 font-mono text-sm resize-none outline-none text-slate-200 placeholder-slate-600 focus:border-sky-500/25 transition select-text selection:bg-sky-500/30 mb-4"
            />

            {/* Clipboard Reference & Actions */}
            <div className="flex items-center justify-between border-t border-white/5 pt-3 flex-shrink-0">
              <div className="flex items-center space-x-2">
                {selectedNote && selectedNote.source_clipboard_id && (
                  <div className="flex items-center space-x-1 bg-slate-900 border border-white/5 px-2.5 py-1 rounded-lg text-[10px] text-slate-400">
                    <Clipboard className="w-3.5 h-3.5 text-sky-400" />
                    <span>Linked Clipboard Item</span>
                  </div>
                )}
              </div>

              {/* Save/Delete buttons */}
              <div className="flex items-center space-x-2">
                {selectedNote && (
                  <button
                    onClick={() => handleDeleteNote(selectedNote)}
                    className="flex items-center justify-center space-x-1.5 px-3 py-2 bg-red-950/20 hover:bg-red-950/45 border border-red-900/30 rounded-lg text-red-400 text-xs font-semibold cursor-pointer transition"
                  >
                    <Trash2 className="w-3.5 h-3.5" />
                    <span>Delete</span>
                  </button>
                )}
                <button
                  onClick={handleSaveNote}
                  className={`flex items-center justify-center space-x-1.5 px-4 py-2 rounded-lg text-xs font-semibold cursor-pointer transition ${
                    isEditing || !selectedNote
                      ? "bg-sky-600 hover:bg-sky-500 text-white shadow-lg shadow-sky-500/20"
                      : "bg-slate-900 hover:bg-slate-800 border border-white/10 text-slate-400"
                  }`}
                >
                  <Save className="w-3.5 h-3.5" />
                  <span>Save Note</span>
                </button>
              </div>
            </div>

            {message && (
              <div className="mt-2 text-center text-xs text-sky-400 bg-sky-950/30 border border-sky-500/20 py-1.5 rounded-lg flex items-center justify-center space-x-1">
                <Sparkles className="w-3.5 h-3.5" />
                <span>{message}</span>
              </div>
            )}
          </div>
        ) : (
          <div className="flex-1 flex flex-col items-center justify-center text-slate-500 text-sm border border-dashed border-white/5 rounded-xl">
            Select a note or click + to create one
          </div>
        )}
      </div>
    </div>
  );
}

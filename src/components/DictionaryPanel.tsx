import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DictionaryEntry, ApiResponse } from "../types";
import { Plus, Trash2, FileSymlink, Sparkles, BookOpen, Search } from "lucide-react";

export default function DictionaryPanel() {
  const [words, setWords] = useState<DictionaryEntry[]>([]);
  const [newWord, setNewWord] = useState("");
  const [activeLang, setActiveLang] = useState("en");
  const [importPath, setImportPath] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [message, setMessage] = useState("");

  const languages = [
    { code: "en", name: "English" },
    { code: "yo", name: "Yoruba" },
    { code: "ha", name: "Hausa" },
    { code: "ig", name: "Igbo" },
    { code: "fr", name: "French" },
    { code: "es", name: "Spanish" },
  ];

  useEffect(() => {
    fetchWords();
  }, [activeLang]);

  const fetchWords = async () => {
    try {
      const res: ApiResponse<DictionaryEntry[]> = await invoke("list_dictionary_words", {
        language: activeLang,
      });
      if (res.success && res.data) {
        setWords(res.data);
      }
    } catch (e) {
      console.error("Failed to load dictionary: ", e);
    }
  };

  const handleAddWord = async (e: React.FormEvent) => {
    e.preventDefault();
    const word = newWord.trim();
    if (!word) return;

    try {
      const res: ApiResponse<string> = await invoke("add_dictionary_word", {
        word,
        language: activeLang,
      });
      if (res.success) {
        showMsg(`"${word}" added to personal dictionary!`);
        setNewWord("");
        fetchWords();
      } else {
        showMsg(res.error?.message || "Failed to add word.");
      }
    } catch (e) {
      console.error("Failed to add word: ", e);
    }
  };

  const handleRemoveWord = async (id: string, word: string) => {
    try {
      const res: ApiResponse<void> = await invoke("remove_dictionary_word", { id });
      if (res.success) {
        showMsg(`"${word}" removed.`);
        fetchWords();
      }
    } catch (e) {
      console.error("Failed to remove word: ", e);
    }
  };

  const handleImport = async () => {
    const path = importPath.trim();
    if (!path) {
      showMsg("Please specify a valid file path to import.");
      return;
    }
    try {
      const res: ApiResponse<number> = await invoke("import_dictionary", { filePath: path });
      if (res.success && res.data !== null) {
        showMsg(`Successfully imported ${res.data} words!`);
        setImportPath("");
        fetchWords();
      } else {
        showMsg(res.error?.message || "Import failed. Check path format.");
      }
    } catch (e) {
      console.error("Failed to import dictionary: ", e);
      showMsg("Error importing dictionary.");
    }
  };

  const handleExport = async () => {
    try {
      const res: ApiResponse<string> = await invoke("export_dictionary", { format: "json" });
      if (res.success && res.data) {
        showMsg(`Exported successfully to: ${res.data}`);
      } else {
        showMsg("Export failed.");
      }
    } catch (e) {
      console.error("Failed to export: ", e);
    }
  };

  const showMsg = (txt: string) => {
    setMessage(txt);
    setTimeout(() => setMessage(""), 4000);
  };

  const filteredWords = words.filter((entry) =>
    entry.word.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex-1 flex overflow-hidden">
      {/* Left controls */}
      <div className="w-1/2 flex flex-col border-r border-white/5 pr-6 overflow-hidden">
        {/* Title */}
        <div className="mb-4">
          <h2 className="text-base font-semibold text-slate-100 flex items-center space-x-2">
            <BookOpen className="w-4 h-4 text-sky-400" />
            <span>Personal Dictionary</span>
          </h2>
          <p className="text-[11px] text-slate-500 mt-1">
            Words added here will never be autocorrected and are prioritized in autocomplete suggestions.
          </p>
        </div>

        {/* Add Word Form */}
        <form onSubmit={handleAddWord} className="space-y-3 mb-5">
          <div className="flex flex-col space-y-1">
            <label className="text-[10px] text-slate-500 font-semibold uppercase">Word</label>
            <input
              type="text"
              value={newWord}
              onChange={(e) => setNewWord(e.target.value)}
              placeholder="e.g. HunaPay"
              className="bg-slate-900 border border-white/5 rounded-lg px-3 py-2 text-sm text-slate-200 focus:border-sky-500/50 outline-none w-full"
            />
          </div>

          <div className="flex items-center justify-between space-x-3">
            <div className="flex-1 flex flex-col space-y-1">
              <label className="text-[10px] text-slate-500 font-semibold uppercase">Language</label>
              <select
                value={activeLang}
                onChange={(e) => setActiveLang(e.target.value)}
                className="bg-slate-900 border border-white/5 rounded-lg px-3 py-2 text-sm text-slate-200 focus:border-sky-500/50 outline-none cursor-pointer"
              >
                {languages.map((l) => (
                  <option key={l.code} value={l.code}>
                    {l.name}
                  </option>
                ))}
              </select>
            </div>

            <button
              type="submit"
              className="mt-5 flex items-center justify-center space-x-1 px-4 py-2 bg-sky-600 hover:bg-sky-500 text-white rounded-lg text-xs font-semibold cursor-pointer shadow-lg shadow-sky-500/15 transition duration-150"
            >
              <Plus className="w-4 h-4" />
              <span>Add Word</span>
            </button>
          </div>
        </form>

        {/* Import / Export Card */}
        <div className="bg-slate-900/35 border border-white/5 rounded-xl p-4 space-y-4">
          <h3 className="text-xs font-semibold text-slate-300">Backup & Import</h3>

          <div className="space-y-2">
            <label className="text-[10px] text-slate-500 font-semibold uppercase block">
              Import JSON File Path
            </label>
            <div className="flex space-x-2">
              <input
                type="text"
                value={importPath}
                onChange={(e) => setImportPath(e.target.value)}
                placeholder="C:\path\to\dictionary.json"
                className="flex-1 bg-slate-950 border border-white/5 rounded-lg px-3 py-1.5 text-xs text-slate-300 outline-none focus:border-sky-500/50"
              />
              <button
                onClick={handleImport}
                className="flex items-center space-x-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 border border-white/10 rounded-lg text-xs font-semibold cursor-pointer transition"
              >
                <FileSymlink className="w-3.5 h-3.5" />
                <span>Import</span>
              </button>
            </div>
          </div>

          <div className="pt-2 border-t border-white/5 flex items-center justify-between">
            <span className="text-[10px] text-slate-500 font-semibold uppercase">Export Backup</span>
            <button
              onClick={handleExport}
              className="flex items-center space-x-1 px-3 py-1.5 bg-slate-850 hover:bg-slate-800 text-sky-400 border border-sky-500/20 rounded-lg text-xs font-semibold cursor-pointer transition"
            >
              <span>Export (JSON)</span>
            </button>
          </div>
        </div>

        {message && (
          <div className="mt-4 text-center text-xs text-sky-400 bg-sky-950/30 border border-sky-500/20 py-2 rounded-lg flex items-center justify-center space-x-1.5 animate-pulse">
            <Sparkles className="w-4 h-4" />
            <span>{message}</span>
          </div>
        )}
      </div>

      {/* Right list column */}
      <div className="w-1/2 flex flex-col pl-6 overflow-hidden">
        {/* Search list */}
        <div className="relative flex items-center bg-slate-900/60 border border-white/5 rounded-lg px-3 py-1.5 mb-3 focus-within:border-sky-500/50 transition flex-shrink-0">
          <Search className="w-4 h-4 text-slate-400 mr-2" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search custom words..."
            className="bg-transparent border-none outline-none text-slate-100 placeholder-slate-500 text-sm w-full"
          />
        </div>

        {/* Word Grid / List */}
        <div className="flex-1 overflow-y-auto space-y-1.5 pr-1">
          {filteredWords.length > 0 ? (
            filteredWords.map((entry) => (
              <div
                key={entry.id}
                className="flex items-center justify-between px-4 py-2 bg-slate-900/40 hover:bg-slate-900/60 border border-white/5 rounded-lg transition"
              >
                <div className="flex flex-col">
                  <span className="text-slate-100 text-sm font-semibold">{entry.word}</span>
                  <span className="text-[9px] text-slate-500 font-bold uppercase mt-0.5">
                    {entry.language_code} • {entry.source}
                  </span>
                </div>
                <button
                  onClick={() => handleRemoveWord(entry.id, entry.word)}
                  className="p-1.5 hover:bg-red-950/20 hover:text-red-400 text-slate-500 rounded-lg transition cursor-pointer"
                  title="Remove from dictionary"
                >
                  <Trash2 className="w-3.5 h-3.5" />
                </button>
              </div>
            ))
          ) : (
            <div className="text-center text-slate-500 text-xs py-12">
              No custom words found.
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

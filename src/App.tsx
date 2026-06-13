import { useState, useEffect } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import ClipboardPanel from "./components/ClipboardPanel";
import NotesPanel from "./components/NotesPanel";
import DictionaryPanel from "./components/DictionaryPanel";
import RulesPanel from "./components/RulesPanel";
import SettingsPanel from "./components/SettingsPanel";
import QuickPaste from "./components/QuickPaste";
import AutocompletePopup from "./components/AutocompletePopup";
import { Clipboard, FileText, BookOpen, Sparkles, Settings as SettingsIcon } from "lucide-react";
import boarddyLogo from "./assets/boarddy-logo.png";

function App() {
  const [label, setLabel] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<"clipboard" | "notes" | "dictionary" | "rules" | "settings">("clipboard");

  useEffect(() => {
    // Determine which view/window to render based on Tauri window label
    try {
      const currentLabel = getCurrentWebviewWindow().label;
      setLabel(currentLabel);
    } catch (e) {
      console.error("Not running in Tauri environment, defaulting to main dashboard:", e);
      setLabel("main");
    }
  }, []);

  if (label === null) {
    return (
      <div className="w-screen h-screen flex flex-col items-center justify-center bg-slate-950 text-slate-400 text-xs gap-3">
        <img src={boarddyLogo} className="w-12 h-12 object-contain animate-pulse" alt="Boarddy Logo" />
        <span className="font-medium tracking-wide">Initializing Boarddy...</span>
      </div>
    );
  }

  if (label === "quick_paste") {
    return <QuickPaste />;
  }

  if (label === "autocomplete") {
    return <AutocompletePopup />;
  }

  return (
    <div className="w-screen h-screen flex bg-slate-950 text-slate-100 overflow-hidden font-sans">
      {/* Sidebar Navigation */}
      <aside className="w-64 bg-slate-900/60 border-r border-white/5 flex flex-col p-4 shrink-0">
        {/* Brand Header */}
        <div className="flex items-center space-x-3 mb-8 px-2">
          <img src={boarddyLogo} className="w-7 h-7 object-contain shadow-md" alt="Boarddy Logo" />
          <div>
            <h1 className="text-sm font-bold tracking-wide text-slate-100">Boarddy</h1>
            <p className="text-[10px] text-sky-400 font-medium">Input & Memory Layer</p>
          </div>
        </div>

        {/* Navigation Items */}
        <nav className="flex-1 space-y-1.5">
          <button
            onClick={() => setActiveTab("clipboard")}
            className={`w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg text-xs font-semibold transition duration-150 cursor-pointer ${
              activeTab === "clipboard"
                ? "bg-sky-600/15 text-sky-400 border border-sky-500/20"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5 border border-transparent"
            }`}
          >
            <Clipboard className="w-4 h-4" />
            <span>Clipboard History</span>
          </button>

          <button
            onClick={() => setActiveTab("notes")}
            className={`w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg text-xs font-semibold transition duration-150 cursor-pointer ${
              activeTab === "notes"
                ? "bg-sky-600/15 text-sky-400 border border-sky-500/20"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5 border border-transparent"
            }`}
          >
            <FileText className="w-4 h-4" />
            <span>Memory Notes</span>
          </button>

          <button
            onClick={() => setActiveTab("dictionary")}
            className={`w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg text-xs font-semibold transition duration-150 cursor-pointer ${
              activeTab === "dictionary"
                ? "bg-sky-600/15 text-sky-400 border border-sky-500/20"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5 border border-transparent"
            }`}
          >
            <BookOpen className="w-4 h-4" />
            <span>Personal Dictionary</span>
          </button>

          <button
            onClick={() => setActiveTab("rules")}
            className={`w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg text-xs font-semibold transition duration-150 cursor-pointer ${
              activeTab === "rules"
                ? "bg-sky-600/15 text-sky-400 border border-sky-500/20"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5 border border-transparent"
            }`}
          >
            <Sparkles className="w-4 h-4" />
            <span>Autocorrect Rules</span>
          </button>

          <button
            onClick={() => setActiveTab("settings")}
            className={`w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg text-xs font-semibold transition duration-150 cursor-pointer ${
              activeTab === "settings"
                ? "bg-sky-600/15 text-sky-400 border border-sky-500/20"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5 border border-transparent"
            }`}
          >
            <SettingsIcon className="w-4 h-4" />
            <span>System Settings</span>
          </button>
        </nav>

        {/* Sidebar Footer */}
        <div className="border-t border-white/5 pt-4 px-2 mt-auto">
          <div className="flex items-center justify-between text-[9px] text-slate-500">
            <span>Status: Running</span>
            <span>V1.0.0</span>
          </div>
        </div>
      </aside>

      {/* Main Content Area */}
      <main className="flex-1 bg-slate-950 flex flex-col p-6 overflow-hidden">
        {activeTab === "clipboard" && <ClipboardPanel />}
        {activeTab === "notes" && <NotesPanel />}
        {activeTab === "dictionary" && <DictionaryPanel />}
        {activeTab === "rules" && <RulesPanel />}
        {activeTab === "settings" && <SettingsPanel />}
      </main>
    </div>
  );
}

export default App;

import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { Sparkles } from "lucide-react";
import { Suggestion } from "../types";

export default function AutocompletePopup() {
  const [suggestions, setSuggestions] = useState<Suggestion[]>([]);

  useEffect(() => {
    // Listen to suggestions updates emitted from Rust
    const unlistenPromise = listen<Suggestion[]>("suggestions:update", (event) => {
      setSuggestions(event.payload);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  if (suggestions.length === 0) {
    return null;
  }

  return (
    <div className="glass w-screen h-screen rounded-lg flex flex-col p-2.5 shadow-xl border border-white/10 select-none overflow-hidden justify-between bg-slate-950/90 backdrop-blur-md">
      <div className="flex items-center justify-between mb-1.5 pb-1 border-b border-white/5">
        <div className="flex items-center space-x-1.5">
          <Sparkles className="w-3.5 h-3.5 text-sky-400 animate-pulse" />
          <span className="text-[10px] uppercase tracking-wider text-slate-400 font-bold">
            Smart Suggestions
          </span>
        </div>
        <span className="text-[9px] text-slate-500 font-mono">PKS Active</span>
      </div>

      <div className="flex-1 space-y-1 overflow-y-auto pr-0.5 custom-scrollbar">
        {suggestions.map((sug, idx) => {
          const isTop = idx === 0;
          return (
            <div
              key={idx}
              className={`flex items-center justify-between px-2.5 py-1.5 rounded-md text-xs transition duration-150 border ${
                isTop
                  ? "bg-gradient-to-r from-sky-600/30 to-sky-600/15 border-sky-500/30 text-white shadow-md shadow-sky-500/5"
                  : "bg-transparent border-transparent hover:bg-white/5 text-slate-300"
              }`}
            >
              <div className="flex items-center space-x-2.5 truncate">
                {/* Shortcut Badge */}
                <span className={`text-[9px] font-bold px-1.5 py-0.5 rounded font-mono ${
                  isTop 
                    ? "bg-sky-500 text-sky-950" 
                    : "bg-slate-800 text-slate-400 border border-white/5"
                }`}>
                  {sug.shortcut.toUpperCase()}
                </span>
                
                {/* Word */}
                <span className={`truncate ${isTop ? "font-semibold text-slate-100" : "font-medium text-slate-300"}`}>
                  {sug.word}
                </span>
              </div>

              {/* Confidence & Action Hints */}
              <div className="flex items-center space-x-2 shrink-0">
                <span className={`text-[9px] font-mono ${isTop ? "text-sky-300 font-bold" : "text-slate-500"}`}>
                  {sug.confidence}%
                </span>
                {isTop && (
                  <div className="flex items-center space-x-1">
                    <span className="text-[8px] text-sky-300 font-bold bg-sky-950/50 border border-sky-500/20 px-1 rounded">
                      TAB
                    </span>
                    <span className="text-[8px] text-sky-300 font-bold bg-sky-950/50 border border-sky-500/20 px-1 rounded">
                      →
                    </span>
                  </div>
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

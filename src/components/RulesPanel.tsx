import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AutocorrectRule, ApiResponse } from "../types";
import { Plus, Trash2, Sparkles, Search, Languages } from "lucide-react";

export default function RulesPanel() {
  const [rules, setRules] = useState<AutocorrectRule[]>([]);
  const [trigger, setTrigger] = useState("");
  const [replacement, setReplacement] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [message, setMessage] = useState("");

  useEffect(() => {
    fetchRules();
  }, []);

  const fetchRules = async () => {
    try {
      const res: ApiResponse<AutocorrectRule[]> = await invoke("get_autocorrect_rules");
      if (res.success && res.data) {
        setRules(res.data);
      }
    } catch (e) {
      console.error("Failed to load rules: ", e);
    }
  };

  const handleAddRule = async (e: React.FormEvent) => {
    e.preventDefault();
    const tWord = trigger.trim().toLowerCase();
    const rWord = replacement.trim();

    if (!tWord || !rWord) {
      showMsg("Both trigger and replacement words are required.");
      return;
    }

    try {
      const res: ApiResponse<void> = await invoke("add_correction_rule", {
        trigger: tWord,
        replacement: rWord,
      });

      if (res.success) {
        showMsg(`Rule "${tWord}" → "${rWord}" saved!`);
        setTrigger("");
        setReplacement("");
        fetchRules();
      } else {
        showMsg(res.error?.message || "Failed to add rule.");
      }
    } catch (e) {
      console.error("Failed to add rule: ", e);
    }
  };

  const handleRemoveRule = async (ruleId: string, ruleTrigger: string) => {
    try {
      const res: ApiResponse<void> = await invoke("remove_correction_rule", { ruleId });
      if (res.success) {
        showMsg(`Rule for "${ruleTrigger}" deleted.`);
        fetchRules();
      } else {
        showMsg(res.error?.message || "Failed to remove rule.");
      }
    } catch (e) {
      console.error("Failed to remove rule: ", e);
    }
  };

  const showMsg = (txt: string) => {
    setMessage(txt);
    setTimeout(() => setMessage(""), 4000);
  };

  const filteredRules = rules.filter(
    (rule) =>
      rule.trigger_word.toLowerCase().includes(searchQuery.toLowerCase()) ||
      rule.replacement_word.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex-1 flex overflow-hidden">
      {/* Left controls */}
      <div className="w-1/2 flex flex-col border-r border-white/5 pr-6 overflow-hidden">
        {/* Title */}
        <div className="mb-4">
          <h2 className="text-base font-semibold text-slate-100 flex items-center space-x-2">
            <Sparkles className="w-4 h-4 text-emerald-400" />
            <span>Autocorrect Rules</span>
          </h2>
          <p className="text-[11px] text-slate-500 mt-1">
            Define custom expansion rules or typos corrections. Whenever you type the trigger word followed by a space, it will be replaced instantly.
          </p>
        </div>

        {/* Add Rule Form */}
        <form onSubmit={handleAddRule} className="space-y-4 mb-5">
          <div className="flex flex-col space-y-1">
            <label className="text-[10px] text-slate-500 font-semibold uppercase">Trigger (Typo or Shortcut)</label>
            <input
              type="text"
              value={trigger}
              onChange={(e) => setTrigger(e.target.value)}
              placeholder="e.g. teh"
              className="bg-slate-900 border border-white/5 rounded-lg px-3 py-2 text-sm text-slate-200 focus:border-emerald-500/50 outline-none w-full"
            />
          </div>

          <div className="flex flex-col space-y-1">
            <label className="text-[10px] text-slate-500 font-semibold uppercase">Replacement (Corrected text)</label>
            <input
              type="text"
              value={replacement}
              onChange={(e) => setReplacement(e.target.value)}
              placeholder="e.g. the"
              className="bg-slate-900 border border-white/5 rounded-lg px-3 py-2 text-sm text-slate-200 focus:border-emerald-500/50 outline-none w-full"
            />
          </div>

          <div className="flex justify-end pt-2">
            <button
              type="submit"
              className="flex items-center justify-center space-x-1.5 px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-xs font-semibold cursor-pointer shadow-lg shadow-emerald-500/15 transition duration-150"
            >
              <Plus className="w-4 h-4" />
              <span>Add Rule</span>
            </button>
          </div>
        </form>

        {/* Info Box */}
        <div className="bg-slate-900/35 border border-white/5 rounded-xl p-4 flex items-start space-x-3 mt-auto mb-4">
          <Languages className="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" />
          <div className="space-y-1">
            <h4 className="text-xs font-semibold text-slate-300">Quick Tips</h4>
            <p className="text-[11px] text-slate-500 leading-relaxed">
              Press <kbd className="px-1.5 py-0.5 bg-slate-950 border border-white/10 rounded text-slate-400 font-mono text-[9px]">Ctrl + Z</kbd> immediately after an autocorrect happens to undo the replacement.
            </p>
          </div>
        </div>
      </div>

      {/* Right list */}
      <div className="w-1/2 flex flex-col pl-6 overflow-hidden">
        {/* Search */}
        <div className="relative mb-4">
          <Search className="absolute left-3 top-2.5 w-3.5 h-3.5 text-slate-500" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search rules..."
            className="w-full bg-slate-900/50 border border-white/5 rounded-lg pl-9 pr-4 py-2 text-xs text-slate-300 outline-none focus:border-emerald-500/50"
          />
        </div>

        {message && (
          <div className="mb-3 px-3 py-1.5 bg-emerald-500/10 border border-emerald-500/20 rounded-lg text-[11px] text-emerald-400 text-center animate-fade-in">
            {message}
          </div>
        )}

        {/* List scroll */}
        <div className="flex-1 overflow-y-auto pr-2 space-y-2 custom-scrollbar">
          {filteredRules.length === 0 ? (
            <div className="text-center py-8 text-slate-600 text-xs">
              No autocorrect rules found.
            </div>
          ) : (
            filteredRules.map((rule) => (
              <div
                key={rule.id}
                className="group flex items-center justify-between p-3 bg-slate-900/40 hover:bg-slate-900/70 border border-white/5 rounded-lg transition duration-150"
              >
                <div className="flex items-center space-x-3 overflow-hidden">
                  <span className="text-xs font-mono font-bold text-slate-200 bg-slate-950 px-2 py-1 rounded border border-white/5 truncate max-w-[120px]">
                    {rule.trigger_word}
                  </span>
                  <span className="text-xs text-slate-500">→</span>
                  <span className="text-xs text-slate-300 truncate max-w-[150px]">
                    {rule.replacement_word}
                  </span>
                </div>

                <button
                  onClick={() => handleRemoveRule(rule.id, rule.trigger_word)}
                  className="p-1.5 text-slate-500 hover:text-rose-400 hover:bg-rose-500/10 rounded transition duration-150 cursor-pointer opacity-0 group-hover:opacity-100 focus:opacity-100"
                  title="Delete rule"
                >
                  <Trash2 className="w-3.5 h-3.5" />
                </button>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}

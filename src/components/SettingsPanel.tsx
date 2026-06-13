import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Language, ApiResponse } from "../types";
import { Settings, Shield, Keyboard, Sliders, ToggleLeft, ToggleRight, Sparkles, AlertTriangle } from "lucide-react";

export default function SettingsPanel() {
  const [settings, setSettings] = useState<Record<string, string>>({});
  const [languages, setLanguages] = useState<Language[]>([]);
  const [recordingAction, setRecordingAction] = useState<string | null>(null);
  const [recordedKeys, setRecordedKeys] = useState<string[]>([]);
  const [message, setMessage] = useState("");
  const [conflictWarning, setConflictWarning] = useState<string | null>(null);

  useEffect(() => {
    fetchSettings();
    fetchLanguages();
  }, []);

  const fetchSettings = async () => {
    try {
      const res: ApiResponse<Record<string, string>> = await invoke("get_settings");
      if (res.success && res.data) {
        setSettings(res.data);
      }
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
  };

  const fetchLanguages = async () => {
    try {
      const res: ApiResponse<Language[]> = await invoke("get_languages");
      if (res.success && res.data) {
        setLanguages(res.data);
      }
    } catch (e) {
      console.error("Failed to load languages:", e);
    }
  };

  const handleUpdateSetting = async (key: string, value: string) => {
    try {
      const res: ApiResponse<void> = await invoke("update_setting", { key, value });
      if (res.success) {
        setSettings((prev) => ({ ...prev, [key]: value }));
        showMsg("Setting updated successfully.");
      }
    } catch (e) {
      console.error(`Failed to update setting ${key}:`, e);
    }
  };

  const handleToggleLanguage = async (lang: Language) => {
    try {
      const command = lang.is_enabled ? "disable_language" : "enable_language";
      const res: ApiResponse<void> = await invoke(command, { code: lang.code });
      if (res.success) {
        fetchLanguages();
        showMsg(`${lang.name} language updated.`);
      }
    } catch (e) {
      console.error("Failed to toggle language:", e);
    }
  };

  const showMsg = (txt: string) => {
    setMessage(txt);
    setTimeout(() => setMessage(""), 3000);
  };

  // Hotkey Recorder
  useEffect(() => {
    if (!recordingAction) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const parts: string[] = [];
      if (e.ctrlKey) parts.push("Ctrl");
      if (e.shiftKey) parts.push("Shift");
      if (e.altKey) parts.push("Alt");

      // Avoid adding modifier keys alone as the main key
      if (e.key !== "Control" && e.key !== "Shift" && e.key !== "Alt") {
        let keyName = e.key;
        if (keyName === " ") keyName = "Space";
        if (keyName.length === 1) keyName = keyName.toUpperCase();
        parts.push(keyName);
      }

      setRecordedKeys(parts);
    };

    const handleKeyUp = async (e: KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();

      if (recordedKeys.length > 0) {
        const shortcutString = recordedKeys.join("+");
        setRecordingAction(null);
        setConflictWarning(null);

        // Conflict detection (if enabled)
        if (settings.gesture_conflict_detection !== "false") {
          try {
            const conflictRes: ApiResponse<string | null> = await invoke("check_shortcut_conflict", {
              action: recordingAction,
              shortcut: shortcutString,
            });
            if (conflictRes.success && conflictRes.data) {
              setConflictWarning(`Conflict: "${shortcutString}" is already assigned to "${conflictRes.data}"`);
              setTimeout(() => setConflictWarning(null), 6000);
            }
          } catch (e) {
            console.error("Conflict check failed:", e);
          }
        }

        try {
          const res: ApiResponse<void> = await invoke("update_shortcut", {
            action: recordingAction,
            shortcut: shortcutString,
          });
          if (res.success) {
            showMsg(`Shortcut bound to ${shortcutString}`);
            fetchSettings();
          } else {
            showMsg(res.error?.message || "Failed to update shortcut.");
          }
        } catch (e) {
          console.error("Failed to save shortcut:", e);
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown, true);
    window.addEventListener("keyup", handleKeyUp, true);

    return () => {
      window.removeEventListener("keydown", handleKeyDown, true);
      window.removeEventListener("keyup", handleKeyUp, true);
    };
  }, [recordingAction, recordedKeys, settings.gesture_conflict_detection]);

  const startRecording = (action: string) => {
    setRecordedKeys([]);
    setRecordingAction(action);
  };

  const gestureDefinitions = [
    { key: "gesture_word_delete_backspace", label: "Delete Previous Word", desc: "Backspace + Left Arrow deletes word backwards.", default: "Backspace+Left" },
    { key: "gesture_word_delete_delete", label: "Delete Next Word", desc: "Delete + Right Arrow deletes word forwards.", default: "Delete+Right" },
    { key: "gesture_line_delete_up", label: "Delete Previous Line", desc: "Alt + Backspace + Up Arrow deletes previous line.", default: "Alt+Backspace+Up" },
    { key: "gesture_line_delete_down", label: "Delete Next Line", desc: "Alt + Delete + Down Arrow deletes next line.", default: "Alt+Delete+Down" },
    { key: "gesture_cursor_move_char", label: "Move Cursor (Char)", desc: "Space + Left / Right Arrow moves cursor character-by-character.", default: "Space+Left/Right" },
    { key: "gesture_cursor_move_word", label: "Move Cursor (Word)", desc: "Space + Up / Down Arrow jumps word-by-word.", default: "Space+Up/Down" },
    { key: "gesture_select_word", label: "Select Previous Word", desc: "Space + W selects previous word.", default: "Space+W" },
    { key: "gesture_select_line", label: "Select Current Line", desc: "Space + L selects entire line.", default: "Space+L" },
    { key: "gesture_select_paragraph", label: "Select Paragraph", desc: "Space + P selects paragraph.", default: "Space+P" },
  ];

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {/* Title */}
      <div className="mb-6">
        <h2 className="text-base font-semibold text-slate-100 flex items-center space-x-2">
          <Settings className="w-4 h-4 text-violet-400" />
          <span>System Settings</span>
        </h2>
        <p className="text-[11px] text-slate-500 mt-1">
          Configure Boarddy autocorrect, autocomplete, system behaviors, language support, gestures, and shortcut mappings.
        </p>
      </div>

      {message && (
        <div className="mb-4 px-3 py-1.5 bg-violet-500/10 border border-violet-500/20 rounded-lg text-[11px] text-violet-400 text-center animate-fade-in w-full max-w-md mx-auto">
          {message}
        </div>
      )}

      {conflictWarning && (
        <div className="mb-4 px-3 py-1.5 bg-amber-500/10 border border-amber-500/20 rounded-lg text-[11px] text-amber-400 flex items-center justify-center space-x-1.5 animate-bounce w-full max-w-md mx-auto">
          <AlertTriangle className="w-3.5 h-3.5" />
          <span>{conflictWarning}</span>
        </div>
      )}

      {/* Grid Settings */}
      <div className="flex-1 overflow-y-auto pr-2 space-y-6 custom-scrollbar pb-6">
        {/* Section: General Controls */}
        <div className="bg-slate-900/30 border border-white/5 rounded-xl p-5 space-y-4">
          <h3 className="text-xs font-semibold text-slate-300 flex items-center space-x-2 border-b border-white/5 pb-2">
            <Sliders className="w-3.5 h-3.5 text-violet-400" />
            <span>Core Behaviors</span>
          </h3>

          <div className="space-y-4">
            {/* Autocorrect Switch */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">System Autocorrect</h4>
                <p className="text-[10px] text-slate-500">Automatically replace misspelled words system-wide.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "autocorrect_enabled",
                    settings.autocorrect_enabled === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.autocorrect_enabled === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Autocomplete Switch */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Smart Autocomplete</h4>
                <p className="text-[10px] text-slate-500">Show a floating word suggestion overlay as you type.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "autocomplete_enabled",
                    settings.autocomplete_enabled === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.autocomplete_enabled === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Launch on Startup */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Launch on Startup</h4>
                <p className="text-[10px] text-slate-500">Launch Boarddy silently in the tray when Windows boots.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "launch_on_startup",
                    settings.launch_on_startup === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.launch_on_startup === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Clipboard Limit */}
            <div className="flex items-center justify-between pt-2 border-t border-white/5">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Clipboard Storage Limit</h4>
                <p className="text-[10px] text-slate-500">Maximum number of entries to keep in clipboard history.</p>
              </div>
              <div className="flex items-center space-x-2">
                <input
                  type="number"
                  min="10"
                  max="1000"
                  value={settings.clipboard_limit || "100"}
                  onChange={(e) => handleUpdateSetting("clipboard_limit", e.target.value)}
                  className="bg-slate-950 border border-white/10 rounded-lg px-2 py-1 text-xs text-slate-300 outline-none w-16 text-center focus:border-violet-500/50"
                />
                <span className="text-[10px] text-slate-500">entries</span>
              </div>
            </div>
          </div>
        </div>

        {/* Section: Advanced Input */}
        <div className="bg-slate-900/30 border border-white/5 rounded-xl p-5 space-y-4">
          <h3 className="text-xs font-semibold text-slate-300 flex items-center space-x-2 border-b border-white/5 pb-2">
            <Sparkles className="w-3.5 h-3.5 text-violet-400" />
            <span>Advanced Input & Autocomplete Preferences</span>
          </h3>

          <div className="space-y-4">
            {/* Number Selection */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Enable Number Selection</h4>
                <p className="text-[10px] text-slate-500">Type number keys (1-9) to choose autocomplete suggestions instantly.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "number_selection",
                    settings.number_selection === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.number_selection === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Letter Selection */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Enable Letter Selection (Experimental)</h4>
                <p className="text-[10px] text-slate-500">Assigns unique activation letters to suggestions. Overrides number keys.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "letter_selection",
                    settings.letter_selection === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.letter_selection === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Right Arrow Acceptance */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Enable Right Arrow Acceptance</h4>
                <p className="text-[10px] text-slate-500">Accept the top suggestion using either Tab or Right Arrow.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "right_arrow_acceptance",
                    settings.right_arrow_acceptance === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.right_arrow_acceptance === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Adaptive Ranking */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Enable Adaptive Ranking</h4>
                <p className="text-[10px] text-slate-500">Personalize and reorder suggestions locally based on your typing habits.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "adaptive_ranking",
                    settings.adaptive_ranking === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.adaptive_ranking === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Double Shift Trigger */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Enable Double Shift Clipboard</h4>
                <p className="text-[10px] text-slate-500">Press the Shift key twice rapidly to summon the Quick Paste overlay.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "double_shift_clipboard",
                    settings.double_shift_clipboard === "true" ? "false" : "true"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.double_shift_clipboard === "true" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>

            {/* Gesture Conflict Detection */}
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-xs font-semibold text-slate-200">Gesture Conflict Detection</h4>
                <p className="text-[10px] text-slate-500">Warn if customized gestures conflict with other system bindings.</p>
              </div>
              <button
                onClick={() =>
                  handleUpdateSetting(
                    "gesture_conflict_detection",
                    settings.gesture_conflict_detection === "false" ? "true" : "false"
                  )
                }
                className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
              >
                {settings.gesture_conflict_detection !== "false" ? (
                  <ToggleRight className="w-8 h-8 text-sky-500" />
                ) : (
                  <ToggleLeft className="w-8 h-8 text-slate-600" />
                )}
              </button>
            </div>
          </div>
        </div>

        {/* Section: Keyboard Gesture Engine */}
        <div className="bg-slate-900/30 border border-white/5 rounded-xl p-5 space-y-4">
          <div className="flex items-center justify-between border-b border-white/5 pb-2">
            <h3 className="text-xs font-semibold text-slate-300 flex items-center space-x-2">
              <Keyboard className="w-3.5 h-3.5 text-violet-400" />
              <span>Keyboard Gesture Customization</span>
            </h3>

            <button
              onClick={() =>
                handleUpdateSetting(
                  "keyboard_gestures_enabled",
                  settings.keyboard_gestures_enabled === "true" ? "false" : "true"
                )
              }
              className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
            >
              {settings.keyboard_gestures_enabled === "true" ? (
                <ToggleRight className="w-8 h-8 text-sky-500" />
              ) : (
                <ToggleLeft className="w-8 h-8 text-slate-600" />
              )}
            </button>
          </div>

          <p className="text-[10px] text-slate-500">
            Allow advanced text editing and cursor actions directly from your keyboard without shifting your hands to the mouse.
          </p>

          <div className={`space-y-2 transition duration-200 ${settings.keyboard_gestures_enabled === "false" ? "opacity-40 pointer-events-none" : ""}`}>
            <div className="border border-white/5 rounded-lg overflow-hidden bg-slate-950/20">
              <table className="w-full text-left border-collapse text-[11px]">
                <thead>
                  <tr className="border-b border-white/5 bg-slate-900/40 text-slate-400 font-semibold">
                    <th className="p-3">Gesture / Action</th>
                    <th className="p-3">Description</th>
                    <th className="p-3 text-right">Trigger Shortcut</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-white/5">
                  {gestureDefinitions.map((gesture) => {
                    const isRec = recordingAction === gesture.key;
                    return (
                      <tr key={gesture.key} className="hover:bg-white/2 transition duration-100">
                        <td className="p-3 font-semibold text-slate-200">{gesture.label}</td>
                        <td className="p-3 text-slate-500">{gesture.desc}</td>
                        <td className="p-3 text-right">
                          <div className="flex items-center justify-end space-x-2">
                            <button
                              onClick={() => startRecording(gesture.key)}
                              disabled={recordingAction !== null}
                              className={`px-2 py-1 rounded text-[10px] font-semibold border transition cursor-pointer ${
                                isRec
                                  ? "bg-amber-500/10 border-amber-500/30 text-amber-400 animate-pulse"
                                  : "bg-slate-800 hover:bg-slate-700 border-white/10 text-slate-300 disabled:opacity-40"
                              }`}
                            >
                              {isRec ? "Record..." : "Rebind"}
                            </button>
                            <kbd className="px-2 py-1 bg-slate-950 border border-white/10 rounded text-slate-200 font-mono text-[10px] font-semibold min-w-[80px] text-center shadow-inner">
                              {isRec
                                ? recordedKeys.join(" + ") || "Press..."
                                : settings[gesture.key] || gesture.default}
                            </kbd>
                          </div>
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          </div>
        </div>

        {/* Section: Quick Paste Hotkey */}
        <div className="bg-slate-900/30 border border-white/5 rounded-xl p-5 space-y-4">
          <h3 className="text-xs font-semibold text-slate-300 flex items-center space-x-2 border-b border-white/5 pb-2">
            <Keyboard className="w-3.5 h-3.5 text-violet-400" />
            <span>Quick Paste Shortcut</span>
          </h3>

          <div className="flex items-center justify-between">
            <div>
              <h4 className="text-xs font-semibold text-slate-200">Global Overlay Hotkey</h4>
              <p className="text-[10px] text-slate-500">Global system-wide shortcut to summon the Boarddy paste list.</p>
            </div>
            <div className="flex items-center space-x-2">
              <button
                onClick={() => startRecording("quick_paste")}
                disabled={recordingAction !== null}
                className={`px-3 py-1.5 rounded-lg text-xs font-semibold border transition cursor-pointer ${
                  recordingAction === "quick_paste"
                    ? "bg-amber-500/10 border-amber-500/30 text-amber-400 animate-pulse"
                    : "bg-slate-800 hover:bg-slate-700 border-white/10 text-slate-300 disabled:opacity-40"
                }`}
              >
                {recordingAction === "quick_paste" ? "Press keys..." : "Record New"}
              </button>
              <kbd className="px-2.5 py-1.5 bg-slate-950 border border-white/10 rounded-lg text-slate-200 font-mono text-xs font-semibold min-w-[100px] text-center shadow-inner">
                {recordingAction === "quick_paste"
                  ? recordedKeys.join(" + ") || "Press keys..."
                  : settings.quick_paste_shortcut || "Ctrl+Shift+V"}
              </kbd>
            </div>
          </div>
        </div>

        {/* Section: Languages */}
        <div className="bg-slate-900/30 border border-white/5 rounded-xl p-5 space-y-4">
          <h3 className="text-xs font-semibold text-slate-300 flex items-center space-x-2 border-b border-white/5 pb-2">
            <Shield className="w-3.5 h-3.5 text-violet-400" />
            <span>Enabled Languages</span>
          </h3>
          <p className="text-[10px] text-slate-500">
            Select the languages Boarddy will monitor for autocorrect matching and dictionary autocompletes.
          </p>

          <div className="grid grid-cols-2 gap-3 pt-2">
            {languages.map((lang) => (
              <div
                key={lang.code}
                className="flex items-center justify-between p-3 bg-slate-950/40 border border-white/5 rounded-lg hover:bg-slate-950/70 transition duration-150"
              >
                <div className="flex items-center space-x-2">
                  <span className="text-xs font-bold text-slate-400 uppercase bg-slate-900 px-1.5 py-0.5 rounded border border-white/5">
                    {lang.code}
                  </span>
                  <span className="text-xs text-slate-200 font-semibold">{lang.name}</span>
                </div>
                <button
                  onClick={() => handleToggleLanguage(lang)}
                  className="text-slate-400 hover:text-white transition duration-150 cursor-pointer"
                >
                  {lang.is_enabled ? (
                    <ToggleRight className="w-7 h-7 text-emerald-500" />
                  ) : (
                    <ToggleLeft className="w-7 h-7 text-slate-600" />
                  )}
                </button>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

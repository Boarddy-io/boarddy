#![cfg(target_os = "windows")]

use std::mem;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_V,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, GetForegroundWindow, GetGUIThreadInfo, GetWindowThreadProcessId,
    GetWindowTextW, GUITHREADINFO,
};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

// Retrieve caret position or fallback to mouse cursor
pub fn get_caret_position() -> (i32, i32) {
    let mut pt = POINT::default();
    let hwnd = unsafe { GetForegroundWindow() };
    let thread_id = unsafe { GetWindowThreadProcessId(hwnd, None) };
    
    let mut gui = GUITHREADINFO::default();
    gui.cbSize = mem::size_of::<GUITHREADINFO>() as u32;
    
    let ok = unsafe { GetGUIThreadInfo(thread_id, &mut gui) };
    if ok.is_ok() && gui.hwndCaret.0 != 0 {
        let mut caret_pt = POINT {
            x: gui.rcCaret.left,
            y: gui.rcCaret.bottom,
        };
        unsafe {
            ClientToScreen(gui.hwndCaret, &mut caret_pt);
        }
        // If caret is visible and has valid coordinates
        if caret_pt.x != 0 || caret_pt.y != 0 {
            return (caret_pt.x, caret_pt.y);
        }
    }
    
    // Fallback: Mouse cursor coordinates
    unsafe {
        let _ = GetCursorPos(&mut pt);
    }
    (pt.x, pt.y)
}

// Retrieve active process executable name (e.g. chrome.exe)
pub fn get_process_name(pid: u32) -> String {
    let mut process_name = String::from("unknown");
    if let Ok(snapshot) = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
        let mut entry = PROCESSENTRY32W::default();
        entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;
        if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
            loop {
                if entry.th32ProcessID == pid {
                    let name_len = entry
                        .szExeFile
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry.szExeFile.len());
                    process_name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);
                    break;
                }
                if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                    break;
                }
            }
        }
        unsafe {
            let _ = windows::Win32::Foundation::CloseHandle(snapshot);
        }
    }
    process_name
}

pub struct ActiveWindowDetails {
    pub process_name: String,
    pub title: String,
}

// Retrieve foreground window details
pub fn get_active_window_details() -> ActiveWindowDetails {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0 == 0 {
        return ActiveWindowDetails {
            process_name: "unknown".to_string(),
            title: "unknown".to_string(),
        };
    }
    
    let mut pid = 0;
    let _thread_id = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    let process_name = get_process_name(pid);
    
    let mut text: [u16; 512] = [0; 512];
    let len = unsafe { GetWindowTextW(hwnd, &mut text) };
    let title = String::from_utf16_lossy(&text[..len as usize]);
    
    ActiveWindowDetails {
        process_name,
        title,
    }
}

// Simulate sending backspace keys to erase characters
pub fn send_backspaces(count: usize) {
    for _ in 0..count {
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];
        unsafe {
            SendInput(&inputs, mem::size_of::<INPUT>() as i32);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

// Simulate typing a string using Unicode inputs
pub fn send_string(text: &str) {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    for &ch in &utf16 {
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(KEYEVENTF_UNICODE.0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(KEYEVENTF_UNICODE.0 | KEYEVENTF_KEYUP.0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];
        unsafe {
            SendInput(&inputs, mem::size_of::<INPUT>() as i32);
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}

// Simulate pressing Ctrl + V to paste
pub fn send_paste_shortcut() {
    let inputs = [
        // Ctrl down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // V down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // V up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Ctrl up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    unsafe {
        SendInput(&inputs, mem::size_of::<INPUT>() as i32);
    }
}

pub fn send_key_combo(vk: u32, ctrl: bool, shift: bool, alt: bool) {
    use windows::Win32::UI::Input::KeyboardAndMouse::{VK_CONTROL, VK_SHIFT, VK_MENU};
    let mut inputs = Vec::new();
    
    // Key downs
    if ctrl {
        inputs.push(make_key_input(VK_CONTROL.0 as u16, false));
    }
    if shift {
        inputs.push(make_key_input(VK_SHIFT.0 as u16, false));
    }
    if alt {
        inputs.push(make_key_input(VK_MENU.0 as u16, false));
    }
    
    inputs.push(make_key_input(vk as u16, false));
    inputs.push(make_key_input(vk as u16, true)); // Up
    
    // Key ups
    if alt {
        inputs.push(make_key_input(VK_MENU.0 as u16, true));
    }
    if shift {
        inputs.push(make_key_input(VK_SHIFT.0 as u16, true));
    }
    if ctrl {
        inputs.push(make_key_input(VK_CONTROL.0 as u16, true));
    }
    
    unsafe {
        SendInput(&inputs, mem::size_of::<INPUT>() as i32);
    }
    std::thread::sleep(std::time::Duration::from_millis(5));
}

fn make_key_input(vk: u16, is_up: bool) -> INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS;
    let flags = if is_up { KEYEVENTF_KEYUP.0 } else { 0 };
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(flags),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

#[cfg(not(target_os = "windows"))]
pub struct ActiveWindowDetails {
    pub process_name: String,
    pub title: String,
}

#[cfg(not(target_os = "windows"))]
pub fn get_caret_position() -> (i32, i32) {
    (0, 0)
}

#[cfg(not(target_os = "windows"))]
pub fn get_process_name(_pid: u32) -> String {
    "unknown".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_active_window_details() -> ActiveWindowDetails {
    ActiveWindowDetails {
        process_name: "unknown".to_string(),
        title: "unknown".to_string(),
    }
}

#[cfg(not(target_os = "windows"))]
pub fn send_backspaces(_count: usize) {}

#[cfg(not(target_os = "windows"))]
pub fn send_string(_text: &str) {}

#[cfg(not(target_os = "windows"))]
pub fn send_paste_shortcut() {}

#[cfg(not(target_os = "windows"))]
pub fn send_key_combo(_vk: u32, _ctrl: bool, _shift: bool, _alt: bool) {}


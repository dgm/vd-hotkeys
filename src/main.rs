#![windows_subsystem = "windows"]

mod config;

use config::HOTKEYS;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, SendInput, UnregisterHotKey, INPUT, INPUT_0, INPUT_KEYBOARD,
    KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, MOD_NOREPEAT, VIRTUAL_KEY,
};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
    PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, OpenProcess, TerminateProcess,
    PROCESS_TERMINATE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetMessageW, GetWindowLongW, GetWindowTextLengthW,
    IsIconic, IsWindow, IsWindowVisible, MessageBoxW, SetForegroundWindow,
    GWL_EXSTYLE, MSG, MB_ICONERROR, MB_OK, WM_HOTKEY, WS_EX_TOOLWINDOW,
};
use windows::core::PCWSTR;

fn main() {
    kill_existing_instances();

    let mut registered: Vec<i32> = Vec::new();
    let mut failed: Vec<i32> = Vec::new();

    for (id, hk) in HOTKEYS.iter().enumerate() {
        let id = id as i32;
        let ok = unsafe {
            RegisterHotKey(
                None,
                id,
                hk.modifiers | MOD_NOREPEAT,
                hk.vkey.0 as u32,
            )
        };
        if ok.is_ok() {
            registered.push(id);
        } else {
            failed.push(id);
        }
    }

    if !failed.is_empty() {
        let names = ["Ctrl+1","Ctrl+2","Ctrl+3","Ctrl+4","Ctrl+5",
                      "Ctrl+6","Ctrl+7","Ctrl+8","Ctrl+9"];
        let list: String = failed
            .iter()
            .filter_map(|&id| names.get(id as usize))
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        let msg = format!(
            "vd-hotkeys: failed to register hotkey(s): {}\n\
             Another application may have claimed them.",
            list
        );
        show_error(&msg);
    }

    if registered.is_empty() {
        return;
    }

    // Track last-focused window per desktop (0-based index)
    let mut focus_map: [HWND; 9] = [HWND::default(); 9];

    // Windows message loop — blocks until WM_QUIT
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.message == WM_HOTKEY {
                let id = msg.wParam.0 as usize;
                if id < HOTKEYS.len() {
                    let target = HOTKEYS[id].desktop as usize;

                    // Save the currently focused window for the current desktop
                    if let Ok(current) = winvd::get_current_desktop() {
                        let cur_idx = current.get_index().unwrap_or(0) as usize;
                        if cur_idx < focus_map.len() {
                            let fg = GetForegroundWindow();
                            if fg.0 != std::ptr::null_mut() {
                                focus_map[cur_idx] = fg;
                            }
                        }
                    }

                    // Switch to the target desktop
                    let _ = winvd::switch_desktop(HOTKEYS[id].desktop);

                    // Brief pause to let the desktop switch complete
                    std::thread::sleep(std::time::Duration::from_millis(50));

                    // Restore focus on the target desktop
                    if target < focus_map.len() {
                        let saved = focus_map[target];
                        if saved.0 != std::ptr::null_mut() && IsWindow(saved).as_bool() {
                            force_foreground(saved);
                        } else {
                            focus_map[target] = HWND::default();
                            // Fallback: focus the topmost visible app window
                            if let Some(fallback) = find_fallback_window() {
                                force_foreground(fallback);
                            }
                        }
                    }
                }
            }
        }
    }

    for id in registered {
        unsafe {
            let _ = UnregisterHotKey(None, id);
        }
    }
}

/// Force a window to the foreground using the Alt-key trick.
/// Windows restricts SetForegroundWindow unless the caller has input focus;
/// simulating an Alt press/release satisfies that requirement.
fn force_foreground(hwnd: HWND) {
    unsafe {
        let alt_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0x12), // VK_MENU (Alt)
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    ..Default::default()
                },
            },
        };
        let alt_up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0x12),
                    dwFlags: KEYEVENTF_KEYUP,
                    ..Default::default()
                },
            },
        };
        SendInput(&[alt_down, alt_up], std::mem::size_of::<INPUT>() as i32);
        let _ = SetForegroundWindow(hwnd);
    }
}

/// Kill any other running vd-hotkeys.exe processes so we can take over the hotkeys.
fn kill_existing_instances() {
    unsafe {
        let our_pid = GetCurrentProcessId();
        let Ok(snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
            return;
        };
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                if entry.th32ProcessID != our_pid {
                    let name = String::from_utf16_lossy(
                        &entry.szExeFile[..entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)],
                    );
                    if name.eq_ignore_ascii_case("vd-hotkeys.exe") {
                        if let Ok(handle) = OpenProcess(PROCESS_TERMINATE, false, entry.th32ProcessID) {
                            let _ = TerminateProcess(handle, 0);
                            let _ = windows::Win32::Foundation::CloseHandle(handle);
                        }
                    }
                }
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
        let _ = windows::Win32::Foundation::CloseHandle(snapshot);
    }
    // Brief pause to let the old process release hotkeys
    std::thread::sleep(std::time::Duration::from_millis(200));
}

/// Find the topmost visible, non-minimized app window (EnumWindows walks Z-order).
fn find_fallback_window() -> Option<HWND> {
    unsafe {
        let mut result: HWND = HWND::default();
        let _ = EnumWindows(
            Some(enum_callback),
            LPARAM(&mut result as *mut HWND as isize),
        );
        if result.0 != std::ptr::null_mut() {
            Some(result)
        } else {
            None
        }
    }
}

unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // Skip invisible or minimized windows
    if !IsWindowVisible(hwnd).as_bool() || IsIconic(hwnd).as_bool() {
        return BOOL(1); // continue
    }
    // Skip tool windows (floating palettes, etc.)
    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
    if ex_style & WS_EX_TOOLWINDOW.0 != 0 {
        return BOOL(1);
    }
    // Skip windows with no title
    if GetWindowTextLengthW(hwnd) == 0 {
        return BOOL(1);
    }
    // Found a good candidate
    let out = &mut *(lparam.0 as *mut HWND);
    *out = hwnd;
    BOOL(0) // stop enumeration
}

fn show_error(text: &str) {
    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let title: Vec<u16> = "vd-hotkeys\0".encode_utf16().collect();
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(wide.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONERROR,
        );
    }
}

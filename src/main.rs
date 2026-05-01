#![windows_subsystem = "windows"]

mod config;

use config::HOTKEYS;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_NOREPEAT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetMessageW, MessageBoxW, MSG, MB_ICONERROR, MB_OK, WM_HOTKEY,
};
use windows::core::PCWSTR;

fn main() {
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

    // Windows message loop — blocks until WM_QUIT
    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.message == WM_HOTKEY {
                let id = msg.wParam.0 as usize;
                if id < HOTKEYS.len() {
                    let _ = winvd::switch_desktop(HOTKEYS[id].desktop);
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

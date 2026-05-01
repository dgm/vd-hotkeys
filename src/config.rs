use windows::Win32::UI::Input::KeyboardAndMouse::{HOT_KEY_MODIFIERS, MOD_CONTROL, VIRTUAL_KEY};

pub struct Hotkey {
    pub modifiers: HOT_KEY_MODIFIERS,
    pub vkey: VIRTUAL_KEY,
    pub desktop: u32, // 0-based
}

// To change hotkeys, edit the modifiers field.
// MOD_CONTROL = Ctrl, MOD_ALT = Alt, MOD_SHIFT = Shift, MOD_WIN = Win
// Combine with | e.g. MOD_CONTROL | MOD_ALT
pub const HOTKEYS: [Hotkey; 9] = [
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x31), desktop: 0 }, // Ctrl+1
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x32), desktop: 1 }, // Ctrl+2
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x33), desktop: 2 }, // Ctrl+3
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x34), desktop: 3 }, // Ctrl+4
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x35), desktop: 4 }, // Ctrl+5
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x36), desktop: 5 }, // Ctrl+6
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x37), desktop: 6 }, // Ctrl+7
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x38), desktop: 7 }, // Ctrl+8
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x39), desktop: 8 }, // Ctrl+9
];

# vd-hotkeys

**Direct virtual desktop switching on Windows 11 via hotkey, no mouse needed.**

Windows 11 lets you create unlimited virtual desktops, but navigating to desktop N requires either clicking the Task View button or stepping sequentially through them. There's no built-in way to _jump directly_ to a specific desktop.

`vd-hotkeys` fixes that. Press `Ctrl+9` and you're on desktop 9 — instantly.

## Quick Start

| Hotkey   | Action            |
| -------- | ----------------- |
| `Ctrl+1` | Jump to desktop 1 |
| `Ctrl+2` | Jump to desktop 2 |
| `Ctrl+3` | Jump to desktop 3 |
| `Ctrl+4` | Jump to desktop 4 |
| `Ctrl+5` | Jump to desktop 5 |
| `Ctrl+6` | Jump to desktop 6 |
| `Ctrl+7` | Jump to desktop 7 |
| `Ctrl+8` | Jump to desktop 8 |
| `Ctrl+9` | Jump to desktop 9 |

Switching to a desktop that doesn't exist yet is a no-op.

## Install

Two delivery methods are available as release assets.

### Installer

**vd-hotkeys-v0.2.0-setup.exe**

- Auto-starts at login (registered in HKCU)
- Adds an entry in Add/Remove Programs
- Installs to `%LOCALAPPDATA%\vd-hotkeys\`
- Cleans up via Add/Remove Programs (kills running process first)
- No admin rights required

Built with [Inno Setup](https://jrsoftware.org/isinfo.php).

### Portable

**vd-hotkeys-v0.2.0-windows-x64.zip**

- Just extract and run `vd-hotkeys.exe`
- No system changes
- No uninstall step needed — just delete the folder

### Upgrade

- **Installer:** run the new `.exe` — it kills the old process and updates in place.
- **Portable:** replace the `.exe` in your folder.

**Requirements:** Windows 11 24H2 (build 26100+)

## Build from Source

```bash
cargo build --release
# Output: target/release/vd-hotkeys.exe
```

No external dependencies. The project depends only on:

| Crate     | Purpose                 | Publisher  |
| --------- | ----------------------- | ---------- |
| `winvd`   | Virtual desktop COM API | community  |
| `windows` | Win32 bindings          | Microsoft  |

## Configuration

Hotkeys are defined in `src/config.rs`. To remap, change the `modifiers` and `desktop` fields:

```rust
pub const HOTKEYS: [Hotkey; 9] = [
    Hotkey { modifiers: MOD_CONTROL, vkey: VIRTUAL_KEY(0x31), desktop: 0 }, // Ctrl+1
    Hotkey { modifiers: MOD_ALT,     vkey: VIRTUAL_KEY(0x32), desktop: 1 }, // Alt+2
    // ...
];
```

Available modifier flags: `MOD_CONTROL`, `MOD_ALT`, `MOD_SHIFT`, `MOD_WIN`.
Combine with `|`, e.g. `MOD_CONTROL | MOD_ALT`.

## Technical Notes

- **Self-replacement:** launching a new instance automatically terminates the old one, so upgrading is drag-and-drop with no "hotkey already registered" errors.
- **Focus tracking:** the last active window on each desktop is saved and restored when you return — no more clicking to refocus.
- **No elevation needed** — runs at user level.
- **No telemetry, no network access** — fully offline, zero calls home.

## License

MIT

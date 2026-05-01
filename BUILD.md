# vd-hotkeys

A minimal Windows utility to switch virtual desktops directly by index using
global hotkeys. No sequential stepping through desktops — jumps straight to
the target.

## Problem Statement

Windows 11 provides no documented public API for virtual desktop management.
The built-in hotkeys (Win+Ctrl+Left/Right) step sequentially through desktops,
making direct navigation to desktop N impossible without third-party tooling.

Existing tools (SylphyHorn, etc.) solve this but include pre-compiled binary
dependencies that cannot be fully audited — a blocker for use on managed/work
machines where the full supply chain must be accountable.

## Goals

- Switch directly to virtual desktop 1–N by hotkey (no sequential stepping)
- Fully auditable supply chain: every dependency is source-available and
  buildable from scratch
- Single static `.exe` output, no runtime dependencies
- No telemetry, no network access, no elevation required at runtime
- Suitable for use on managed/corporate Windows machines

## Non-Goals

- Moving windows between desktops (out of scope for v1)
- Desktop naming, wallpaper management
- System tray UI
- Support for Windows 10 or Windows versions prior to 24H2 (build 26100)

## Technology

### Language: Rust

Chosen over C#, PowerShell, and Go for the following reasons:

- **Supply chain**: all dependencies are source-available Rust crates with no
  vendored binaries
- **Single binary output**: compiles to a self-contained `.exe` with no .NET
  runtime or other runtime dependency
- **Official Windows support**: Microsoft publishes and maintains `windows-rs`,
  the crate used for Win32/COM interop
- **Auditability**: the full dependency tree is small and readable

### Dependencies

| Crate | Version | Purpose | Publisher |
|-------|---------|---------|-----------|
| `winvd` | 0.0.49 | Virtual desktop COM API wrapper | community |
| `windows` | 0.58 | Win32 API bindings (hotkeys, message loop) | Microsoft |

`winvd` itself depends only on `windows` 0.58 and `windows-core` 0.58 —
both Microsoft-published crates. There are no binary dependencies.

#### Supply chain verification

- **Rust toolchain**: install via [rustup.rs](https://rustup.rs). The toolchain
  itself is open source at github.com/rust-lang/rust.
- **`windows-rs`**: published by Microsoft at github.com/microsoft/windows-rs.
  Same team that owns the APIs being called.
- **`winvd`**: published at github.com/Ciantic/VirtualDesktopAccessor. Small
  crate (~1500 lines). Full source readable before use. Handles per-build COM
  interface versioning for Windows 11.
- **Cargo.lock**: pins exact versions and SHA256 hashes of all crates.
  Reproducible builds are possible with `cargo build --locked`.

### Windows API Surface

The program uses two Win32 subsystems:

**Hotkey registration** (`user32.dll`):
- `RegisterHotKey` — registers a global hotkey combination
- `UnregisterHotKey` — cleanup on exit
- `GetMessage` / `TranslateMessage` / `DispatchMessage` — Windows message loop
  to receive hotkey events

**Virtual desktop switching** (via `winvd`, through COM):
- `CoCreateInstance(CLSID_ImmersiveShell)` — get shell service provider
- `IVirtualDesktopManagerInternal::GetDesktops()` — enumerate desktops
- `IVirtualDesktopManagerInternal::SwitchDesktop(pDesktop)` — direct switch

`winvd` abstracts the COM layer and handles the fact that
`IVirtualDesktopManagerInternal`'s interface ID and vtable layout change
between Windows builds. Requires Windows 11 24H2 (build 26100.2605+).

## Default Hotkey Scheme

| Hotkey | Action |
|--------|--------|
| Ctrl+1 | Switch to desktop 1 |
| Ctrl+2 | Switch to desktop 2 |
| Ctrl+3 | Switch to desktop 3 |
| Ctrl+4 | Switch to desktop 4 |
| Ctrl+5 | Switch to desktop 5 |
| Ctrl+6 | Switch to desktop 6 |
| Ctrl+7 | Switch to desktop 7 |
| Ctrl+8 | Switch to desktop 8 |
| Ctrl+9 | Switch to desktop 9 |

Switching to a desktop index that doesn't exist is a no-op (silently ignored).

`RegisterHotKey` is system-wide and consumes the keypress — no other application
sees it once registered. This means Ctrl+1 will switch desktops even when a
browser is focused, and the browser will not switch tabs. This is the intended
behavior.

Ctrl+N was chosen to match the macOS muscle memory of the primary user.
Applications handle Ctrl+1 internally (not via global hotkey registration), so
there are no conflicts in practice.

The scheme is configurable in `src/config.rs` without touching any other file.

## Build Requirements

- Windows 11 24H2 (build 26100.2605 or later) — target runtime
- Rust toolchain (stable, x86_64-pc-windows-msvc)
- [Inno Setup 6](https://jrsoftware.org/isinfo.php) — required to build the installer
- No elevated privileges required to build or run

## Build Instructions

```
# Install Rust (if not already installed)
winget install Rustlang.Rustup

# Install Inno Setup 6 (if not already installed)
winget install JRSoftware.InnoSetup

# Clone and build
git clone <this repo>
cd vd-hotkeys
powershell -ExecutionPolicy Bypass -File package.ps1

# Outputs
dist\vd-hotkeys-v0.1.1-setup.exe        # installer (requires Inno Setup)
dist\vd-hotkeys-v0.1.1-windows-x64.zip  # portable zip (exe + docs)
```

The installer registers autostart via `HKCU\...\Run` and adds an entry to
Add/Remove Programs. No elevation required.

## Reference

- SylphyHornPlusWin11 source (../SylphyHornPlusWin11) — reference implementation
  in C#; used to understand the COM interface versioning approach and confirm
  the correct CLSIDs/IIDs for Windows 11
- [winvd crate](https://lib.rs/crates/winvd)
- [microsoft/windows-rs](https://github.com/microsoft/windows-rs)
- [Windows Virtual Desktop COM interfaces](https://github.com/Ciantic/VirtualDesktopAccessor)

# install.ps1 — installs vd-hotkeys for the current user
# No elevation required. Run from the repo root after building:
#   cargo build --release
#   powershell -ExecutionPolicy Bypass -File install.ps1

$ErrorActionPreference = "Stop"

$exe_src = "$PSScriptRoot\target\x86_64-pc-windows-msvc\release\vd-hotkeys.exe"
$install_dir = "$env:LOCALAPPDATA\vd-hotkeys"
$exe_dst = "$install_dir\vd-hotkeys.exe"
$startup_lnk = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\vd-hotkeys.lnk"

if (-not (Test-Path $exe_src)) {
    Write-Error "vd-hotkeys.exe not found at $exe_src`nRun 'cargo build --release' first."
    exit 1
}

# Stop any running instance before replacing the exe
$running = Get-Process -Name "vd-hotkeys" -ErrorAction SilentlyContinue
if ($running) {
    Write-Host "Stopping running instance..."
    $running | Stop-Process -Force
    Start-Sleep -Milliseconds 500
}

# Install exe
New-Item -ItemType Directory -Path $install_dir -Force | Out-Null
Copy-Item -Path $exe_src -Destination $exe_dst -Force
Write-Host "Installed: $exe_dst"

# Create Startup shortcut
$ws = New-Object -ComObject WScript.Shell
$sc = $ws.CreateShortcut($startup_lnk)
$sc.TargetPath = $exe_dst
$sc.WorkingDirectory = $install_dir
$sc.Description = "Virtual desktop hotkeys (Ctrl+1-9)"
$sc.Save()
Write-Host "Startup shortcut created: $startup_lnk"

# Launch immediately
Start-Process -FilePath $exe_dst
Write-Host "vd-hotkeys is running."

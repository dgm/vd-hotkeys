# uninstall.ps1 — removes vd-hotkeys for the current user

$ErrorActionPreference = "Stop"

$install_dir = "$env:LOCALAPPDATA\vd-hotkeys"
$startup_lnk = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\vd-hotkeys.lnk"

$running = Get-Process -Name "vd-hotkeys" -ErrorAction SilentlyContinue
if ($running) {
    Write-Host "Stopping running instance..."
    $running | Stop-Process -Force
    Start-Sleep -Milliseconds 500
}

if (Test-Path $startup_lnk) {
    Remove-Item $startup_lnk -Force
    Write-Host "Removed startup shortcut."
}

if (Test-Path $install_dir) {
    Remove-Item $install_dir -Recurse -Force
    Write-Host "Removed install directory: $install_dir"
}

Write-Host "vd-hotkeys uninstalled."

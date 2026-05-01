# package.ps1 — builds the installer and portable zip for distribution
# Run from the repo root on Windows:
#   powershell -ExecutionPolicy Bypass -File package.ps1
# Requires Inno Setup 6 for the installer (https://jrsoftware.org/isinfo.php)

$ErrorActionPreference = "Stop"

# Read version from Cargo.toml
$cargo = Get-Content "$PSScriptRoot\Cargo.toml" | Where-Object { $_ -match '^version\s*=' }
$version = ($cargo -split '"')[1]

$exe = "$PSScriptRoot\target\x86_64-pc-windows-msvc\release\vd-hotkeys.exe"
$out_dir = "$PSScriptRoot\dist"

Write-Host "Building vd-hotkeys v$version..."
cargo build --release --locked
if ($LASTEXITCODE -ne 0) { exit 1 }

New-Item -ItemType Directory -Path $out_dir -Force | Out-Null

# Build installer with Inno Setup if available
$iscc = Get-Command iscc -ErrorAction SilentlyContinue
if (-not $iscc) {
    $iscc_path = "C:\Program Files (x86)\Inno Setup 6\iscc.exe"
    if (Test-Path $iscc_path) { $iscc = $iscc_path } else { $iscc = $null }
}

if ($iscc) {
    Write-Host "Building installer..."
    & $iscc /DAppVersion=$version "$PSScriptRoot\vd-hotkeys.iss"
    if ($LASTEXITCODE -ne 0) { exit 1 }
    Write-Host "Installer ready: $out_dir\vd-hotkeys-v$version-setup.exe"
} else {
    Write-Host "Inno Setup not found — skipping installer build."
    Write-Host "Install from https://jrsoftware.org/isinfo.php then re-run."
}

# Also produce a portable zip (exe only, no scripts needed)
$zip = "$out_dir\vd-hotkeys-v$version-windows-x64.zip"
$stage = "$out_dir\stage"
Remove-Item $stage -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $stage | Out-Null

Copy-Item $exe "$stage\vd-hotkeys.exe"
Copy-Item "$PSScriptRoot\BUILD.md" "$stage\BUILD.md"

Compress-Archive -Path "$stage\*" -DestinationPath $zip -Force
Remove-Item $stage -Recurse -Force

Write-Host ""
Write-Host "Portable zip ready: $zip"
Write-Host ""
Write-Host "To create a Forgejo release:"
Write-Host "  1. git tag v$version && git push origin v$version"
Write-Host "  2. Upload both dist\ files as release assets in Forgejo"

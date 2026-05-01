# package.ps1 — builds a release zip ready for upload to Forgejo
# Run from the repo root in Developer PowerShell for VS 2022:
#   powershell -ExecutionPolicy Bypass -File package.ps1

$ErrorActionPreference = "Stop"

# Read version from Cargo.toml
$cargo = Get-Content "$PSScriptRoot\Cargo.toml" | Where-Object { $_ -match '^version\s*=' }
$version = ($cargo -split '"')[1]

$exe = "$PSScriptRoot\target\x86_64-pc-windows-msvc\release\vd-hotkeys.exe"
$out_dir = "$PSScriptRoot\dist"
$zip = "$out_dir\vd-hotkeys-v$version-windows-x64.zip"

Write-Host "Building vd-hotkeys v$version..."
cargo build --release --locked
if ($LASTEXITCODE -ne 0) { exit 1 }

New-Item -ItemType Directory -Path $out_dir -Force | Out-Null

# Assemble zip contents in a staging folder
$stage = "$out_dir\stage"
Remove-Item $stage -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $stage | Out-Null

Copy-Item $exe "$stage\vd-hotkeys.exe"
Copy-Item "$PSScriptRoot\install.ps1" "$stage\install.ps1"
Copy-Item "$PSScriptRoot\uninstall.ps1" "$stage\uninstall.ps1"
Copy-Item "$PSScriptRoot\BUILD.md" "$stage\BUILD.md"

Compress-Archive -Path "$stage\*" -DestinationPath $zip -Force
Remove-Item $stage -Recurse -Force

Write-Host ""
Write-Host "Release package ready: $zip"
Write-Host ""
Write-Host "To create a Forgejo release:"
Write-Host "  1. git tag v$version && git push origin v$version"
Write-Host "  2. Upload $zip as a release asset in Forgejo"

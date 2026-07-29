# ============================================================
#   [ HALCA MULTI-GAME TERMINAL ARCADE POWERSHELL SETUP ]
# ============================================================

$ErrorActionPreference = "Stop"

$HalcaDir = "$env:USERPROFILE\.halca"
$ManifestLog = "$HalcaDir\install_manifest.log"
$CargoBinDir = "$env:USERPROFILE\.cargo\bin"

New-Item -ItemType Directory -Force -Path $HalcaDir | Out-Null
New-Item -ItemType Directory -Force -Path $CargoBinDir | Out-Null

if (-not (Test-Path $ManifestLog)) {
    New-Item -ItemType File -Force -Path $ManifestLog | Out-Null
}

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "   [ HALCA MULTI-GAME TERMINAL ARCADE WINDOWS SETUP ]     " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

# 1. Check Rust / Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "[+] Rust toolchain not found. Installing rustup..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
    Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y" -Wait
    Add-Content -Path $ManifestLog -Value "DEP:rustup"
} else {
    Write-Host "[*] Rust toolchain already present. SKIP (Preserved safely)." -ForegroundColor Green
}

# 2. Sync Source Code
$InstallSrc = "$HalcaDir\source"
Write-Host "[+] Syncing Halca Arcade platform engine..." -ForegroundColor Yellow

if (Test-Path "$InstallSrc\.git") {
    Set-Location $InstallSrc
    git pull origin main
} else {
    Remove-Item -Recurse -Force $InstallSrc -ErrorAction SilentlyContinue
    git clone https://github.com/WhaTheFoxSay/Halca.git $InstallSrc
    Set-Location $InstallSrc
}

# 3. Compile Halca Release Binary
Write-Host "[+] Compiling Halca Arcade Client binary..." -ForegroundColor Yellow
cargo build --release --bin client

$ClientBin = "$InstallSrc\target\release\client.exe"

# 4. Copy to Cargo Bin Dir as halca.exe and HALCA.exe
Copy-Item -Path $ClientBin -Destination "$CargoBinDir\halca.exe" -Force
Copy-Item -Path $ClientBin -Destination "$CargoBinDir\HALCA.exe" -Force
Add-Content -Path $ManifestLog -Value "APP:halca_binary"

# 5. Add Cargo Bin to Windows User Environment PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$CargoBinDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$CargoBinDir", "User")
    $env:Path = "$env:Path;$CargoBinDir"
}

Write-Host "============================================================" -ForegroundColor Green
Write-Host "   [ HALCA TERMINAL ARCADE INSTALLED SUCCESSFULLY ]         " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Green
Write-Host ""
Write-Host "   >>> Ketik 'halca' atau 'HALCA' lalu tekan ENTER! <<<" -ForegroundColor Yellow
Write-Host ""
Write-Host "============================================================" -ForegroundColor Green

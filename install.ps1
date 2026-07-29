# ============================================================
#   [ HALCA MULTI-GAME TERMINAL ARCADE POWERSHELL SETUP ]
# ============================================================

$ErrorActionPreference = "Stop"

$HalcaDir = "$env:USERPROFILE\.halca"
$ManifestLog = "$HalcaDir\install_manifest.log"
$CargoBinDir = "$env:USERPROFILE\.cargo\bin"
$GithubReleaseUrl = "https://raw.githubusercontent.com/WhaTheFoxSay/Halca/main/releases/halca-windows-x86_64.exe"

New-Item -ItemType Directory -Force -Path $HalcaDir | Out-Null
New-Item -ItemType Directory -Force -Path $CargoBinDir | Out-Null

if (-not (Test-Path $ManifestLog)) {
    New-Item -ItemType File -Force -Path $ManifestLog | Out-Null
}

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "   [ HALCA MULTI-GAME TERMINAL ARCADE WINDOWS FAST SETUP ] " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

$ClientBin = "$HalcaDir\bin\halca.exe"
New-Item -ItemType Directory -Force -Path "$HalcaDir\bin" | Out-Null

Write-Host "[+] DOWNLOADING PRE-COMPILED ARCADE CORE FOR WINDOWS (x64)..." -ForegroundColor Magenta

$BinaryDownloaded = $false

try {
    Invoke-WebRequest -Uri $GithubReleaseUrl -OutFile $ClientBin -UseBasicParsing -TimeoutSec 15
    if ((Get-Item $ClientBin).Length -gt 0) {
        $BinaryDownloaded = $true
        Write-Host "    [✓] INSTANT INSTALL: Pre-compiled Arcade Core retrieved in 2 seconds!" -ForegroundColor Green
    }
} catch {
    Write-Host "    [!] Pre-compiled Windows binary mirror missed. Switching to source compiler..." -ForegroundColor Yellow
}

# Fallback to source compilation if binary download failed
if (-not $BinaryDownloaded) {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "[+] Rust toolchain not found. Installing rustup..." -ForegroundColor Yellow
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
        Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y" -Wait
        Add-Content -Path $ManifestLog -Value "DEP:rustup"
    }

    $InstallSrc = "$HalcaDir\source"
    if (Test-Path "$InstallSrc\.git") {
        Set-Location $InstallSrc
        git pull origin main
    } else {
        Remove-Item -Recurse -Force $InstallSrc -ErrorAction SilentlyContinue
        git clone https://github.com/WhaTheFoxSay/Halca.git $InstallSrc
        Set-Location $InstallSrc
    }

    Write-Host "[+] Compiling Halca Arcade Client binary..." -ForegroundColor Yellow
    cargo build --release --bin client
    $ClientBin = "$InstallSrc\target\release\client.exe"
}

# Copy binary to user's bin folder
Copy-Item -Path $ClientBin -Destination "$CargoBinDir\halca.exe" -Force
Copy-Item -Path $ClientBin -Destination "$CargoBinDir\HALCA.exe" -Force
Add-Content -Path $ManifestLog -Value "APP:halca_binary"

# Add Cargo Bin to Windows User Environment PATH (No Admin Rights Required)
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$CargoBinDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$CargoBinDir", "User")
    $env:Path = "$env:Path;$CargoBinDir"
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host "   [ HALCA TERMINAL ARCADE INSTALLED SUCCESSFULLY ]         " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Green
Write-Host ""
Write-Host "   >>> Ketik 'halca' atau 'HALCA' lalu tekan ENTER untuk main! <<<" -ForegroundColor Yellow
Write-Host ""
Write-Host "============================================================" -ForegroundColor Green

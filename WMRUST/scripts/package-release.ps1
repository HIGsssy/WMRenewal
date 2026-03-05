#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Packages WhoreMaster Renewal for release distribution.

.DESCRIPTION
    Builds release binaries and assembles a distributable folder structure containing
    the game binary, editor binary, bundled fonts, and resource files.

.PARAMETER Target
    Rust target triple. Defaults to the host target.

.PARAMETER OutputDir
    Destination folder for the packaged release. Defaults to ./dist.

.PARAMETER ResourcesPath
    Path to the Resources directory. Defaults to ../WhoreMasterRenewal/Resources.

.PARAMETER SkipBuild
    Skip cargo build (use existing binaries in target/release).
#>
param(
    [string]$Target = "",
    [string]$OutputDir = "dist",
    [string]$ResourcesPath = "../WhoreMasterRenewal/Resources",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
# Run from project root (one level up from scripts/)
Set-Location (Split-Path $PSScriptRoot -Parent)

Write-Host "=== WhoreMaster Renewal Release Packager ===" -ForegroundColor Cyan

# --- Build ---
if (-not $SkipBuild) {
    Write-Host "Building release binaries..." -ForegroundColor Yellow
    if ($Target) {
        cargo build --release --target $Target -p wm-app
        cargo build --release --target $Target -p wm-edit
        $binDir = "target/$Target/release"
    } else {
        cargo build --release -p wm-app
        cargo build --release -p wm-edit
        $binDir = "target/release"
    }
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed."
        exit 1
    }
} else {
    $binDir = if ($Target) { "target/$Target/release" } else { "target/release" }
    Write-Host "Skipping build, using existing binaries in $binDir" -ForegroundColor Yellow
}

# --- Prepare output ---
if (Test-Path $OutputDir) {
    Remove-Item $OutputDir -Recurse -Force
}
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

# --- Copy binaries ---
Write-Host "Copying binaries..." -ForegroundColor Yellow
$ext = if ($IsWindows -or $env:OS -eq "Windows_NT") { ".exe" } else { "" }

$gameBin = Join-Path $binDir "whoremaster$ext"
$editBin = Join-Path $binDir "wm-editor$ext"

if (Test-Path $gameBin) {
    Copy-Item $gameBin $OutputDir
    Write-Host "  Game binary: $(Split-Path $gameBin -Leaf)"
} else {
    Write-Warning "Game binary not found at $gameBin"
}

if (Test-Path $editBin) {
    Copy-Item $editBin $OutputDir
    Write-Host "  Editor binary: $(Split-Path $editBin -Leaf)"
} else {
    Write-Warning "Editor binary not found at $editBin"
}

# --- Copy SDL2 DLLs (Windows) ---
if ($IsWindows -or $env:OS -eq "Windows_NT") {
    Write-Host "Copying SDL2 DLLs..." -ForegroundColor Yellow
    # DLL source locations: build output and downloaded SDL2 extension libs
    $dllSources = @{
        "SDL2.dll"       = @((Join-Path $binDir "SDL2.dll"))
        "SDL2_image.dll" = @((Join-Path $binDir "SDL2_image.dll"), "sdl2-libs/SDL2_image-2.8.4/lib/x64/SDL2_image.dll")
        "SDL2_ttf.dll"   = @((Join-Path $binDir "SDL2_ttf.dll"), "sdl2-libs/SDL2_ttf-2.22.0/lib/x64/SDL2_ttf.dll")
    }
    foreach ($dll in $dllSources.Keys) {
        $copied = $false
        foreach ($src in $dllSources[$dll]) {
            if (Test-Path $src) {
                Copy-Item $src $OutputDir
                Write-Host "  $dll"
                $copied = $true
                break
            }
        }
        if (-not $copied) {
            Write-Warning "$dll not found"
        }
    }
}

# --- Copy bundled assets ---
Write-Host "Copying bundled assets (fonts)..." -ForegroundColor Yellow
$fontDest = Join-Path $OutputDir "assets/fonts"
New-Item -ItemType Directory -Path $fontDest -Force | Out-Null
Copy-Item "assets/fonts/DejaVuSans.ttf" $fontDest
Copy-Item "assets/fonts/DejaVuSansMono.ttf" $fontDest
Copy-Item "assets/fonts/LICENSE-DejaVu.txt" $fontDest

# --- Copy resources ---
if (Test-Path $ResourcesPath) {
    Write-Host "Copying game resources..." -ForegroundColor Yellow
    $resDest = Join-Path $OutputDir "resources"
    
    # Copy Data directory
    $dataDir = Join-Path $ResourcesPath "Data"
    if (Test-Path $dataDir) {
        Copy-Item $dataDir -Destination (Join-Path $resDest "Data") -Recurse
    }
    
    # Copy Interface directory
    $ifaceDir = Join-Path $ResourcesPath "Interface"
    if (Test-Path $ifaceDir) {
        Copy-Item $ifaceDir -Destination (Join-Path $resDest "Interface") -Recurse
    }
    
    # Copy Scripts directory
    $scriptsDir = Join-Path $ResourcesPath "Scripts"
    if (Test-Path $scriptsDir) {
        Copy-Item $scriptsDir -Destination (Join-Path $resDest "Scripts") -Recurse
    }
    
    # Copy Buttons directory
    $buttonsDir = Join-Path $ResourcesPath "Buttons"
    if (Test-Path $buttonsDir) {
        Copy-Item $buttonsDir -Destination (Join-Path $resDest "Buttons") -Recurse
    }
    
    # Copy Images directory
    $imagesDir = Join-Path $ResourcesPath "Images"
    if (Test-Path $imagesDir) {
        Copy-Item $imagesDir -Destination (Join-Path $resDest "Images") -Recurse
    }
    
    # Copy Characters (excluding flagged copyright directories)
    $charDir = Join-Path $ResourcesPath "Characters"
    if (Test-Path $charDir) {
        $charDest = Join-Path $resDest "Characters"
        New-Item -ItemType Directory -Path $charDest -Force | Out-Null
        # Copy XML data files only — skip image directories pending copyright review
        Get-ChildItem $charDir -File | Copy-Item -Destination $charDest
        # Copy non-trademarked character folders
        foreach ($dir in @("Cute Girl", "Dangerous Girl")) {
            $src = Join-Path $charDir $dir
            if (Test-Path $src) {
                Copy-Item $src -Destination (Join-Path $charDest $dir) -Recurse
            }
        }
        Write-Host "  NOTE: Skipped 'Cammy White' and 'Chun-Li' directories (copyright flagged)"
    }
} else {
    Write-Warning "Resources path not found: $ResourcesPath"
    Write-Warning "The packaged release will not include game resources."
}

# --- Copy docs ---
Write-Host "Copying documentation..." -ForegroundColor Yellow
foreach ($doc in @("README.md", "LICENSE", "CHANGELOG.md")) {
    if (Test-Path $doc) {
        Copy-Item $doc $OutputDir
    }
}
$docsDest = Join-Path $OutputDir "docs"
if (Test-Path "docs") {
    Copy-Item "docs" -Destination $docsDest -Recurse
}

# --- Summary ---
Write-Host ""
Write-Host "=== Release package created at: $OutputDir ===" -ForegroundColor Green
$totalSize = (Get-ChildItem $OutputDir -Recurse -File | Measure-Object -Property Length -Sum).Sum
Write-Host ("Total size: {0:N1} MB" -f ($totalSize / 1MB))
Write-Host ""
Write-Host "Contents:" -ForegroundColor Yellow
Get-ChildItem $OutputDir | ForEach-Object {
    if ($_.PSIsContainer) {
        $subSize = (Get-ChildItem $_.FullName -Recurse -File | Measure-Object -Property Length -Sum).Sum
        Write-Host ("  {0}/  ({1:N1} MB)" -f $_.Name, ($subSize / 1MB))
    } else {
        Write-Host ("  {0}  ({1:N1} KB)" -f $_.Name, ($_.Length / 1KB))
    }
}

# Downloads mpv dev library for Windows (required to build with the `mpv` feature)
#
# Usage:
#   .\setup-mpv-dev.ps1                          # outputs to .\mpv-dev\
#   .\setup-mpv-dev.ps1 -OutputDir "C:\some\dir" # outputs to a custom directory
#
# When called from build.rs, -OutputDir is set to %LOCALAPPDATA%\youtube-tui\mpv-dev
# so the files persist across `cargo install` invocations.

param(
    [string]$OutputDir = ""
)

$ErrorActionPreference = "Stop"

# Determine output directory:
#   - Explicit -OutputDir argument (used by build.rs)
#   - Otherwise default to a sibling mpv-dev\ next to this script
if ($OutputDir -eq "") {
    $OutputDir = Join-Path $PSScriptRoot "mpv-dev"
}

$mpvLib = Join-Path $OutputDir "mpv.lib"

if (Test-Path $mpvLib) {
    Write-Host "mpv.lib already exists at $OutputDir, skipping download."
    $env:MPV_LIB_DIR = $OutputDir
    Write-Host "MPV_LIB_DIR set to $OutputDir"
    return
}

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$url = "https://github.com/shinchiro/mpv-winbuild-cmake/releases/download/20260412/mpv-dev-${arch}-20260412-git-062f4bf.7z"

# Use the user-local temp directory (always writable, unlike C:\Windows\Temp)
$userTemp = [System.IO.Path]::GetTempPath()
$tempFile = Join-Path $userTemp "mpv-dev.7z"
$tempExtract = Join-Path $userTemp "mpv-dev-extract"

Write-Host "Downloading mpv-dev for $arch..."
Invoke-WebRequest -Uri $url -OutFile $tempFile -UserAgent "Mozilla/5.0"

# Create output directory
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

# Locate 7z — check PATH first, then common install locations
$7zPaths = @(
    "7z",
    "$env:ProgramFiles\7-Zip\7z.exe",
    "$env:LOCALAPPDATA\Programs\7-Zip\7z.exe",
    "$env:USERPROFILE\scoop\apps\7zip\current\7z.exe"
)

$7zExe = $null
foreach ($p in $7zPaths) {
    if (Get-Command $p -ErrorAction SilentlyContinue) {
        $7zExe = $p
        break
    }
    if (Test-Path $p) {
        $7zExe = $p
        break
    }
}

if (-not $7zExe) {
    Write-Host "7z not found. Attempting to install via scoop..."
    scoop install 7zip
    $7zExe = "$env:USERPROFILE\scoop\apps\7zip\current\7z.exe"
}

Write-Host "Extracting mpv-dev..."
if (Test-Path $tempExtract) { Remove-Item -Recurse -Force $tempExtract }
& $7zExe x $tempFile -o"$tempExtract" -y | Out-Null

# Copy the files we need
$libFile = Get-ChildItem -Path $tempExtract -Filter "mpv.lib" -Recurse | Select-Object -First 1
$dllFile = Get-ChildItem -Path $tempExtract -Filter "libmpv-2.dll" -Recurse | Select-Object -First 1

if (-not $libFile) {
    # The shinchiro builds ship libmpv.dll.a (GCC format), not mpv.lib (MSVC format).
    # Generate mpv.lib from the DLL using MSVC dumpbin + lib tools.
    Write-Host "mpv.lib not found in archive. Generating from libmpv-2.dll..."

    $dllForLib = Get-ChildItem -Path $tempExtract -Filter "libmpv-2.dll" -Recurse | Select-Object -First 1
    if (-not $dllForLib) {
        Write-Error "Could not find libmpv-2.dll in the archive!"
    }

    # Find MSVC tools — installed but not in PATH by default
    $vsBase = "${env:ProgramFiles(x86)}\Microsoft Visual Studio"
    if (-not (Test-Path $vsBase)) { $vsBase = "$env:ProgramFiles\Microsoft Visual Studio" }
    $dumpbin = Get-ChildItem -Path $vsBase -Recurse -Filter "dumpbin.exe" -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -like "*Hostx64\x64*" -or $_.FullName -like "*HostX64\x64*" } |
        Select-Object -First 1 -ExpandProperty FullName
    $libExe = Get-ChildItem -Path $vsBase -Recurse -Filter "lib.exe" -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -like "*Hostx64\x64*" -or $_.FullName -like "*HostX64\x64*" } |
        Select-Object -First 1 -ExpandProperty FullName

    if (-not $dumpbin -or -not $libExe) {
        Write-Error "Could not find MSVC build tools (dumpbin.exe, lib.exe). Install 'Desktop development with C++' workload."
    }

    # Export function names from DLL and write .def file
    $defPath = Join-Path $OutputDir "mpv.def"
    $exports = & $dumpbin /exports "$($dllForLib.FullName)" 2>&1 |
        Select-String "^\s+\d+\s+[0-9A-Fa-f]+\s+[0-9A-Fa-f]+\s+(\S+)" |
        ForEach-Object { $_.Matches[0].Groups[1].Value }

    $defContent = "LIBRARY libmpv-2`nEXPORTS`n"
    $defContent += ($exports | ForEach-Object { "    $_" }) -join "`n"
    Set-Content -Path $defPath -Value $defContent -Encoding ASCII

    # Generate .lib from .def
    $machine = if ($arch -eq "x86_64") { "x64" } else { "x86" }
    & $libExe /def:"$defPath" /machine:$machine /out:"$OutputDir\mpv.lib" 2>&1 | Out-Null

    if (-not (Test-Path "$OutputDir\mpv.lib")) {
        Write-Error "Failed to generate mpv.lib!"
    } else {
        Write-Host "Generated mpv.lib successfully."
    }
} else {
    Copy-Item $libFile.FullName $OutputDir
}

if ($dllFile) {
    Copy-Item $dllFile.FullName $OutputDir
}

# Also grab any other DLLs from the archive
Get-ChildItem -Path $tempExtract -Filter "*.dll" -Recurse | ForEach-Object {
    Copy-Item $_.FullName $OutputDir -Force
}

# Cleanup temp files
Remove-Item -Force $tempFile -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force $tempExtract -ErrorAction SilentlyContinue

$env:MPV_LIB_DIR = $OutputDir
Write-Host ""
Write-Host "Done! mpv dev files extracted to: $OutputDir"
Write-Host "MPV_LIB_DIR set to: $OutputDir"
Write-Host ""
Write-Host "To make MPV_LIB_DIR permanent, add to your PowerShell profile:"
Write-Host "  `$env:MPV_LIB_DIR = '$OutputDir'"

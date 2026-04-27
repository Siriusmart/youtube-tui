# Downloads mpv dev library for Windows (required to build with the `mpv` feature)
# Usage: .\setup-mpv-dev.ps1
# This downloads ~30MB and extracts mpv.lib + libmpv-2.dll into a local 'mpv-dev' folder.
# Then sets MPV_LIB_DIR for the current session.

$ErrorActionPreference = "Stop"

$mpvDevDir = Join-Path $PSScriptRoot "mpv-dev"
$mpvLib = Join-Path $mpvDevDir "mpv.lib"

if (Test-Path $mpvLib) {
    Write-Host "mpv.lib already exists at $mpvDevDir, skipping download."
    $env:MPV_LIB_DIR = $mpvDevDir
    Write-Host "MPV_LIB_DIR set to $mpvDevDir"
    return
}

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$url = "https://github.com/shinchiro/mpv-winbuild-cmake/releases/download/20260412/mpv-dev-${arch}-20260412-git-062f4bf.7z"

Write-Host "Downloading mpv-dev for $arch..."
$tempFile = Join-Path $env:TEMP "mpv-dev.7z"
Invoke-WebRequest -Uri $url -OutFile $tempFile -UserAgent "Mozilla/5.0"

# Create output directory
New-Item -ItemType Directory -Force -Path $mpvDevDir | Out-Null

# Extract using 7z (ships with scoop, git, or can be installed via scoop install 7zip)
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
$tempExtract = Join-Path $env:TEMP "mpv-dev-extract"
if (Test-Path $tempExtract) { Remove-Item -Recurse -Force $tempExtract }
& $7zExe x $tempFile -o"$tempExtract" -y | Out-Null

# Copy the files we need
$libFile = Get-ChildItem -Path $tempExtract -Filter "mpv.lib" -Recurse | Select-Object -First 1
$dllFile = Get-ChildItem -Path $tempExtract -Filter "libmpv-2.dll" -Recurse | Select-Object -First 1

if (-not $libFile) {
    # The shinchiro builds ship libmpv.dll.a (GCC format), not mpv.lib (MSVC format).
    # Generate mpv.lib from the DLL using dumpbin + lib (MSVC tools).
    Write-Host "mpv.lib not found in archive. Generating from libmpv-2.dll..."

    $dllForLib = Get-ChildItem -Path $tempExtract -Filter "libmpv-2.dll" -Recurse | Select-Object -First 1
    if (-not $dllForLib) {
        Write-Error "Could not find libmpv-2.dll in the archive!"
    }

    # Find MSVC tools (dumpbin, lib) - they're installed but not in PATH
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

    # Export function names from DLL
    $defPath = Join-Path $mpvDevDir "mpv.def"
    $exports = & $dumpbin /exports "$($dllForLib.FullName)" 2>&1 |
        Select-String "^\s+\d+\s+[0-9A-Fa-f]+\s+[0-9A-Fa-f]+\s+(\S+)" |
        ForEach-Object { $_.Matches[0].Groups[1].Value }

    # Write .def file
    $defContent = "LIBRARY libmpv-2`nEXPORTS`n"
    $defContent += ($exports | ForEach-Object { "    $_" }) -join "`n"
    Set-Content -Path $defPath -Value $defContent -Encoding ASCII

    # Generate .lib from .def
    $machine = if ($arch -eq "x86_64") { "x64" } else { "x86" }
    & $libExe /def:"$defPath" /machine:$machine /out:"$mpvDevDir\mpv.lib" 2>&1 | Out-Null

    if (-not (Test-Path "$mpvDevDir\mpv.lib")) {
        Write-Error "Failed to generate mpv.lib!"
    } else {
        Write-Host "Generated mpv.lib successfully."
    }
} else {
    Copy-Item $libFile.FullName $mpvDevDir
}

if ($dllFile) {
    Copy-Item $dllFile.FullName $mpvDevDir
    # Also copy DLL next to the built binary location
    $targetRelease = Join-Path $PSScriptRoot "target\release"
    if (Test-Path $targetRelease) {
        Copy-Item $dllFile.FullName $targetRelease
    }
}

# Also grab any other DLLs we might need
Get-ChildItem -Path $tempExtract -Filter "*.dll" -Recurse | ForEach-Object {
    Copy-Item $_.FullName $mpvDevDir -Force
}

# Cleanup
Remove-Item -Force $tempFile -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force $tempExtract -ErrorAction SilentlyContinue

$env:MPV_LIB_DIR = $mpvDevDir
Write-Host ""
Write-Host "Done! mpv dev files extracted to: $mpvDevDir"
Write-Host "MPV_LIB_DIR set to: $mpvDevDir"
Write-Host ""
Write-Host "You can now build with:"
Write-Host "  cargo build --release"
Write-Host ""
Write-Host "To make this permanent, add to your PowerShell profile:"
Write-Host "  `$env:MPV_LIB_DIR = '$mpvDevDir'"

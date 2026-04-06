@echo off
echo Setting up Rust environment...
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

echo.
echo Checking Rust installation...
cargo --version
if %errorlevel% neq 0 (
    echo ERROR: Cargo not found. Please restart PowerShell.
    pause
    exit /b 1
)

echo.
echo Building Squan Desktop (Tauri)...
echo This will take 10-15 minutes the first time.
echo.

cargo tauri build

if %errorlevel% equ 0 (
    echo.
    echo ===================================
    echo BUILD SUCCESSFUL!
    echo ===================================
    echo.
    echo Portable executable:
    echo src-tauri\target\release\squan.exe
    echo.
    echo Installers:
    echo src-tauri\target\release\bundle\nsis\
    echo.
    echo Test it now:
    echo .\src-tauri\target\release\squan.exe
    echo.
) else (
    echo.
    echo ===================================
    echo BUILD FAILED!
    echo ===================================
    echo.
    echo Common fixes:
    echo 1. Install Visual Studio Build Tools
    echo 2. Restart PowerShell
    echo 3. Check BUILD_TAURI_NOW.md for troubleshooting
    echo.
)

pause

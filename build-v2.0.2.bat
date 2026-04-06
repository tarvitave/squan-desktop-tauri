@echo off
echo ================================
echo Building Squan v2.0.2
echo ================================
echo.

REM Add Rust to PATH
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

REM Verify Rust is available
echo Checking Rust installation...
cargo --version
if errorlevel 1 (
    echo ERROR: Cargo not found!
    echo Please install Rust from: https://rustup.rs/
    pause
    exit /b 1
)
echo.

REM Build release version
echo Building release version...
echo This will take 10-15 minutes on first build...
echo.
cargo tauri build --release

if errorlevel 1 (
    echo.
    echo ================================
    echo BUILD FAILED!
    echo ================================
    echo.
    echo Common issues:
    echo 1. Visual Studio Build Tools not installed
    echo 2. Missing dependencies
    echo.
    echo See error messages above for details.
    pause
    exit /b 1
)

echo.
echo ================================
echo BUILD SUCCESSFUL!
echo ================================
echo.
echo Output location:
echo src-tauri\target\release\squan.exe
echo.
echo Next steps:
echo 1. Test: .\src-tauri\target\release\squan.exe
echo 2. Rename: copy src-tauri\target\release\squan.exe Squan-2.0.2.exe
echo 3. Upload: gh release create v2.0.2 Squan-2.0.2.exe
echo.
pause

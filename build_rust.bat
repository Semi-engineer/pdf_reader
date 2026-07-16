@echo off
REM Build script for DocLens Rust Edition

echo Building DocLens (Rust)...
echo.

REM Check if Rust is installed
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Cargo not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

REM Clean previous build
echo Cleaning previous build...
cargo clean

REM Build release version
echo Building release version...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================
    echo Build successful!
    echo ========================================
    echo.
    echo Executable location: target\release\doclens.exe
    echo.
    echo Run the application with:
    echo   cargo run --release
    echo.
    echo Or directly:
    echo   target\release\doclens.exe
    echo.
) else (
    echo.
    echo ========================================
    echo Build failed!
    echo ========================================
    echo.
    echo Please check the error messages above.
    echo.
)

pause

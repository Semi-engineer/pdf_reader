@echo off
REM Run script for PDF Reader

echo Starting PDF Reader...
echo.

if not exist build\bin\Release\PDFReader.exe (
    echo ERROR: PDFReader.exe not found!
    echo Please build the project first by running: build.bat
    echo.
    pause
    exit /b 1
)

REM Run with optional PDF file argument
if "%~1"=="" (
    start "" "build\bin\Release\PDFReader.exe"
) else (
    start "" "build\bin\Release\PDFReader.exe" "%~1"
)

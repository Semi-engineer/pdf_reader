@echo off
REM Build script for PDF Reader on Windows

echo ====================================
echo PDF Reader - Build Script
echo ====================================
echo.

REM Check if build directory exists
if not exist build (
    echo Creating build directory...
    mkdir build
)

cd build

echo Configuring CMake...
cmake .. -G "Visual Studio 17 2022" -A x64

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: CMake configuration failed!
    echo.
    echo Please ensure:
    echo - Qt6 is installed and in CMAKE_PREFIX_PATH
    echo - Poppler-Qt6 is installed
    echo - Visual Studio 2022 is installed
    echo.
    pause
    exit /b 1
)

echo.
echo Building project...
cmake --build . --config Release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Build failed!
    pause
    exit /b 1
)

cd ..

echo.
echo ====================================
echo Build completed successfully!
echo ====================================
echo.
echo Executable location: build\bin\Release\PDFReader.exe
echo.
echo To run the application, execute:
echo   .\build\bin\Release\PDFReader.exe
echo.
pause

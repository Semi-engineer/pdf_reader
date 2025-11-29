@echo off
REM Setup script for installing dependencies on Windows using vcpkg

echo ====================================
echo PDF Reader - Dependency Setup
echo ====================================
echo.
echo This script will help you install the required dependencies.
echo.

REM Check if vcpkg is installed
where vcpkg >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo WARNING: vcpkg not found in PATH!
    echo.
    echo Please install vcpkg first:
    echo   1. git clone https://github.com/Microsoft/vcpkg.git
    echo   2. cd vcpkg
    echo   3. .\bootstrap-vcpkg.bat
    echo   4. Add vcpkg to your PATH
    echo.
    echo Or specify the vcpkg directory when prompted.
    echo.
    set /p VCPKG_DIR="Enter vcpkg directory (or press Enter to exit): "
    
    if "%VCPKG_DIR%"=="" (
        echo Exiting...
        pause
        exit /b 1
    )
    
    set VCPKG_EXE=%VCPKG_DIR%\vcpkg.exe
) else (
    set VCPKG_EXE=vcpkg
)

echo.
echo Installing Qt6...
%VCPKG_EXE% install qt6-base:x64-windows

if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Failed to install Qt6
    pause
    exit /b 1
)

echo.
echo Installing Poppler with Qt6 support...
%VCPKG_EXE% install poppler[qt6]:x64-windows

if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Failed to install Poppler
    pause
    exit /b 1
)

echo.
echo Integrating vcpkg with Visual Studio...
%VCPKG_EXE% integrate install

echo.
echo ====================================
echo Setup Complete!
echo ====================================
echo.
echo Dependencies installed successfully.
echo You can now build the project using build.bat
echo.
pause

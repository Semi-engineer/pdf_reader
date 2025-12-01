@echo off
REM Deployment script for PDF Viewer (Windows) - Commercial Edition Support

echo ===================================
echo DocLens Deployment Script
echo Commercial Edition Support
echo ===================================

REM Check Python version
python --version

REM Select Edition
echo.
echo Select Edition:
echo 1. Free Edition (MIT License)
echo 2. Commercial Edition (Proprietary License)
echo.
set /p edition="Enter your choice (1 or 2): "

if "%edition%"=="2" (
    set EDITION=commercial
    set LICENSE_TYPE=proprietary
    echo Selected: Commercial Edition
) else (
    set EDITION=free
    set LICENSE_TYPE=MIT
    echo Selected: Free Edition
)

REM Create virtual environment if not exists
if not exist ".venv" (
    echo Creating virtual environment...
    python -m venv .venv
)

REM Activate virtual environment
echo Activating virtual environment...
call .venv\Scripts\activate.bat

REM Upgrade pip
echo Upgrading pip...
python -m pip install --upgrade pip

REM Install dependencies
echo Installing dependencies...
pip install -r requirements.txt

REM Install build tools
echo Installing build tools...
pip install build twine pyinstaller

REM Set build configuration
if "%EDITION%"=="commercial" (
    echo.
    echo ===================================
    echo Commercial Edition Configuration
    echo ===================================
    set /p company_name="Company Name: "
    set /p product_name="Product Name [DocLens Pro]: "
    if "%product_name%"=="" set product_name=DocLens Pro
    set /p version="Version [1.0.0]: "
    if "%version%"=="" set version=1.0.0
    set /p license_server="License Server URL (optional): "
    
    REM Create commercial config
    echo Creating commercial configuration...
    (
        echo {
        echo   "edition": "commercial",
        echo   "company": "%company_name%",
        echo   "product": "%product_name%",
        echo   "version": "%version%",
        echo   "license_type": "proprietary",
        echo   "license_server": "%license_server%",
        echo   "features": {
        echo     "watermark": false,
        echo     "trial_days": 30,
        echo     "require_activation": true,
        echo     "advanced_features": true
        echo   }
        echo }
    ) > build_config.json
) else (
    REM Create free config
    echo Creating free edition configuration...
    (
        echo {
        echo   "edition": "free",
        echo   "product": "DocLens",
        echo   "version": "1.0.0",
        echo   "license_type": "MIT",
        echo   "features": {
        echo     "watermark": false,
        echo     "trial_days": 0,
        echo     "require_activation": false,
        echo     "advanced_features": false
        echo   }
        echo }
    ) > build_config.json
)

REM Build package
echo.
echo Building package...
python -m build

REM Build executable
echo.
set /p build_exe="Do you want to build executable? (y/n): "
if /i "%build_exe%"=="y" (
    echo Building executable for %EDITION% edition...
    python build_exe.py --edition=%EDITION%
    
    if "%EDITION%"=="commercial" (
        echo.
        echo Creating installer...
        set /p create_installer="Create installer package? (y/n): "
        if /i "%create_installer%"=="y" (
            echo Building installer...
            if exist "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" (
                "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer_commercial.iss
            ) else (
                echo Inno Setup not found. Skipping installer creation.
                echo Download from: https://jrsoftware.org/isdl.php
            )
        )
    )
)

echo.
echo ===================================
echo Deployment complete!
echo ===================================
echo.
echo Edition: %EDITION%
echo License: %LICENSE_TYPE%
echo.
if "%EDITION%"=="commercial" (
    echo Commercial Edition Features:
    echo   - No watermark
    echo   - 30-day trial period
    echo   - License activation required
    echo   - Advanced features enabled
    echo   - Custom branding
    echo.
    echo Next steps:
    echo   1. Configure license server
    echo   2. Set up activation system
    echo   3. Distribute to customers
    echo.
    echo Executable: dist\%product_name%.exe
    echo Installer: dist\%product_name%_Setup.exe
) else (
    echo Free Edition Features:
    echo   - Open source (MIT License)
    echo   - No activation required
    echo   - Basic features
    echo.
    echo To install the package:
    echo   pip install dist\doclens-1.0.0-py3-none-any.whl
    echo.
    echo To upload to PyPI:
    echo   twine upload dist\*
    echo.
    echo Executable: dist\DocLens.exe
)
echo.

pause

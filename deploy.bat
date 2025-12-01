@echo off
REM Deployment script for PDF Viewer (Windows)

echo ===================================
echo PDF Viewer Deployment Script
echo ===================================

REM Check Python version
python --version

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

REM Build package
echo Building package...
python -m build

REM Build executable
set /p build_exe="Do you want to build executable? (y/n): "
if /i "%build_exe%"=="y" (
    echo Building executable...
    python build_exe.py
)

echo.
echo ===================================
echo Deployment complete!
echo ===================================
echo.
echo To install the package:
echo   pip install dist\pdf_viewer-1.0.0-py3-none-any.whl
echo.
echo To upload to PyPI:
echo   twine upload dist\*
echo.
echo Executable location (if built):
echo   dist\PDFViewer.exe
echo.

pause

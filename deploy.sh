#!/bin/bash
# Deployment script for PDF Viewer

set -e

echo "==================================="
echo "PDF Viewer Deployment Script"
echo "==================================="

# Check Python version
python_version=$(python3 --version 2>&1 | awk '{print $2}')
echo "Python version: $python_version"

# Create virtual environment if not exists
if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
fi

# Activate virtual environment
echo "Activating virtual environment..."
source .venv/bin/activate

# Upgrade pip
echo "Upgrading pip..."
pip install --upgrade pip

# Install dependencies
echo "Installing dependencies..."
pip install -r requirements.txt

# Install build tools
echo "Installing build tools..."
pip install build twine pyinstaller

# Build package
echo "Building package..."
python -m build

# Build executable (optional)
read -p "Do you want to build executable? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Building executable..."
    python build_exe.py
fi

echo ""
echo "==================================="
echo "Deployment complete!"
echo "==================================="
echo ""
echo "To install the package:"
echo "  pip install dist/pdf_viewer-1.0.0-py3-none-any.whl"
echo ""
echo "To upload to PyPI:"
echo "  twine upload dist/*"
echo ""
echo "Executable location (if built):"
echo "  dist/PDFViewer"
echo ""

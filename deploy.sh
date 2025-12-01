#!/bin/bash
# Deployment script for PDF Viewer - Commercial Edition Support

set -e

echo "==================================="
echo "DocLens Deployment Script"
echo "Commercial Edition Support"
echo "==================================="

# Check Python version
python_version=$(python3 --version 2>&1 | awk '{print $2}')
echo "Python version: $python_version"

# Select Edition
echo ""
echo "Select Edition:"
echo "1. Free Edition (MIT License)"
echo "2. Commercial Edition (Proprietary License)"
echo ""
read -p "Enter your choice (1 or 2): " edition

if [ "$edition" == "2" ]; then
    EDITION="commercial"
    LICENSE_TYPE="proprietary"
    echo "Selected: Commercial Edition"
else
    EDITION="free"
    LICENSE_TYPE="MIT"
    echo "Selected: Free Edition"
fi

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

# Set build configuration
if [ "$EDITION" == "commercial" ]; then
    echo ""
    echo "==================================="
    echo "Commercial Edition Configuration"
    echo "==================================="
    read -p "Company Name: " company_name
    read -p "Product Name [DocLens Pro]: " product_name
    product_name=${product_name:-"DocLens Pro"}
    read -p "Version [1.0.0]: " version
    version=${version:-"1.0.0"}
    read -p "License Server URL (optional): " license_server
    
    # Create commercial config
    echo "Creating commercial configuration..."
    cat > build_config.json <<EOF
{
  "edition": "commercial",
  "company": "$company_name",
  "product": "$product_name",
  "version": "$version",
  "license_type": "proprietary",
  "license_server": "$license_server",
  "features": {
    "watermark": false,
    "trial_days": 30,
    "require_activation": true,
    "advanced_features": true
  }
}
EOF
else
    # Create free config
    echo "Creating free edition configuration..."
    cat > build_config.json <<EOF
{
  "edition": "free",
  "product": "DocLens",
  "version": "1.0.0",
  "license_type": "MIT",
  "features": {
    "watermark": false,
    "trial_days": 0,
    "require_activation": false,
    "advanced_features": false
  }
}
EOF
fi

# Build package
echo ""
echo "Building package..."
python -m build

# Build executable (optional)
echo ""
read -p "Do you want to build executable? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Building executable for $EDITION edition..."
    python build_exe.py --edition=$EDITION
    
    if [ "$EDITION" == "commercial" ]; then
        echo ""
        read -p "Create installer package? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "Building installer..."
            # For macOS, create DMG
            if [[ "$OSTYPE" == "darwin"* ]]; then
                if command -v create-dmg &> /dev/null; then
                    create-dmg \
                        --volname "$product_name" \
                        --window-pos 200 120 \
                        --window-size 800 400 \
                        --icon-size 100 \
                        --app-drop-link 600 185 \
                        "dist/${product_name// /_}_Setup.dmg" \
                        "dist/$product_name.app"
                else
                    echo "create-dmg not found. Install with: brew install create-dmg"
                fi
            # For Linux, create AppImage or DEB
            elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
                echo "Creating Linux package..."
                # Add Linux packaging logic here
            fi
        fi
    fi
fi

echo ""
echo "==================================="
echo "Deployment complete!"
echo "==================================="
echo ""
echo "Edition: $EDITION"
echo "License: $LICENSE_TYPE"
echo ""
if [ "$EDITION" == "commercial" ]; then
    echo "Commercial Edition Features:"
    echo "  - No watermark"
    echo "  - 30-day trial period"
    echo "  - License activation required"
    echo "  - Advanced features enabled"
    echo "  - Custom branding"
    echo ""
    echo "Next steps:"
    echo "  1. Configure license server"
    echo "  2. Set up activation system"
    echo "  3. Distribute to customers"
    echo ""
    echo "Executable: dist/$product_name"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Installer: dist/${product_name// /_}_Setup.dmg"
    fi
else
    echo "Free Edition Features:"
    echo "  - Open source (MIT License)"
    echo "  - No activation required"
    echo "  - Basic features"
    echo ""
    echo "To install the package:"
    echo "  pip install dist/doclens-1.0.0-py3-none-any.whl"
    echo ""
    echo "To upload to PyPI:"
    echo "  twine upload dist/*"
    echo ""
    echo "Executable: dist/DocLens"
fi
echo ""

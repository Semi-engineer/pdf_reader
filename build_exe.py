"""
Build executable using Nuitka for DocLens
Run: python build_exe.py
"""

import subprocess
import sys
import json
from pathlib import Path

# Get the directory of this script
script_dir = Path(__file__).parent

# Load build configuration
config = {}
config_path = script_dir / "build_config.json"
if config_path.exists():
    with open(config_path, 'r') as f:
        config = json.load(f)

# Determine app name based on edition
edition = config.get('edition', 'free')
if edition == 'commercial':
    app_name = config.get('product', 'DocLens Pro').replace(' ', '')
else:
    app_name = 'DocLens'

# Nuitka arguments
args = [
    sys.executable,
    '-m', 'nuitka',
    'main.py',
    '--standalone',  # Create standalone distribution
    '--onefile',     # Single executable file (use --standalone for directory mode)
    f'--output-filename={app_name}',
    '--enable-plugin=pyside6',  # PySide6 plugin
    '--assume-yes-for-downloads',  # Auto-download dependencies
    '--remove-output',  # Clean previous builds
    
    # Include data files
    '--include-data-dir=icon=icon',
    
    # Performance optimizations
    '--lto=yes',  # Link Time Optimization
    '--jobs=4',   # Parallel compilation
]

# Platform-specific settings
if sys.platform == 'win32':
    args.append('--windows-disable-console')  # No console window
    # Windows icon
    icon_path = script_dir / 'icon' / 'icon.ico'
    if icon_path.exists():
        args.append(f'--windows-icon-from-ico={icon_path}')
    # Company info (optional)
    args.append('--windows-company-name=DocLens')
    args.append(f'--windows-product-name={app_name}')
    args.append('--windows-file-version=1.0.0.0')
    args.append('--windows-product-version=1.0.0.0')
elif sys.platform == 'darwin':
    args.append('--macos-disable-console')  # No console window
    # macOS icon
    icon_path = script_dir / 'icon' / 'icon.icns'
    if icon_path.exists():
        args.append(f'--macos-app-icon={icon_path}')
    args.append(f'--macos-app-name={app_name}')
    args.append('--macos-create-app-bundle')
else:
    args.append('--linux-icon=icon/icon.ico')

# Run Nuitka
print("Starting Nuitka build...")
print(f"Command: {' '.join(args)}")
print("="*50)

result = subprocess.run(args, cwd=script_dir)

if result.returncode == 0:
    print("\n" + "="*50)
    print("Build complete!")
    print(f"Edition: {edition}")
    if sys.platform == 'win32':
        print(f"  Windows: {app_name}.exe")
    elif sys.platform == 'darwin':
        print(f"  macOS: {app_name}.app")
    else:
        print(f"  Linux: {app_name}")
    print("="*50)
else:
    print("\n" + "="*50)
    print("Build failed!")
    print(f"Exit code: {result.returncode}")
    print("="*50)
    sys.exit(result.returncode)

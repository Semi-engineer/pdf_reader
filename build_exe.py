"""
Build executable using PyInstaller for DocLens
Run: python build_exe.py
"""

import PyInstaller.__main__
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

# PyInstaller arguments
args = [
    'main.py',
    f'--name={app_name}',
    '--windowed',  # No console window
    '--onefile',   # Single executable
    '--clean',
    '--noconfirm',
    
    # Include all Python modules
    '--hidden-import=PySide6',
    '--hidden-import=PyMuPDF',
    '--hidden-import=fitz',
    '--hidden-import=Pillow',
    '--hidden-import=PIL',
    
    # Collect all submodules
    '--collect-all=PySide6',
    '--collect-all=fitz',
    
    # Additional modules
    f'--additional-hooks-dir={script_dir}',
    
    # Add icon directory
    '--add-data=icon:icon',
]

# Platform-specific settings
if sys.platform == 'win32':
    # Windows icon
    icon_path = script_dir / 'icon' / 'icon.ico'
    if icon_path.exists():
        args.append(f'--icon={icon_path}')
elif sys.platform == 'darwin':
    # macOS icon (convert from .ico if needed)
    icon_path = script_dir / 'icon' / 'icon.icns'
    if icon_path.exists():
        args.append(f'--icon={icon_path}')
    else:
        # Try to use .ico on macOS (PyInstaller can handle it)
        icon_path = script_dir / 'icon' / 'icon.ico'
        if icon_path.exists():
            args.append(f'--icon={icon_path}')

# Run PyInstaller
PyInstaller.__main__.run(args)

print("\n" + "="*50)
print("Build complete!")
print(f"Edition: {edition}")
print(f"Executable location: dist/{app_name}")
if sys.platform == 'win32':
    print(f"  Windows: dist/{app_name}.exe")
elif sys.platform == 'darwin':
    print(f"  macOS: dist/{app_name}.app")
else:
    print(f"  Linux: dist/{app_name}")
print("="*50)

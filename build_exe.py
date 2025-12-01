"""
Build executable using PyInstaller
Run: python build_exe.py
"""

import PyInstaller.__main__
import sys
from pathlib import Path

# Get the directory of this script
script_dir = Path(__file__).parent

# PyInstaller arguments
args = [
    'main.py',
    '--name=PDFViewer',
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
    
    # Add data files
    f'--add-data={script_dir / "README.md"}{";." if sys.platform == "win32" else ":"}.',
    
    # Collect all submodules
    '--collect-all=PySide6',
    '--collect-all=fitz',
    
    # Additional modules
    f'--additional-hooks-dir={script_dir}',
]

# Platform-specific settings
if sys.platform == 'win32':
    # Windows icon (if you have one)
    # args.append('--icon=icon.ico')
    pass
elif sys.platform == 'darwin':
    # macOS icon (if you have one)
    # args.append('--icon=icon.icns')
    pass

# Run PyInstaller
PyInstaller.__main__.run(args)

print("\n" + "="*50)
print("Build complete!")
print("Executable location: dist/PDFViewer")
print("="*50)

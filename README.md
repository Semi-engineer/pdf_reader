# PDF Viewer

A feature-rich PDF viewer built with PySide6 and PyMuPDF.

## Features

- Continuous scroll viewing
- Thumbnail sidebar
- Background rendering with page cache
- Smooth zoom (Ctrl + Mouse Wheel)
- Text search with highlighting
- Text selection and copy
- Annotations (highlight, rectangle)
- Two-page view mode
- Page rotation
- Dark mode
- Export pages as images
- Session restore
- Keyboard shortcuts

## Installation

```bash
pip install -r requirements.txt
```

## Usage

```bash
python main.py
```

## Keyboard Shortcuts

- `Ctrl+O` - Open file
- `Ctrl+S` - Save copy
- `Ctrl+F` - Search
- `Ctrl+Plus` - Zoom in
- `Ctrl+Minus` - Zoom out
- `Page Up/Down` - Navigate pages
- `Left/Right Arrow` - Navigate pages
- `Space` - Scroll down
- `Home/End` - First/Last page

## Requirements

- Python 3.8+
- PySide6
- PyMuPDF (fitz)
- Pillow

# PDF Reader

A modern, feature-rich PDF viewer built with Qt6 and Poppler-Qt6.

![PDF Reader](screenshot.png)

## Features

✨ **Core Features**
- 📖 Open and view PDF documents
- 🔍 Zoom in/out with smooth scaling (25% - 400%)
- 🔄 Rotate pages (90° increments)
- ⬅️➡️ Navigate between pages (Next, Previous, First, Last)
- 🖼️ Thumbnail view for quick navigation
- 🔎 Full-text search across all pages
- 🖨️ Print documents
- ⌨️ Keyboard shortcuts for all major functions
- 🌓 Modern dark theme UI

## Requirements

### Build Dependencies
- **Qt 6.x** (Core, Gui, Widgets)
- **Poppler-Qt6** (PDF rendering library)
- **CMake 3.16+**
- **C++17 compatible compiler**

### Runtime Dependencies
- Qt6 libraries
- Poppler-Qt6 library

## Installation

### Windows

#### Installing Dependencies

1. **Install Qt6**
   - Download Qt from [qt.io](https://www.qt.io/download)
   - Or use vcpkg:
     ```powershell
     vcpkg install qt6-base:x64-windows
     ```

2. **Install Poppler-Qt6**
   ```powershell
   vcpkg install poppler[qt6]:x64-windows
   ```

3. **Set up environment**
   ```powershell
   # Add Qt to PATH
   $env:PATH += ";C:\Qt\6.x.x\msvc2019_64\bin"
   
   # Set CMAKE_PREFIX_PATH
   $env:CMAKE_PREFIX_PATH = "C:\Qt\6.x.x\msvc2019_64;C:\vcpkg\installed\x64-windows"
   ```

### Linux (Ubuntu/Debian)

```bash
# Install Qt6
sudo apt install qt6-base-dev

# Install Poppler-Qt6
sudo apt install libpoppler-qt6-dev

# Install CMake
sudo apt install cmake build-essential
```

### macOS

```bash
# Using Homebrew
brew install qt@6
brew install poppler-qt6
brew install cmake
```

## Building

### Using CMake (All Platforms)

```bash
# Create build directory
mkdir build
cd build

# Configure
cmake ..

# Build
cmake --build . --config Release

# Run
./bin/PDFReader  # Linux/macOS
.\bin\Release\PDFReader.exe  # Windows
```

### Windows Quick Build Script

```powershell
# build.bat
@echo off
if not exist build mkdir build
cd build
cmake .. -G "Visual Studio 17 2022" -A x64
cmake --build . --config Release
cd ..
echo Build complete! Run: .\build\bin\Release\PDFReader.exe
```

### Linux/macOS Quick Build Script

```bash
#!/bin/bash
# build.sh
mkdir -p build
cd build
cmake ..
make -j$(nproc)
cd ..
echo "Build complete! Run: ./build/bin/PDFReader"
```

## Usage

### Opening a PDF

1. **From the application:**
   - Click `File → Open` or press `Ctrl+O`
   - Select a PDF file

2. **From command line:**
   ```bash
   PDFReader path/to/document.pdf
   ```

### Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Open File | `Ctrl+O` |
| Print | `Ctrl+P` |
| Quit | `Ctrl+Q` |
| Zoom In | `Ctrl++` |
| Zoom Out | `Ctrl+-` |
| Actual Size | `Ctrl+0` |
| Full Screen | `F11` |
| Next Page | `Page Down` / `→` |
| Previous Page | `Page Up` / `←` |
| First Page | `Home` |
| Last Page | `End` |
| Rotate Left | `Ctrl+L` |
| Rotate Right | `Ctrl+R` |
| Search | `Ctrl+F` |

### Navigation

- **Page Navigation:**
  - Use toolbar buttons (◀ ▶)
  - Use page number spinner
  - Click thumbnails in the sidebar
  - Use keyboard shortcuts

- **Zoom:**
  - Use toolbar buttons (+ -)
  - Use zoom slider
  - `Ctrl + Mouse Wheel`
  - `View → Fit to Window`

- **Rotation:**
  - Use toolbar rotation buttons
  - `Navigate → Rotate Left/Right`

### Search

1. Press `Ctrl+F` or click the search button
2. Enter search text
3. Click "Search"
4. Results show page numbers and match counts
5. Click a result to jump to that page

## Project Structure

```
pdf_reader/
├── CMakeLists.txt          # CMake build configuration
├── README.md               # This file
├── src/
│   ├── main.cpp           # Application entry point
│   ├── MainWindow.h/cpp   # Main window with UI
│   ├── PDFViewer.h/cpp    # PDF rendering widget
│   ├── ThumbnailWidget.h/cpp  # Thumbnail sidebar
│   └── SearchDialog.h/cpp # Search dialog
└── build/                 # Build directory (generated)
```

## Architecture

### Components

1. **MainWindow**
   - Application main window
   - Menu bar, toolbar, status bar
   - Manages all UI components
   - Handles user actions

2. **PDFViewer**
   - Core PDF rendering widget
   - Handles zoom, rotation, navigation
   - Uses Poppler-Qt6 for rendering
   - Implements smooth scrolling

3. **ThumbnailWidget**
   - Displays page thumbnails
   - Quick page navigation
   - Visual page overview

4. **SearchDialog**
   - Text search functionality
   - Results display
   - Page navigation from results

### Technologies

- **Qt6 Widgets**: Modern C++ GUI framework
- **Poppler-Qt6**: PDF rendering engine
- **CMake**: Cross-platform build system

## Customization

### Changing Theme Colors

Edit the stylesheet in `MainWindow.cpp`:

```cpp
// Primary color (cyan/teal)
#0d7377  // Normal state
#14ffec  // Hover/active state

// Background colors
#2b2b2b  // Main background
#3c3c3c  // Secondary background
#4a4a4a  // Tertiary background
```

### Adding Features

The codebase is modular and easy to extend:

- Add new actions in `MainWindow::createActions()`
- Implement functionality in respective classes
- Connect signals and slots

## Troubleshooting

### Common Issues

1. **"Cannot find Poppler-Qt6"**
   - Ensure Poppler is installed with Qt6 support
   - Set `CMAKE_PREFIX_PATH` to include Poppler installation

2. **"Qt6 not found"**
   - Install Qt6 development packages
   - Add Qt6 to `CMAKE_PREFIX_PATH`

3. **PDF not rendering**
   - Check if Poppler-Qt6 is properly linked
   - Verify PDF file is not corrupted or password-protected

4. **Slow performance**
   - Large PDFs may take time to generate thumbnails
   - Reduce thumbnail size in `ThumbnailWidget.cpp`

## Contributing

Contributions are welcome! Areas for improvement:

- [ ] Annotation support
- [ ] Bookmarks
- [ ] Form filling
- [ ] Multiple document tabs
- [ ] Recent files list
- [ ] Continuous scroll mode
- [ ] Two-page view
- [ ] Export to images

## License

This project is provided as-is for educational and personal use.

## Credits

- **Qt Framework**: [qt.io](https://www.qt.io/)
- **Poppler**: [poppler.freedesktop.org](https://poppler.freedesktop.org/)

## Support

For issues and questions:
- Check the troubleshooting section
- Review Qt6 and Poppler documentation
- Ensure all dependencies are correctly installed

---

**Built with ❤️ using Qt6 and Poppler**

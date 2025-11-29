# PDF Reader - Installation Guide

## Quick Start Guide

### Windows Installation

#### Prerequisites
1. **Visual Studio 2022** (Community Edition is free)
   - Download from: https://visualstudio.microsoft.com/
   - Install "Desktop development with C++" workload

2. **CMake**
   - Download from: https://cmake.org/download/
   - Or install via: `winget install Kitware.CMake`

3. **vcpkg** (Package Manager)
   ```powershell
   # Clone vcpkg
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   .\bootstrap-vcpkg.bat
   
   # Add to PATH (optional)
   $env:PATH += ";$(pwd)"
   ```

#### Install Dependencies

```powershell
# Install Qt6
vcpkg install qt6-base:x64-windows

# Install Poppler with Qt6 support
vcpkg install poppler[qt6]:x64-windows

# Integrate vcpkg with Visual Studio
vcpkg integrate install
```

#### Build the Project

```powershell
# Navigate to project directory
cd e:\dev\pdf_reader

# Run build script
.\build.bat

# Or build manually:
mkdir build
cd build
cmake .. -G "Visual Studio 17 2022" -A x64 -DCMAKE_TOOLCHAIN_FILE=C:/vcpkg/scripts/buildsystems/vcpkg.cmake
cmake --build . --config Release
```

#### Run the Application

```powershell
# Run without opening a file
.\run.bat

# Or run with a PDF file
.\run.bat "path\to\document.pdf"

# Or directly
.\build\bin\Release\PDFReader.exe
```

---

### Linux (Ubuntu/Debian) Installation

#### Install Dependencies

```bash
# Update package list
sudo apt update

# Install build tools
sudo apt install build-essential cmake git

# Install Qt6
sudo apt install qt6-base-dev qt6-base-dev-tools

# Install Poppler-Qt6
sudo apt install libpoppler-qt6-dev

# Optional: Install Qt6 documentation
sudo apt install qt6-documentation-tools
```

#### Build the Project

```bash
# Navigate to project directory
cd ~/pdf_reader

# Make build script executable
chmod +x build.sh

# Run build script
./build.sh

# Or build manually:
mkdir build
cd build
cmake ..
make -j$(nproc)
```

#### Run the Application

```bash
# Run the application
./build/bin/PDFReader

# Or with a PDF file
./build/bin/PDFReader /path/to/document.pdf
```

---

### macOS Installation

#### Install Dependencies

```bash
# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install cmake
brew install qt@6
brew install poppler-qt6

# Add Qt6 to PATH
echo 'export PATH="/opt/homebrew/opt/qt@6/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### Build the Project

```bash
# Navigate to project directory
cd ~/pdf_reader

# Make build script executable
chmod +x build.sh

# Run build script
./build.sh

# Or build manually:
mkdir build
cd build
cmake .. -DCMAKE_PREFIX_PATH=/opt/homebrew/opt/qt@6
make -j$(sysctl -n hw.ncpu)
```

#### Run the Application

```bash
# Run the application
./build/bin/PDFReader

# Or with a PDF file
./build/bin/PDFReader /path/to/document.pdf
```

---

## Alternative Installation Methods

### Using Qt Creator

1. **Install Qt Creator**
   - Download from: https://www.qt.io/download-qt-installer
   - Select Qt 6.x during installation

2. **Open Project**
   - Launch Qt Creator
   - File → Open File or Project
   - Select `CMakeLists.txt`

3. **Configure Kit**
   - Select Desktop Qt 6.x kit
   - Click "Configure Project"

4. **Build**
   - Click the hammer icon or press `Ctrl+B`

5. **Run**
   - Click the play icon or press `Ctrl+R`

### Using Visual Studio Code

1. **Install Extensions**
   - C/C++
   - CMake Tools
   - Qt tools

2. **Open Folder**
   - Open `pdf_reader` folder in VS Code

3. **Configure CMake**
   - Press `Ctrl+Shift+P`
   - Select "CMake: Configure"

4. **Build**
   - Press `F7` or click "Build" in status bar

5. **Run**
   - Press `Ctrl+F5`

---

## Troubleshooting

### Windows Issues

**Problem: "Qt6 not found"**
```powershell
# Solution: Set CMAKE_PREFIX_PATH
$env:CMAKE_PREFIX_PATH = "C:\vcpkg\installed\x64-windows"

# Or specify in CMake command:
cmake .. -DCMAKE_PREFIX_PATH="C:\vcpkg\installed\x64-windows"
```

**Problem: "Poppler-Qt6 not found"**
```powershell
# Reinstall with Qt6 support
vcpkg remove poppler
vcpkg install poppler[qt6]:x64-windows
```

**Problem: "MSVCP140.dll missing"**
- Install Visual C++ Redistributable
- Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe

### Linux Issues

**Problem: "Qt6 not found"**
```bash
# Ubuntu 22.04+
sudo apt install qt6-base-dev

# Older versions - add PPA
sudo add-apt-repository ppa:okirby/qt6-backports
sudo apt update
sudo apt install qt6-base-dev
```

**Problem: "Poppler-Qt6 not found"**
```bash
# Install development package
sudo apt install libpoppler-qt6-dev libpoppler-cpp-dev
```

### macOS Issues

**Problem: "Qt6 not found"**
```bash
# Ensure Qt6 is in PATH
export CMAKE_PREFIX_PATH=/opt/homebrew/opt/qt@6

# Or for Intel Macs:
export CMAKE_PREFIX_PATH=/usr/local/opt/qt@6
```

**Problem: "Poppler-Qt6 not found"**
```bash
# Reinstall Poppler
brew reinstall poppler-qt6
```

---

## Verifying Installation

### Check Dependencies

**Windows:**
```powershell
# Check if Qt6 is installed
vcpkg list | Select-String "qt6"

# Check if Poppler is installed
vcpkg list | Select-String "poppler"
```

**Linux:**
```bash
# Check Qt6
dpkg -l | grep qt6

# Check Poppler
dpkg -l | grep poppler
```

**macOS:**
```bash
# Check installations
brew list | grep qt
brew list | grep poppler
```

### Test Build

```bash
# Navigate to project
cd pdf_reader

# Clean build
rm -rf build

# Build
mkdir build && cd build
cmake ..
cmake --build .

# Should output: "Build succeeded"
```

---

## Next Steps

After successful installation:

1. **Run the application** using the instructions above
2. **Open a PDF file** to test functionality
3. **Explore features** (zoom, rotate, search, etc.)
4. **Read the README.md** for usage details

For development:
- Check `src/` directory for source code
- Modify and rebuild to see changes
- Refer to Qt6 and Poppler documentation for API details

---

## Getting Help

- **Qt Documentation**: https://doc.qt.io/qt-6/
- **Poppler Documentation**: https://poppler.freedesktop.org/
- **CMake Documentation**: https://cmake.org/documentation/

For build issues, ensure:
1. All dependencies are installed
2. Environment variables are set correctly
3. CMake can find Qt6 and Poppler
4. Compiler is properly configured

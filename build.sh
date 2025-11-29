#!/bin/bash
# Build script for PDF Reader on Linux/macOS

echo "===================================="
echo "PDF Reader - Build Script"
echo "===================================="
echo

# Create build directory if it doesn't exist
if [ ! -d "build" ]; then
    echo "Creating build directory..."
    mkdir build
fi

cd build

echo "Configuring CMake..."
cmake ..

if [ $? -ne 0 ]; then
    echo
    echo "ERROR: CMake configuration failed!"
    echo
    echo "Please ensure:"
    echo "- Qt6 is installed"
    echo "- Poppler-Qt6 is installed"
    echo "- CMake is installed"
    echo
    exit 1
fi

echo
echo "Building project..."
make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

if [ $? -ne 0 ]; then
    echo
    echo "ERROR: Build failed!"
    exit 1
fi

cd ..

echo
echo "===================================="
echo "Build completed successfully!"
echo "===================================="
echo
echo "Executable location: build/bin/PDFReader"
echo
echo "To run the application, execute:"
echo "  ./build/bin/PDFReader"
echo

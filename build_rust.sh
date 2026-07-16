#!/bin/bash
# Build script for DocLens Rust Edition

echo "Building DocLens (Rust)..."
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Clean previous build
echo "Cleaning previous build..."
cargo clean

# Build release version
echo "Building release version..."
cargo build --release

if [ $? -eq 0 ]; then
    echo
    echo "========================================"
    echo "Build successful!"
    echo "========================================"
    echo
    echo "Executable location: target/release/doclens"
    echo
    echo "Run the application with:"
    echo "  cargo run --release"
    echo
    echo "Or directly:"
    echo "  ./target/release/doclens"
    echo
else
    echo
    echo "========================================"
    echo "Build failed!"
    echo "========================================"
    echo
    echo "Please check the error messages above."
    echo
    exit 1
fi

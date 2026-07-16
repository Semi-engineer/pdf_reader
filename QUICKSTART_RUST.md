# Quick Start Guide - DocLens Rust Edition

Get up and running with DocLens Rust in under 10 minutes!

## Prerequisites

### Windows
1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Run the installer and follow prompts

### macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

### Option 1: Build from Source (Recommended)

```bash
# Navigate to project directory
cd pdf_reader

# Build release version
cargo build --release

# Run the application
cargo run --release
```

### Option 2: Use Build Scripts

**Windows:**
```cmd
build_rust.bat
```

**Linux/macOS:**
```bash
chmod +x build_rust.sh
./build_rust.sh
```

## First Run

1. **Launch the app**
   ```bash
   cargo run --release
   ```

2. **Open a PDF**
   - Click the 📁 "Open" button in the toolbar
   - Or drag and drop a PDF file (coming soon)

3. **Navigate**
   - Use ◀ ▶ buttons to go between pages
   - Or enter a page number directly
   - Use mouse wheel to scroll

4. **Zoom**
   - Click 🔍+ to zoom in
   - Click 🔍- to zoom out
   - Or use Ctrl + Mouse Wheel

5. **Annotate**
   - Click "🎨 Tools" to open tool palette
   - Choose a color
   - Select an annotation type
   - Click and drag on the PDF

## Key Features

### Navigation
```
◀ ▶         Previous/Next page
Page: [__]  Jump to specific page
```

### Zoom
```
🔍-         Zoom out
🔍+         Zoom in
100%        Current zoom level
```

### Rotation
```
↶          Rotate left (counter-clockwise)
↷          Rotate right (clockwise)
```

### Tools
```
✏ Highlight    Highlight text/areas
▭ Rectangle    Draw rectangles
◯ Circle       Draw circles
─ Line         Draw lines
→ Arrow        Draw arrows
✎ Pen          Freehand drawing
T Text         Add text boxes
```

### Sidebar
- Click "📋 Sidebar" to toggle thumbnail view
- Click any thumbnail to jump to that page

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Open File | `Ctrl+O` |
| Previous Page | `Page Up` |
| Next Page | `Page Down` |
| Zoom In | `Ctrl++` |
| Zoom Out | `Ctrl+-` |
| Search | `Ctrl+F` |

## Configuration

Settings are automatically saved in:
- **Windows**: `%APPDATA%\doclens\settings.json`
- **macOS**: `~/Library/Application Support/doclens/settings.json`
- **Linux**: `~/.config/doclens/settings.json`

Edit this file to customize:
```json
{
  "sidebar_visible": true,
  "tool_palette_visible": true,
  "annotation_color": [255, 255, 0, 100]
}
```

## Troubleshooting

### "PDFium library not found"

**Solution 1: Let it auto-detect**
- Usually works on most systems

**Solution 2: Manual installation**

**Windows:**
1. Download pdfium.dll from [Google's PDFium](https://github.com/bblanchon/pdfium-binaries/releases)
2. Place in project root or System32

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get install libpdfium-dev

# Fedora
sudo dnf install pdfium-devel
```

**macOS:**
```bash
brew install pdfium
```

### "Cargo not found"

**Solution:**
```bash
# Reinstall Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your terminal
```

### Slow first build

**This is normal!**
- First build downloads and compiles all dependencies
- Takes 5-10 minutes
- Subsequent builds are much faster (30 seconds)

### Application crashes on startup

**Check:**
1. Rust version: `rustc --version` (should be 1.70+)
2. PDFium library installed
3. Graphics drivers updated

## Tips & Tricks

### Speed up development builds
```bash
# Use debug build for faster compilation
cargo run

# Check without building
cargo check
```

### Better error messages
```bash
# Enable backtraces
export RUST_BACKTRACE=1
cargo run
```

### Reduce binary size
Already optimized in release build:
```toml
[profile.release]
opt-level = 3
lto = true
strip = true
```

Result: ~8-12 MB binary

### Customize cache size

Edit `src/app.rs`:
```rust
page_cache: PageCache::new(50),  // Increase from 30 to 50
```

### Change thumbnail quality

Edit `src/thumbnail_manager.rs`:
```rust
thumbnail_size: 30.0,  // Increase from 25.0 for better quality
```

## Performance Tuning

### For Low-End Systems
```rust
// Reduce cache size
page_cache: PageCache::new(10),

// Reduce thumbnail quality
thumbnail_size: 15.0,
```

### For High-End Systems
```rust
// Increase cache size
page_cache: PageCache::new(100),

// Increase thumbnail quality
thumbnail_size: 35.0,
```

## Common Workflows

### Reading a Document
1. Open PDF
2. Toggle sidebar (📋) for overview
3. Jump to interesting pages via thumbnails
4. Use zoom (🔍+/-) for comfortable reading

### Annotating a Document
1. Open PDF
2. Show tools (🎨)
3. Pick color
4. Select tool (e.g., ✏ Highlight)
5. Click and drag to annotate
6. Save (future: Ctrl+S)

### Searching a Document
1. Open PDF
2. Type in search box
3. Click 🔍 or press Enter
4. Results highlighted automatically

## Next Steps

### Learn More
- Read [README_RUST.md](README_RUST.md) for full documentation
- Read [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) if coming from Python
- Read [COMPARISON.md](COMPARISON.md) for Python vs Rust comparison

### Customize
- Edit `src/app.rs` for application logic
- Edit `src/ui/*.rs` for UI components
- Edit `Cargo.toml` for dependencies

### Contribute
- Report issues
- Submit pull requests
- Improve documentation

## FAQ

**Q: Why is the first build so slow?**  
A: Rust compiles all dependencies from source. Subsequent builds are fast.

**Q: Can I use without installing Rust?**  
A: Yes, if someone provides a pre-built binary for your platform.

**Q: Does it work on Raspberry Pi?**  
A: Yes! Cross-compile for ARM:
```bash
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target=armv7-unknown-linux-gnueabihf
```

**Q: How do I update?**  
A: Pull latest code and rebuild:
```bash
git pull
cargo build --release
```

**Q: Can I package it as a Windows installer?**  
A: Yes, use [Inno Setup](https://jrsoftware.org/isinfo.php) or [WiX](https://wixtoolset.org/).

**Q: Does it support encrypted PDFs?**  
A: Currently no, but PDFium supports it - just needs implementation.

**Q: Can I change the UI theme?**  
A: Yes, egui supports custom themes. Edit the style in `src/app.rs`.

## Getting Help

- **Documentation**: Start with README_RUST.md
- **Examples**: See `src/ui/*.rs` for examples
- **Community**: Join Rust Discord or forums
- **Issues**: Report bugs on GitHub

## Benchmarking Your Build

Test your build performance:
```bash
# Time the build
time cargo build --release

# Check binary size
ls -lh target/release/doclens

# Test startup time
time target/release/doclens
```

## Distribution Checklist

Before distributing your build:

- [ ] Build in release mode
- [ ] Test on clean system
- [ ] Include PDFium library if needed
- [ ] Create README for users
- [ ] Test all major features
- [ ] Check binary size
- [ ] Verify platform compatibility

## Success!

You should now have a working DocLens installation. Open a PDF and start reading!

For detailed features and configuration, see [README_RUST.md](README_RUST.md).

---

**Build time**: ~5-10 minutes (first time)  
**Binary size**: ~8-15 MB  
**Startup time**: <1 second  

Enjoy your blazingly fast PDF viewer! 🦀⚡

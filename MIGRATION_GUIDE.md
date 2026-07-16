# Migration Guide: Python to Rust

This guide helps developers understand the architectural differences between the Python and Rust versions of DocLens.

## Overview

The Rust version maintains the same feature set as the Python version but with significant performance improvements and memory safety guarantees.

## Architecture Comparison

### Python Version
```
PySide6 (Qt) → PyMuPDF (fitz) → Python Runtime
```

### Rust Version
```
egui/eframe → pdfium-render → Native Binary
```

## Code Structure Mapping

### Main Application

| Python | Rust | Notes |
|--------|------|-------|
| `main.py` | `src/main.rs` | Entry point |
| `main_window.py` | `src/app.rs` | Main app logic |
| `MainWindow` class | `DocLensApp` struct | Core application state |

### Components

| Python File | Rust File | Purpose |
|-------------|-----------|---------|
| `page_cache.py` | `src/page_cache.rs` | LRU page caching |
| `render_worker.py` | `src/render_worker.rs` | Background rendering |
| `thumbnail_manager.py` | `src/thumbnail_manager.rs` | Thumbnail generation |
| `annotation_manager.py` | `src/annotation.rs` | Annotation handling |
| `search_manager.py` | `src/search.rs` | Text search |
| `pdf_page_widget.py` | `src/ui/viewer.rs` | PDF display |
| `tool_palette.py` | `src/ui/tool_palette.rs` | Tool palette |

### Configuration

| Python | Rust |
|--------|------|
| `settings.json` in `config/` | `settings.json` in platform config dir |
| JSON with `json` module | Serde JSON serialization |

## Key Differences

### 1. GUI Framework

**Python (PySide6/Qt)**
```python
class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setup_ui()
```

**Rust (egui)**
```rust
impl eframe::App for DocLensApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Immediate mode GUI
    }
}
```

**Key Difference**: Qt uses retained mode (widgets persist), egui uses immediate mode (widgets recreated each frame).

### 2. PDF Rendering

**Python (PyMuPDF)**
```python
import fitz
doc = fitz.open("file.pdf")
page = doc[0]
pix = page.get_pixmap()
```

**Rust (pdfium-render)**
```rust
use pdfium_render::prelude::*;
let pdfium = Pdfium::new(...);
let doc = pdfium.load_pdf_from_file(path, None)?;
let page = doc.pages().get(0)?;
```

### 3. Threading and Concurrency

**Python**
```python
from threading import Thread, Lock
from queue import Queue

# GIL limits true parallelism
```

**Rust**
```rust
use tokio::sync::mpsc;
use rayon::prelude::*;

// True parallelism without GIL
// Thread-safe by default
```

### 4. Memory Management

**Python**
```python
# Garbage collected
# Can have circular references
# Memory freed when no references
```

**Rust**
```rust
// Ownership system
// No garbage collector
// Memory freed when owner goes out of scope
// Arc<T> for shared ownership
```

### 5. Error Handling

**Python**
```python
try:
    doc = fitz.open(path)
except Exception as e:
    print(f"Error: {e}")
```

**Rust**
```rust
use anyhow::Result;

fn open_doc(path: &str) -> Result<Document> {
    let doc = PdfDocument::open(path)?;
    Ok(doc)
}
```

### 6. State Management

**Python**
```python
class MainWindow:
    def __init__(self):
        self.doc = None
        self.current_page = 0
        self.zoom_level = 100
```

**Rust**
```rust
pub struct DocLensApp {
    document: Option<Arc<PdfDocument>>,
    current_page: usize,
    zoom_level: f32,
}
```

## Feature Implementation Comparison

### Page Rendering

**Python**
```python
def render_page(self, page_num, zoom, rotation):
    page = self.doc[page_num]
    mat = fitz.Matrix(zoom/100, zoom/100)
    if rotation:
        mat = mat.prerotate(rotation)
    pix = page.get_pixmap(matrix=mat)
    return QPixmap.fromImage(pix_to_qimage(pix))
```

**Rust**
```rust
fn render_page(&self, page: usize, zoom: f32, rotation: PdfPageRenderRotation) 
    -> Result<ColorImage> 
{
    let page = self.get_page(page)?;
    let render_config = PdfRenderConfig::new()
        .set_target_width((page.width() * zoom / 100.0) as i32)
        .rotate(rotation, true);
    let bitmap = page.render_with_config(&render_config)?;
    Ok(bitmap.as_image_buffer())
}
```

### Annotations

**Python**
```python
class Annotation:
    def __init__(self, page, rect, color, type):
        self.page = page
        self.rect = rect
        self.color = color
        self.type = type
```

**Rust**
```rust
#[derive(Serialize, Deserialize)]
pub struct Annotation {
    pub page: usize,
    pub rect: AnnotationRect,
    pub color: [u8; 4],
    pub annotation_type: AnnotationType,
}
```

### Search

**Python**
```python
class SearchManager:
    def search_text(self, query):
        results = []
        for page in self.doc:
            text_instances = page.search_for(query)
            results.extend(text_instances)
        return results
```

**Rust**
```rust
impl SearchManager {
    pub fn search_text(&mut self, doc: &PdfDocument, query: &str) 
        -> Vec<SearchResult> 
    {
        (0..doc.page_count())
            .filter_map(|page| {
                doc.search_page(page, query).ok()
            })
            .flatten()
            .collect()
    }
}
```

## Performance Improvements

### Startup Time
- **Python**: 2-3 seconds (loading Qt, PyMuPDF, dependencies)
- **Rust**: 0.5-1 second (native binary)

### Memory Usage
- **Python**: ~150-200 MB baseline
- **Rust**: ~50-80 MB baseline

### Rendering Speed
- **Python**: ~100-150ms per page (medium PDFs)
- **Rust**: ~30-60ms per page (medium PDFs)

### Binary Size
- **Python**: 50-100 MB with dependencies
- **Rust**: 8-15 MB (with optimization)

## Development Workflow

### Python
```bash
# Setup
python -m venv .venv
pip install -r requirements.txt

# Run
python main.py

# Package
nuitka --standalone main.py
```

### Rust
```bash
# Setup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run
cargo run --release

# Build
cargo build --release

# Package
# Binary is already standalone: target/release/doclens
```

## Testing

### Python
```python
import unittest

class TestPdfViewer(unittest.TestCase):
    def test_open_file(self):
        window = MainWindow()
        window.open_file("test.pdf")
        self.assertIsNotNone(window.doc)
```

### Rust
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_open_file() {
        let doc = PdfDocument::open("test.pdf");
        assert!(doc.is_ok());
    }
}
```

## Common Patterns

### Singleton/Global State

**Python**
```python
# Global variable or class attribute
_instance = None

def get_instance():
    global _instance
    if _instance is None:
        _instance = Manager()
    return _instance
```

**Rust**
```rust
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref MANAGER: Arc<Mutex<Manager>> = 
        Arc::new(Mutex::new(Manager::new()));
}
```

### Callbacks/Signals

**Python (Qt Signals)**
```python
from PySide6.QtCore import Signal

class Worker(QObject):
    finished = Signal(int)
    
    def run(self):
        result = do_work()
        self.finished.emit(result)
```

**Rust (Channels)**
```rust
use tokio::sync::mpsc;

async fn worker(tx: mpsc::Sender<i32>) {
    let result = do_work().await;
    tx.send(result).await.unwrap();
}
```

## Migration Checklist

When porting code from Python to Rust:

- [ ] Identify core data structures
- [ ] Map Python classes to Rust structs
- [ ] Convert Python threads to Rust async/threads
- [ ] Replace Qt widgets with egui equivalents
- [ ] Add proper error handling with Result<T>
- [ ] Implement Serialize/Deserialize for config
- [ ] Use Arc<T> for shared ownership
- [ ] Use Mutex<T> for shared mutable state
- [ ] Add #[derive(...)] for common traits
- [ ] Write unit tests
- [ ] Profile and optimize hot paths
- [ ] Document public APIs

## Resources

### Rust Learning
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [egui Documentation](https://docs.rs/egui/)

### PDF Processing
- [pdfium-render docs](https://docs.rs/pdfium-render/)
- [PDFium API](https://pdfium.googlesource.com/pdfium/)

### GUI Development
- [egui GitHub](https://github.com/emilk/egui)
- [eframe examples](https://github.com/emilk/egui/tree/master/examples)

## Conclusion

The Rust version provides:
- ✅ Better performance
- ✅ Memory safety
- ✅ Smaller binary size
- ✅ No runtime dependencies
- ✅ True parallelism

Trade-offs:
- ⚠️ Longer compile times
- ⚠️ Steeper learning curve
- ⚠️ Different GUI paradigm (immediate mode)

The choice depends on your priorities: rapid prototyping (Python) vs. production performance (Rust).

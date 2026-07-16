# DocLens Rust Architecture

## System Overview

```
┌────────────────────────────────────────────────────────────┐
│                     DocLens Application                     │
│                    (Native Rust Binary)                     │
└────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│   UI Layer   │      │  Core Logic  │      │   Managers   │
│   (egui)     │◄────►│   (app.rs)   │◄────►│  (various)   │
└──────────────┘      └──────────────┘      └──────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│   PDFium     │      │    Tokio     │      │     LRU      │
│  (rendering) │      │   (async)    │      │   (cache)    │
└──────────────┘      └──────────────┘      └──────────────┘
```

## Component Architecture

```
src/
├── main.rs ─────────────────┐
│                             │
├── app.rs ◄─────────────────┤ Entry & Core
│   └── DocLensApp            │
│       ├── State             │
│       ├── Managers          │
│       └── UI Components     │
│                             │
├── config.rs ◄──────────────┤
│   └── Settings              │ Configuration
│       ├── Load              │
│       └── Save              │
│                             │
├── pdf_document.rs ◄────────┤
│   └── PdfDocument           │
│       ├── Open              │ PDF Operations
│       ├── Render            │
│       ├── Search            │
│       └── Text Extract      │
│                             │
├── page_cache.rs ◄──────────┤
│   └── PageCache             │
│       ├── LRU Cache         │ Caching
│       ├── Get/Put           │
│       └── Thread-safe       │
│                             │
├── render_worker.rs ◄───────┤
│   └── RenderWorker          │
│       ├── Request Queue     │ Async Rendering
│       ├── Response Channel  │
│       └── Tokio Runtime     │
│                             │
├── thumbnail_manager.rs ◄───┤
│   └── ThumbnailManager      │ Thumbnails
│       ├── Generate          │
│       └── Cache             │
│                             │
├── annotation.rs ◄──────────┤
│   ├── Annotation            │
│   ├── AnnotationType        │ Annotations
│   └── AnnotationManager     │
│       ├── Add/Remove        │
│       └── Persist           │
│                             │
├── search.rs ◄──────────────┤
│   └── SearchManager         │ Search
│       ├── Query             │
│       ├── Results           │
│       └── Navigation        │
│                             │
└── ui/ ◄────────────────────┘
    ├── mod.rs
    ├── toolbar.rs ──────────┐
    │   └── Toolbar           │
    ├── sidebar.rs ──────────┤
    │   └── Sidebar           │ UI Components
    ├── viewer.rs ───────────┤
    │   └── PdfViewer         │
    ├── statusbar.rs ────────┤
    │   └── StatusBar         │
    └── tool_palette.rs ─────┘
        └── ToolPalette
```

## Data Flow

### Opening a PDF

```
User Action
    │
    ▼
┌────────────┐
│  UI Event  │ (Open button clicked)
└─────┬──────┘
      │
      ▼
┌─────────────────┐
│  File Dialog    │ (rfd)
└────────┬────────┘
         │ file path
         ▼
┌──────────────────┐
│  DocLensApp      │
│  .open_file()    │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  PdfDocument     │
│  ::open()        │ ← Opens with pdfium
└────────┬─────────┘
         │
         ├────────────────────┐
         │                    │
         ▼                    ▼
┌─────────────────┐  ┌─────────────────┐
│ ThumbnailMgr    │  │ RenderWorker    │
│ .generate()     │  │ .render_page()  │
└─────────────────┘  └─────────────────┘
         │                    │
         ▼                    ▼
┌─────────────────────────────────────┐
│         Page Cache                  │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────┐
│  UI Update      │ (Display rendered page)
└─────────────────┘
```

### Rendering Pipeline

```
┌─────────────────┐
│ Visible Pages   │ (Determined by scroll position)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Check Cache     │
└────┬────────┬───┘
     │        │
  Hit│        │Miss
     │        │
     ▼        ▼
┌────────┐ ┌──────────────┐
│Display │ │ RenderWorker │
│Cached  │ │  Request     │
└────────┘ └──────┬───────┘
                  │
                  ▼
         ┌─────────────────┐
         │ Tokio Runtime   │
         │ + Thread Pool   │
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │ PDFium Render   │
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │ Response Queue  │
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │ Store in Cache  │
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │ Request Repaint │
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │ Display Page    │
         └─────────────────┘
```

## Threading Model

```
┌────────────────────────────────────────────────────────┐
│                    Main Thread                         │
│  ┌──────────────────────────────────────────────┐     │
│  │            egui Event Loop                   │     │
│  │  • UI rendering (immediate mode)             │     │
│  │  • User input handling                       │     │
│  │  • State updates                             │     │
│  └────────────────┬─────────────────────────────┘     │
│                   │                                    │
│                   │ spawn                              │
└───────────────────┼────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────────────────┐
│                 Render Worker Thread                   │
│  ┌──────────────────────────────────────────────┐     │
│  │          Tokio Runtime                       │     │
│  │  • Async request handling                    │     │
│  │  • Task spawning                             │     │
│  │  └───► spawn_blocking ───┐                  │     │
│  └──────────────────────────┼───────────────────┘     │
│                             │                          │
│                             ▼                          │
│               ┌──────────────────────────┐             │
│               │   Rayon Thread Pool      │             │
│               │  • Parallel rendering    │             │
│               │  • CPU-intensive work    │             │
│               └──────────────────────────┘             │
└────────────────────────────────────────────────────────┘

Communication: 
  Main ──(mpsc)──► Render Worker
  Render Worker ──(mpsc)──► Main
```

## Memory Management

```
┌─────────────────────────────────────────────────────┐
│              Memory Ownership Model                 │
└─────────────────────────────────────────────────────┘

DocLensApp
  │
  ├─ document: Option<Arc<PdfDocument>>
  │    │
  │    └─ Shared ownership (reference counted)
  │       Multiple components can read
  │
  ├─ page_cache: PageCache
  │    │
  │    └─ cache: Arc<Mutex<LruCache<...>>>
  │         │
  │         └─ Thread-safe interior mutability
  │
  ├─ managers: Owned
  │    │
  │    ├─ ThumbnailManager
  │    ├─ AnnotationManager
  │    └─ SearchManager
  │
  └─ ui_components: Owned
       │
       ├─ Toolbar
       ├─ Sidebar
       └─ Viewer

Legend:
  Owned        - Single owner, exclusive access
  Arc<T>       - Shared ownership, read-only
  Mutex<T>     - Thread-safe mutable access
  Option<T>    - May or may not exist
```

## Cache Strategy

```
┌─────────────────────────────────────────────┐
│          LRU Page Cache (30 pages)          │
└─────────────────────────────────────────────┘

Cache Key: (page_num, zoom, rotation)

Most Recent                         Least Recent
    │                                      │
    ▼                                      ▼
┌────────┬────────┬────────┬───────┬────────┐
│ Page 5 │ Page 4 │ Page 6 │  ...  │ Page 1 │
│ 100%   │ 100%   │ 100%   │       │ 100%   │
│ 0°     │ 0°     │ 0°     │       │ 0°     │
└────────┴────────┴────────┴───────┴────────┘
     ▲
     │
  Current

On Access:
  1. Check if page in cache
  2. If found: Move to front, return
  3. If not: Request render, add when ready

On Add (cache full):
  1. Remove least recently used
  2. Add new page at front

Benefits:
  ✓ O(1) access time
  ✓ Automatic memory management
  ✓ Predictable memory usage
```

## Annotation System

```
┌─────────────────────────────────────────────┐
│          Annotation Architecture            │
└─────────────────────────────────────────────┘

AnnotationManager
  │
  ├─ annotations: Vec<Annotation>
  │     │
  │     ├─ id: u64 (unique)
  │     ├─ page: usize
  │     ├─ rect: AnnotationRect
  │     ├─ color: [u8; 4] (RGBA)
  │     ├─ annotation_type: AnnotationType
  │     ├─ points: Vec<Point> (for pen/line)
  │     └─ text: Option<String>
  │
  └─ next_id: u64 (auto-increment)

AnnotationType (enum):
  ├─ Highlight
  ├─ Rectangle
  ├─ Circle
  ├─ Line
  ├─ Arrow
  ├─ Pen
  └─ Text

Operations:
  • add_annotation()
  • remove_annotation()
  • get_page_annotations()
  • clear()

Rendering:
  For each page:
    1. Get annotations for that page
    2. Draw each annotation based on type
    3. Apply color and opacity
```

## Configuration System

```
┌─────────────────────────────────────────────┐
│         Settings Management Flow            │
└─────────────────────────────────────────────┘

Startup:
    │
    ▼
┌─────────────────┐
│ Settings::load()│
└────────┬────────┘
         │
         ▼
┌──────────────────────────────┐
│ Read from platform directory │
│  Windows: %APPDATA%          │
│  macOS: ~/Library/...        │
│  Linux: ~/.config/           │
└────────┬─────────────────────┘
         │
         ▼
┌─────────────────────┐
│ Deserialize JSON    │
│ (serde_json)        │
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│ Settings struct     │
│  • last_file        │
│  • last_page        │
│  • zoom_level       │
│  • colors           │
│  • etc.             │
└─────────────────────┘
         │
         │ (Used during session)
         │
         ▼
On Exit:
┌─────────────────────┐
│ Settings::save()    │
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│ Serialize JSON      │
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│ Write to file       │
└─────────────────────┘
```

## Error Handling

```
┌─────────────────────────────────────────────┐
│           Error Propagation Flow            │
└─────────────────────────────────────────────┘

Low Level (pdfium):
    │
    │ Result<_, PdfiumError>
    ▼
┌─────────────────┐
│ pdf_document.rs │
└────────┬────────┘
         │
         │ Result<_, anyhow::Error>
         ▼
┌─────────────────┐
│ render_worker.rs│
└────────┬────────┘
         │
         │ RenderResponse::Error
         ▼
┌─────────────────┐
│ app.rs          │
└────────┬────────┘
         │
         │ Log and handle
         ▼
┌─────────────────┐
│ UI (error msg)  │
└─────────────────┘

Strategy:
  • Use Result<T, E> for recoverable errors
  • Use anyhow::Error for easy propagation
  • Use ? operator for clean code
  • Log errors with eprintln! or logging crate
  • Show user-friendly messages in UI
```

## Performance Optimization Points

```
1. Page Cache
   └─ LRU with configurable size
      Benefits: Faster page switches, less rendering

2. Background Rendering
   └─ Async with tokio + rayon
      Benefits: Non-blocking UI, parallel work

3. Thumbnail Generation
   └─ Lower quality, cached
      Benefits: Fast sidebar, less memory

4. Visible Page Detection
   └─ Only render visible + buffer
      Benefits: Less CPU, faster scrolling

5. Arc<T> for Sharing
   └─ Reference counted pointers
      Benefits: No cloning large data

6. Release Build Optimizations
   └─ LTO, strip, opt-level=3
      Benefits: Smaller, faster binary
```

## Comparison: Python vs Rust Architecture

```
┌──────────────────────────────────────────────────────┐
│                  Python (Original)                    │
├──────────────────────────────────────────────────────┤
│  Qt Event Loop                                       │
│    ├─ QMainWindow                                    │
│    ├─ QWidgets (retained mode)                       │
│    └─ Signal/Slot connections                        │
│                                                       │
│  Threading                                           │
│    ├─ Python threads (GIL limited)                   │
│    ├─ QThread for rendering                          │
│    └─ Queue for communication                        │
│                                                       │
│  Memory                                              │
│    ├─ Garbage collected                              │
│    ├─ Reference counting                             │
│    └─ Manual cache management                        │
└──────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────┐
│                   Rust (New)                          │
├──────────────────────────────────────────────────────┤
│  egui Event Loop                                     │
│    ├─ Immediate mode GUI                             │
│    ├─ State in app struct                            │
│    └─ Direct function calls                          │
│                                                       │
│  Threading                                           │
│    ├─ True parallelism (no GIL)                      │
│    ├─ Tokio async runtime                            │
│    └─ mpsc channels                                  │
│                                                       │
│  Memory                                              │
│    ├─ Ownership system                               │
│    ├─ Compile-time guarantees                        │
│    └─ Arc/Mutex for sharing                          │
└──────────────────────────────────────────────────────┘
```

## Future Architecture Enhancements

```
Potential Additions:

1. Plugin System
   ┌──────────────────┐
   │  Plugin Manager  │
   │  ┌────────────┐  │
   │  │ Plugin API │  │
   │  └────────────┘  │
   │  • Load .so/.dll │
   │  • FFI interface │
   └──────────────────┘

2. Network Sync
   ┌──────────────────┐
   │   Sync Manager   │
   │  ┌────────────┐  │
   │  │Cloud API   │  │
   │  └────────────┘  │
   │  • Upload/DL    │
   │  • Conflicts    │
   └──────────────────┘

3. Database Backend
   ┌──────────────────┐
   │     Database     │
   │  ┌────────────┐  │
   │  │  SQLite    │  │
   │  └────────────┘  │
   │  • Annotations  │
   │  • History      │
   └──────────────────┘
```

---

This architecture provides:
- ✅ Separation of concerns
- ✅ Thread safety
- ✅ Memory safety
- ✅ Performance
- ✅ Maintainability
- ✅ Extensibility

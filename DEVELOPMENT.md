# PDF Reader - Development Notes

## Architecture Overview

### Class Hierarchy

```
QMainWindow
    └── MainWindow
            ├── PDFViewer (QScrollArea)
            │       └── QLabel (for image display)
            ├── ThumbnailWidget (QListWidget)
            └── SearchDialog (QDialog)
```

### Component Responsibilities

#### MainWindow
- **Purpose**: Main application window and UI coordinator
- **Responsibilities**:
  - Create and manage menu bar, toolbar, status bar
  - Handle user actions and route to appropriate components
  - Manage application state
  - Coordinate between different widgets
- **Key Methods**:
  - `openFile()` - Load a PDF document
  - `createActions()` - Set up all QActions
  - `createToolBar()` - Build the toolbar
  - `updateActions()` - Enable/disable actions based on state

#### PDFViewer
- **Purpose**: Core PDF rendering and display
- **Responsibilities**:
  - Load and render PDF pages using Poppler
  - Handle zoom, rotation, and navigation
  - Manage page rendering at different DPI levels
  - Provide search functionality
- **Key Methods**:
  - `loadDocument()` - Load PDF file
  - `renderPage()` - Render current page to QImage
  - `setZoom()` - Change zoom level
  - `rotateLeft/Right()` - Rotate page
  - `search()` - Search text in page
- **Signals**:
  - `pageChanged(int, int)` - Emitted when page changes
  - `zoomChanged(qreal)` - Emitted when zoom changes

#### ThumbnailWidget
- **Purpose**: Display page thumbnails for navigation
- **Responsibilities**:
  - Generate thumbnails for all pages
  - Display thumbnails in a grid layout
  - Handle thumbnail selection
- **Key Methods**:
  - `setDocument()` - Set PDF document
  - `generateThumbnails()` - Create thumbnails
- **Signals**:
  - `pageSelected(int)` - Emitted when user clicks thumbnail

#### SearchDialog
- **Purpose**: Text search interface
- **Responsibilities**:
  - Provide search UI
  - Coordinate search across all pages
  - Display search results
  - Navigate to search results
- **Key Methods**:
  - `performSearch()` - Execute search
  - `onResultClicked()` - Handle result selection

---

## Data Flow

### Opening a PDF

```
User clicks Open
    ↓
MainWindow::onOpenFile()
    ↓
QFileDialog::getOpenFileName()
    ↓
MainWindow::openFile(filePath)
    ↓
PDFViewer::loadDocument(filePath)
    ↓
Poppler::Document::load()
    ↓
PDFViewer::renderPage()
    ↓
Poppler::Page::renderToImage()
    ↓
QLabel::setPixmap()
    ↓
ThumbnailWidget::setDocument()
    ↓
ThumbnailWidget::generateThumbnails()
```

### Zooming

```
User changes zoom
    ↓
MainWindow::onZoomChanged(value)
    ↓
PDFViewer::setZoom(value)
    ↓
PDFViewer::renderPage()
    ↓
emit zoomChanged(value)
    ↓
MainWindow::updateZoomInfo(value)
    ↓
Update slider and label
```

### Searching

```
User enters search text
    ↓
SearchDialog::onSearch()
    ↓
SearchDialog::performSearch()
    ↓
Loop through all pages:
    PDFViewer::search(text, page)
        ↓
    Poppler::Page::search()
        ↓
    Return QList<QRectF> results
    ↓
Display results in list
    ↓
User clicks result
    ↓
SearchDialog::onResultClicked()
    ↓
PDFViewer::setCurrentPage(page)
```

---

## Rendering Pipeline

### Page Rendering Process

1. **Request Render**
   - User navigates to page or changes zoom/rotation
   - `PDFViewer::renderPage()` is called

2. **Calculate DPI**
   - If fit-to-window mode: calculate DPI based on window size
   - Otherwise: DPI = 72 * (zoom / 100)

3. **Render Page**
   - Get `Poppler::Page` object
   - Call `renderToImage(dpi, dpi, rotation)`
   - Returns QImage

4. **Display Image**
   - Convert QImage to QPixmap
   - Set pixmap on QLabel
   - QLabel displays in QScrollArea

### Thumbnail Rendering

- Rendered at fixed 72 DPI for performance
- Scaled to 150x200 pixels
- Generated once when document loads
- Cached in QListWidget items

---

## Performance Considerations

### Optimization Strategies

1. **Lazy Rendering**
   - Only render current page
   - Thumbnails rendered once at load time
   - No pre-rendering of adjacent pages

2. **DPI Management**
   - Thumbnails: 72 DPI (low quality, fast)
   - Normal view: 72-288 DPI (based on zoom)
   - Print: 300 DPI (high quality)

3. **Memory Management**
   - Delete Poppler::Page after rendering
   - Only one page in memory at a time
   - Thumbnails stored as QPixmap (compressed)

4. **UI Responsiveness**
   - Rendering happens on main thread (could be improved)
   - Large documents may cause UI freeze during thumbnail generation
   - Future: Move rendering to worker thread

### Potential Improvements

- [ ] Multi-threaded rendering
- [ ] Page caching (LRU cache)
- [ ] Progressive thumbnail loading
- [ ] Async search
- [ ] Render quality settings

---

## Styling System

### Dark Theme Implementation

The application uses a modern dark theme implemented via Qt stylesheets.

**Color Palette:**
```css
Primary Background:   #2b2b2b
Secondary Background: #3c3c3c
Tertiary Background:  #4a4a4a
Accent Color:         #0d7377
Accent Hover:         #14ffec
Text Color:           #ffffff
Border Color:         #5a5a5a
```

**Styled Components:**
- QMainWindow
- QToolBar
- QToolButton
- QStatusBar
- QDockWidget
- QSlider
- QSpinBox
- QLabel
- QListWidget
- QLineEdit
- QPushButton

### Customization

To change theme:
1. Edit stylesheets in `MainWindow.cpp`
2. Edit stylesheets in `ThumbnailWidget.cpp`
3. Edit stylesheets in `SearchDialog.cpp`

---

## Dependencies

### Qt6 Modules Used

- **Qt6::Core** - Core non-GUI functionality
- **Qt6::Gui** - GUI base classes
- **Qt6::Widgets** - Widget classes (QMainWindow, etc.)

### Poppler-Qt6

- **Poppler::Document** - PDF document representation
- **Poppler::Page** - Individual page rendering
- **Render Hints**:
  - Antialiasing - Smooth edges
  - TextAntialiasing - Smooth text
  - TextHinting - Better text rendering

---

## Build System

### CMake Configuration

**Minimum Version:** 3.16

**Features Used:**
- `CMAKE_AUTOMOC` - Automatic MOC generation
- `CMAKE_AUTORCC` - Automatic resource compilation
- `CMAKE_AUTOUIC` - Automatic UI file processing

**Find Packages:**
- Qt6 (Core, Gui, Widgets)
- Poppler (Qt6 component)

**Output:**
- Executable in `build/bin/`

---

## Testing Strategy

### Manual Testing Checklist

**File Operations:**
- [ ] Open valid PDF
- [ ] Open invalid file
- [ ] Open password-protected PDF
- [ ] Open large PDF (100+ pages)
- [ ] Open small PDF (1 page)

**Navigation:**
- [ ] Next/Previous page
- [ ] First/Last page
- [ ] Go to page number
- [ ] Thumbnail navigation
- [ ] Keyboard shortcuts

**Zoom:**
- [ ] Zoom in/out with buttons
- [ ] Zoom with slider
- [ ] Zoom with Ctrl+Wheel
- [ ] Fit to window
- [ ] Actual size
- [ ] Extreme zoom levels (25%, 400%)

**Rotation:**
- [ ] Rotate left
- [ ] Rotate right
- [ ] Multiple rotations
- [ ] Rotation with zoom

**Search:**
- [ ] Search existing text
- [ ] Search non-existing text
- [ ] Case insensitive search
- [ ] Navigate to results
- [ ] Search in scanned PDF (no text layer)

**Print:**
- [ ] Print single page
- [ ] Print all pages
- [ ] Print to PDF
- [ ] Print preview (if available)

**UI:**
- [ ] Toolbar responsiveness
- [ ] Menu functionality
- [ ] Status bar updates
- [ ] Thumbnail panel show/hide
- [ ] Full screen mode
- [ ] Window resize

---

## Known Limitations

1. **No Annotations**
   - Cannot add comments or highlights
   - Read-only viewer

2. **No Form Support**
   - Cannot fill PDF forms
   - Forms display but not interactive

3. **No Digital Signatures**
   - Cannot verify or add signatures

4. **Single Document**
   - Cannot open multiple PDFs in tabs
   - One document at a time

5. **No Bookmarks**
   - PDF bookmarks not displayed
   - No bookmark navigation

6. **No Text Selection**
   - Cannot select and copy text
   - Search only

7. **Synchronous Rendering**
   - UI may freeze on large documents
   - No progress indicator

8. **No Page Caching**
   - Each page rendered on demand
   - No pre-rendering

---

## Future Enhancements

### High Priority
- [ ] Text selection and copy
- [ ] Multi-threaded rendering
- [ ] Page caching
- [ ] Recent files list
- [ ] Continuous scroll mode

### Medium Priority
- [ ] Annotation support
- [ ] Bookmark navigation
- [ ] Multiple document tabs
- [ ] Two-page view
- [ ] Export to images

### Low Priority
- [ ] Form filling
- [ ] Digital signature verification
- [ ] Presentation mode
- [ ] Night mode (inverted colors)
- [ ] Custom keyboard shortcuts

---

## Code Style Guidelines

### Naming Conventions

- **Classes**: PascalCase (e.g., `PDFViewer`)
- **Methods**: camelCase (e.g., `loadDocument`)
- **Member Variables**: m_camelCase (e.g., `m_document`)
- **Slots**: onCamelCase (e.g., `onOpenFile`)
- **Signals**: camelCase (e.g., `pageChanged`)

### File Organization

- Header files: Class declaration, public interface
- Source files: Implementation details
- One class per file pair
- Include guards in headers

### Qt Conventions

- Use Qt types (QString, QList, etc.)
- Connect signals/slots in constructor
- Delete heap objects in destructor
- Use `nullptr` instead of NULL
- Use `override` keyword

---

## Debugging Tips

### Common Issues

**PDF not rendering:**
- Check if Poppler::Document is valid
- Verify page index is in range
- Check DPI calculation
- Verify QImage is not null

**Memory leaks:**
- Ensure Poppler::Page is deleted after use
- Check QObject parent-child relationships
- Use Qt Creator's memory profiler

**UI not updating:**
- Check if signals are connected
- Verify slots are called
- Use qDebug() for debugging
- Check event loop

### Useful Debug Output

```cpp
qDebug() << "Page count:" << m_document->numPages();
qDebug() << "Current page:" << m_currentPage;
qDebug() << "Zoom level:" << m_zoomLevel;
qDebug() << "DPI:" << dpi;
qDebug() << "Image size:" << image.size();
```

---

## License and Credits

### Third-Party Libraries

- **Qt Framework** - LGPL v3 / Commercial
  - https://www.qt.io/

- **Poppler** - GPL v2 / GPL v3
  - https://poppler.freedesktop.org/

### Acknowledgments

- Qt Project for excellent documentation
- Poppler team for PDF rendering engine
- FontConfig, FreeType for font rendering

---

**Last Updated:** 2025-11-29
**Version:** 1.0.0

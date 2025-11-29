# PDF Reader - User Guide

## Welcome to PDF Reader! 📖

PDF Reader is a modern, easy-to-use application for viewing PDF documents with a beautiful dark theme interface.

---

## Getting Started

### Opening a PDF Document

There are several ways to open a PDF file:

1. **Using the Menu**
   - Click `File` → `Open...`
   - Browse to your PDF file
   - Click "Open"

2. **Using the Toolbar**
   - Click the 📂 (folder) icon in the toolbar
   - Select your PDF file

3. **Keyboard Shortcut**
   - Press `Ctrl+O`
   - Select your PDF file

4. **Command Line**
   ```bash
   PDFReader "path/to/your/document.pdf"
   ```

5. **Drag and Drop** (if supported by your OS)
   - Drag a PDF file onto the application window

---

## Navigating Your Document

### Page Navigation

**Using the Toolbar:**
- ◀ **Previous Page** - Go to the previous page
- ▶ **Next Page** - Go to the next page
- **Page Number Box** - Type a page number and press Enter

**Using Keyboard:**
- `Page Down` or `→` - Next page
- `Page Up` or `←` - Previous page
- `Home` - First page
- `End` - Last page

**Using Thumbnails:**
- Click the "Pages" panel on the left
- Click any thumbnail to jump to that page

### Scrolling

- Use your mouse wheel to scroll up and down
- Click and drag the scrollbar
- Use keyboard arrow keys

---

## Zooming

### Zoom Controls

**Toolbar Zoom:**
- ➖ **Zoom Out** button
- 🎚️ **Zoom Slider** - Drag to adjust zoom (25% - 400%)
- ➕ **Zoom In** button
- **Zoom Percentage** - Shows current zoom level

**Keyboard Shortcuts:**
- `Ctrl++` - Zoom in
- `Ctrl+-` - Zoom out
- `Ctrl+0` - Reset to 100% (Actual Size)
- `Ctrl+Mouse Wheel` - Zoom in/out smoothly

**Menu Options:**
- `View` → `Zoom In`
- `View` → `Zoom Out`
- `View` → `Fit to Window` - Auto-fit page to window size
- `View` → `Actual Size` - Reset to 100%

### Fit to Window

This mode automatically adjusts the zoom level to fit the entire page in the window. The zoom will update automatically when you resize the window.

---

## Rotating Pages

Sometimes PDFs are scanned in the wrong orientation. You can rotate pages:

**Using Toolbar:**
- 🔄 **Rotate Left** - Rotate 90° counter-clockwise
- 🔃 **Rotate Right** - Rotate 90° clockwise

**Using Keyboard:**
- `Ctrl+L` - Rotate left
- `Ctrl+R` - Rotate right

**Using Menu:**
- `Navigate` → `Rotate Left`
- `Navigate` → `Rotate Right`

---

## Searching Text

### How to Search

1. **Open Search Dialog**
   - Press `Ctrl+F`
   - Or click the 🔍 search icon
   - Or go to `Edit` → `Search...`

2. **Enter Search Text**
   - Type the text you want to find
   - Press Enter or click "Search"

3. **View Results**
   - Results show which pages contain your search text
   - Number of matches per page is displayed
   - Click any result to jump to that page

### Search Tips

- Search is **case-insensitive** (finds "hello", "Hello", "HELLO")
- Searches across **all pages** in the document
- Results are organized by page number
- Click a result to navigate to that page

---

## Printing

### How to Print

1. **Open Print Dialog**
   - Press `Ctrl+P`
   - Or click `File` → `Print...`

2. **Configure Print Settings**
   - Select your printer
   - Choose page range
   - Set number of copies
   - Adjust other settings

3. **Print**
   - Click "Print" to start printing

### Print Tips

- Preview is not currently available
- All pages are printed at high quality (300 DPI)
- Pages maintain their aspect ratio

---

## Full Screen Mode

For distraction-free reading:

1. **Enter Full Screen**
   - Press `F11`
   - Or click `View` → `Full Screen`

2. **Exit Full Screen**
   - Press `F11` again
   - Or press `Esc`

In full screen mode:
- Menu bar is hidden
- Toolbar is hidden
- Only the PDF content is visible
- All keyboard shortcuts still work

---

## Interface Overview

### Main Window Components

```
┌─────────────────────────────────────────────┐
│ File  View  Navigate  Edit  Help            │ ← Menu Bar
├─────────────────────────────────────────────┤
│ 📂 🖨️ │ ◀ ▶ [1] /10 │ ➖ 🎚️ ➕ 100% │ 🔄 🔃 │ 🔍 │ ← Toolbar
├──────┬──────────────────────────────────────┤
│Pages │                                      │
│      │                                      │
│ [1]  │         PDF Content                  │
│ [2]  │         Displayed Here               │
│ [3]  │                                      │
│ [4]  │                                      │
│      │                                      │
└──────┴──────────────────────────────────────┘
```

**Components:**
1. **Menu Bar** - Access all features
2. **Toolbar** - Quick access to common actions
3. **Pages Panel** - Thumbnail view (can be hidden)
4. **Main View** - PDF content display
5. **Status Bar** - Shows current page info

### Toolbar Icons

| Icon | Function |
|------|----------|
| 📂 | Open file |
| 🖨️ | Print |
| ◀ | Previous page |
| ▶ | Next page |
| ➖ | Zoom out |
| ➕ | Zoom in |
| 🔄 | Rotate left |
| 🔃 | Rotate right |
| 🔍 | Search |

---

## Keyboard Shortcuts Reference

### File Operations
| Action | Shortcut |
|--------|----------|
| Open | `Ctrl+O` |
| Print | `Ctrl+P` |
| Quit | `Ctrl+Q` |

### View
| Action | Shortcut |
|--------|----------|
| Zoom In | `Ctrl++` |
| Zoom Out | `Ctrl+-` |
| Actual Size | `Ctrl+0` |
| Full Screen | `F11` |

### Navigation
| Action | Shortcut |
|--------|----------|
| Next Page | `Page Down`, `→` |
| Previous Page | `Page Up`, `←` |
| First Page | `Home` |
| Last Page | `End` |
| Rotate Left | `Ctrl+L` |
| Rotate Right | `Ctrl+R` |

### Edit
| Action | Shortcut |
|--------|----------|
| Search | `Ctrl+F` |

---

## Tips and Tricks

### Performance Tips

1. **Large Documents**
   - Thumbnails may take time to generate for large PDFs
   - Close the Pages panel if not needed
   - Use page number box for quick navigation

2. **Smooth Zooming**
   - Use `Ctrl+Mouse Wheel` for smooth zoom
   - Slider provides precise control

3. **Quick Navigation**
   - Use keyboard shortcuts for fastest navigation
   - Thumbnails for visual overview
   - Page number box for direct jumps

### Viewing Tips

1. **Best Zoom Level**
   - Use "Fit to Window" for full page view
   - Use 100% for actual size
   - Use 150-200% for detailed viewing

2. **Reading Mode**
   - Enter full screen (`F11`)
   - Use Page Down/Up to read
   - Exit with `F11` or `Esc`

3. **Rotated Documents**
   - Rotation persists while document is open
   - Rotation resets when reopening

---

## Troubleshooting

### Common Issues

**Problem: PDF won't open**
- Check if file is corrupted
- Verify file has .pdf extension
- Try opening with another PDF viewer first

**Problem: Text appears blurry**
- Increase zoom level
- Check if PDF is low resolution
- Try "Actual Size" (Ctrl+0)

**Problem: Search not finding text**
- PDF might be scanned images (no text layer)
- Try different search terms
- Check spelling

**Problem: Slow performance**
- Close Pages panel
- Reduce zoom level
- Close other applications

**Problem: Can't print**
- Check printer connection
- Verify printer drivers
- Try printing a test page from OS

---

## Supported Features

✅ **Currently Supported:**
- View PDF documents
- Zoom (25% - 400%)
- Rotate pages
- Navigate pages
- Search text
- Print documents
- Thumbnail view
- Keyboard shortcuts
- Full screen mode

❌ **Not Yet Supported:**
- Annotations/Comments
- Form filling
- Digital signatures
- Bookmarks
- Attachments
- Multiple documents (tabs)
- Export to images
- Copy text

---

## Getting Help

If you encounter issues:

1. Check this user guide
2. Review the README.md file
3. Check the INSTALL.md for setup issues
4. Verify all dependencies are installed

---

## Feedback

We hope you enjoy using PDF Reader! This application is designed to be simple, fast, and beautiful.

**Happy Reading! 📚**

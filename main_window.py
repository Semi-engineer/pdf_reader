"""
Main Window - Primary application window with all UI components
"""

from PySide6.QtWidgets import (QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, 
                               QToolBar, QStatusBar, QScrollArea, QListWidget,
                               QListWidgetItem, QDockWidget, QFileDialog, QLineEdit,
                               QPushButton, QLabel, QMessageBox, QMenu, QApplication,
                               QSpinBox, QComboBox, QDialog, QDialogButtonBox)
from PySide6.QtCore import Qt, Signal, QTimer, QSize
from PySide6.QtGui import QAction, QIcon, QKeySequence, QPixmap, QColor, QPainter
import fitz
import os
from pathlib import Path

from render_worker import RenderWorker
from page_cache import PageCache
from thumbnail_manager import ThumbnailManager
from search_manager import SearchManager
from annotation_manager import AnnotationManager
from pdf_page_widget import PDFPageWidget


class MainWindow(QMainWindow):
    """Main application window"""
    
    def __init__(self, settings=None):
        super().__init__()
        
        self.settings = settings or {}
        self.doc = None
        self.doc_path = None
        self.current_page = 0
        self.zoom_level = 100
        self.rotation = 0
        self.page_rotations = {}  # Per-page rotations
        self.two_page_mode = False
        self.dark_mode = False
        
        # Managers
        self.render_worker = RenderWorker()
        self.page_cache = PageCache(max_size=30)
        self.thumbnail_manager = ThumbnailManager()
        self.search_manager = SearchManager()
        self.annotation_manager = AnnotationManager()
        
        # Page widgets
        self.page_widgets = {}
        
        # Setup UI
        self._setup_ui()
        self._setup_connections()
        self._apply_theme()
        
        # Restore settings
        if self.settings.get('dark_mode'):
            self.dark_mode = True
            self._apply_theme()
        if self.settings.get('sidebar_visible', True):
            self.sidebar_dock.setVisible(True)
    
    def _setup_ui(self):
        """Setup user interface"""
        self.setWindowTitle("PDF Viewer")
        self.setGeometry(100, 100, 1200, 800)
        
        # Central widget with scroll area
        self.scroll_area = QScrollArea()
        self.scroll_area.setWidgetResizable(False)
        self.scroll_area.setAlignment(Qt.AlignCenter)
        self.scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarAsNeeded)
        self.scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarAsNeeded)
        
        self.scroll_widget = QWidget()
        from PySide6.QtWidgets import QSizePolicy
        self.scroll_widget.setSizePolicy(QSizePolicy.Preferred, QSizePolicy.Preferred)
        
        self.scroll_layout = QVBoxLayout(self.scroll_widget)
        self.scroll_layout.setSpacing(10)
        self.scroll_layout.setContentsMargins(20, 20, 20, 20)
        self.scroll_layout.setAlignment(Qt.AlignTop | Qt.AlignHCenter)
        self.scroll_layout.setSizeConstraint(QVBoxLayout.SetMinAndMaxSize)
        
        self.scroll_widget.setLayout(self.scroll_layout)
        self.scroll_area.setWidget(self.scroll_widget)
        self.setCentralWidget(self.scroll_area)
        
        # Setup toolbar
        self._setup_toolbar()
        
        # Setup sidebar
        self._setup_sidebar()
        
        # Setup statusbar
        self._setup_statusbar()
        
        # Setup context menu
        self.scroll_area.setContextMenuPolicy(Qt.CustomContextMenu)
        self.scroll_area.customContextMenuRequested.connect(self._show_context_menu)
    
    def _setup_toolbar(self):
        """Setup toolbar with actions"""
        toolbar = QToolBar("Main Toolbar")
        toolbar.setIconSize(QSize(24, 24))
        self.addToolBar(toolbar)
        
        # File actions
        open_action = QAction("Open", self)
        open_action.setShortcut(QKeySequence.Open)
        open_action.triggered.connect(self.open_file_dialog)
        toolbar.addAction(open_action)
        
        save_action = QAction("Save Copy", self)
        save_action.setShortcut(QKeySequence.Save)
        save_action.triggered.connect(self.save_copy)
        toolbar.addAction(save_action)
        
        toolbar.addSeparator()
        
        # Navigation
        prev_action = QAction("Previous", self)
        prev_action.setShortcut(Qt.Key_PageUp)
        prev_action.triggered.connect(self.prev_page)
        toolbar.addAction(prev_action)
        
        next_action = QAction("Next", self)
        next_action.setShortcut(Qt.Key_PageDown)
        next_action.triggered.connect(self.next_page)
        toolbar.addAction(next_action)
        
        # Page number
        toolbar.addWidget(QLabel("  Page: "))
        self.page_spinbox = QSpinBox()
        self.page_spinbox.setMinimum(1)
        self.page_spinbox.valueChanged.connect(self._on_page_spinbox_changed)
        toolbar.addWidget(self.page_spinbox)
        
        toolbar.addSeparator()
        
        # Zoom controls
        zoom_out_action = QAction("Zoom Out", self)
        zoom_out_action.setShortcut(QKeySequence.ZoomOut)
        zoom_out_action.triggered.connect(self.zoom_out)
        toolbar.addAction(zoom_out_action)
        
        zoom_in_action = QAction("Zoom In", self)
        zoom_in_action.setShortcut(QKeySequence.ZoomIn)
        zoom_in_action.triggered.connect(self.zoom_in)
        toolbar.addAction(zoom_in_action)
        
        self.zoom_combo = QComboBox()
        self.zoom_combo.addItems(["50%", "75%", "100%", "125%", "150%", "200%", "Fit Width", "Fit Page"])
        self.zoom_combo.setCurrentText("100%")
        self.zoom_combo.currentTextChanged.connect(self._on_zoom_combo_changed)
        toolbar.addWidget(self.zoom_combo)
        
        toolbar.addSeparator()
        
        # View modes
        two_page_action = QAction("Two Pages", self)
        two_page_action.setCheckable(True)
        two_page_action.triggered.connect(self.toggle_two_page_mode)
        toolbar.addAction(two_page_action)
        self.two_page_action = two_page_action
        
        rotate_action = QAction("Rotate", self)
        rotate_action.triggered.connect(self.rotate_page)
        toolbar.addAction(rotate_action)
        
        toolbar.addSeparator()
        
        # Search
        toolbar.addWidget(QLabel("  Search: "))
        self.search_input = QLineEdit()
        self.search_input.setPlaceholderText("Find text...")
        self.search_input.setMaximumWidth(200)
        self.search_input.returnPressed.connect(self.search_text)
        toolbar.addWidget(self.search_input)
        
        search_action = QAction("Find", self)
        search_action.setShortcut(QKeySequence.Find)
        search_action.triggered.connect(self.search_text)
        toolbar.addAction(search_action)
        
        search_next_action = QAction("Next", self)
        search_next_action.setShortcut(QKeySequence.FindNext)
        search_next_action.triggered.connect(self.search_next)
        toolbar.addAction(search_next_action)
        
        toolbar.addSeparator()
        
        # Dark mode
        dark_mode_action = QAction("Dark Mode", self)
        dark_mode_action.setCheckable(True)
        dark_mode_action.triggered.connect(self.toggle_dark_mode)
        toolbar.addAction(dark_mode_action)
        self.dark_mode_action = dark_mode_action
        
        # Annotation tools
        toolbar.addSeparator()
        highlight_action = QAction("Highlight", self)
        highlight_action.setCheckable(True)
        highlight_action.triggered.connect(lambda: self.set_annotation_mode('highlight'))
        toolbar.addAction(highlight_action)
        self.highlight_action = highlight_action
        
        rect_action = QAction("Rectangle", self)
        rect_action.setCheckable(True)
        rect_action.triggered.connect(lambda: self.set_annotation_mode('rectangle'))
        toolbar.addAction(rect_action)
        self.rect_action = rect_action
    
    def _setup_sidebar(self):
        """Setup thumbnail sidebar"""
        self.sidebar_dock = QDockWidget("Thumbnails", self)
        self.sidebar_dock.setAllowedAreas(Qt.LeftDockWidgetArea | Qt.RightDockWidgetArea)
        
        self.thumbnail_list = QListWidget()
        # Use ListMode for vertical list instead of IconMode (grid)
        self.thumbnail_list.setViewMode(QListWidget.ListMode)
        self.thumbnail_list.setIconSize(QSize(120, 160))
        self.thumbnail_list.setSpacing(5)
        self.thumbnail_list.setMovement(QListWidget.Static)
        self.thumbnail_list.setResizeMode(QListWidget.Adjust)
        # Style for better appearance
        self.thumbnail_list.setStyleSheet("""
            QListWidget {
                background-color: #f5f5f5;
                border: none;
            }
            QListWidget::item {
                padding: 5px;
                border: 1px solid #ddd;
                background-color: white;
                margin: 2px;
            }
            QListWidget::item:selected {
                background-color: #e3f2fd;
                border: 2px solid #2196f3;
            }
            QListWidget::item:hover {
                background-color: #f0f0f0;
            }
        """)
        self.thumbnail_list.itemClicked.connect(self._on_thumbnail_clicked)
        
        self.sidebar_dock.setWidget(self.thumbnail_list)
        self.addDockWidget(Qt.LeftDockWidgetArea, self.sidebar_dock)
    
    def _setup_statusbar(self):
        """Setup status bar"""
        self.statusbar = QStatusBar()
        self.setStatusBar(self.statusbar)
        
        self.status_label = QLabel("Ready")
        self.statusbar.addWidget(self.status_label)
        
        self.page_label = QLabel("")
        self.statusbar.addPermanentWidget(self.page_label)
        
        self.zoom_label = QLabel("100%")
        self.statusbar.addPermanentWidget(self.zoom_label)
    
    def _setup_connections(self):
        """Setup signal connections"""
        self.render_worker.page_rendered.connect(self._on_page_rendered)
        self.render_worker.render_error.connect(self._on_render_error)
        self.thumbnail_manager.thumbnail_ready.connect(self._on_thumbnail_ready)
        self.search_manager.search_completed.connect(self._on_search_completed)
        self.annotation_manager.annotations_changed.connect(self._on_annotations_changed)
        
        # Scroll area viewport change
        self.scroll_area.verticalScrollBar().valueChanged.connect(self._on_scroll)
    
    def _apply_theme(self):
        """Apply dark/light theme"""
        if self.dark_mode:
            self.setStyleSheet("""
                QMainWindow, QWidget { background-color: #2b2b2b; color: #ffffff; }
                QScrollArea { background-color: #1e1e1e; }
                QToolBar { background-color: #3c3c3c; border: none; }
                QStatusBar { background-color: #3c3c3c; color: #ffffff; }
                QLineEdit, QSpinBox, QComboBox { 
                    background-color: #3c3c3c; 
                    color: #ffffff; 
                    border: 1px solid #555555;
                    padding: 4px;
                }
                QListWidget { background-color: #2b2b2b; color: #ffffff; }
                QDockWidget { background-color: #2b2b2b; color: #ffffff; }
            """)
        else:
            self.setStyleSheet("")
    
    def open_file_dialog(self):
        """Open file dialog to select PDF"""
        file_path, _ = QFileDialog.getOpenFileName(
            self,
            "Open PDF File",
            "",
            "PDF Files (*.pdf);;All Files (*)"
        )
        
        if file_path:
            self.open_file(file_path)
    
    def open_file(self, file_path):
        """Open a PDF file"""
        try:
            # Close existing document
            if self.doc:
                self.doc.close()
            
            # Open new document
            self.doc = fitz.open(file_path)
            self.doc_path = file_path
            self.current_page = 0
            self.page_rotations = {}
            
            # Update UI
            self.setWindowTitle(f"PDF Viewer - {os.path.basename(file_path)}")
            self.page_spinbox.setMaximum(len(self.doc))
            self.page_spinbox.setValue(1)
            
            # Clear caches
            self.page_cache.clear()
            self.thumbnail_manager.clear()
            self.page_widgets.clear()
            
            # Clear layout
            while self.scroll_layout.count():
                item = self.scroll_layout.takeAt(0)
                if item.widget():
                    item.widget().deleteLater()
            
            # Generate thumbnails
            self._generate_thumbnails()
            
            # Render pages (this will create all placeholder widgets)
            self._render_visible_pages()
            
            # Force update container size for correct scrollbar
            QTimer.singleShot(100, self._update_container_size)
            
            self.status_label.setText(f"Loaded: {os.path.basename(file_path)}")
            self._update_page_label()
            
        except Exception as e:
            QMessageBox.critical(self, "Error", f"Failed to open PDF: {str(e)}")
    
    def _generate_thumbnails(self):
        """Generate thumbnails for all pages"""
        if not self.doc:
            return
        
        self.thumbnail_list.clear()
        
        for i in range(len(self.doc)):
            item = QListWidgetItem(f"Page {i + 1}")
            item.setData(Qt.UserRole, i)
            self.thumbnail_list.addItem(item)
            
            # Request thumbnail generation
            self.thumbnail_manager.generate_thumbnail(self.doc_path, i)
    
    def _on_thumbnail_ready(self, page_num, pixmap):
        """Handle thumbnail generation completion"""
        for i in range(self.thumbnail_list.count()):
            item = self.thumbnail_list.item(i)
            if item.data(Qt.UserRole) == page_num:
                item.setIcon(QIcon(pixmap))
                break
    
    def _on_thumbnail_clicked(self, item):
        """Handle thumbnail click"""
        page_num = item.data(Qt.UserRole)
        self.goto_page(page_num)
    
    def _render_visible_pages(self):
        """Render pages visible in viewport (Lazy Loading)"""
        if not self.doc:
            return
        
        # Determine which pages to render
        if self.two_page_mode:
            pages_to_render = [self.current_page]
            if self.current_page + 1 < len(self.doc):
                pages_to_render.append(self.current_page + 1)
            
            # Clear existing widgets in two-page mode
            while self.scroll_layout.count():
                item = self.scroll_layout.takeAt(0)
                if item.widget():
                    item.widget().deleteLater()
            self.page_widgets.clear()
        else:
            # Create placeholder widgets for ALL pages first (lightweight)
            if len(self.page_widgets) == 0:
                for i in range(len(self.doc)):
                    self._create_page_widget(i)
            
            # Only render visible pages + buffer
            pages_to_render = self._get_visible_pages()
        
        # Render only the visible pages
        for page_num in pages_to_render:
            # Check cache first
            rotation = self.page_rotations.get(page_num, self.rotation)
            cache_key = (page_num, self.zoom_level, rotation)
            cached_pixmap = self.page_cache.get(cache_key)
            
            if cached_pixmap:
                self._display_page(page_num, cached_pixmap)
            else:
                # Request render only if not already rendering
                if page_num not in getattr(self, '_rendering_pages', set()):
                    if not hasattr(self, '_rendering_pages'):
                        self._rendering_pages = set()
                    self._rendering_pages.add(page_num)
                    self.render_worker.render_page(self.doc_path, page_num, self.zoom_level, rotation)
        
        # Update container size after adding all widgets
        self._update_container_size()
    
    def _get_visible_pages(self):
        """Get list of pages that are visible or near visible area"""
        if not self.doc:
            return []
        
        # Get scroll position
        scrollbar = self.scroll_area.verticalScrollBar()
        scroll_pos = scrollbar.value()
        viewport_height = self.scroll_area.viewport().height()
        
        # Calculate which pages are visible
        visible_pages = []
        current_y = self.scroll_layout.contentsMargins().top()
        
        for page_num in range(len(self.doc)):
            if page_num in self.page_widgets:
                widget = self.page_widgets[page_num]
                widget_height = widget.height()
                
                # Check if page is in viewport (with buffer)
                buffer = viewport_height  # Load 1 viewport above and below
                if (current_y + widget_height + buffer >= scroll_pos and 
                    current_y - buffer <= scroll_pos + viewport_height):
                    visible_pages.append(page_num)
                
                current_y += widget_height + self.scroll_layout.spacing()
        
        # If no pages found (initial load), load first few pages
        if not visible_pages:
            visible_pages = list(range(min(3, len(self.doc))))
        
        return visible_pages
    
    def _create_page_widget(self, page_num):
        """Create widget for a page"""
        if page_num in self.page_widgets and not self.two_page_mode:
            return
        
        # Create simple QLabel for now - simpler and more reliable
        from PySide6.QtWidgets import QLabel
        widget = QLabel()
        widget.setAlignment(Qt.AlignCenter)
        widget.setStyleSheet("background-color: white; border: 1px solid #ccc;")
        widget.setScaledContents(False)
        
        # Set initial size based on PDF page dimensions
        try:
            page = self.doc[page_num]
            rotation = self.page_rotations.get(page_num, self.rotation)
            
            # Calculate size with current zoom
            if rotation in [90, 270]:
                width = int(page.rect.height * self.zoom_level / 100)
                height = int(page.rect.width * self.zoom_level / 100)
            else:
                width = int(page.rect.width * self.zoom_level / 100)
                height = int(page.rect.height * self.zoom_level / 100)
            
            widget.setFixedSize(width, height)
        except:
            # Fallback size
            widget.setFixedSize(600, 800)
        
        if self.two_page_mode:
            # Add to horizontal layout
            if not hasattr(self, 'two_page_layout'):
                self.two_page_layout = QHBoxLayout()
                self.scroll_layout.addLayout(self.two_page_layout)
            self.two_page_layout.addWidget(widget, 0, Qt.AlignCenter)
        else:
            # Add to vertical layout
            self.scroll_layout.addWidget(widget, 0, Qt.AlignHCenter)
        
        self.page_widgets[page_num] = widget
        
        # Force visibility
        widget.setVisible(True)
        widget.show()
    
    def _display_page(self, page_num, pixmap):
        """Display rendered page"""
        if page_num in self.page_widgets:
            widget = self.page_widgets[page_num]
            widget.setPixmap(pixmap)
            widget.setFixedSize(pixmap.size())
            widget.update()  # Force repaint
    
    def _on_page_rendered(self, page_num, pixmap):
        """Handle page render completion"""
        # Remove from rendering set
        if hasattr(self, '_rendering_pages') and page_num in self._rendering_pages:
            self._rendering_pages.remove(page_num)
        
        # Cache the pixmap
        rotation = self.page_rotations.get(page_num, self.rotation)
        cache_key = (page_num, self.zoom_level, rotation)
        self.page_cache.put(cache_key, pixmap)
        
        # Display the page
        self._display_page(page_num, pixmap)
        
        # Update container size
        self._update_container_size()
        
        self.status_label.setText("Ready")
    
    def _update_container_size(self):
        """Update scroll widget size to fit all page widgets"""
        if not self.page_widgets or not self.doc:
            return
        
        # Calculate total height and max width from ALL pages
        total_height = 0
        max_width = 0
        
        # Iterate through all pages in order
        for page_num in range(len(self.doc)):
            if page_num in self.page_widgets:
                widget = self.page_widgets[page_num]
                total_height += widget.height()
                max_width = max(max_width, widget.width())
        
        # Add spacing and margins
        spacing = self.scroll_layout.spacing() * (len(self.doc) - 1)
        margins = self.scroll_layout.contentsMargins()
        
        total_height += spacing + margins.top() + margins.bottom()
        max_width += margins.left() + margins.right()
        
        # Set minimum size for container to show correct scrollbar
        self.scroll_widget.setMinimumSize(max_width, total_height)
        
        # Force update
        self.scroll_widget.updateGeometry()
        self.scroll_area.updateGeometry()
    
    def _on_render_error(self, page_num, error):
        """Handle render error"""
        self.status_label.setText(f"Error rendering page {page_num + 1}: {error}")
    
    def _on_scroll(self):
        """Handle scroll event - lazy load visible pages"""
        if not self.doc or self.two_page_mode:
            return
        
        # Cancel previous timer if exists
        if hasattr(self, '_scroll_timer') and self._scroll_timer.isActive():
            self._scroll_timer.stop()
        
        # Debounce scroll events - render after 150ms of no scrolling
        self._scroll_timer = QTimer()
        self._scroll_timer.setSingleShot(True)
        self._scroll_timer.timeout.connect(self._render_visible_pages)
        self._scroll_timer.start(150)
    
    def goto_page(self, page_num):
        """Go to specific page"""
        if not self.doc or page_num < 0 or page_num >= len(self.doc):
            return
        
        self.current_page = page_num
        self.page_spinbox.setValue(page_num + 1)
        
        if self.two_page_mode:
            self._render_visible_pages()
        else:
            # Scroll to page widget
            if page_num in self.page_widgets:
                widget = self.page_widgets[page_num]
                self.scroll_area.ensureWidgetVisible(widget)
        
        self._update_page_label()
    
    def next_page(self):
        """Go to next page"""
        if self.two_page_mode:
            step = 2
        else:
            step = 1
        
        if self.current_page + step < len(self.doc):
            self.goto_page(self.current_page + step)
    
    def prev_page(self):
        """Go to previous page"""
        if self.two_page_mode:
            step = 2
        else:
            step = 1
        
        if self.current_page - step >= 0:
            self.goto_page(self.current_page - step)
    
    def _on_page_spinbox_changed(self, value):
        """Handle page spinbox change"""
        self.goto_page(value - 1)
    
    def zoom_in(self):
        """Zoom in"""
        self.set_zoom(min(400, self.zoom_level + 25))
    
    def zoom_out(self):
        """Zoom out"""
        self.set_zoom(max(25, self.zoom_level - 25))
    
    def set_zoom(self, zoom):
        """Set zoom level"""
        self.zoom_level = zoom
        self.zoom_label.setText(f"{zoom}%")
        
        # Update combo box
        zoom_text = f"{zoom}%"
        index = self.zoom_combo.findText(zoom_text)
        if index >= 0:
            self.zoom_combo.setCurrentIndex(index)
        
        # Clear cache and re-render
        self.page_cache.clear()
        self._render_visible_pages()
    
    def _on_zoom_combo_changed(self, text):
        """Handle zoom combo change"""
        if text == "Fit Width":
            self._fit_width()
        elif text == "Fit Page":
            self._fit_page()
        else:
            try:
                zoom = int(text.rstrip('%'))
                self.set_zoom(zoom)
            except:
                pass
    
    def _fit_width(self):
        """Fit page to width"""
        if not self.doc:
            return
        
        page = self.doc[self.current_page]
        page_width = page.rect.width
        viewport_width = self.scroll_area.viewport().width() - 40
        
        zoom = int((viewport_width / page_width) * 100)
        self.set_zoom(zoom)
    
    def _fit_page(self):
        """Fit entire page"""
        if not self.doc:
            return
        
        page = self.doc[self.current_page]
        page_width = page.rect.width
        page_height = page.rect.height
        
        viewport_width = self.scroll_area.viewport().width() - 40
        viewport_height = self.scroll_area.viewport().height() - 40
        
        zoom_w = int((viewport_width / page_width) * 100)
        zoom_h = int((viewport_height / page_height) * 100)
        
        self.set_zoom(min(zoom_w, zoom_h))
    
    def toggle_two_page_mode(self):
        """Toggle two-page view"""
        self.two_page_mode = not self.two_page_mode
        self.two_page_action.setChecked(self.two_page_mode)
        
        # Clear and re-render
        while self.scroll_layout.count():
            item = self.scroll_layout.takeAt(0)
            if item.widget():
                item.widget().deleteLater()
            elif item.layout():
                # Remove layout
                while item.layout().count():
                    sub_item = item.layout().takeAt(0)
                    if sub_item.widget():
                        sub_item.widget().deleteLater()
        
        self.page_widgets.clear()
        if hasattr(self, 'two_page_layout'):
            delattr(self, 'two_page_layout')
        
        self._render_visible_pages()
    
    def rotate_page(self):
        """Rotate current page"""
        if not self.doc:
            return
        
        # Rotate current page by 90 degrees
        current_rotation = self.page_rotations.get(self.current_page, self.rotation)
        new_rotation = (current_rotation + 90) % 360
        self.page_rotations[self.current_page] = new_rotation
        
        # Clear cache for this page and re-render
        for key in list(self.page_cache.cache.keys()):
            if key[0] == self.current_page:
                self.page_cache.remove(key)
        
        self._render_visible_pages()
    
    def search_text(self):
        """Search for text"""
        query = self.search_input.text()
        if not query or not self.doc:
            return
        
        self.status_label.setText("Searching...")
        self.search_manager.search(self.doc_path, query)
    
    def _on_search_completed(self, results):
        """Handle search completion"""
        if results:
            self.status_label.setText(f"Found {len(results)} results")
            # Go to first result
            first_result = results[0]
            self.goto_page(first_result['page'])
            
            # Update page widgets with search results (only if they support it)
            for page_num in self.page_widgets:
                widget = self.page_widgets[page_num]
                if hasattr(widget, 'set_search_results'):
                    page_results = self.search_manager.get_results_for_page(page_num)
                    widget.set_search_results(page_results)
        else:
            self.status_label.setText("No results found")
    
    def search_next(self):
        """Go to next search result"""
        result = self.search_manager.next_result()
        if result:
            self.goto_page(result['page'])
    
    def toggle_dark_mode(self):
        """Toggle dark mode"""
        self.dark_mode = not self.dark_mode
        self.dark_mode_action.setChecked(self.dark_mode)
        self._apply_theme()
    
    def set_annotation_mode(self, mode):
        """Set annotation mode"""
        # Uncheck other annotation actions
        if mode == 'highlight':
            self.highlight_action.setChecked(True)
            self.rect_action.setChecked(False)
            color = QColor(255, 255, 0, 100)
        elif mode == 'rectangle':
            self.highlight_action.setChecked(False)
            self.rect_action.setChecked(True)
            color = QColor(255, 0, 0, 200)
        else:
            self.highlight_action.setChecked(False)
            self.rect_action.setChecked(False)
            color = None
        
        # Set mode on all page widgets (only if they support it)
        for widget in self.page_widgets.values():
            if hasattr(widget, 'set_annotation_mode'):
                widget.set_annotation_mode(mode, color)
    
    def _on_text_selected(self, text):
        """Handle text selection"""
        clipboard = QApplication.clipboard()
        clipboard.setText(text)
        self.status_label.setText("Text copied to clipboard")
    
    def _on_annotation_added(self, page_num, rect, color, annotation_type):
        """Handle annotation added"""
        self.annotation_manager.add_annotation(page_num, rect, color, annotation_type)
        self.status_label.setText(f"Annotation added to page {page_num + 1}")
    
    def _on_annotations_changed(self):
        """Handle annotations changed"""
        # Update all page widgets (only if they support it)
        for page_num, widget in self.page_widgets.items():
            if hasattr(widget, 'set_annotations'):
                annotations = self.annotation_manager.get_annotations_for_page(page_num)
                widget.set_annotations(annotations)
    
    def save_copy(self):
        """Save a copy of PDF with annotations"""
        if not self.doc_path:
            return
        
        file_path, _ = QFileDialog.getSaveFileName(
            self,
            "Save PDF Copy",
            "",
            "PDF Files (*.pdf)"
        )
        
        if file_path:
            if self.annotation_manager.annotations:
                # Save with annotations
                success = self.annotation_manager.save_to_pdf(self.doc_path, file_path)
                if success:
                    QMessageBox.information(self, "Success", "PDF saved with annotations")
                else:
                    QMessageBox.warning(self, "Error", "Failed to save annotations")
            else:
                # Just copy the file
                import shutil
                shutil.copy(self.doc_path, file_path)
                QMessageBox.information(self, "Success", "PDF copied")
    
    def _show_context_menu(self, pos):
        """Show context menu"""
        menu = QMenu(self)
        
        copy_action = menu.addAction("Copy Text")
        copy_action.triggered.connect(lambda: None)  # Handled by widget
        
        menu.addSeparator()
        
        highlight_action = menu.addAction("Add Highlight")
        highlight_action.triggered.connect(lambda: self.set_annotation_mode('highlight'))
        
        rect_action = menu.addAction("Add Rectangle")
        rect_action.triggered.connect(lambda: self.set_annotation_mode('rectangle'))
        
        menu.addSeparator()
        
        export_action = menu.addAction("Export Page as Image")
        export_action.triggered.connect(self.export_page_as_image)
        
        menu.exec(self.scroll_area.mapToGlobal(pos))
    
    def export_page_as_image(self):
        """Export current page as image"""
        if not self.doc:
            return
        
        file_path, _ = QFileDialog.getSaveFileName(
            self,
            "Export Page as Image",
            f"page_{self.current_page + 1}.png",
            "PNG Files (*.png);;JPEG Files (*.jpg)"
        )
        
        if file_path:
            try:
                page = self.doc[self.current_page]
                mat = fitz.Matrix(2.0, 2.0)  # High resolution
                pix = page.get_pixmap(matrix=mat)
                pix.save(file_path)
                QMessageBox.information(self, "Success", "Page exported")
            except Exception as e:
                QMessageBox.warning(self, "Error", f"Export failed: {str(e)}")
    
    def _update_page_label(self):
        """Update page label in status bar"""
        if self.doc:
            self.page_label.setText(f"Page {self.current_page + 1} / {len(self.doc)}")
        else:
            self.page_label.setText("")
    
    def wheelEvent(self, event):
        """Handle mouse wheel for zoom"""
        if event.modifiers() == Qt.ControlModifier:
            if event.angleDelta().y() > 0:
                self.zoom_in()
            else:
                self.zoom_out()
            event.accept()
        else:
            super().wheelEvent(event)
    
    def keyPressEvent(self, event):
        """Handle keyboard shortcuts"""
        if event.key() == Qt.Key_Space:
            # Scroll down
            scrollbar = self.scroll_area.verticalScrollBar()
            scrollbar.setValue(scrollbar.value() + scrollbar.pageStep())
        elif event.key() == Qt.Key_Left:
            self.prev_page()
        elif event.key() == Qt.Key_Right:
            self.next_page()
        elif event.key() == Qt.Key_Home:
            self.goto_page(0)
        elif event.key() == Qt.Key_End:
            if self.doc:
                self.goto_page(len(self.doc) - 1)
        else:
            super().keyPressEvent(event)
    
    def get_settings(self):
        """Get current settings for persistence"""
        return {
            'last_file': self.doc_path,
            'last_page': self.current_page,
            'last_zoom': self.zoom_level,
            'dark_mode': self.dark_mode,
            'sidebar_visible': self.sidebar_dock.isVisible()
        }
    
    def closeEvent(self, event):
        """Handle window close"""
        if self.doc:
            self.doc.close()
        
        self.render_worker.cancel_all()
        self.thumbnail_manager.clear()
        
        super().closeEvent(event)

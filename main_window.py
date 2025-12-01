"""
Main Window - Primary application window with all UI components
"""

from PySide6.QtWidgets import (QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, 
                               QToolBar, QStatusBar, QScrollArea, QListWidget,
                               QListWidgetItem, QDockWidget, QFileDialog, QLineEdit,
                               QPushButton, QLabel, QMessageBox, QSpinBox, QComboBox,
                               QColorDialog)
from PySide6.QtCore import Qt, QTimer, QSize
from PySide6.QtGui import QAction, QKeySequence, QPixmap, QIcon, QColor
import fitz
import os

from render_worker import RenderWorker
from page_cache import PageCache
from thumbnail_manager import ThumbnailManager
from search_manager import SearchManager
from annotation_manager import AnnotationManager
from pdf_label_with_overlay import PDFLabelWithOverlay
from tool_palette import ToolPalette


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
        
        # Tool Palette
        self.tool_palette = None
        
        # Setup UI
        self._setup_ui()
        self._setup_connections()
        
        # Create and show tool palette
        self._setup_tool_palette()
        
        # Restore settings
        if self.settings.get('sidebar_visible', True):
            self.sidebar_dock.setVisible(True)
    
    def _setup_ui(self):
        """Setup user interface"""
        self.setWindowTitle("DocLens")
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
        

    
    def _setup_toolbar(self):
        """Setup toolbar with actions - organized by function"""
        
        # === FILE & NAVIGATION TOOLBAR ===
        file_toolbar = QToolBar("File & Navigation")
        file_toolbar.setIconSize(QSize(24, 24))
        self.addToolBar(file_toolbar)
        
        # File actions
        open_action = QAction("📂 Open", self)
        open_action.setShortcut(QKeySequence.Open)
        open_action.triggered.connect(self.open_file_dialog)
        file_toolbar.addAction(open_action)
        
        file_toolbar.addSeparator()
        
        # Tool Palette Toggle
        palette_action = QAction("🎨 Tools", self)
        palette_action.setCheckable(True)
        palette_action.setChecked(True)
        palette_action.setShortcut("T")
        palette_action.triggered.connect(self._toggle_tool_palette)
        file_toolbar.addAction(palette_action)
        self.palette_action = palette_action
        
        file_toolbar.addSeparator()
        
        # Navigation
        prev_action = QAction("◀ Previous", self)
        prev_action.setShortcut(Qt.Key_PageUp)
        prev_action.triggered.connect(self.prev_page)
        file_toolbar.addAction(prev_action)
        
        next_action = QAction("▶ Next", self)
        next_action.setShortcut(Qt.Key_PageDown)
        next_action.triggered.connect(self.next_page)
        file_toolbar.addAction(next_action)
        
        # Page number
        file_toolbar.addWidget(QLabel("  Page: "))
        self.page_spinbox = QSpinBox()
        self.page_spinbox.setMinimum(1)
        self.page_spinbox.valueChanged.connect(self._on_page_spinbox_changed)
        file_toolbar.addWidget(self.page_spinbox)
        
        file_toolbar.addSeparator()
        
        # Save & Print
        save_action = QAction("💾 Save", self)
        save_action.setShortcut(QKeySequence.Save)
        save_action.triggered.connect(self.save_with_annotations)
        file_toolbar.addAction(save_action)
        
        print_action = QAction("🖨 Print", self)
        print_action.setShortcut(QKeySequence.Print)
        print_action.triggered.connect(self.print_pdf)
        file_toolbar.addAction(print_action)
        
        # === VIEW TOOLBAR ===
        self.addToolBarBreak()
        view_toolbar = QToolBar("View Controls")
        view_toolbar.setIconSize(QSize(24, 24))
        self.addToolBar(view_toolbar)
        
        # Zoom controls
        zoom_out_action = QAction("🔍- Zoom Out", self)
        zoom_out_action.setShortcut(QKeySequence.ZoomOut)
        zoom_out_action.triggered.connect(self.zoom_out)
        view_toolbar.addAction(zoom_out_action)
        
        zoom_in_action = QAction("🔍+ Zoom In", self)
        zoom_in_action.setShortcut(QKeySequence.ZoomIn)
        zoom_in_action.triggered.connect(self.zoom_in)
        view_toolbar.addAction(zoom_in_action)
        
        self.zoom_combo = QComboBox()
        self.zoom_combo.addItems(["50%", "75%", "100%", "125%", "150%", "200%", "Fit Width", "Fit Page"])
        self.zoom_combo.setCurrentText("100%")
        self.zoom_combo.currentTextChanged.connect(self._on_zoom_combo_changed)
        view_toolbar.addWidget(self.zoom_combo)
        
        view_toolbar.addSeparator()
        
        # Fit controls
        fit_width_action = QAction("↔ Fit Width", self)
        fit_width_action.triggered.connect(self._fit_width)
        view_toolbar.addAction(fit_width_action)
        
        fit_page_action = QAction("⛶ Fit Page", self)
        fit_page_action.triggered.connect(self._fit_page)
        view_toolbar.addAction(fit_page_action)
        
        view_toolbar.addSeparator()
        
        # Rotate
        rotate_left_action = QAction("↶ Rotate Left", self)
        rotate_left_action.triggered.connect(self.rotate_left)
        view_toolbar.addAction(rotate_left_action)
        
        rotate_right_action = QAction("↷ Rotate Right", self)
        rotate_right_action.triggered.connect(self.rotate_right)
        view_toolbar.addAction(rotate_right_action)
        
        view_toolbar.addSeparator()
        
        # Fullscreen
        fullscreen_action = QAction("⛶ Fullscreen", self)
        fullscreen_action.setShortcut(Qt.Key_F11)
        fullscreen_action.setCheckable(True)
        fullscreen_action.triggered.connect(self.toggle_fullscreen)
        view_toolbar.addAction(fullscreen_action)
        self.fullscreen_action = fullscreen_action
        
        # === ANNOTATION TOOLBAR ===
        self.addToolBarBreak()
        annotation_toolbar = QToolBar("Annotations")
        annotation_toolbar.setIconSize(QSize(24, 24))
        self.addToolBar(annotation_toolbar)
        
        # Color Picker
        annotation_toolbar.addWidget(QLabel("  Color: "))
        
        # Preset colors
        preset_colors = [
            ("Yellow", QColor(255, 255, 0, 100)),
            ("Red", QColor(255, 0, 0, 200)),
            ("Blue", QColor(0, 0, 255, 200)),
            ("Green", QColor(0, 200, 0, 200)),
            ("Black", QColor(0, 0, 0, 255)),
        ]
        
        for name, color in preset_colors:
            btn = QPushButton()
            btn.setFixedSize(25, 25)
            btn.setStyleSheet(
                f"background-color: rgba({color.red()}, {color.green()}, {color.blue()}, {color.alpha()}); "
                f"border: 1px solid #666; border-radius: 3px;"
            )
            btn.setToolTip(name)
            btn.clicked.connect(lambda checked, c=color: self._set_color(c))
            annotation_toolbar.addWidget(btn)
        
        # Custom color picker button
        self.color_button = QPushButton("...")
        self.color_button.setFixedSize(30, 25)
        self.current_color = QColor(255, 255, 0, 100)  # Default yellow
        self.color_button.setStyleSheet("border: 2px solid #333; font-weight: bold;")
        self.color_button.setToolTip("Choose custom color")
        self.color_button.clicked.connect(self._choose_color)
        annotation_toolbar.addWidget(self.color_button)
        
        annotation_toolbar.addSeparator()
        
        # Highlight & Rectangle
        highlight_action = QAction("🖍 Highlight", self)
        highlight_action.setCheckable(True)
        highlight_action.triggered.connect(lambda: self.set_drawing_mode('highlight'))
        annotation_toolbar.addAction(highlight_action)
        self.highlight_action = highlight_action
        
        rectangle_action = QAction("▭ Rectangle", self)
        rectangle_action.setCheckable(True)
        rectangle_action.triggered.connect(lambda: self.set_drawing_mode('rectangle'))
        annotation_toolbar.addAction(rectangle_action)
        self.rectangle_action = rectangle_action
        
        circle_action = QAction("⭕ Circle", self)
        circle_action.setCheckable(True)
        circle_action.triggered.connect(lambda: self.set_drawing_mode('circle'))
        annotation_toolbar.addAction(circle_action)
        self.circle_action = circle_action
        
        annotation_toolbar.addSeparator()
        
        # Text
        text_action = QAction("📝 Text Box", self)
        text_action.setCheckable(True)
        text_action.triggered.connect(lambda: self.set_drawing_mode('text'))
        annotation_toolbar.addAction(text_action)
        self.text_action = text_action
        
        # === DRAWING TOOLBAR ===
        self.addToolBarBreak()
        drawing_toolbar = QToolBar("Drawing Tools")
        drawing_toolbar.setIconSize(QSize(24, 24))
        self.addToolBar(drawing_toolbar)
        
        pen_action = QAction("✏ Pen", self)
        pen_action.setCheckable(True)
        pen_action.triggered.connect(lambda: self.set_drawing_mode('pen'))
        drawing_toolbar.addAction(pen_action)
        self.pen_action = pen_action
        
        line_action = QAction("📏 Line", self)
        line_action.setCheckable(True)
        line_action.triggered.connect(lambda: self.set_drawing_mode('line'))
        drawing_toolbar.addAction(line_action)
        self.line_action = line_action
        
        arrow_action = QAction("➡ Arrow", self)
        arrow_action.setCheckable(True)
        arrow_action.triggered.connect(lambda: self.set_drawing_mode('arrow'))
        drawing_toolbar.addAction(arrow_action)
        self.arrow_action = arrow_action
        
        drawing_toolbar.addSeparator()
        
        # Erase & Clear
        erase_action = QAction("🧹 Erase", self)
        erase_action.setCheckable(True)
        erase_action.triggered.connect(lambda: self.set_drawing_mode('erase'))
        drawing_toolbar.addAction(erase_action)
        self.erase_action = erase_action
        
        clear_mode_action = QAction("⊗ Clear Mode", self)
        clear_mode_action.triggered.connect(self.clear_drawing_mode)
        drawing_toolbar.addAction(clear_mode_action)
        
        clear_all_action = QAction("🗑 Clear All", self)
        clear_all_action.triggered.connect(self.clear_all_annotations)
        drawing_toolbar.addAction(clear_all_action)
        
        # === TEXT & SEARCH TOOLBAR ===
        self.addToolBarBreak()
        text_toolbar = QToolBar("Text & Search")
        text_toolbar.setIconSize(QSize(24, 24))
        self.addToolBar(text_toolbar)
        
        # Text selection
        select_text_action = QAction("📋 Select Text", self)
        select_text_action.setCheckable(True)
        select_text_action.triggered.connect(lambda: self.set_drawing_mode('select_text'))
        text_toolbar.addAction(select_text_action)
        self.select_text_action = select_text_action
        
        text_toolbar.addSeparator()
        
        # Search
        text_toolbar.addWidget(QLabel("  Search: "))
        self.search_input = QLineEdit()
        self.search_input.setPlaceholderText("Find text...")
        self.search_input.setMaximumWidth(200)
        self.search_input.returnPressed.connect(self.search_text)
        text_toolbar.addWidget(self.search_input)
        
        search_action = QAction("🔍 Find", self)
        search_action.setShortcut(QKeySequence.Find)
        search_action.triggered.connect(self.search_text)
        text_toolbar.addAction(search_action)
        
        search_next_action = QAction("▶ Next", self)
        search_next_action.setShortcut(QKeySequence.FindNext)
        search_next_action.triggered.connect(self.search_next)
        text_toolbar.addAction(search_next_action)
    
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
        
        # Scroll area viewport change
        self.scroll_area.verticalScrollBar().valueChanged.connect(self._on_scroll)
    
    def _setup_tool_palette(self):
        """Setup floating tool palette"""
        self.tool_palette = ToolPalette(self)
        
        # Connect signals
        self.tool_palette.tool_selected.connect(self._on_palette_tool_selected)
        self.tool_palette.color_changed.connect(self._on_palette_color_changed)
        
        # Position palette at right side of window
        palette_x = self.width() - self.tool_palette.width() - 20
        palette_y = 100
        self.tool_palette.move(palette_x, palette_y)
        
        # Show palette
        self.tool_palette.show()
    
    def _on_palette_tool_selected(self, tool):
        """Handle tool selection from palette"""
        if tool is None:
            self.clear_drawing_mode()
        else:
            self.set_drawing_mode(tool)
    
    def _on_palette_color_changed(self, color):
        """Handle color change from palette"""
        self.current_color = color
        
        # Update all widgets with new color
        for widget in self.page_widgets.values():
            if widget.drawing_mode:
                widget.set_drawing_mode(widget.drawing_mode, color)
    
    def _toggle_tool_palette(self, checked):
        """Toggle tool palette visibility"""
        if self.tool_palette:
            if checked:
                self.tool_palette.show()
            else:
                self.tool_palette.hide()

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
            self.setWindowTitle(f"DocLens - {os.path.basename(file_path)}")
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
    
    def _update_current_page_from_scroll(self):
        """Update current page number based on scroll position"""
        if not self.doc:
            return
        
        # Get scroll position
        scrollbar = self.scroll_area.verticalScrollBar()
        scroll_pos = scrollbar.value()
        viewport_height = self.scroll_area.viewport().height()
        viewport_center = scroll_pos + viewport_height / 2
        
        # Find which page is at the center of viewport
        current_y = self.scroll_layout.contentsMargins().top()
        
        for page_num in range(len(self.doc)):
            if page_num in self.page_widgets:
                widget = self.page_widgets[page_num]
                widget_height = widget.height()
                
                # Check if viewport center is within this page
                if current_y <= viewport_center <= current_y + widget_height:
                    if self.current_page != page_num:
                        self.current_page = page_num
                        self.page_spinbox.blockSignals(True)
                        self.page_spinbox.setValue(page_num + 1)
                        self.page_spinbox.blockSignals(False)
                        self._update_page_label()
                    break
                
                current_y += widget_height + self.scroll_layout.spacing()
    
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
        
        # Create PDFLabelWithOverlay for search and annotation support
        widget = PDFLabelWithOverlay(page_num)
        widget.set_zoom(self.zoom_level)
        widget.doc_path = self.doc_path  # Set document path for text extraction
        
        # Connect annotation signals
        widget.annotation_added.connect(self._on_annotation_added)
        widget.annotation_removed.connect(self._on_annotation_removed)
        
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
        
        # Update current page based on scroll position
        self._update_current_page_from_scroll()
        
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
        
        # Update zoom on all widgets
        for widget in self.page_widgets.values():
            widget.set_zoom(zoom)
        
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
            
            # Update ALL page widgets with search results
            for page_num in range(len(self.doc)):
                if page_num in self.page_widgets:
                    widget = self.page_widgets[page_num]
                    page_results = self.search_manager.get_results_for_page(page_num)
                    widget.set_search_results(page_results)
        else:
            self.status_label.setText("No results found")
            # Clear search highlights
            for widget in self.page_widgets.values():
                widget.set_search_results([])
    
    def search_next(self):
        """Go to next search result"""
        result = self.search_manager.next_result()
        if result:
            self.goto_page(result['page'])
    
    def rotate_left(self):
        """Rotate all pages 90 degrees counter-clockwise"""
        if not self.doc:
            return
        
        self.rotation = (self.rotation - 90) % 360
        
        # Clear cache and re-render
        self.page_cache.clear()
        self.page_widgets.clear()
        
        # Clear layout
        while self.scroll_layout.count():
            item = self.scroll_layout.takeAt(0)
            if item.widget():
                item.widget().deleteLater()
        
        self._render_visible_pages()
        self.status_label.setText(f"Rotated: {self.rotation}°")
    
    def rotate_right(self):
        """Rotate all pages 90 degrees clockwise"""
        if not self.doc:
            return
        
        self.rotation = (self.rotation + 90) % 360
        
        # Clear cache and re-render
        self.page_cache.clear()
        self.page_widgets.clear()
        
        # Clear layout
        while self.scroll_layout.count():
            item = self.scroll_layout.takeAt(0)
            if item.widget():
                item.widget().deleteLater()
        
        self._render_visible_pages()
        self.status_label.setText(f"Rotated: {self.rotation}°")
    
    def print_pdf(self):
        """Print the PDF"""
        if not self.doc:
            return
        
        from PySide6.QtPrintSupport import QPrinter, QPrintDialog
        from PySide6.QtGui import QPainter
        
        printer = QPrinter(QPrinter.HighResolution)
        dialog = QPrintDialog(printer, self)
        
        if dialog.exec() == QPrintDialog.Accepted:
            painter = QPainter(printer)
            
            try:
                for page_num in range(len(self.doc)):
                    if page_num > 0:
                        printer.newPage()
                    
                    # Render page at high resolution
                    page = self.doc[page_num]
                    mat = fitz.Matrix(2.0, 2.0)
                    pix = page.get_pixmap(matrix=mat)
                    
                    # Convert to QPixmap
                    from PySide6.QtGui import QImage
                    img_format = QImage.Format_RGB888
                    qimg = QImage(pix.samples, pix.width, pix.height, pix.stride, img_format)
                    pixmap = QPixmap.fromImage(qimg.copy())
                    
                    # Scale to fit printer page
                    scaled = pixmap.scaled(
                        printer.pageRect().size().toSize(),
                        Qt.KeepAspectRatio,
                        Qt.SmoothTransformation
                    )
                    
                    painter.drawPixmap(0, 0, scaled)
                
                painter.end()
                self.status_label.setText("Print completed")
                
            except Exception as e:
                painter.end()
                QMessageBox.warning(self, "Print Error", f"Failed to print: {str(e)}")
    
    def toggle_fullscreen(self):
        """Toggle fullscreen mode"""
        if self.isFullScreen():
            self.showNormal()
            self.fullscreen_action.setChecked(False)
        else:
            self.showFullScreen()
            self.fullscreen_action.setChecked(True)
    
    def set_drawing_mode(self, mode):
        """Set drawing mode: 'highlight', 'rectangle', 'pen', 'line', 'arrow', 'circle', 'text', 'erase', 'select_text'"""
        # Uncheck all drawing actions
        self.highlight_action.setChecked(False)
        self.rectangle_action.setChecked(False)
        self.pen_action.setChecked(False)
        self.line_action.setChecked(False)
        self.arrow_action.setChecked(False)
        self.circle_action.setChecked(False)
        self.text_action.setChecked(False)
        self.erase_action.setChecked(False)
        self.select_text_action.setChecked(False)
        
        # Use current selected color for drawing modes
        if mode == 'highlight':
            self.highlight_action.setChecked(True)
            color = self.current_color
            self.status_label.setText("Highlight mode: Click and drag to highlight")
        elif mode == 'rectangle':
            self.rectangle_action.setChecked(True)
            color = self.current_color
            self.status_label.setText("Rectangle mode: Click and drag to draw rectangle")
        elif mode == 'pen':
            self.pen_action.setChecked(True)
            color = self.current_color
            self.status_label.setText("Pen mode: Click and drag to draw")
        elif mode == 'line':
            self.line_action.setChecked(True)
            color = self.current_color
            self.status_label.setText("Line mode: Click and drag to draw line")
        elif mode == 'arrow':
            self.arrow_action.setChecked(True)
            color = self.current_color
            self.status_label.setText("Arrow mode: Click and drag to draw arrow")
        elif mode == 'circle':
            self.circle_action.setChecked(True)
            color = self.current_color
            self.status_label.setText("Circle mode: Click and drag to draw circle")
        elif mode == 'text':
            self.text_action.setChecked(True)
            color = self.current_color
            self.status_label.setText("Text mode: Click to add text")
        elif mode == 'erase':
            self.erase_action.setChecked(True)
            color = None
            self.status_label.setText("Erase mode: Click on annotation to delete")
        elif mode == 'select_text':
            self.select_text_action.setChecked(True)
            color = None
            self.status_label.setText("Select Text mode: Click and drag to select text")
        else:
            color = None
            self.status_label.setText("Ready")
        
        # Set mode on all page widgets
        for widget in self.page_widgets.values():
            widget.set_drawing_mode(mode, color)
    
    def _set_color(self, color):
        """Set annotation color from preset"""
        self.current_color = color
        
        # Update all widgets with new color
        for widget in self.page_widgets.values():
            if widget.drawing_mode:
                widget.set_drawing_mode(widget.drawing_mode, color)
        
        self.status_label.setText(f"Color changed")
    
    def _choose_color(self):
        """Open color picker dialog"""
        color = QColorDialog.getColor(self.current_color, self, "Choose Annotation Color",
                                      QColorDialog.ShowAlphaChannel)
        
        if color.isValid():
            self._set_color(color)
    
    def clear_drawing_mode(self):
        """Clear drawing mode (stop drawing)"""
        self.highlight_action.setChecked(False)
        self.rectangle_action.setChecked(False)
        self.pen_action.setChecked(False)
        self.line_action.setChecked(False)
        self.arrow_action.setChecked(False)
        self.circle_action.setChecked(False)
        self.text_action.setChecked(False)
        self.erase_action.setChecked(False)
        self.select_text_action.setChecked(False)
        
        for widget in self.page_widgets.values():
            widget.set_drawing_mode(None)
        
        self.status_label.setText("Drawing mode cleared")
    
    def clear_all_annotations(self):
        """Clear all annotations"""
        if not self.annotation_manager.annotations:
            self.status_label.setText("No annotations to clear")
            return
        
        reply = QMessageBox.question(
            self,
            "Clear All Annotations",
            f"Are you sure you want to clear all {len(self.annotation_manager.annotations)} annotations?",
            QMessageBox.Yes | QMessageBox.No,
            QMessageBox.No
        )
        
        if reply == QMessageBox.Yes:
            self.annotation_manager.clear_annotations()
            
            # Clear from all widgets
            for widget in self.page_widgets.values():
                widget.set_annotations([])
                widget.update()
            
            self.status_label.setText("All annotations cleared")
    
    def _on_annotation_added(self, page_num, rect, color, annotation_type):
        """Handle annotation added from widget"""
        self.annotation_manager.add_annotation(page_num, rect, color, annotation_type)
        
        # Update the widget to show the annotation
        if page_num in self.page_widgets:
            widget = self.page_widgets[page_num]
            annotations = self.annotation_manager.get_annotations_for_page(page_num)
            widget.set_annotations(annotations)
            widget.update()  # Force repaint
        
        self.status_label.setText(f"Annotation added to page {page_num + 1} (Total: {len(self.annotation_manager.annotations)})")
    
    def _on_annotation_removed(self, page_num, annotation):
        """Handle annotation removed from widget"""
        # Remove from annotation manager
        if annotation in self.annotation_manager.annotations:
            self.annotation_manager.annotations.remove(annotation)
            
            # Update all widgets on this page to reflect the removal
            if page_num in self.page_widgets:
                widget = self.page_widgets[page_num]
                annotations = self.annotation_manager.get_annotations_for_page(page_num)
                widget.set_annotations(annotations)
                widget.update()
            
            self.status_label.setText(f"Annotation removed from page {page_num + 1} (Total: {len(self.annotation_manager.annotations)})")
    
    def save_with_annotations(self):
        """Save PDF with annotations"""
        if not self.doc_path:
            QMessageBox.warning(self, "No Document", "Please open a PDF first")
            return
        
        if not self.annotation_manager.annotations:
            QMessageBox.information(self, "No Annotations", "No annotations to save")
            return
        
        file_path, _ = QFileDialog.getSaveFileName(
            self,
            "Save PDF with Annotations",
            self.doc_path.replace('.pdf', '_annotated.pdf'),
            "PDF Files (*.pdf)"
        )
        
        if file_path:
            success = self.annotation_manager.save_to_pdf(self.doc_path, file_path)
            if success:
                QMessageBox.information(self, "Success", f"PDF saved with {len(self.annotation_manager.annotations)} annotations")
            else:
                QMessageBox.warning(self, "Error", "Failed to save annotations")
    
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

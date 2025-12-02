"""
Tool Palette - Floating tool palette for annotations and drawing
"""

from PySide6.QtWidgets import (QWidget, QVBoxLayout, QHBoxLayout, QPushButton, 
                               QLabel, QFrame, QColorDialog, QButtonGroup)
from PySide6.QtCore import Qt, Signal, QPoint, QSize
from PySide6.QtGui import QColor, QCursor
from icon_manager import IconManager


class ToolPalette(QFrame):
    """Floating tool palette for PDF annotations"""
    
    tool_selected = Signal(str)  # Emit tool name when selected
    color_changed = Signal(QColor)  # Emit when color changes
    
    def __init__(self, parent=None):
        super().__init__(parent)
        # Remove Qt.Tool flag to keep it within parent window
        self.setWindowFlags(Qt.FramelessWindowHint)
        self.setAttribute(Qt.WA_TranslucentBackground)
        
        self.current_tool = None
        self.current_color = QColor(255, 255, 0, 100)
        self.dragging = False
        self.drag_position = QPoint()
        self.user_wants_visible = True  # Track user preference separately from actual visibility
        
        # Set window icon same as main application
        if parent:
            self.setWindowIcon(parent.windowIcon())
        
        self._setup_ui()
        
    def _setup_ui(self):
        """Setup the palette UI"""
        # Main layout
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(0, 0, 0, 0)
        main_layout.setSpacing(0)
        
        # Container with rounded corners and shadow
        container = QFrame()
        container.setObjectName("paletteContainer")
        container.setStyleSheet("""
            QFrame#paletteContainer {
                background-color: rgba(45, 45, 45, 240);
                border-radius: 5px;
                border: 1px solid rgba(255, 255, 255, 0.1);
            }
            QPushButton {
                background-color: rgba(70, 70, 70, 200);
                border: none;
                border-radius: 4px;
                color: white;
                font-size: 14px;
                padding: 4px;
                min-width: 32px;
                min-height: 32px;
                max-width: 32px;
                max-height: 32px;
            }
            QPushButton:hover {
                background-color: rgba(90, 90, 90, 255);
            }
            QPushButton:checked {
                background-color: rgba(33, 150, 243, 255);
                border: 2px solid rgba(255, 255, 255, 0.3);
            }
            QLabel {
                color: rgba(255, 255, 255, 200);
                font-size: 10px;
                padding: 2px;
            }
        """)
        
        layout = QVBoxLayout(container)
        layout.setContentsMargins(12, 12, 12, 12)
        layout.setSpacing(12)
        
        # Title bar (for dragging)
        title_bar = QFrame()
        title_bar.setFixedHeight(30)
        title_bar.setCursor(Qt.SizeAllCursor)
        title_layout = QHBoxLayout(title_bar)
        title_layout.setContentsMargins(8, 0, 8, 0)
        
        # Icon label
        icon_label = QLabel()
        if self.parent() and self.parent().windowIcon() and not self.parent().windowIcon().isNull():
            # Use application icon
            pixmap = self.parent().windowIcon().pixmap(20, 20)
            icon_label.setPixmap(pixmap)
        else:
            # Fallback to emoji
            icon_label.setText("🎨")
            icon_label.setStyleSheet("font-size: 16px;")
        title_layout.addWidget(icon_label)
        
        # Title text
        title_label = QLabel("Tools")
        title_label.setStyleSheet("font-size: 12px; font-weight: bold; color: white; margin-left: 4px;")
        title_layout.addWidget(title_label)
        
        title_layout.addStretch()
        
        # Close button
        close_btn = QPushButton("×")
        close_btn.setFixedSize(24, 24)
        close_btn.setStyleSheet("""
            QPushButton {
                background-color: rgba(255, 70, 70, 200);
                font-size: 16px;
                font-weight: bold;
                min-width: 24px;
                min-height: 24px;
                padding: 0px;
            }
            QPushButton:hover {
                background-color: rgba(255, 50, 50, 255);
            }
        """)
        close_btn.clicked.connect(self._on_close_clicked)
        title_layout.addWidget(close_btn)
        
        layout.addWidget(title_bar)
        
        # Content container (can be collapsed)
        self.content_widget = QWidget()
        content_layout = QVBoxLayout(self.content_widget)
        content_layout.setContentsMargins(0, 0, 0, 0)
        content_layout.setSpacing(10)
        
        # Separator
        separator = QFrame()
        separator.setFrameShape(QFrame.HLine)
        separator.setStyleSheet("background-color: rgba(255, 255, 255, 0.1);")
        separator.setFixedHeight(1)
        content_layout.addWidget(separator)
        
        # Color section
        color_label = QLabel("Color:")
        color_label.setStyleSheet("font-weight: bold; font-size: 10px;")
        content_layout.addWidget(color_label)
        
        # Color buttons in grid (2 rows)
        color_grid = QHBoxLayout()
        color_grid.setSpacing(6)
        
        # Preset colors
        self.preset_colors = [
            QColor(255, 255, 0, 100),   # Yellow
            QColor(255, 0, 0, 200),     # Red
            QColor(0, 0, 255, 200),     # Blue
            QColor(0, 200, 0, 200),     # Green
            QColor(0, 0, 0, 255),       # Black
        ]
        
        for color in self.preset_colors:
            btn = QPushButton()
            btn.setFixedSize(24, 24)
            btn.setStyleSheet(f"""
                QPushButton {{
                    background-color: rgba({color.red()}, {color.green()}, {color.blue()}, {color.alpha()});
                    border: 2px solid rgba(255, 255, 255, 0.3);
                    border-radius: 12px;
                    min-width: 24px;
                    min-height: 24px;
                }}
                QPushButton:hover {{
                    border: 2px solid rgba(255, 255, 255, 0.8);
                }}
            """)
            btn.clicked.connect(lambda checked, c=color: self._set_color(c))
            color_grid.addWidget(btn)
        
        # Custom color button
        custom_color_btn = QPushButton()
        custom_color_btn.setIcon(IconManager.get_icon('palette', '#9C27B0'))
        custom_color_btn.setIconSize(QSize(14, 14))
        custom_color_btn.setFixedSize(24, 24)
        custom_color_btn.setToolTip("Custom color")
        custom_color_btn.clicked.connect(self._choose_custom_color)
        color_grid.addWidget(custom_color_btn)
        
        content_layout.addLayout(color_grid)
        
        # Separator
        separator2 = QFrame()
        separator2.setFrameShape(QFrame.HLine)
        separator2.setStyleSheet("background-color: rgba(255, 255, 255, 0.1);")
        separator2.setFixedHeight(1)
        content_layout.addWidget(separator2)
        
        # Annotation tools
        anno_label = QLabel("Annotations")
        anno_label.setStyleSheet("font-weight: bold; font-size: 11px;")
        content_layout.addWidget(anno_label)
        
        anno_layout = QHBoxLayout()
        anno_layout.setSpacing(8)
        
        self.button_group = QButtonGroup(self)
        self.button_group.setExclusive(True)
        
        # Store button-to-tool mapping
        self.tool_buttons = {}
        
        anno_tools = [
            ("highlight", "#FFEB3B", "Highlight"),
            ("rectangle", "#F44336", "Rectangle"),
            ("circle", "#4CAF50", "Circle"),
            ("text", "#2196F3", "Text Box"),
        ]
        
        for tool, color, tooltip in anno_tools:
            btn = QPushButton()
            btn.setIcon(IconManager.get_icon(tool, color))
            btn.setIconSize(QSize(18, 18))
            btn.setCheckable(True)
            btn.setToolTip(tooltip)
            btn.clicked.connect(lambda checked, t=tool: self._select_tool(t))
            self.button_group.addButton(btn)
            self.tool_buttons[tool] = btn
            anno_layout.addWidget(btn)
        
        content_layout.addLayout(anno_layout)
        
        # Drawing tools
        draw_label = QLabel("Drawing")
        draw_label.setStyleSheet("font-weight: bold; font-size: 11px;")
        content_layout.addWidget(draw_label)
        
        draw_layout = QHBoxLayout()
        draw_layout.setSpacing(8)
        
        draw_tools = [
            ("pen", "#3F51B5", "Pen"),
            ("line", "#607D8B", "Line"),
            ("arrow", "#FF5722", "Arrow"),
        ]
        
        for tool, color, tooltip in draw_tools:
            btn = QPushButton()
            btn.setIcon(IconManager.get_icon(tool, color))
            btn.setIconSize(QSize(18, 18))
            btn.setCheckable(True)
            btn.setToolTip(tooltip)
            btn.clicked.connect(lambda checked, t=tool: self._select_tool(t))
            self.button_group.addButton(btn)
            self.tool_buttons[tool] = btn
            draw_layout.addWidget(btn)
        
        content_layout.addLayout(draw_layout)
        
        # Separator
        separator3 = QFrame()
        separator3.setFrameShape(QFrame.HLine)
        separator3.setStyleSheet("background-color: rgba(255, 255, 255, 0.1);")
        separator3.setFixedHeight(1)
        content_layout.addWidget(separator3)
        
        # Utility tools
        util_layout = QHBoxLayout()
        util_layout.setSpacing(8)
        
        util_tools = [
            ("select_text", "#00BCD4", "Select Text"),
            ("erase", "#FF9800", "Erase"),
            ("clear", "#9E9E9E", "Clear Mode"),
        ]
        
        for tool, color, tooltip in util_tools:
            btn = QPushButton()
            btn.setIcon(IconManager.get_icon(tool, color))
            btn.setIconSize(QSize(18, 18))
            if tool != "clear":
                btn.setCheckable(True)
                self.button_group.addButton(btn)
                self.tool_buttons[tool] = btn
            btn.setToolTip(tooltip)
            btn.clicked.connect(lambda checked, t=tool: self._select_tool(t))
            util_layout.addWidget(btn)
        
        content_layout.addLayout(util_layout)
        
        # Ensure content widget is visible
        self.content_widget.setVisible(True)
        layout.addWidget(self.content_widget)
        
        main_layout.addWidget(container)
        
        # Set fixed width (reduced to fit smaller buttons)
        self.setFixedWidth(280)
        self.setFixedHeight(400)
    
    def _on_close_clicked(self):
        """Handle close button click - notify parent instead of hiding directly"""
        # Mark that user wants to hide
        self.user_wants_visible = False
        # Hide the palette
        self.hide()
        # Notify parent to update toolbar button state
        if self.parent():
            # Try to uncheck the palette toggle button in parent
            parent = self.parent()
            if hasattr(parent, 'palette_action'):
                parent.palette_action.setChecked(False)
    
    def _select_tool(self, tool):
        """Handle tool selection"""
        if tool == "clear":
            # Clear all selections
            for btn in self.button_group.buttons():
                btn.setChecked(False)
            self.current_tool = None
            self.tool_selected.emit(None)
        else:
            self.current_tool = tool
            self.tool_selected.emit(tool)
    
    def _set_color(self, color):
        """Set current color"""
        self.current_color = color
        self.color_changed.emit(color)
    
    def _choose_custom_color(self):
        """Open color picker dialog"""
        color = QColorDialog.getColor(
            self.current_color, 
            self, 
            "Choose Color",
            QColorDialog.ShowAlphaChannel
        )
        
        if color.isValid():
            self._set_color(color)
    
    def restore_tool_selection(self, tool):
        """Restore tool selection from settings"""
        if not tool or tool not in self.tool_buttons:
            return
        
        # Check the corresponding button
        btn = self.tool_buttons[tool]
        btn.setChecked(True)
        
        # Set current tool and emit signal
        self.current_tool = tool
        self.tool_selected.emit(tool)
    
    def mousePressEvent(self, event):
        """Handle mouse press for dragging"""
        if event.button() == Qt.LeftButton:
            # Check if clicking on title bar area (top 42px)
            if event.position().y() < 42:
                self.dragging = True
                self.drag_position = event.position().toPoint()
                event.accept()
    
    def mouseMoveEvent(self, event):
        """Handle mouse move for dragging"""
        if self.dragging and event.buttons() == Qt.LeftButton:
            # Calculate new position relative to parent
            new_pos = self.mapToParent(event.position().toPoint() - self.drag_position)
            
            # Constrain to parent window bounds
            if self.parent():
                parent_width = self.parent().width()
                parent_height = self.parent().height()
                palette_width = self.width()
                palette_height = self.height()
                
                # Keep within parent bounds
                x = max(0, min(new_pos.x(), parent_width - palette_width))
                y = max(0, min(new_pos.y(), parent_height - palette_height))
                
                self.move(x, y)
            else:
                self.move(new_pos)
            
            event.accept()
    
    def mouseReleaseEvent(self, event):
        """Handle mouse release"""
        if event.button() == Qt.LeftButton:
            self.dragging = False
            event.accept()

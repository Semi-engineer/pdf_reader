"""
Tool Palette - Floating tool palette for annotations and drawing
"""

from PySide6.QtWidgets import (QWidget, QVBoxLayout, QHBoxLayout, QPushButton, 
                               QLabel, QFrame, QColorDialog, QButtonGroup)
from PySide6.QtCore import Qt, Signal, QPoint
from PySide6.QtGui import QColor, QCursor


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
        self.is_collapsed = False
        
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
                border-radius: 12px;
                border: 1px solid rgba(255, 255, 255, 0.1);
            }
            QPushButton {
                background-color: rgba(70, 70, 70, 200);
                border: none;
                border-radius: 6px;
                color: white;
                font-size: 18px;
                padding: 8px;
                min-width: 40px;
                min-height: 40px;
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
        layout.setSpacing(8)
        
        # Title bar (for dragging)
        title_bar = QFrame()
        title_bar.setFixedHeight(30)
        title_bar.setCursor(Qt.SizeAllCursor)
        title_layout = QHBoxLayout(title_bar)
        title_layout.setContentsMargins(8, 0, 8, 0)
        
        title_label = QLabel("🎨 Tools")
        title_label.setStyleSheet("font-size: 12px; font-weight: bold; color: white;")
        title_layout.addWidget(title_label)
        
        title_layout.addStretch()
        
        # Collapse/Expand button
        self.collapse_btn = QPushButton("▼")
        self.collapse_btn.setFixedSize(24, 24)
        self.collapse_btn.setStyleSheet("""
            QPushButton {
                background-color: rgba(100, 100, 100, 200);
                font-size: 12px;
                font-weight: bold;
                min-width: 24px;
                min-height: 24px;
                padding: 0px;
            }
            QPushButton:hover {
                background-color: rgba(120, 120, 120, 255);
            }
        """)
        self.collapse_btn.setToolTip("Collapse/Expand")
        self.collapse_btn.clicked.connect(self._toggle_collapse)
        title_layout.addWidget(self.collapse_btn)
        
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
        close_btn.clicked.connect(self.hide)
        title_layout.addWidget(close_btn)
        
        layout.addWidget(title_bar)
        
        # Content container (can be collapsed)
        self.content_widget = QWidget()
        content_layout = QVBoxLayout(self.content_widget)
        content_layout.setContentsMargins(0, 0, 0, 0)
        content_layout.setSpacing(8)
        
        # Separator
        separator = QFrame()
        separator.setFrameShape(QFrame.HLine)
        separator.setStyleSheet("background-color: rgba(255, 255, 255, 0.1);")
        separator.setFixedHeight(1)
        content_layout.addWidget(separator)
        
        # Color section
        color_layout = QHBoxLayout()
        color_layout.setSpacing(4)
        
        color_label = QLabel("Color:")
        color_layout.addWidget(color_label)
        
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
            btn.setFixedSize(28, 28)
            btn.setStyleSheet(f"""
                QPushButton {{
                    background-color: rgba({color.red()}, {color.green()}, {color.blue()}, {color.alpha()});
                    border: 2px solid rgba(255, 255, 255, 0.3);
                    border-radius: 14px;
                    min-width: 28px;
                    min-height: 28px;
                }}
                QPushButton:hover {{
                    border: 2px solid rgba(255, 255, 255, 0.8);
                }}
            """)
            btn.clicked.connect(lambda checked, c=color: self._set_color(c))
            color_layout.addWidget(btn)
        
        # Custom color button
        custom_color_btn = QPushButton("⊕")
        custom_color_btn.setFixedSize(28, 28)
        custom_color_btn.setToolTip("Custom color")
        custom_color_btn.clicked.connect(self._choose_custom_color)
        color_layout.addWidget(custom_color_btn)
        
        content_layout.addLayout(color_layout)
        
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
        anno_layout.setSpacing(6)
        
        self.button_group = QButtonGroup(self)
        self.button_group.setExclusive(True)
        
        anno_tools = [
            ("🖍", "highlight", "Highlight"),
            ("▭", "rectangle", "Rectangle"),
            ("⭕", "circle", "Circle"),
            ("📝", "text", "Text Box"),
        ]
        
        for icon, tool, tooltip in anno_tools:
            btn = QPushButton(icon)
            btn.setCheckable(True)
            btn.setToolTip(tooltip)
            btn.clicked.connect(lambda checked, t=tool: self._select_tool(t))
            self.button_group.addButton(btn)
            anno_layout.addWidget(btn)
        
        content_layout.addLayout(anno_layout)
        
        # Drawing tools
        draw_label = QLabel("Drawing")
        draw_label.setStyleSheet("font-weight: bold; font-size: 11px;")
        content_layout.addWidget(draw_label)
        
        draw_layout = QHBoxLayout()
        draw_layout.setSpacing(6)
        
        draw_tools = [
            ("✏", "pen", "Pen"),
            ("📏", "line", "Line"),
            ("➡", "arrow", "Arrow"),
        ]
        
        for icon, tool, tooltip in draw_tools:
            btn = QPushButton(icon)
            btn.setCheckable(True)
            btn.setToolTip(tooltip)
            btn.clicked.connect(lambda checked, t=tool: self._select_tool(t))
            self.button_group.addButton(btn)
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
        util_layout.setSpacing(6)
        
        util_tools = [
            ("📋", "select_text", "Select Text"),
            ("🧹", "erase", "Erase"),
            ("⊗", "clear", "Clear Mode"),
        ]
        
        for icon, tool, tooltip in util_tools:
            btn = QPushButton(icon)
            if tool != "clear":
                btn.setCheckable(True)
                self.button_group.addButton(btn)
            btn.setToolTip(tooltip)
            btn.clicked.connect(lambda checked, t=tool: self._select_tool(t))
            util_layout.addWidget(btn)
        
        content_layout.addLayout(util_layout)
        
        layout.addWidget(self.content_widget)
        
        main_layout.addWidget(container)
        
        # Set fixed width
        self.setFixedWidth(220)
    
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
    
    def _toggle_collapse(self):
        """Toggle collapse/expand state"""
        self.is_collapsed = not self.is_collapsed
        
        if self.is_collapsed:
            self.content_widget.hide()
            self.collapse_btn.setText("▲")
            self.setFixedHeight(54)  # Just title bar height
        else:
            self.content_widget.show()
            self.collapse_btn.setText("▼")
            self.setFixedHeight(self.sizeHint().height())
        
        self.adjustSize()
    
    def set_collapsed(self, collapsed):
        """Set collapse state"""
        if collapsed != self.is_collapsed:
            self._toggle_collapse()
    
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

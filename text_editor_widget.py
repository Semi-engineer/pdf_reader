"""
Movable Text Editor Widget with formatting controls
"""

from PySide6.QtWidgets import (QWidget, QTextEdit, QVBoxLayout, QHBoxLayout, 
                               QPushButton, QSpinBox, QFontComboBox, QColorDialog,
                               QLabel, QFrame)
from PySide6.QtCore import Qt, Signal, QPoint
from PySide6.QtGui import QFont, QColor, QTextOption, QTextCharFormat, QCursor, QTextCursor


class MovableTextEditor(QWidget):
    """Text editor widget that can be moved and has formatting controls"""
    
    finished = Signal(str, QFont, QColor)  # text, font, color
    cancelled = Signal()
    
    def __init__(self, parent=None, initial_color=None):
        super().__init__(parent)
        
        self.initial_color = initial_color or QColor(0, 0, 0, 255)
        self.current_font = QFont()
        self.current_font.setPointSize(12)
        self.current_color = self.initial_color
        
        # For dragging
        self.dragging = False
        self.drag_position = QPoint()
        
        self._setup_ui()
        
        # Make widget frameless but keep it as a window
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.Tool)
        self.setAttribute(Qt.WA_TranslucentBackground, False)
        
    def _setup_ui(self):
        """Setup UI components"""
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)
        
        # Main container with border
        container = QFrame()
        container.setStyleSheet("""
            QFrame {
                background-color: rgba(255, 255, 255, 120);
                border: 3px solid #2196F3;
                border-radius: 8px;
            }
        """)
        container_layout = QVBoxLayout(container)
        container_layout.setContentsMargins(5, 5, 5, 5)
        container_layout.setSpacing(5)
        
        # Title bar for dragging
        title_bar = QWidget()
        title_bar.setStyleSheet("""
            QWidget {
                background-color: rgba(33, 150, 243, 180);
                border-radius: 4px;
            }
        """)
        title_bar.setCursor(QCursor(Qt.SizeAllCursor))
        title_bar.setFixedHeight(25)
        title_bar_layout = QHBoxLayout(title_bar)
        title_bar_layout.setContentsMargins(8, 2, 8, 2)
        
        title_label = QLabel("✎ Text Box (Drag to move)")
        title_label.setStyleSheet("color: white; font-weight: bold; font-size: 10pt;")
        title_bar_layout.addWidget(title_label)
        
        # Store title bar for mouse events
        self.title_bar = title_bar
        title_bar.mousePressEvent = self._title_mouse_press
        title_bar.mouseMoveEvent = self._title_mouse_move
        title_bar.mouseReleaseEvent = self._title_mouse_release
        
        container_layout.addWidget(title_bar)
        
        # Toolbar
        toolbar = QHBoxLayout()
        toolbar.setSpacing(5)
        
        # Font selector
        self.font_combo = QFontComboBox()
        self.font_combo.setCurrentFont(self.current_font)
        self.font_combo.setMaximumWidth(150)
        self.font_combo.currentFontChanged.connect(self._on_font_changed)
        toolbar.addWidget(QLabel("Font:"))
        toolbar.addWidget(self.font_combo)
        
        # Font size
        self.size_spin = QSpinBox()
        self.size_spin.setRange(8, 72)
        self.size_spin.setValue(12)
        self.size_spin.setSuffix(" pt")
        self.size_spin.setMaximumWidth(70)
        self.size_spin.valueChanged.connect(self._on_size_changed)
        toolbar.addWidget(self.size_spin)
        
        # Color button
        self.color_btn = QPushButton("Color")
        self.color_btn.setMaximumWidth(60)
        self.color_btn.clicked.connect(self._choose_color)
        self._update_color_button()
        toolbar.addWidget(self.color_btn)
        
        toolbar.addStretch()
        container_layout.addLayout(toolbar)
        
        # Text editor
        self.text_edit = QTextEdit()
        self.text_edit.setMinimumSize(200, 80)
        self.text_edit.setMaximumSize(800, 600)
        self.text_edit.setVerticalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        self.text_edit.setHorizontalScrollBarPolicy(Qt.ScrollBarAlwaysOff)
        self.text_edit.setWordWrapMode(QTextOption.WordWrap)
        self.text_edit.setLineWrapMode(QTextEdit.WidgetWidth)
        self.text_edit.setStyleSheet("""
            QTextEdit {
                background-color: rgba(255, 255, 255, 100);
                border: 1px solid #ccc;
                border-radius: 4px;
                padding: 8px;
            }
        """)
        self.text_edit.setFont(self.current_font)
        self.text_edit.textChanged.connect(self._auto_resize)
        container_layout.addWidget(self.text_edit)
        
        # Buttons
        button_layout = QHBoxLayout()
        button_layout.setSpacing(5)
        
        help_label = QLabel("Ctrl+Enter: Save | Esc: Cancel")
        help_label.setStyleSheet("color: #666; font-size: 9pt;")
        button_layout.addWidget(help_label)
        
        button_layout.addStretch()
        
        cancel_btn = QPushButton("Cancel")
        cancel_btn.clicked.connect(self._on_cancel)
        button_layout.addWidget(cancel_btn)
        
        save_btn = QPushButton("Save")
        save_btn.setStyleSheet("""
            QPushButton {
                background-color: #4CAF50;
                color: white;
                font-weight: bold;
                padding: 5px 15px;
                border-radius: 4px;
            }
            QPushButton:hover {
                background-color: #45a049;
            }
        """)
        save_btn.clicked.connect(self._on_save)
        button_layout.addWidget(save_btn)
        
        container_layout.addLayout(button_layout)
        
        layout.addWidget(container)
        
        # Install event filter for keyboard shortcuts
        self.text_edit.installEventFilter(self)
    
    def _title_mouse_press(self, event):
        """Handle mouse press on title bar"""
        if event.button() == Qt.LeftButton:
            self.dragging = True
            self.drag_position = event.globalPosition().toPoint() - self.frameGeometry().topLeft()
            event.accept()
    
    def _title_mouse_move(self, event):
        """Handle mouse move on title bar"""
        if self.dragging:
            self.move(event.globalPosition().toPoint() - self.drag_position)
            event.accept()
    
    def _title_mouse_release(self, event):
        """Handle mouse release on title bar"""
        if event.button() == Qt.LeftButton:
            self.dragging = False
            event.accept()
    
    def _on_font_changed(self, font):
        """Handle font change"""
        self.current_font.setFamily(font.family())
        self._apply_format()
    
    def _on_size_changed(self, size):
        """Handle size change"""
        self.current_font.setPointSize(size)
        self._apply_format()
        self._auto_resize()
    
    def _choose_color(self):
        """Open color picker"""
        color = QColorDialog.getColor(self.current_color, self, "Choose Text Color",
                                      QColorDialog.ShowAlphaChannel)
        if color.isValid():
            self.current_color = color
            self._update_color_button()
            self._apply_format()
    
    def _update_color_button(self):
        """Update color button appearance"""
        self.color_btn.setStyleSheet(f"""
            QPushButton {{
                background-color: rgba({self.current_color.red()}, 
                                      {self.current_color.green()}, 
                                      {self.current_color.blue()}, 
                                      {self.current_color.alpha()});
                color: {'white' if self.current_color.lightness() < 128 else 'black'};
                border: 2px solid #666;
                border-radius: 4px;
                font-weight: bold;
                padding: 4px;
            }}
        """)
    
    def _apply_format(self):
        """Apply current formatting to text editor"""
        # Apply to entire document
        cursor = self.text_edit.textCursor()
        cursor.select(QTextCursor.SelectionType.Document)
        
        fmt = QTextCharFormat()
        fmt.setFont(self.current_font)
        fmt.setForeground(self.current_color)
        
        cursor.mergeCharFormat(fmt)
        
        # Also set default font
        self.text_edit.setFont(self.current_font)
        self.text_edit.setTextColor(self.current_color)
    
    def _auto_resize(self):
        """Auto-resize text editor based on content"""
        doc = self.text_edit.document()
        text = self.text_edit.toPlainText()
        
        # Minimum and maximum sizes
        min_width = 200
        min_height = 80
        max_width = 800
        max_height = 600
        
        if not text.strip():
            new_width = min_width
            new_height = min_height
        else:
            # Calculate size
            padding = 20
            
            current_width = self.text_edit.width()
            wrap_width = min(current_width, max_width) - padding
            if wrap_width < min_width - padding:
                wrap_width = max_width - padding
            
            doc.setTextWidth(wrap_width)
            doc_size = doc.size()
            
            new_width = int(doc_size.width() + padding)
            new_height = int(doc_size.height() + padding)
            
            new_width = max(min_width, min(max_width, new_width))
            new_height = max(min_height, min(max_height, new_height))
            
            # Recalculate with final width
            doc.setTextWidth(new_width - padding)
            final_doc_size = doc.size()
            new_height = int(final_doc_size.height() + padding)
            new_height = max(min_height, min(max_height, new_height))
        
        self.text_edit.setFixedSize(new_width, new_height)
        self.adjustSize()
    
    def eventFilter(self, obj, event):
        """Filter events for keyboard shortcuts"""
        if obj == self.text_edit and event.type() == event.Type.KeyPress:
            if event.key() == Qt.Key_Escape:
                self._on_cancel()
                return True
            elif event.key() == Qt.Key_Return and event.modifiers() == Qt.ControlModifier:
                self._on_save()
                return True
        return super().eventFilter(obj, event)
    
    def _on_save(self):
        """Save and close"""
        text = self.text_edit.toPlainText().strip()
        if text:
            self.finished.emit(text, self.current_font, self.current_color)
        self.close()
        self.deleteLater()
    
    def _on_cancel(self):
        """Cancel and close"""
        self.cancelled.emit()
        self.close()
        self.deleteLater()
    
    def showEvent(self, event):
        """Handle show event"""
        super().showEvent(event)
        self.text_edit.setFocus()
        self._auto_resize()

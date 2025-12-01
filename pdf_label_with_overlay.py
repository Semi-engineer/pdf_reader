"""
PDF Label with Overlay - QLabel with search highlights and annotations
"""

import math
from PySide6.QtWidgets import QLabel, QInputDialog, QTextEdit
from PySide6.QtCore import Qt, QRectF, QPointF, Signal
from PySide6.QtGui import QPainter, QColor, QPen, QBrush, QPainterPath, QPolygonF, QFont


class PDFLabelWithOverlay(QLabel):
    """QLabel that can display search highlights and annotations with drawing support"""
    
    annotation_added = Signal(int, QRectF, QColor, str)  # page_num, rect, color, type
    annotation_removed = Signal(int, object)  # page_num, annotation
    
    def __init__(self, page_num, parent=None):
        super().__init__(parent)
        self.page_num = page_num
        self.search_results = []
        self.annotations = []
        self.zoom = 100
        self.doc_path = None  # Will be set by parent
        
        # Drawing state
        self.drawing_mode = None  # None, 'highlight', 'rectangle', 'pen'
        self.drawing_color = QColor(255, 255, 0, 100)
        self.is_drawing = False
        self.start_point = None
        self.current_point = None
        self.drawing_path = QPainterPath()
        self.temp_drawings = []  # Temporary drawings before saving
        self._temp_selection_rect = None  # Temporary selection for feedback
        self._temp_selection_color = None
        
        # Text editing
        self.text_editor = None
        self.text_editor_pos = None
        self.text_preview = None  # Preview text while editing
        
        self.setAlignment(Qt.AlignCenter)
        self.setStyleSheet("background-color: white; border: 1px solid #ccc;")
        self.setScaledContents(False)
        self.setMouseTracking(True)
    
    def set_search_results(self, results):
        """Set search results to highlight"""
        self.search_results = results
        self.update()
    
    def set_annotations(self, annotations):
        """Set annotations to display"""
        self.annotations = annotations
        self.update()
    
    def set_zoom(self, zoom):
        """Set zoom level for coordinate conversion"""
        self.zoom = zoom
    
    def set_drawing_mode(self, mode, color=None):
        """Set drawing mode: None, 'highlight', 'rectangle', 'pen', 'erase', 'select_text'"""
        self.drawing_mode = mode
        if color:
            self.drawing_color = color
        
        if mode == 'erase':
            self.setCursor(Qt.PointingHandCursor)
        elif mode == 'select_text':
            self.setCursor(Qt.IBeamCursor)
        elif mode:
            self.setCursor(Qt.CrossCursor)
        else:
            self.setCursor(Qt.ArrowCursor)
    
    def paintEvent(self, event):
        """Paint the label with overlays"""
        # First paint the base label (pixmap)
        super().paintEvent(event)
        
        # Then paint overlays
        if not self.pixmap() or self.pixmap().isNull():
            return
        
        painter = QPainter(self)
        painter.setRenderHint(QPainter.Antialiasing)
        
        # Draw search highlights
        if self.search_results:
            painter.setPen(Qt.NoPen)
            painter.setBrush(QBrush(QColor(255, 255, 0, 100)))  # Yellow highlight
            
            for result in self.search_results:
                rect = self._convert_pdf_rect_to_widget(result['rect'])
                painter.drawRect(rect)
        
        # Draw annotations
        for annotation in self.annotations:
            if annotation.annotation_type == 'highlight':
                painter.setPen(Qt.NoPen)
                painter.setBrush(QBrush(annotation.color))
                rect = self._convert_pdf_rect_to_widget(annotation.rect)
                painter.drawRect(rect)
            elif annotation.annotation_type == 'rectangle':
                pen = QPen(annotation.color, 2)
                painter.setPen(pen)
                painter.setBrush(Qt.NoBrush)
                rect = self._convert_pdf_rect_to_widget(annotation.rect)
                painter.drawRect(rect)
            elif annotation.annotation_type == 'line':
                pen = QPen(annotation.color, 2)
                painter.setPen(pen)
                rect = self._convert_pdf_rect_to_widget(annotation.rect)
                painter.drawLine(rect.topLeft(), rect.bottomRight())
            elif annotation.annotation_type == 'arrow':
                pen = QPen(annotation.color, 2)
                painter.setPen(pen)
                painter.setBrush(QBrush(annotation.color))
                rect = self._convert_pdf_rect_to_widget(annotation.rect)
                start = rect.topLeft()
                end = rect.bottomRight()
                painter.drawLine(start, end)
                self._draw_arrowhead(painter, start, end)
            elif annotation.annotation_type == 'circle':
                pen = QPen(annotation.color, 2)
                painter.setPen(pen)
                painter.setBrush(Qt.NoBrush)
                rect = self._convert_pdf_rect_to_widget(annotation.rect)
                painter.drawEllipse(rect)
            elif annotation.annotation_type.startswith('text:'):
                # Extract text from type field
                text = annotation.annotation_type[5:]  # Remove 'text:' prefix
                pen = QPen(annotation.color)
                painter.setPen(pen)
                from PySide6.QtGui import QFont
                font = QFont()
                font.setPointSize(12)
                painter.setFont(font)
                rect = self._convert_pdf_rect_to_widget(annotation.rect)
                painter.drawText(rect, Qt.AlignLeft | Qt.AlignTop, text)
            elif annotation.annotation_type == 'pen':
                pen = QPen(annotation.color, 3)
                pen.setCapStyle(Qt.RoundCap)
                pen.setJoinStyle(Qt.RoundJoin)
                painter.setPen(pen)
                painter.setBrush(Qt.NoBrush)
                if hasattr(annotation, 'path'):
                    # Scale path if needed
                    scale = self.zoom / 100.0
                    if hasattr(annotation, 'original_zoom'):
                        # Path was drawn at different zoom
                        scale_factor = scale / (annotation.original_zoom / 100.0)
                        scaled_path = QPainterPath()
                        for i in range(annotation.path.elementCount()):
                            element = annotation.path.elementAt(i)
                            if i == 0:
                                scaled_path.moveTo(element.x * scale_factor, element.y * scale_factor)
                            else:
                                scaled_path.lineTo(element.x * scale_factor, element.y * scale_factor)
                        painter.drawPath(scaled_path)
                    else:
                        painter.drawPath(annotation.path)
        
        # Draw current drawing
        if self.is_drawing and self.start_point and self.current_point:
            if self.drawing_mode == 'highlight':
                painter.setPen(Qt.NoPen)
                painter.setBrush(QBrush(self.drawing_color))
                rect = QRectF(self.start_point, self.current_point).normalized()
                painter.drawRect(rect)
            elif self.drawing_mode == 'rectangle':
                pen = QPen(self.drawing_color, 2)
                painter.setPen(pen)
                painter.setBrush(Qt.NoBrush)
                rect = QRectF(self.start_point, self.current_point).normalized()
                painter.drawRect(rect)
            elif self.drawing_mode == 'line':
                pen = QPen(self.drawing_color, 2)
                painter.setPen(pen)
                painter.drawLine(self.start_point, self.current_point)
            elif self.drawing_mode == 'arrow':
                pen = QPen(self.drawing_color, 2)
                painter.setPen(pen)
                painter.setBrush(QBrush(self.drawing_color))
                # Draw line
                painter.drawLine(self.start_point, self.current_point)
                # Draw arrowhead
                self._draw_arrowhead(painter, self.start_point, self.current_point)
            elif self.drawing_mode == 'circle':
                pen = QPen(self.drawing_color, 2)
                painter.setPen(pen)
                painter.setBrush(Qt.NoBrush)
                rect = QRectF(self.start_point, self.current_point).normalized()
                painter.drawEllipse(rect)
            elif self.drawing_mode == 'select_text':
                # Show selection rectangle
                pen = QPen(QColor(0, 120, 215), 2, Qt.DashLine)
                painter.setPen(pen)
                painter.setBrush(QBrush(QColor(0, 120, 215, 30)))
                rect = QRectF(self.start_point, self.current_point).normalized()
                painter.drawRect(rect)
            elif self.drawing_mode == 'pen':
                pen = QPen(self.drawing_color, 3)
                pen.setCapStyle(Qt.RoundCap)
                pen.setJoinStyle(Qt.RoundJoin)
                painter.setPen(pen)
                painter.drawPath(self.drawing_path)
        
        # Draw temporary selection feedback (after text copy)
        if self._temp_selection_rect:
            painter.setPen(QPen(QColor(0, 255, 0), 2))
            painter.setBrush(QBrush(self._temp_selection_color))
            painter.drawRect(self._temp_selection_rect)
        
        # Draw text preview while editing
        if self.text_preview and self.text_editor_pos:
            # Draw semi-transparent background box
            editor_rect = self.text_editor.geometry() if self.text_editor else QRectF(self.text_editor_pos.x(), self.text_editor_pos.y(), 250, 120)
            preview_rect = QRectF(editor_rect)
            
            # Draw background - more transparent
            painter.setPen(Qt.NoPen)
            painter.setBrush(QBrush(QColor(255, 255, 255, 80)))
            painter.drawRect(preview_rect)
            
            # Draw border
            painter.setPen(QPen(self.drawing_color, 2))
            painter.setBrush(Qt.NoBrush)
            painter.drawRect(preview_rect)
            
            # Draw text preview
            painter.setPen(QPen(self.drawing_color))
            font = QFont()
            font.setPointSize(12)
            painter.setFont(font)
            text_rect = preview_rect.adjusted(8, 8, -8, -8)
            painter.drawText(text_rect, Qt.AlignLeft | Qt.AlignTop | Qt.TextWordWrap, self.text_preview)
        
        painter.end()
    
    def mousePressEvent(self, event):
        """Handle mouse press for drawing or erasing"""
        if event.button() == Qt.LeftButton and self.drawing_mode:
            if self.drawing_mode == 'erase':
                # Erase mode - find and remove annotation at click position
                click_pos = self._get_pixmap_position(event.position())
                self._erase_annotation_at(click_pos)
            elif self.drawing_mode == 'text':
                # Text mode - create text editor overlay
                click_pos = self._get_pixmap_position(event.position())
                self._create_text_editor(click_pos)
            else:
                # Drawing mode
                self.is_drawing = True
                # Get position relative to pixmap
                self.start_point = self._get_pixmap_position(event.position())
                self.current_point = self.start_point
                
                if self.drawing_mode == 'pen':
                    self.drawing_path = QPainterPath()
                    self.drawing_path.moveTo(self.start_point)
                
                self.update()
    
    def mouseMoveEvent(self, event):
        """Handle mouse move for drawing"""
        if self.is_drawing and self.drawing_mode:
            # Get position relative to pixmap
            self.current_point = self._get_pixmap_position(event.position())
            
            if self.drawing_mode == 'pen':
                self.drawing_path.lineTo(self.current_point)
            
            self.update()
    
    def _create_text_editor(self, pos):
        """Create text editor overlay at position"""
        # Remove existing text editor if any
        if self.text_editor:
            self._finish_text_editing()
        
        # Create text editor
        self.text_editor = QTextEdit(self)
        self.text_editor.setGeometry(int(pos.x()), int(pos.y()), 250, 120)
        self.text_editor_pos = pos
        
        # Style the text editor with better visibility and transparency
        self.text_editor.setStyleSheet("""
            QTextEdit {
                background-color: rgba(255, 255, 255, 150);
                border: 3px solid #2196F3;
                border-radius: 6px;
                padding: 8px;
                font-size: 12pt;
                color: #000000;
            }
        """)
        
        # Set font to match preview
        font = QFont()
        font.setPointSize(12)
        self.text_editor.setFont(font)
        
        # Show and focus
        self.text_editor.show()
        self.text_editor.setFocus()
        
        # Connect signals for real-time preview
        self.text_editor.textChanged.connect(self._update_text_preview)
        self.text_editor.installEventFilter(self)
    
    def eventFilter(self, obj, event):
        """Filter events for text editor"""
        if obj == self.text_editor:
            # Handle Escape key to cancel
            if event.type() == event.Type.KeyPress:
                if event.key() == Qt.Key_Escape:
                    self._cancel_text_editing()
                    return True
                elif event.key() == Qt.Key_Return and event.modifiers() == Qt.ControlModifier:
                    # Ctrl+Enter to finish
                    self._finish_text_editing()
                    return True
            # Handle focus out to finish editing
            elif event.type() == event.Type.FocusOut:
                # Delay to allow clicking buttons
                from PySide6.QtCore import QTimer
                QTimer.singleShot(100, self._finish_text_editing)
        
        return super().eventFilter(obj, event)
    
    def _finish_text_editing(self):
        """Finish text editing and save annotation"""
        if not self.text_editor:
            return
        
        text = self.text_editor.toPlainText().strip()
        
        if text and self.text_editor_pos:
            # Get text editor size
            editor_rect = self.text_editor.geometry()
            widget_rect = QRectF(
                self.text_editor_pos.x(),
                self.text_editor_pos.y(),
                editor_rect.width(),
                editor_rect.height()
            )
            pdf_rect = self._convert_widget_rect_to_pdf(widget_rect)
            
            # Store text in annotation
            self.annotation_added.emit(
                self.page_num,
                pdf_rect,
                self.drawing_color,
                f'text:{text}'
            )
        
        # Remove text editor and preview
        self.text_editor.deleteLater()
        self.text_editor = None
        self.text_editor_pos = None
        self.text_preview = None
        self.update()
    
    def _update_text_preview(self):
        """Update text preview in real-time"""
        if self.text_editor:
            self.text_preview = self.text_editor.toPlainText()
            self.update()  # Trigger repaint to show preview
    
    def _cancel_text_editing(self):
        """Cancel text editing without saving"""
        if self.text_editor:
            self.text_editor.deleteLater()
            self.text_editor = None
            self.text_editor_pos = None
            self.text_preview = None
            self.update()
    
    def mouseReleaseEvent(self, event):
        """Handle mouse release to finish drawing"""
        if event.button() == Qt.LeftButton:
            
            if self.is_drawing:
                self.is_drawing = False
                
                if self.start_point and self.current_point:
                    # Create annotation
                    if self.drawing_mode in ['highlight', 'rectangle', 'line', 'arrow', 'circle']:
                        # Widget coordinates
                        widget_rect = QRectF(self.start_point, self.current_point).normalized()
                        
                        if widget_rect.width() > 5 or widget_rect.height() > 5:
                            # Convert to PDF coordinates for storage
                            pdf_rect = self._convert_widget_rect_to_pdf(widget_rect)
                            
                            self.annotation_added.emit(
                                self.page_num,
                                pdf_rect,  # Store as PDF coordinates
                                self.drawing_color,
                                self.drawing_mode
                            )
                    elif self.drawing_mode == 'select_text':
                        # Select text mode
                        widget_rect = QRectF(self.start_point, self.current_point).normalized()
                        
                        if widget_rect.width() > 5 and widget_rect.height() > 5:
                            self._extract_text_from_rect(widget_rect)
                    elif self.drawing_mode == 'pen':
                        # For pen, save the path
                        if not self.drawing_path.isEmpty():
                            # Create a custom annotation with path
                            from annotation_manager import Annotation
                            annotation = Annotation(
                                self.page_num,
                                self.drawing_path.boundingRect(),
                                self.drawing_color,
                                'pen'
                            )
                            annotation.path = self.drawing_path
                            annotation.original_zoom = self.zoom  # Store zoom level
                            
                            # Add directly to widget's annotations
                            self.annotations.append(annotation)
                            self.update()
            
            self.start_point = None
            self.current_point = None
            self.drawing_path = QPainterPath()
            self.update()
    
    def _convert_pdf_rect_to_widget(self, pdf_rect):
        """Convert PDF coordinate rect to widget coordinates"""
        scale = self.zoom / 100.0
        
        if hasattr(pdf_rect, 'x0'):
            # fitz.Rect - convert from PDF to widget
            return QRectF(
                pdf_rect.x0 * scale,
                pdf_rect.y0 * scale,
                (pdf_rect.x1 - pdf_rect.x0) * scale,
                (pdf_rect.y1 - pdf_rect.y0) * scale
            )
        else:
            # QRectF in PDF coordinates - scale to widget
            return QRectF(
                pdf_rect.x() * scale,
                pdf_rect.y() * scale,
                pdf_rect.width() * scale,
                pdf_rect.height() * scale
            )
    
    def _get_pixmap_position(self, widget_pos):
        """Get position relative to pixmap (accounting for alignment)"""
        if not self.pixmap() or self.pixmap().isNull():
            return widget_pos
        
        # Widget has fixed size = pixmap size, so no offset needed
        # Just return the position as-is
        return widget_pos
    
    def _extract_text_from_rect(self, widget_rect):
        """Extract text from the selected rectangle"""
        try:
            import fitz
            from PySide6.QtWidgets import QApplication
            
            if not self.doc_path:
                print("Document path not set")
                return
            
            # Open document
            doc = fitz.open(self.doc_path)
            page = doc[self.page_num]
            
            # Convert widget rect to PDF coordinates
            pdf_rect = self._convert_widget_rect_to_pdf(widget_rect)
            
            # Create fitz.Rect
            fitz_rect = fitz.Rect(
                pdf_rect.x(),
                pdf_rect.y(),
                pdf_rect.x() + pdf_rect.width(),
                pdf_rect.y() + pdf_rect.height()
            )
            
            # Extract text
            text = page.get_textbox(fitz_rect)
            
            doc.close()
            
            if text.strip():
                # Copy to clipboard
                clipboard = QApplication.clipboard()
                clipboard.setText(text)
                
                # Show notification (emit signal to parent)
                print(f"Copied {len(text)} characters: {text[:50]}...")
                
                # Show temporary highlight
                self._show_text_copied_feedback(widget_rect)
            else:
                print("No text found in selection")
                
        except Exception as e:
            print(f"Error extracting text: {e}")
    
    def _show_text_copied_feedback(self, widget_rect):
        """Show temporary feedback for copied text"""
        # Store the selection rect temporarily
        self._temp_selection_rect = widget_rect
        self._temp_selection_color = QColor(0, 255, 0, 50)  # Green
        self.update()
        
        # Clear after 1 second
        from PySide6.QtCore import QTimer
        QTimer.singleShot(1000, self._clear_temp_selection)
    
    def _clear_temp_selection(self):
        """Clear temporary selection highlight"""
        self._temp_selection_rect = None
        self.update()
    
    def _erase_annotation_at(self, pos):
        """Erase annotation at the given position"""
        # Find annotation that contains this point
        for i, annotation in enumerate(self.annotations):
            # Get annotation type (handle text: prefix)
            ann_type = annotation.annotation_type
            if ann_type.startswith('text:'):
                ann_type = 'text'
            
            if ann_type in ['highlight', 'rectangle', 'line', 'arrow', 'circle', 'text']:
                # Check if point is inside rectangle or near line
                rect = self._convert_pdf_rect_to_widget(annotation.rect)
                
                # Expand rect slightly for easier clicking on lines
                if ann_type in ['line', 'arrow']:
                    rect = rect.adjusted(-5, -5, 5, 5)
                
                if rect.contains(pos):
                    # Remove this annotation
                    removed = self.annotations.pop(i)
                    self.update()
                    
                    # Emit signal to remove from manager
                    self.annotation_removed.emit(self.page_num, removed)
                    print(f"Erased {ann_type} annotation at page {self.page_num}")
                    return
            elif annotation.annotation_type == 'pen':
                # Check if point is near the path
                if hasattr(annotation, 'path'):
                    # Simple check: if point is in bounding rect
                    rect = annotation.path.boundingRect()
                    if hasattr(annotation, 'original_zoom'):
                        scale = self.zoom / 100.0
                        scale_factor = scale / (annotation.original_zoom / 100.0)
                        rect = QRectF(
                            rect.x() * scale_factor,
                            rect.y() * scale_factor,
                            rect.width() * scale_factor,
                            rect.height() * scale_factor
                        )
                    
                    if rect.contains(pos):
                        removed = self.annotations.pop(i)
                        self.update()
                        
                        # Emit signal to remove from manager
                        self.annotation_removed.emit(self.page_num, removed)
                        print(f"Erased pen annotation at page {self.page_num}")
                        return
    
    def _convert_widget_rect_to_pdf(self, widget_rect):
        """Convert widget coordinate rect to PDF coordinates (as QRectF with PDF scale)"""
        scale = self.zoom / 100.0
        
        # Return QRectF with PDF coordinates
        return QRectF(
            widget_rect.x() / scale,
            widget_rect.y() / scale,
            widget_rect.width() / scale,
            widget_rect.height() / scale
        )
    
    def _draw_arrowhead(self, painter, start, end):
        """Draw arrowhead at the end of a line"""
        # Calculate angle
        dx = end.x() - start.x()
        dy = end.y() - start.y()
        angle = math.atan2(dy, dx)
        
        # Arrowhead size
        arrow_size = 15
        arrow_angle = math.pi / 6  # 30 degrees
        
        # Calculate arrowhead points
        point1 = QPointF(
            end.x() - arrow_size * math.cos(angle - arrow_angle),
            end.y() - arrow_size * math.sin(angle - arrow_angle)
        )
        point2 = QPointF(
            end.x() - arrow_size * math.cos(angle + arrow_angle),
            end.y() - arrow_size * math.sin(angle + arrow_angle)
        )
        
        # Draw arrowhead
        arrow_head = QPolygonF([end, point1, point2])
        painter.drawPolygon(arrow_head)

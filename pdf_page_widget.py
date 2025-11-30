"""
PDF Page Widget - Custom widget for displaying PDF pages with overlays
"""

from PySide6.QtWidgets import QWidget
from PySide6.QtCore import Qt, Signal, QRectF, QPointF
from PySide6.QtGui import QPainter, QPixmap, QColor, QPen, QBrush, QCursor
import fitz


class PDFPageWidget(QWidget):
    """Widget for displaying a single PDF page with overlays"""
    
    text_selected = Signal(str)  # Emitted when text is selected
    annotation_added = Signal(int, QRectF, QColor, str)  # page, rect, color, type
    
    def __init__(self, page_num, parent=None):
        super().__init__(parent)
        self.page_num = page_num
        self.pixmap = None
        self.search_results = []
        self.annotations = []
        self.selection_rects = []
        
        # Selection state
        self.selecting = False
        self.selection_start = None
        self.selection_end = None
        self.selection_rect = None
        
        # Annotation mode
        self.annotation_mode = None  # None, 'highlight', 'rectangle'
        self.annotation_color = QColor(255, 255, 0, 100)
        
        # Text extraction data
        self.text_blocks = []
        self.doc_path = None
        self.zoom = 100
        self.rotation = 0
        
        self.setMouseTracking(True)
        self.setAutoFillBackground(True)
        
        # Set white background
        from PySide6.QtGui import QPalette
        palette = self.palette()
        palette.setColor(QPalette.Window, QColor(255, 255, 255))
        self.setPalette(palette)
    
    def set_pixmap(self, pixmap):
        """Set the page pixmap"""
        self.pixmap = pixmap
        if pixmap:
            self.setFixedSize(pixmap.size())
        self.update()
    
    def set_search_results(self, results):
        """Set search result rectangles to highlight"""
        self.search_results = results
        self.update()
    
    def set_annotations(self, annotations):
        """Set annotations to display"""
        self.annotations = annotations
        self.update()
    
    def set_annotation_mode(self, mode, color=None):
        """Set annotation mode"""
        self.annotation_mode = mode
        if color:
            self.annotation_color = color
        
        if mode:
            self.setCursor(QCursor(Qt.CrossCursor))
        else:
            self.setCursor(QCursor(Qt.ArrowCursor))
    
    def set_text_blocks(self, doc_path, zoom, rotation):
        """Extract text blocks for selection"""
        self.doc_path = doc_path
        self.zoom = zoom
        self.rotation = rotation
        
        try:
            doc = fitz.open(doc_path)
            page = doc[self.page_num]
            
            # Get text with positions
            mat = fitz.Matrix(zoom / 100.0, zoom / 100.0)
            if rotation:
                mat = mat.prerotate(rotation)
            
            self.text_blocks = page.get_text("dict", flags=fitz.TEXT_PRESERVE_WHITESPACE)
            doc.close()
        except:
            self.text_blocks = []
    
    def paintEvent(self, event):
        """Paint the page with overlays"""
        painter = QPainter(self)
        painter.setRenderHint(QPainter.Antialiasing)
        
        # Fill background with white
        painter.fillRect(self.rect(), QColor(255, 255, 255))
        
        # Draw page pixmap
        if self.pixmap and not self.pixmap.isNull():
            painter.drawPixmap(0, 0, self.pixmap)
        
        # Draw search highlights
        if self.search_results:
            painter.setPen(Qt.NoPen)
            painter.setBrush(QBrush(QColor(255, 255, 0, 100)))
            for result in self.search_results:
                rect = self._convert_pdf_rect_to_widget(result['rect'])
                painter.drawRect(rect)
        
        # Draw annotations
        for annotation in self.annotations:
            if annotation.annotation_type == 'highlight':
                painter.setPen(Qt.NoPen)
                painter.setBrush(QBrush(annotation.color))
            else:  # rectangle
                pen = QPen(annotation.color, 2)
                painter.setPen(pen)
                painter.setBrush(Qt.NoBrush)
            
            rect = self._convert_pdf_rect_to_widget(annotation.rect)
            painter.drawRect(rect)
        
        # Draw current selection
        if self.selection_rect:
            painter.setPen(QPen(QColor(0, 120, 215), 2))
            painter.setBrush(QBrush(QColor(0, 120, 215, 50)))
            painter.drawRect(self.selection_rect)
        
        # Draw selection rectangles
        if self.selection_rects:
            painter.setPen(Qt.NoPen)
            painter.setBrush(QBrush(QColor(0, 120, 215, 100)))
            for rect in self.selection_rects:
                painter.drawRect(rect)
    
    def _convert_pdf_rect_to_widget(self, pdf_rect):
        """Convert PDF coordinate rect to widget coordinates"""
        if hasattr(pdf_rect, 'x0'):
            # fitz.Rect
            scale = self.zoom / 100.0
            return QRectF(
                pdf_rect.x0 * scale,
                pdf_rect.y0 * scale,
                (pdf_rect.x1 - pdf_rect.x0) * scale,
                (pdf_rect.y1 - pdf_rect.y0) * scale
            )
        else:
            # Already QRectF
            return pdf_rect
    
    def mousePressEvent(self, event):
        """Handle mouse press"""
        if event.button() == Qt.LeftButton:
            self.selecting = True
            self.selection_start = event.pos()
            self.selection_end = event.pos()
            self.selection_rect = None
            self.selection_rects = []
    
    def mouseMoveEvent(self, event):
        """Handle mouse move"""
        if self.selecting:
            self.selection_end = event.pos()
            
            # Update selection rectangle
            x = min(self.selection_start.x(), self.selection_end.x())
            y = min(self.selection_start.y(), self.selection_end.y())
            w = abs(self.selection_end.x() - self.selection_start.x())
            h = abs(self.selection_end.y() - self.selection_start.y())
            
            self.selection_rect = QRectF(x, y, w, h)
            self.update()
    
    def mouseReleaseEvent(self, event):
        """Handle mouse release"""
        if event.button() == Qt.LeftButton and self.selecting:
            self.selecting = False
            
            if self.selection_rect and self.selection_rect.width() > 5 and self.selection_rect.height() > 5:
                if self.annotation_mode:
                    # Add annotation
                    self._add_annotation()
                else:
                    # Select text
                    self._select_text()
            
            self.selection_rect = None
            self.update()
    
    def _add_annotation(self):
        """Add annotation from selection"""
        if self.selection_rect:
            # Convert to PDF coordinates
            scale = self.zoom / 100.0
            pdf_rect = fitz.Rect(
                self.selection_rect.x() / scale,
                self.selection_rect.y() / scale,
                (self.selection_rect.x() + self.selection_rect.width()) / scale,
                (self.selection_rect.y() + self.selection_rect.height()) / scale
            )
            
            self.annotation_added.emit(
                self.page_num,
                self.selection_rect,
                self.annotation_color,
                self.annotation_mode
            )
    
    def _select_text(self):
        """Select text in the selection rectangle"""
        if not self.doc_path or not self.selection_rect:
            return
        
        try:
            doc = fitz.open(self.doc_path)
            page = doc[self.page_num]
            
            # Convert selection to PDF coordinates
            scale = self.zoom / 100.0
            pdf_rect = fitz.Rect(
                self.selection_rect.x() / scale,
                self.selection_rect.y() / scale,
                (self.selection_rect.x() + self.selection_rect.width()) / scale,
                (self.selection_rect.y() + self.selection_rect.height()) / scale
            )
            
            # Get text in rectangle
            text = page.get_textbox(pdf_rect)
            
            if text.strip():
                self.text_selected.emit(text)
                
                # Get word rectangles for visual feedback
                words = page.get_text("words")
                self.selection_rects = []
                for word in words:
                    word_rect = fitz.Rect(word[:4])
                    if pdf_rect.intersects(word_rect):
                        widget_rect = self._convert_pdf_rect_to_widget(word_rect)
                        self.selection_rects.append(widget_rect)
                
                self.update()
            
            doc.close()
            
        except Exception as e:
            print(f"Text selection error: {e}")

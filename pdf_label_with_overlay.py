"""
PDF Label with Overlay - QLabel with search highlights and annotations
"""

from PySide6.QtWidgets import QLabel
from PySide6.QtCore import Qt, QRectF
from PySide6.QtGui import QPainter, QColor, QPen, QBrush


class PDFLabelWithOverlay(QLabel):
    """QLabel that can display search highlights and annotations"""
    
    def __init__(self, page_num, parent=None):
        super().__init__(parent)
        self.page_num = page_num
        self.search_results = []
        self.annotations = []
        self.zoom = 100
        
        self.setAlignment(Qt.AlignCenter)
        self.setStyleSheet("background-color: white; border: 1px solid #ccc;")
        self.setScaledContents(False)
    
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
            else:  # rectangle
                pen = QPen(annotation.color, 2)
                painter.setPen(pen)
                painter.setBrush(Qt.NoBrush)
            
            rect = self._convert_pdf_rect_to_widget(annotation.rect)
            painter.drawRect(rect)
        
        painter.end()
    
    def _convert_pdf_rect_to_widget(self, pdf_rect):
        """Convert PDF coordinate rect to widget coordinates"""
        scale = self.zoom / 100.0
        
        if hasattr(pdf_rect, 'x0'):
            # fitz.Rect
            return QRectF(
                pdf_rect.x0 * scale,
                pdf_rect.y0 * scale,
                (pdf_rect.x1 - pdf_rect.x0) * scale,
                (pdf_rect.y1 - pdf_rect.y0) * scale
            )
        else:
            # Already QRectF
            return pdf_rect

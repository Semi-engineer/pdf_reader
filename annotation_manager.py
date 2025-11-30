"""
Annotation Manager - Handles PDF annotations (highlights, rectangles)
"""

from PySide6.QtCore import QObject, Signal, QRectF
from PySide6.QtGui import QColor
import fitz


class Annotation:
    """Represents a single annotation"""
    
    def __init__(self, page_num, rect, color, annotation_type='highlight'):
        self.page_num = page_num
        self.rect = rect  # QRectF or fitz.Rect
        self.color = color  # QColor
        self.annotation_type = annotation_type  # 'highlight' or 'rectangle'
    
    def to_dict(self):
        """Convert to dictionary for serialization"""
        return {
            'page': self.page_num,
            'rect': [self.rect.x0, self.rect.y0, self.rect.x1, self.rect.y1] if hasattr(self.rect, 'x0') else [self.rect.x(), self.rect.y(), self.rect.width(), self.rect.height()],
            'color': [self.color.red(), self.color.green(), self.color.blue(), self.color.alpha()],
            'type': self.annotation_type
        }


class AnnotationManager(QObject):
    """Manages PDF annotations"""
    
    annotations_changed = Signal()
    
    def __init__(self):
        super().__init__()
        self.annotations = []  # List of Annotation objects
    
    def add_annotation(self, page_num, rect, color, annotation_type='highlight'):
        """Add a new annotation"""
        annotation = Annotation(page_num, rect, color, annotation_type)
        self.annotations.append(annotation)
        self.annotations_changed.emit()
    
    def get_annotations_for_page(self, page_num):
        """Get all annotations for a specific page"""
        return [a for a in self.annotations if a.page_num == page_num]
    
    def clear_annotations(self):
        """Clear all annotations"""
        self.annotations.clear()
        self.annotations_changed.emit()
    
    def save_to_pdf(self, input_path, output_path):
        """Save annotations to a new PDF file"""
        try:
            doc = fitz.open(input_path)
            
            for annotation in self.annotations:
                page = doc[annotation.page_num]
                
                # Convert QRectF to fitz.Rect if needed
                if isinstance(annotation.rect, QRectF):
                    rect = fitz.Rect(
                        annotation.rect.x(),
                        annotation.rect.y(),
                        annotation.rect.x() + annotation.rect.width(),
                        annotation.rect.y() + annotation.rect.height()
                    )
                else:
                    rect = annotation.rect
                
                # Convert QColor to RGB tuple
                color = (
                    annotation.color.red() / 255.0,
                    annotation.color.green() / 255.0,
                    annotation.color.blue() / 255.0
                )
                
                if annotation.annotation_type == 'highlight':
                    # Add highlight annotation
                    highlight = page.add_highlight_annot(rect)
                    highlight.set_colors(stroke=color)
                    highlight.update()
                elif annotation.annotation_type == 'rectangle':
                    # Add rectangle annotation
                    annot = page.add_rect_annot(rect)
                    annot.set_colors(stroke=color)
                    annot.set_border(width=2)
                    annot.update()
            
            # Save to new file
            doc.save(output_path, garbage=4, deflate=True)
            doc.close()
            return True
            
        except Exception as e:
            print(f"Error saving annotations: {e}")
            return False
    
    def remove_annotation(self, annotation):
        """Remove a specific annotation"""
        if annotation in self.annotations:
            self.annotations.remove(annotation)
            self.annotations_changed.emit()

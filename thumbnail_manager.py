"""
Thumbnail Manager - Generates and manages page thumbnails
"""

from PySide6.QtCore import QObject, Signal, QRunnable, QThreadPool
from PySide6.QtGui import QPixmap, QImage
import fitz
from pathlib import Path
import hashlib


class ThumbnailTask(QRunnable):
    """Task for generating a single thumbnail"""
    
    def __init__(self, doc_path, page_num, callback):
        super().__init__()
        self.doc_path = doc_path
        self.page_num = page_num
        self.callback = callback
        self.cancelled = False
    
    def cancel(self):
        self.cancelled = True
    
    def run(self):
        if self.cancelled:
            return
        
        try:
            doc = fitz.open(self.doc_path)
            page = doc[self.page_num]
            
            # Render at low resolution for thumbnail
            mat = fitz.Matrix(0.2, 0.2)  # 20% scale
            pix = page.get_pixmap(matrix=mat, alpha=False)
            
            if self.cancelled:
                doc.close()
                return
            
            # Convert to QPixmap
            img_format = QImage.Format_RGB888 if pix.n == 3 else QImage.Format_RGBA8888
            qimg = QImage(pix.samples, pix.width, pix.height, pix.stride, img_format)
            pixmap = QPixmap.fromImage(qimg.copy())
            
            doc.close()
            
            if not self.cancelled:
                self.callback(self.page_num, pixmap)
                
        except Exception as e:
            pass


class ThumbnailManager(QObject):
    """Manages thumbnail generation"""
    
    thumbnail_ready = Signal(int, QPixmap)
    
    def __init__(self):
        super().__init__()
        self.thread_pool = QThreadPool()
        self.thread_pool.setMaxThreadCount(2)
        self.active_tasks = {}
        self.cache = {}
    
    def generate_thumbnail(self, doc_path, page_num):
        """Generate thumbnail for a page"""
        # Check cache first
        if page_num in self.cache:
            self.thumbnail_ready.emit(page_num, self.cache[page_num])
            return
        
        # Cancel existing task
        if page_num in self.active_tasks:
            self.active_tasks[page_num].cancel()
        
        # Create new task
        task = ThumbnailTask(doc_path, page_num, self._on_thumbnail_ready)
        self.active_tasks[page_num] = task
        self.thread_pool.start(task)
    
    def _on_thumbnail_ready(self, page_num, pixmap):
        """Handle thumbnail completion"""
        if page_num in self.active_tasks:
            del self.active_tasks[page_num]
        self.cache[page_num] = pixmap
        self.thumbnail_ready.emit(page_num, pixmap)
    
    def clear(self):
        """Clear all thumbnails"""
        for task in self.active_tasks.values():
            task.cancel()
        self.active_tasks.clear()
        self.cache.clear()

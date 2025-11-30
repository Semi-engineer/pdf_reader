"""
Render Worker - Background thread rendering for PDF pages
"""

from PySide6.QtCore import QObject, Signal, QRunnable, QThreadPool
from PySide6.QtGui import QPixmap, QImage
import fitz


class RenderTask(QRunnable):
    """Runnable task for rendering a single page"""
    
    def __init__(self, doc_path, page_num, zoom, rotation, callback, error_callback):
        super().__init__()
        self.doc_path = doc_path
        self.page_num = page_num
        self.zoom = zoom
        self.rotation = rotation
        self.callback = callback
        self.error_callback = error_callback
        self.cancelled = False
    
    def cancel(self):
        """Cancel this render task"""
        self.cancelled = True
    
    def run(self):
        """Execute rendering"""
        if self.cancelled:
            return
        
        try:
            doc = fitz.open(self.doc_path)
            page = doc[self.page_num]
            
            # Apply rotation
            mat = fitz.Matrix(self.zoom / 100.0, self.zoom / 100.0)
            if self.rotation:
                mat = mat.prerotate(self.rotation)
            
            # Render to pixmap
            pix = page.get_pixmap(matrix=mat, alpha=False)
            
            if self.cancelled:
                doc.close()
                return
            
            # Convert to QImage
            img_format = QImage.Format_RGB888 if pix.n == 3 else QImage.Format_RGBA8888
            qimg = QImage(pix.samples, pix.width, pix.height, pix.stride, img_format)
            
            # Convert to QPixmap
            pixmap = QPixmap.fromImage(qimg.copy())
            
            doc.close()
            
            if not self.cancelled:
                self.callback(self.page_num, pixmap)
                
        except Exception as e:
            if not self.cancelled:
                self.error_callback(self.page_num, str(e))


class RenderWorker(QObject):
    """Manages background rendering tasks"""
    
    page_rendered = Signal(int, QPixmap)
    render_error = Signal(int, str)
    
    def __init__(self):
        super().__init__()
        self.thread_pool = QThreadPool()
        self.thread_pool.setMaxThreadCount(4)
        self.active_tasks = {}
    
    def render_page(self, doc_path, page_num, zoom, rotation=0):
        """Queue a page for rendering"""
        # Cancel existing task for this page
        if page_num in self.active_tasks:
            self.active_tasks[page_num].cancel()
        
        # Create new task
        task = RenderTask(
            doc_path, page_num, zoom, rotation,
            self._on_page_rendered,
            self._on_render_error
        )
        self.active_tasks[page_num] = task
        self.thread_pool.start(task)
    
    def _on_page_rendered(self, page_num, pixmap):
        """Handle successful render"""
        if page_num in self.active_tasks:
            del self.active_tasks[page_num]
        self.page_rendered.emit(page_num, pixmap)
    
    def _on_render_error(self, page_num, error):
        """Handle render error"""
        if page_num in self.active_tasks:
            del self.active_tasks[page_num]
        self.render_error.emit(page_num, error)
    
    def cancel_all(self):
        """Cancel all pending tasks"""
        for task in self.active_tasks.values():
            task.cancel()
        self.active_tasks.clear()

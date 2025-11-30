"""
Search Manager - Text search functionality with highlighting
"""

from PySide6.QtCore import QObject, Signal, QRunnable, QThreadPool, QRectF
import fitz


class SearchTask(QRunnable):
    """Task for searching text in PDF"""
    
    def __init__(self, doc_path, query, callback):
        super().__init__()
        self.doc_path = doc_path
        self.query = query
        self.callback = callback
        self.cancelled = False
    
    def cancel(self):
        self.cancelled = True
    
    def run(self):
        if self.cancelled or not self.query:
            return
        
        try:
            doc = fitz.open(self.doc_path)
            results = []
            
            for page_num in range(len(doc)):
                if self.cancelled:
                    break
                
                page = doc[page_num]
                # Search for text instances
                text_instances = page.search_for(self.query)
                
                if text_instances:
                    for rect in text_instances:
                        results.append({
                            'page': page_num,
                            'rect': rect,
                            'text': self.query
                        })
            
            doc.close()
            
            if not self.cancelled:
                self.callback(results)
                
        except Exception as e:
            pass


class SearchManager(QObject):
    """Manages text search operations"""
    
    search_completed = Signal(list)  # List of search results
    
    def __init__(self):
        super().__init__()
        self.thread_pool = QThreadPool()
        self.current_task = None
        self.results = []
        self.current_index = -1
    
    def search(self, doc_path, query):
        """Start a new search"""
        # Cancel existing search
        if self.current_task:
            self.current_task.cancel()
        
        self.results = []
        self.current_index = -1
        
        if not query:
            self.search_completed.emit([])
            return
        
        # Create new search task
        self.current_task = SearchTask(doc_path, query, self._on_search_completed)
        self.thread_pool.start(self.current_task)
    
    def _on_search_completed(self, results):
        """Handle search completion"""
        self.results = results
        self.current_index = 0 if results else -1
        self.search_completed.emit(results)
    
    def next_result(self):
        """Move to next search result"""
        if self.results and self.current_index < len(self.results) - 1:
            self.current_index += 1
            return self.results[self.current_index]
        return None
    
    def prev_result(self):
        """Move to previous search result"""
        if self.results and self.current_index > 0:
            self.current_index -= 1
            return self.results[self.current_index]
        return None
    
    def current_result(self):
        """Get current search result"""
        if self.results and 0 <= self.current_index < len(self.results):
            return self.results[self.current_index]
        return None
    
    def get_results_for_page(self, page_num):
        """Get all search results for a specific page"""
        return [r for r in self.results if r['page'] == page_num]

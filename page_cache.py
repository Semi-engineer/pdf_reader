"""
Page Cache Manager - LRU cache for rendered pages
"""

from collections import OrderedDict
from PySide6.QtGui import QPixmap
from threading import Lock


class PageCache:
    """LRU cache for rendered page pixmaps"""
    
    def __init__(self, max_size=20):
        self.max_size = max_size
        self.cache = OrderedDict()
        self.lock = Lock()
    
    def get(self, key):
        """Get cached pixmap, returns None if not found"""
        with self.lock:
            if key in self.cache:
                # Move to end (most recently used)
                self.cache.move_to_end(key)
                return self.cache[key]
            return None
    
    def put(self, key, pixmap):
        """Add pixmap to cache"""
        with self.lock:
            if key in self.cache:
                self.cache.move_to_end(key)
            else:
                self.cache[key] = pixmap
                if len(self.cache) > self.max_size:
                    # Remove oldest
                    self.cache.popitem(last=False)
    
    def clear(self):
        """Clear all cache"""
        with self.lock:
            self.cache.clear()
    
    def remove(self, key):
        """Remove specific key from cache"""
        with self.lock:
            if key in self.cache:
                del self.cache[key]

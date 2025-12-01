"""
Icon Manager - SVG icon management for modern UI
"""

from PySide6.QtGui import QIcon, QPixmap, QPainter, QColor
from PySide6.QtSvg import QSvgRenderer
from PySide6.QtCore import QByteArray, QSize


class IconManager:
    """Manages SVG icons for the application"""
    
    # SVG icon definitions
    ICONS = {
        # File operations
        'open': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M3 8L3 18C3 19.1046 3.89543 20 5 20H19C20.1046 20 21 19.1046 21 18V10C21 8.89543 20.1046 8 19 8H13L11 6H5C3.89543 6 3 6.89543 3 8Z" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'save': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M19 21H5C3.89543 21 3 20.1046 3 19V5C3 3.89543 3.89543 3 5 3H16L21 8V19C21 20.1046 20.1046 21 19 21Z" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M7 3V8H15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M7 21V13H17V21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'print': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M6 9V2H18V9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M6 18H4C2.89543 18 2 17.1046 2 16V11C2 9.89543 2.89543 9 4 9H20C21.1046 9 22 9.89543 22 11V16C22 17.1046 21.1046 18 20 18H18" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <rect x="6" y="14" width="12" height="8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        # Navigation
        'previous': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'next': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M9 18L15 12L9 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        # Zoom
        'zoom_in': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="11" cy="11" r="8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M21 21L16.65 16.65" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M11 8V14M8 11H14" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'zoom_out': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="11" cy="11" r="8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M21 21L16.65 16.65" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M8 11H14" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'fit_width': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M8 3H5C3.89543 3 3 3.89543 3 5V8M16 3H19C20.1046 3 21 3.89543 21 5V8M8 21H5C3.89543 21 3 20.1046 3 19V16M16 21H19C20.1046 21 21 20.1046 21 19V16" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M9 12H15M9 12L11 10M9 12L11 14M15 12L13 10M15 12L13 14" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'fit_page': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M8 3H5C3.89543 3 3 3.89543 3 5V8M16 3H19C20.1046 3 21 3.89543 21 5V8M8 21H5C3.89543 21 3 20.1046 3 19V16M16 21H19C20.1046 21 21 20.1046 21 19V16" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        # Rotate
        'rotate_left': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M2.5 2V9H9.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M4.51 15C5.15839 17.2161 6.64972 19.1077 8.66455 20.2945C10.6794 21.4814 13.0696 21.8792 15.3576 21.4088C17.6456 20.9383 19.6661 19.6318 21.0281 17.7442C22.3901 15.8566 23.0001 13.5194 22.7501 11.1954C22.5001 8.87138 21.4061 6.71843 19.6781 5.14139C17.9501 3.56434 15.7041 2.66464 13.3501 2.60673C11.0001 2.54882 8.70405 3.33646 6.89405 4.82146L2.5 9" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'rotate_right': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M21.5 2V9H14.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M19.49 15C18.8416 17.2161 17.3503 19.1077 15.3355 20.2945C13.3206 21.4814 10.9304 21.8792 8.64243 21.4088C6.35443 20.9383 4.33393 19.6318 2.97193 17.7442C1.60993 15.8566 0.999927 13.5194 1.24993 11.1954C1.49993 8.87138 2.59393 6.71843 4.32193 5.14139C6.04993 3.56434 8.29593 2.66464 10.6499 2.60673C12.9999 2.54882 15.296 3.33646 17.106 4.82146L21.5 9" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'fullscreen': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M8 3H5C3.89543 3 3 3.89543 3 5V8M21 8V5C21 3.89543 20.1046 3 19 3H16M16 21H19C20.1046 21 21 20.1046 21 19V16M3 16V19C3 20.1046 3.89543 21 5 21H8" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        # Annotation tools
        'highlight': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 20H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M16.5 3.5C16.8978 3.10217 17.4374 2.87868 18 2.87868C18.2786 2.87868 18.5544 2.93355 18.8118 3.04015C19.0692 3.14674 19.303 3.30301 19.5 3.5C19.697 3.69698 19.8532 3.93083 19.9598 4.18819C20.0665 4.44556 20.1213 4.72141 20.1213 5C20.1213 5.27859 20.0665 5.55444 19.9598 5.81181C19.8532 6.06917 19.697 6.30301 19.5 6.5L7 19L3 20L4 16L16.5 3.5Z" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'rectangle': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="3" y="3" width="18" height="18" rx="2" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'circle': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'text': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M4 7V4H20V7" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M9 20H15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M12 4V20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'pen': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M17 3C17.2626 2.73735 17.5744 2.52901 17.9176 2.38687C18.2608 2.24473 18.6286 2.17157 19 2.17157C19.3714 2.17157 19.7392 2.24473 20.0824 2.38687C20.4256 2.52901 20.7374 2.73735 21 3C21.2626 3.26264 21.471 3.57444 21.6131 3.9176C21.7553 4.26077 21.8284 4.62856 21.8284 5C21.8284 5.37143 21.7553 5.73923 21.6131 6.08239C21.471 6.42555 21.2626 6.73735 21 7L7.5 20.5L2 22L3.5 16.5L17 3Z" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'line': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 19L19 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'arrow': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 12H19M19 12L12 5M19 12L12 19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'erase': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M20 20H7L3 16L12 7L17 12M11 11L13 13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'select_text': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M17 3H21V7M7 3H3V7M7 21H3V17M17 21H21V17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M9 9H15M9 12H15M9 15H13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'clear': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M15 9L9 15M9 9L15 15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'trash': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M3 6H5H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'search': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="11" cy="11" r="8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M21 21L16.65 16.65" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        ''',
        
        'palette': '''
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 2C6.49 2 2 6.49 2 12C2 17.51 6.49 22 12 22C13.01 22 14 21.01 14 20C14 19.5 13.82 19.04 13.52 18.7C13.23 18.37 13.05 17.93 13.05 17.45C13.05 16.65 13.68 16 14.5 16H16C19.31 16 22 13.31 22 10C22 5.58 17.51 2 12 2Z" 
                      stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <circle cx="6.5" cy="11.5" r="1.5" fill="currentColor"/>
                <circle cx="9.5" cy="7.5" r="1.5" fill="currentColor"/>
                <circle cx="14.5" cy="7.5" r="1.5" fill="currentColor"/>
                <circle cx="17.5" cy="11.5" r="1.5" fill="currentColor"/>
            </svg>
        ''',
    }
    
    @staticmethod
    def get_icon(name, color="#FFFFFF", size=24):
        """
        Get QIcon from SVG
        
        Args:
            name: Icon name from ICONS dict
            color: Color for the icon (hex or color name)
            size: Icon size in pixels
            
        Returns:
            QIcon object
        """
        if name not in IconManager.ICONS:
            return QIcon()
        
        # Get SVG content and replace color
        svg_content = IconManager.ICONS[name]
        svg_content = svg_content.replace('currentColor', color)
        
        # Create QIcon from SVG
        svg_bytes = QByteArray(svg_content.encode('utf-8'))
        renderer = QSvgRenderer(svg_bytes)
        
        pixmap = QPixmap(size, size)
        pixmap.fill(QColor(0, 0, 0, 0))  # Transparent background
        
        painter = QPainter(pixmap)
        renderer.render(painter)
        painter.end()
        
        return QIcon(pixmap)
    
    @staticmethod
    def get_colored_icon(name, color, size=24):
        """
        Get colored icon (convenience method)
        
        Args:
            name: Icon name
            color: QColor object or hex string
            size: Icon size
            
        Returns:
            QIcon object
        """
        if isinstance(color, QColor):
            color_str = color.name()
        else:
            color_str = color
        
        return IconManager.get_icon(name, color_str, size)
    
    @staticmethod
    def get_available_icons():
        """Get list of available icon names"""
        return list(IconManager.ICONS.keys())

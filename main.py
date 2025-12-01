"""
DocLens - PDF Viewer Application using PySide6 and PyMuPDF
Main entry point
"""

import sys
import os
import json
from pathlib import Path
from PySide6.QtWidgets import QApplication
from PySide6.QtCore import Qt
from main_window import MainWindow


def get_app_dir():
    """Get application directory (where exe is located)"""
    if getattr(sys, 'frozen', False):
        # Running as compiled executable
        return Path(sys.executable).parent
    else:
        # Running as script
        return Path(__file__).parent


def load_settings():
    """Load application settings"""
    # Create config folder if not exists
    config_dir = get_app_dir() / "config"
    config_dir.mkdir(exist_ok=True)
    
    settings_path = config_dir / "settings.json"
    if settings_path.exists():
        try:
            with open(settings_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        except:
            pass
    return {}


def save_settings(settings):
    """Save application settings"""
    # Create config folder if not exists
    config_dir = get_app_dir() / "config"
    config_dir.mkdir(exist_ok=True)
    
    settings_path = config_dir / "settings.json"
    try:
        with open(settings_path, 'w', encoding='utf-8') as f:
            json.dump(settings, f, indent=2, ensure_ascii=False)
    except:
        pass


def main():
    # Enable high DPI scaling
    QApplication.setHighDpiScaleFactorRoundingPolicy(
        Qt.HighDpiScaleFactorRoundingPolicy.PassThrough
    )
    QApplication.setAttribute(Qt.AA_EnableHighDpiScaling)
    QApplication.setAttribute(Qt.AA_UseHighDpiPixmaps)
    
    app = QApplication(sys.argv)
    app.setApplicationName("DocLens")
    app.setOrganizationName("DocLens")
    
    # Set application icon
    icon_path = Path(__file__).parent / "icon" / "icon.ico"
    if icon_path.exists():
        from PySide6.QtGui import QIcon
        app.setWindowIcon(QIcon(str(icon_path)))
    
    # Load settings
    settings = load_settings()
    
    # Create main window
    window = MainWindow(settings)
    window.show()
    
    # Restore last file if exists
    if settings.get('last_file') and os.path.exists(settings.get('last_file')):
        window.open_file(settings['last_file'])
        if 'last_page' in settings:
            window.goto_page(settings['last_page'])
        if 'last_zoom' in settings:
            window.set_zoom(settings['last_zoom'])
    
    # Run application
    result = app.exec()
    
    # Save settings on exit
    save_settings(window.get_settings())
    
    sys.exit(result)


if __name__ == '__main__':
    main()

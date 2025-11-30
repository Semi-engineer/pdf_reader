"""
Simple test to verify PyMuPDF and PySide6 rendering works
"""

import sys
import fitz
from PySide6.QtWidgets import QApplication, QMainWindow, QLabel, QScrollArea, QVBoxLayout, QWidget
from PySide6.QtGui import QPixmap, QImage
from PySide6.QtCore import Qt

app = QApplication(sys.argv)

# Open a PDF
doc = fitz.open("test.pdf")  # Replace with your PDF path
page = doc[0]

# Render to pixmap
mat = fitz.Matrix(1.0, 1.0)
pix = page.get_pixmap(matrix=mat, alpha=False)

# Convert to QImage
img_format = QImage.Format_RGB888 if pix.n == 3 else QImage.Format_RGBA8888
qimg = QImage(pix.samples, pix.width, pix.height, pix.stride, img_format)

# Convert to QPixmap
pixmap = QPixmap.fromImage(qimg.copy())

# Display in QLabel
window = QMainWindow()
label = QLabel()
label.setPixmap(pixmap)

scroll = QScrollArea()
scroll.setWidget(label)
window.setCentralWidget(scroll)
window.setGeometry(100, 100, 800, 600)
window.show()

doc.close()

sys.exit(app.exec())

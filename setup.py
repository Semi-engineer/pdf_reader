"""
Setup script for DocLens application
"""

from setuptools import setup, find_packages
from pathlib import Path

# Read README
readme_path = Path(__file__).parent / "README.md"
long_description = readme_path.read_text(encoding="utf-8") if readme_path.exists() else ""

# Read requirements
requirements_path = Path(__file__).parent / "requirements.txt"
requirements = []
if requirements_path.exists():
    requirements = [
        line.strip() 
        for line in requirements_path.read_text(encoding="utf-8").splitlines() 
        if line.strip() and not line.startswith("#")
    ]

setup(
    name="doclens",
    version="1.0.0",
    author="Your Name",
    author_email="your.email@example.com",
    description="DocLens - A feature-rich PDF viewer built with PySide6 and PyMuPDF",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/yourusername/doclens",
    py_modules=[
        "main",
        "main_window",
        "pdf_drawing_widget",
        "pdf_label_with_overlay",
        "pdf_page_widget",
        "page_cache",
        "render_worker",
        "annotation_manager",
        "search_manager",
        "thumbnail_manager",
    ],
    install_requires=requirements,
    python_requires=">=3.8",
    entry_points={
        "console_scripts": [
            "doclens=main:main",
        ],
    },
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: End Users/Desktop",
        "Topic :: Office/Business",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
    ],
    keywords="doclens pdf viewer pyside6 pymupdf",
)
